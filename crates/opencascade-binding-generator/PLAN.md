# OCCT Binding Generator Plan

A Rust tool using libclang to parse OCCT headers and generate CXX bridge code, organized into per-module Rust files with type aliasing for cross-module references.

## Status: Core Implementation Complete ✅

The binding generator is functional and can parse OCCT headers and generate CXX bridge code.
See **[TRANSITION_PLAN.md](./TRANSITION_PLAN.md)** for the plan to migrate `opencascade-sys` to use generated bindings.

## TODO

- [ ] System include paths: Hardcoding include paths in the binding generator is ugly.
  - Can we run `clang -E -v -x c++ /dev/null` to get system includes?
  - Or use libclang's built-in include path detection?
- [x] `const char*` parameters: CXX can't accept `&str` directly from C++. Need C++ wrappers that:
  - Accept `rust::Str` on the C++ side
  - Convert to `const char*` via `std::string(str).c_str()`
  - Affected: `BRepTools::Write`, `BRepTools::Read` (file path params), `IGESControl_Writer` constructor
- [x] `const char*` return types: These are usually debug/type-info methods.
  - C++ wrappers return `rust::String` (owned, requires copy)
  - Rust side maps `const char*` return → `String`

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

## Open Questions

### 1. Handle Inheritance?

OCCT uses inheritance heavily (e.g., `TopoDS_Edge` extends `TopoDS_Shape`).

**When helpers are needed:**
- **Upcasts (subclass → superclass):** Required to pass a subclass instance to a function expecting a superclass reference. CXX sees them as unrelated opaque types.
- **Downcasts (superclass → subclass):** Required when retrieving from containers or iterators that return the base type.
- **Inherited methods:** NOT needed - can declare inherited methods directly on subclass in CXX bridge.

**Detection strategy for automatic generation:**

*Upcasts:* Scan all bound functions for parameters taking superclass references (e.g., `const TopoDS_Shape&`). For each such superclass, generate upcast helpers from all its known subclasses:
```cpp
// Generated when we see functions taking TopoDS_Shape&
inline const TopoDS_Shape& TopoDS_Edge_as_Shape(const TopoDS_Edge& edge) { return edge; }
inline const TopoDS_Shape& TopoDS_Face_as_Shape(const TopoDS_Face& face) { return face; }
```

*Downcasts:* Scan for container types or iterators that return superclass references (e.g., `TopExp_Explorer` yields `TopoDS_Shape`). Generate downcast helpers to known subclasses:
```cpp
// Generated when we see iterators/containers returning TopoDS_Shape
inline const TopoDS_Edge& TopoDS_Shape_to_Edge(const TopoDS_Shape& shape) { 
    return TopoDS::Edge(shape);  // OCCT's safe downcast
}
```

Options:
- **A)** Generate upcast/downcast helpers automatically based on detection
- **B)** Defer to manual binding for now
- **Recommendation:** (A) - Generate automatically based on usage detection

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
