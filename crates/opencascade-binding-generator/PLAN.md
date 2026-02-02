# OCCT Binding Generator Plan

A Rust tool using libclang to parse OCCT headers and generate CXX bridge code, organized into per-module Rust files with type aliasing for cross-module references.

## Status: Core Implementation Complete ✅

The binding generator is functional and can parse OCCT headers and generate CXX bridge code.
See **[TRANSITION_PLAN.md](./TRANSITION_PLAN.md)** for the plan to migrate `opencascade-sys` to use generated bindings.

## TODO

- [ ] System include paths: Hardcoding include paths in the binding generator is ugly.
  - Can we run `clang -E -v -x c++ /dev/null` to get system includes?
  - Or use libclang's built-in include path detection?

## Methods Skipped Due to CXX/OCCT Limitations

The following patterns cause methods to be intentionally skipped during binding generation:

1. **Methods using enums** - CXX requires `enum class` (C++11 scoped enums), but OCCT uses traditional unscoped enums. Methods with enum parameters or returns are skipped.

2. **Methods with ambiguous lifetimes** - CXX cannot handle methods returning `Pin<&mut Self>` when there are also reference parameters. The lifetime of the returned reference is ambiguous. These are skipped.

3. **Abstract class constructors** - Abstract classes cannot be instantiated, so constructor wrappers and `to_handle()` functions are not generated.

4. **Methods with by-value Handle parameters** - CXX cannot pass `Handle<T>` by value across the FFI boundary. Methods taking `Handle<T>` parameters (not `const Handle<T>&`) are skipped.

5. **Methods with by-value class parameters** - Similar to Handles, methods taking class types by value (not by reference) are skipped.

6. **Classes with protected destructors** - These are excluded from CXX type declarations entirely since CXX auto-generates destructor code.

7. **Inherited methods with signature mismatches** - When a method pointer's declaring class differs from the binding class, C++ compilation fails. Inherited method generation is disabled.

8. **Const/mut return mismatches** - Methods returning `const T&` when the method is non-const are problematic for CXX's ownership model and are skipped.

### Key Insight: Filter Consistency

When any method is filtered out of FFI generation, it MUST also be filtered out of impl generation. Otherwise the impl will try to call a non-existent FFI function. All filter functions (like `method_uses_enum()`, `needs_explicit_lifetimes()`, etc.) must be applied in both `generate_*_ffi()` and `generate_*_impls()` functions.

- [x] **Inherited methods**: Many OCCT classes inherit from base classes but the generator only
  emits methods declared directly on each class, not inherited methods.
  - **Problem**: Classes like `BRepOffsetAPI_ThruSections`, `BRepBuilderAPI_MakeFace`, etc.
    inherit from `BRepBuilderAPI_MakeShape` which provides `Shape()`, `IsDone()`, and `Build()`.
    Without these methods, the generated bindings are incomplete.
  - **Solution implemented**:
    1. The parser tracks `base_classes` for each `ParsedClass`
    2. After parsing all classes, build an inheritance map
    3. For each class, collect methods from all ancestor classes
    4. When generating CXX bindings, emit inherited methods with the derived class as `self` type
    5. CXX allows calling inherited methods on derived types directly
  - **Protected destructor handling**: Abstract base classes like `BRepAlgoAPI_Algo` often have
    protected destructors to prevent direct instantiation. CXX generates destructor code for any
    declared type, so these classes must be completely excluded from bindings:
    1. Parser detects protected destructors via libclang's `EntityKind::Destructor` + accessibility check
    2. Classes with `has_protected_destructor = true` are excluded from CXX type declarations
    3. Upcast methods filter out base classes with protected destructors
    4. Concrete derived classes (e.g., `BRepAlgoAPI_Common`) are still generated normally
- [x] `const char*` parameters: CXX can't accept `&str` directly from C++. Need C++ wrappers that:
  - Accept `rust::Str` on the C++ side
  - Convert to `const char*` via `std::string(str).c_str()`
  - Affected: `BRepTools::Write`, `BRepTools::Read` (file path params), `IGESControl_Writer` constructor
- [x] `const char*` return types: These are usually debug/type-info methods.
  - C++ wrappers return `rust::String` (owned, requires copy)
  - Rust side maps `const char*` return → `String`
- [x] Enum handling: Generate CXX shared enums with Rust-style names.
  - Enum names use short form: `TopAbs_ShapeEnum` → `ShapeEnum` (with `#[cxx_name]`)
  - Variant names use PascalCase: `TopAbs_COMPOUND` → `Compound` (with `#[cxx_name]`)
  - Enums are publicly re-exported from each module
  - Cross-module type aliases reference the short Rust name

## Goals

1. ✅ Parse OCCT C++ headers and extract class/function declarations
2. ✅ Generate `#[cxx::bridge]` modules per OCCT module (gp, TopoDS, BRep, etc.)
3. ✅ Generate C++ wrapper functions where needed (constructors, return-by-value, static methods)
4. ✅ Use CXX's type aliasing (`type Foo = crate::other::ffi::Foo`) for cross-module references
5. ✅ Preserve doc comments from C++ headers

## Architecture

```
crates/opencascade-binding-generator/
├── Cargo.toml
├── src/
│   ├── main.rs           # CLI entry point
│   ├── parser.rs         # libclang-based header parser
│   ├── model.rs          # IR for parsed declarations
│   ├── module_graph.rs   # Module dependency analysis
│   ├── codegen/
│   │   ├── mod.rs
│   │   ├── rust.rs       # Generate Rust #[cxx::bridge] code
│   │   └── cpp.rs        # Generate C++ wrapper code
│   └── type_mapping.rs   # OCCT type → Rust type mappings
```

## Implementation Steps

### Step 1: Create Crate Structure

Create `crates/opencascade-binding-generator/Cargo.toml` with dependencies:
- `clang` - libclang wrapper for C++ parsing
- `clap` - CLI argument parsing
- `quote`, `proc-macro2` - Rust code generation
- `heck` - Case conversion (snake_case, etc.)

### Step 2: Header Parser (`parser.rs`)

Use libclang to extract from OCCT headers:

```rust
pub struct ParsedClass {
    pub name: String,              // e.g., "gp_Pnt"
    pub module: String,            // e.g., "gp" (derived from prefix)
    pub comment: Option<String>,
    pub constructors: Vec<Constructor>,
    pub methods: Vec<Method>,
    pub static_methods: Vec<StaticMethod>,
    pub is_handle_type: bool,      // Has DEFINE_STANDARD_HANDLE
}

pub struct Method {
    pub name: String,
    pub comment: Option<String>,
    pub is_const: bool,            // Determines &self vs Pin<&mut self>
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
}
```

Key parsing logic:
- Walk AST looking for `ClassDecl`, `StructDecl`
- Extract methods via `Method` child entities
- Detect `const` qualifier for method binding style
- Resolve `Standard_Real` → `double` → `f64` via canonical types
- Extract comments via `entity.get_comment()`

### Step 3: Module Dependency Graph (`module_graph.rs`)

Analyze which types each class references:

```rust
pub struct ModuleGraph {
    pub modules: HashMap<String, Module>,
}

pub struct Module {
    pub name: String,                           // "gp"
    pub types: Vec<String>,                     // ["gp_Pnt", "gp_Vec", ...]
    pub dependencies: HashSet<String>,          // Other modules referenced
}
```

Extract module from type name:
- `gp_Pnt` → module `gp`
- `TopoDS_Shape` → module `TopoDS` (normalize to `topo_ds` for Rust)
- `BRepPrimAPI_MakeBox` → module `BRepPrimAPI` → `brep_prim_api`

### Step 4: Rust Code Generation (`codegen/rust.rs`)

Generate per-module `#[cxx::bridge]`, organized by source header:

```rust
// Generated: src/brep_prim_api/mod.rs

#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("opencascade-sys/include/brep_prim_api.hxx");

        // ========================
        // Cross-module type aliases
        // ========================
        // Note: CXX requires types to be declared in the bridge before use.
        // We use full C++ names here to avoid conflicts between modules.

        /// 3D point - from gp module
        type gp_Pnt = crate::gp::ffi::Pnt;

        /// 2D coordinate system - from gp module
        type gp_Ax2 = crate::gp::ffi::Ax2;

        /// Topological shape - from topo_ds module
        type TopoDS_Shape = crate::topo_ds::ffi::Shape;

        /// Topological shell - from topo_ds module
        type TopoDS_Shell = crate::topo_ds::ffi::Shell;

        /// Topological solid - from topo_ds module
        type TopoDS_Solid = crate::topo_ds::ffi::Solid;

        /// Topological face - from topo_ds module
        type TopoDS_Face = crate::topo_ds::ffi::Face;

        // ========================
        // BRepPrimAPI_MakeBox.hxx
        // ========================

        /// Builds parallelepiped box shapes (BRepPrimAPI_MakeBox)
        #[cxx_name = "BRepPrimAPI_MakeBox"]
        type MakeBox;

        /// Make a box with a corner at 0,0,0 and the other dx,dy,dz
        #[cxx_name = "BRepPrimAPI_MakeBox_ctor_dims"]
        fn MakeBox_from_dims(dx: f64, dy: f64, dz: f64) -> UniquePtr<MakeBox>;

        /// Make a box with a corner at P and size dx, dy, dz.
        #[cxx_name = "BRepPrimAPI_MakeBox_ctor_point_dims"]
        fn MakeBox_from_point_dims(p: &gp_Pnt, dx: f64, dy: f64, dz: f64) -> UniquePtr<MakeBox>;

        /// Make a box with corners P1, P2.
        #[cxx_name = "BRepPrimAPI_MakeBox_ctor_points"]
        fn MakeBox_from_points(p1: &gp_Pnt, p2: &gp_Pnt) -> UniquePtr<MakeBox>;

        /// Make a box with Ax2 (the left corner and the axis) and size dx, dy, dz.
        #[cxx_name = "BRepPrimAPI_MakeBox_ctor_ax2_dims"]
        fn MakeBox_from_ax2_dims(axes: &gp_Ax2, dx: f64, dy: f64, dz: f64) -> UniquePtr<MakeBox>;

        /// Returns the constructed box as a shell.
        #[cxx_name = "Shell"]
        fn shell(self: &MakeBox) -> &TopoDS_Shell;

        /// Returns the constructed box as a solid.
        #[cxx_name = "Solid"]
        fn solid(self: &MakeBox) -> &TopoDS_Solid;

        /// Returns ZMin face
        #[cxx_name = "BottomFace"]
        fn bottom_face(self: &MakeBox) -> &TopoDS_Face;

        /// Returns ZMax face
        #[cxx_name = "TopFace"]
        fn top_face(self: &MakeBox) -> &TopoDS_Face;

        /// Returns XMin face
        #[cxx_name = "BackFace"]
        fn back_face(self: &MakeBox) -> &TopoDS_Face;

        /// Returns XMax face
        #[cxx_name = "FrontFace"]
        fn front_face(self: &MakeBox) -> &TopoDS_Face;

        /// Returns YMin face
        #[cxx_name = "LeftFace"]
        fn left_face(self: &MakeBox) -> &TopoDS_Face;

        /// Returns YMax face
        #[cxx_name = "RightFace"]
        fn right_face(self: &MakeBox) -> &TopoDS_Face;
    }

    // Request UniquePtr instantiation for types defined in other modules
    impl UniquePtr<MakeBox> {}
}

// Re-export types
pub use ffi::MakeBox;

// Impl block provides constructor methods as associated functions
impl MakeBox {
    /// Make a box with a corner at 0,0,0 and the other dx,dy,dz
    pub fn new_real3(dx: f64, dy: f64, dz: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeBox_ctor_real3(dx, dy, dz)
    }

    /// Make a box with a corner at P and size dx, dy, dz.
    pub fn new_pnt_real3(p: &Pnt, dx: f64, dy: f64, dz: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeBox_ctor_pnt_real3(p, dx, dy, dz)
    }

    /// Make a box with corners P1, P2.
    pub fn new_pnt2(p1: &Pnt, p2: &Pnt) -> cxx::UniquePtr<Self> {
        ffi::MakeBox_ctor_pnt2(p1, p2)
    }

    /// Make a box with Ax2 (the left corner and the axis) and size dx, dy, dz.
    pub fn new_ax2_real3(axes: &Ax2, dx: f64, dy: f64, dz: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeBox_ctor_ax2_real3(axes, dx, dy, dz)
    }
}
```

Key conventions:
- **Cross-module types first** with `type Foo = crate::other::ffi::Foo` aliasing
- **Organize by header file** with clear section comments
- **Strip module prefix** from type names: `BRepPrimAPI_MakeBox` → `MakeBox`
- **Use `#[cxx_name = "..."]`** to map Rust name to C++ name
- **Doc comment includes original C++ name** for searchability
- **Method names use snake_case** while preserving C++ name via `#[cxx_name]`
- **Explicit `impl UniquePtr<T> {}`** for types from other modules
- **Constructor methods** via impl blocks outside ffi: `Type::new()`, `Type::new_pnt_real3()`
- **Compressed overload suffixes**: consecutive identical types combine (e.g., `_real3` not `_real_real_real`, `_pnt2` not `_pnt_pnt`)

#### Impl Block Generation

The `ffi` module is declared `pub(crate)` to keep raw FFI bindings internal. To provide ergonomic public access, impl blocks are generated outside the ffi module with public methods that wrap the FFI functions.

**Three categories of methods in impl blocks:**

1. **Constructors** - Associated functions like `Type::new()`, `Type::new_pnt_real3()` that call FFI constructor functions
   - Overloads use parameter-based suffixes: `new()`, `new_pnt_real3()`, `new_ax2_real3()`

2. **Wrapper methods for by-value returns** - Instance methods where C++ returns a class type by value
   - CXX can't directly bind these; C++ wrapper functions are generated that return `std::unique_ptr<T>`
   - FFI functions like `Box__corner_min(self_: &Box_)` are exposed as `Box_::corner_min(&self)`
   - Determined by `needs_wrapper_function()` - returns true for methods returning class types by value

3. **Static methods** - Class static methods become associated functions
   - FFI functions like `BRepBndLib_add(S, B, useTriangulation)` become `BRepBndLib::add(S, B, useTriangulation)`
   - Parameters requiring `Pin<&mut T>` are passed through directly
   - Cross-module enum types use full crate paths: `crate::top_abs::ShapeEnum`

**Type handling in impl blocks:**
- Class parameters use `&ffi::Type` or `std::pin::Pin<&mut ffi::Type>` depending on mutability
- Mutable references to class types always use `Pin<&mut T>` to satisfy CXX requirements
- Return types for classes use `cxx::UniquePtr<ffi::Type>`
- Enums from the same module use `ffi::EnumName`
- Enums from other modules use `crate::module_name::EnumName`
- **`const char*` parameters**: mapped to `&str` in Rust; C++ wrappers convert `rust::Str` to `const char*`
- **`const char*` returns**: mapped to `String` in Rust; C++ wrappers convert `const char*` to `rust::String`

### Step 5: C++ Wrapper Generation (`codegen/cpp.rs`)

Generate wrappers for patterns CXX can't handle directly:

```cpp
// Generated: include/wrappers/gp_generated.hxx
#pragma once
#include <memory>
#include <gp_Pnt.hxx>
#include <gp_Vec.hxx>
// ...

// Return-by-value methods need wrappers
inline std::unique_ptr<gp_Pnt> gp_Pnt_Mirrored(const gp_Pnt& self, const gp_Pnt& point) {
    return std::make_unique<gp_Pnt>(self.Mirrored(point));
}

// Static methods
inline const gp_Ax1& gp_OX() {
    return gp::OX();
}
```

### Step 6: Type Mapping (`type_mapping.rs`)

```rust
pub fn map_cpp_type(cpp_type: &str) -> RustType {
    match cpp_type {
        "Standard_Real" | "double" => RustType::Primitive("f64"),
        "Standard_Integer" | "int" => RustType::Primitive("i32"),
        "Standard_Boolean" | "bool" => RustType::Primitive("bool"),
        "void" => RustType::Unit,
        
        // Reference types
        t if t.starts_with("const ") && t.ends_with("&") => {
            let inner = &t[6..t.len()-1].trim();
            RustType::Ref(Box::new(map_cpp_type(inner)))
        }
        
        // Handle types
        t if t.starts_with("Handle(") => {
            let inner = &t[7..t.len()-1];
            RustType::Handle(inner.to_string())
        }
        
        // OCCT class types
        t => RustType::OcctType(t.to_string()),
    }
}
```

### Step 7: CLI Interface (`main.rs`)

```rust
#[derive(Parser)]
struct Args {
    /// OCCT headers to process
    #[arg(required = true)]
    headers: Vec<PathBuf>,

    /// OCCT include directory
    #[arg(short = 'I', long)]
    include_dir: PathBuf,

    /// Output directory for generated code
    #[arg(short, long, default_value = ".")]
    output: PathBuf,

    /// Only generate for specific module
    #[arg(long)]
    module: Option<String>,
}
```

Usage:
```bash
cargo run -p opencascade-binding-generator -- \
    -I ../occt-sys/OCCT/src \
    -o ../opencascade-sys \
    ../occt-sys/OCCT/src/gp/gp_Pnt.hxx \
    ../occt-sys/OCCT/src/gp/gp_Vec.hxx \
    ../occt-sys/OCCT/src/gp/gp_Dir.hxx
```

## Patterns Requiring Wrappers

| C++ Pattern | Problem | Generated Wrapper |
|-------------|---------|-------------------|
| Constructor | CXX can't return `T` | Use `construct_unique<T>` template |
| Return by value | CXX needs `UniquePtr<T>` | `make_unique<T>(obj.Method())` |
| Static method | No `self` | Free function wrapper |
| `Handle<T>` | Template | `typedef opencascade::handle<T> HandleT;` |
| Overloaded method | Name collision | Suffix: `Method_1`, `Method_2` or `Method_ParamType` |
| Output params | `void Foo(T& out)` | Return struct or tuple |

## Enum Handling

OCCT enums are translated to CXX shared enums with Rust-style naming conventions:

```rust
// Generated for TopAbs_ShapeEnum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cxx_name = "TopAbs_ShapeEnum"]
enum ShapeEnum {
    #[cxx_name = "TopAbs_COMPOUND"]
    Compound,
    #[cxx_name = "TopAbs_COMPSOLID"]
    Compsolid,
    #[cxx_name = "TopAbs_SOLID"]
    Solid,
    // ...
}

// Re-exported publicly from the module
pub use ffi::{Orientation, ShapeEnum};
```

**Naming conventions:**
- **Enum type name:** Strip module prefix, keep suffix → `TopAbs_ShapeEnum` → `ShapeEnum`
- **Variant names:** Strip prefix, convert SCREAMING_CASE to PascalCase → `TopAbs_COMPOUND` → `Compound`
- **`#[cxx_name]` attributes:** Preserve original C++ names for FFI compatibility

**Cross-module references:** When an enum from another module is used, a type alias is generated:
```rust
// In topo_ds.rs, referencing top_abs enum
type TopAbs_ShapeEnum = crate::top_abs::ffi::ShapeEnum;
```

## Open Questions

### 1. Handle Inheritance? ✅ (Upcasts Implemented)

OCCT uses inheritance heavily (e.g., `TopoDS_Edge` extends `TopoDS_Shape`).

**When helpers are needed:**
- **Upcasts (subclass → superclass):** Required to pass a subclass instance to a function expecting a superclass reference. CXX sees them as unrelated opaque types.
- **Downcasts (superclass → subclass):** Required when retrieving from containers or iterators that return the base type.
- **Inherited methods:** NOT needed - can declare inherited methods directly on subclass in CXX bridge.

**Implementation (Upcasts - DONE):**

The generator now parses base classes from C++ headers using libclang's `BaseSpecifier` entity kind. For each class with base classes in our known set, it generates:

```cpp
// C++ wrapper (e.g., in wrapper_topo_ds.hxx):
inline const TopoDS_Shape& TopoDS_Edge_as_TopoDS_Shape(const TopoDS_Edge& self) { return self; }
inline const TopoDS_Shape& TopoDS_Face_as_TopoDS_Shape(const TopoDS_Face& self) { return self; }
```

```rust
// Rust bridge (e.g., in topo_ds.rs ffi module):
#[doc = "Upcast TopoDS_Edge to TopoDS_Shape"]
#[cxx_name = "TopoDS_Edge_as_TopoDS_Shape"]
fn Edge_as_shape(self_: &Edge) -> &Shape;

#[doc = "Upcast TopoDS_Face to TopoDS_Shape"]
#[cxx_name = "TopoDS_Face_as_TopoDS_Shape"]
fn Face_as_shape(self_: &Face) -> &Shape;

// Public API via impl block (outside ffi):
impl Edge {
    pub fn as_shape(&self) -> &Shape {
        ffi::Edge_as_shape(self)
    }
}

impl Face {
    pub fn as_shape(&self) -> &Shape {
        ffi::Face_as_shape(self)
    }
}
```

**Key implementation details:**
- Base classes are extracted from `EntityKind::BaseSpecifier` in libclang
- Only base classes that are in our known set of parsed classes are included (e.g., `XSControl_Reader` is skipped if not parsed)
- Rust function names use snake_case: `face_as_shape`, `edge_as_shape`
- Return types use short names for same-module bases, full C++ names for cross-module

**Implementation (Downcasts - DONE):**

TopoDS downcasts are implemented using OCCT's `TopoDS::Edge()`, `TopoDS::Face()`, etc. static methods. These require the `<TopoDS.hxx>` header which provides safe casting that checks shape type at runtime.

```cpp
// C++ wrapper (in wrapper_topo_ds.hxx):
#include <TopoDS.hxx>

inline TopoDS_Edge TopoDS_Shape_to_TopoDS_Edge(const TopoDS_Shape& shape) { 
    return TopoDS::Edge(shape); 
}
inline TopoDS_Face TopoDS_Shape_to_TopoDS_Face(const TopoDS_Shape& shape) { 
    return TopoDS::Face(shape); 
}
```

```rust
// Rust bridge (in topo_ds.rs):
#[doc = "Downcast TopoDS_Shape to TopoDS_Edge"]
#[cxx_name = "TopoDS_Shape_to_TopoDS_Edge"]
fn shape_to_edge(shape: &Shape) -> UniquePtr<Edge>;

#[doc = "Downcast TopoDS_Shape to TopoDS_Face"]
#[cxx_name = "TopoDS_Shape_to_TopoDS_Face"]
fn shape_to_face(shape: &Shape) -> UniquePtr<Face>;
```

**Key implementation details:**
- Downcasts are TopoDS-specific (not general inheritance) because OCCT provides safe casting only for shape types
- All TopoDS subtype names are hardcoded: Vertex, Edge, Wire, Face, Shell, Solid, CompSolid, Compound
- Functions are re-exported from the module for public access (`pub use ffi::shape_to_edge;`)
- Returns `UniquePtr<T>` because the C++ function returns by value

### 2. Method Overload Disambiguation?

Options:
- **A)** Suffix with parameter types: `UVBounds_Face`, `UVBounds_Face_Wire`
- **B)** Numeric suffix: `UVBounds_1`, `UVBounds_2`
- **C)** Skip overloads, require manual binding
- **Recommendation:** (A) - Suffix with parameter types for clarity. Handle edge cases with (B) if needed.

### 3. Which Methods to Auto-Bind?

Bind methods that appear in OCCT's public API documentation:

- **`Standard_EXPORT` methods** - Explicitly marked as public API
- **Inline methods with doc comments** - Simple getters/setters like `X()`, `Y()`, `Z()` that are documented but not exported (they're defined inline in headers)
- **Public constructors** - Even if not exported, constructors in public section are intended for use

**Detection heuristic:**
1. Has `Standard_EXPORT` annotation, OR
2. Is in `public:` section AND has a Doxygen comment (`//!` or `///`)

Options:
- **A)** All public methods
- **B)** Only `Standard_EXPORT` methods  
- **C)** Methods in OCCT docs (Standard_EXPORT + documented inline methods)
- **D)** Config file with allowlist/blocklist
- **Recommendation:** (C) - Match OCCT's documented API. Add (D) for fine-tuning edge cases.

### 4. Incremental vs Full Generation?

- **Full:** Regenerate all modules each run
- **Incremental:** Only update changed modules
- **Recommendation:** Full generation initially, add incremental later

## Output Structure

After running on BRepPrimAPI module headers:

```
crates/opencascade-sys/
├── include/
│   ├── common.hxx                        # Shared utilities (construct_unique, etc.)
│   ├── gp.hxx                            # C++ header for gp module
│   ├── topo_ds.hxx                       # C++ header for topo_ds module
│   └── brep_prim_api.hxx                 # C++ header for brep_prim_api module
├── src/
│   ├── lib.rs                            # pub mod gp; pub mod topo_ds; pub mod brep_prim_api;
│   ├── gp/
│   │   └── mod.rs                        # Generated #[cxx::bridge] for gp
│   ├── topo_ds/
│   │   └── mod.rs                        # Generated #[cxx::bridge] for topo_ds
│   └── brep_prim_api/
│       └── mod.rs                        # Generated #[cxx::bridge] for brep_prim_api
```

Each module has its own C++ header that includes only what it needs:

```cpp
// include/common.hxx - shared utilities
#pragma once
#include "rust/cxx.h"
#include <memory>

template <typename T, typename... Args> 
std::unique_ptr<T> construct_unique(Args... args) {
    return std::unique_ptr<T>(new T(args...));
}
```

```cpp
// include/gp.hxx - gp module (no dependencies)
#pragma once
#include "common.hxx"
#include <gp_Pnt.hxx>
#include <gp_Vec.hxx>
#include <gp_Dir.hxx>
#include <gp_Ax1.hxx>
#include <gp_Ax2.hxx>
// ... other gp headers

// Generated wrappers for gp module
inline std::unique_ptr<gp_Pnt> gp_Pnt_Mirrored(const gp_Pnt& self, const gp_Pnt& point) {
    return std::make_unique<gp_Pnt>(self.Mirrored(point));
}
// ...
```

```cpp
// include/brep_prim_api.hxx - depends on gp, topo_ds
#pragma once
#include "common.hxx"
#include <gp_Pnt.hxx>          // Cross-module dependency
#include <gp_Ax2.hxx>          // Cross-module dependency  
#include <TopoDS_Shape.hxx>    // Cross-module dependency
#include <TopoDS_Shell.hxx>    // Cross-module dependency
#include <TopoDS_Solid.hxx>    // Cross-module dependency
#include <BRepPrimAPI_MakeBox.hxx>
// ... other BRepPrimAPI headers

// Generated wrappers for brep_prim_api module
inline std::unique_ptr<BRepPrimAPI_MakeBox> BRepPrimAPI_MakeBox_ctor_dims(
    Standard_Real dx, Standard_Real dy, Standard_Real dz) {
    return construct_unique<BRepPrimAPI_MakeBox>(dx, dy, dz);
}
// ...
```

**Benefits of per-module includes:**
- Each Rust module only parses the C++ headers it needs
- Faster incremental compilation when one module changes
- Clearer dependency tracking

## Testing Strategy

1. **Unit tests** for parser on sample OCCT headers
2. **Snapshot tests** comparing generated code to expected output
3. **Integration test** building generated bindings and calling from Rust
4. **Comparison test** ensuring generated bindings match existing manual bindings

## Future Enhancements

- Config file for per-class/method customization
- Support for callback/function pointer parameters
- Automatic `Send`/`Sync` impl generation for thread-safe types
- Generate higher-level Rust wrappers (not just FFI)
