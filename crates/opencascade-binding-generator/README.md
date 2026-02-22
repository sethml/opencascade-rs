# opencascade-binding-generator

Automatically generates Rust `extern "C"` FFI bindings for the [OpenCASCADE](https://dev.opencascade.org/) C++ CAD kernel.

Given a set of OCCT header files, the generator produces a complete Rust FFI layer: type declarations, method bindings, constructor wrappers, Handle smart pointer support, collection iterators, inheritance upcasts, and per-module re-exports with ergonomic short names.

## Quick Start

```bash
# Regenerate all bindings (from repo root)
./scripts/regenerate-bindings.sh
```

This parses OCCT headers configured in `bindings.toml` (expanding with automatic dependency resolution), and writes generated code to `crates/opencascade-sys/generated/`.

## CLI Usage

```bash
# Standard regeneration (from repo root):
./scripts/regenerate-bindings.sh

# Manual invocation with TOML config:
cargo run -p opencascade-binding-generator -- \
    --config crates/opencascade-sys/bindings.toml \
    -I target/OCCT/include \
    -o crates/opencascade-sys/generated

# Or with explicit header arguments (legacy):
cargo run -p opencascade-binding-generator -- \
    -I target/OCCT/include \
    -o crates/opencascade-sys/generated \
    target/OCCT/include/gp_Pnt.hxx target/OCCT/include/TopoDS_Shape.hxx ...
```

**Flags:**
- `--config <file>` — TOML configuration file specifying headers (recommended)
- `--resolve-deps` — Auto-include header dependencies (default: true)
- `--dump-symbols` — Dump symbol table for debugging
- `--dry-run` — Parse without generating
- `-v, --verbose` — Verbose output
- `--module <name>` — Filter to a specific module

## Generated Public API

Users interact with `opencascade-sys` through per-module re-exports. Each OCCT module (gp, TopoDS, BRepPrimAPI, etc.) becomes a Rust module with short type names and idiomatic method signatures.

### Geometry Primitives

Simple value types like points, vectors, and directions get constructors, accessors, and transformation methods. All methods go through extern "C" C++ wrappers. Methods that return class types by value return `OwnedPtr<T>`.

```rust
use opencascade_sys::gp::{Pnt, Vec, Dir, Ax1, Ax2, Trsf};

// Constructors -- overloads distinguished by parameter type suffix
let origin = Pnt::new();                          // default constructor
let p = Pnt::new_real3(1.0, 2.0, 3.0);           // from coordinates
let p2 = Pnt::new_xyz(&xyz);                      // from XYZ

// Direct accessors
let x: f64 = p.x();
let y: f64 = p.y();
p.set_x(10.0);

// Transformations return OwnedPtr (by-value return needs C++ wrapper)
let mirrored: OwnedPtr<Pnt> = p.mirrored_pnt(&origin);
let rotated = p.rotated(&axis, std::f64::consts::FRAC_PI_2);
let transformed = p.transformed(&trsf);

// Copy into a new OwnedPtr
let p_copy = p.to_owned();
```

### Topology Types and Inheritance

Topological shapes form an inheritance hierarchy: `Shape` is the base, with subtypes `Vertex`, `Edge`, `Wire`, `Face`, `Shell`, `Solid`, `CompSolid`, and `Compound`. Extern "C" FFI treats these as unrelated opaque types, so the generator produces explicit upcast methods (both const and mutable) and downcast free functions.

```rust
use opencascade_sys::topo_ds::{Shape, Edge, Face, Solid, Wire};

// Upcasting -- every subtype can upcast to its ancestors
let edge = Edge::new();
let shape_ref: &Shape = edge.as_shape();                    // const upcast
let shape_mut: &mut Shape = edge.as_shape_mut();            // mutable upcast

// Downcasting -- free functions in the topo_ds module
// (wraps OCCT's TopoDS::Edge(), TopoDS::Face(), etc.)
use opencascade_sys::topo_ds;
let edge: OwnedPtr<Edge> = topo_ds::edge(&some_shape);       // const
let face: OwnedPtr<Face> = topo_ds::face_mut(&mut shape);    // mutable

// Copy semantics (TopoDS shapes are reference-counted internally)
let shape_copy = shape.to_owned();
```

### Primitive Shape Construction (Builder Pattern)

OCCT builder classes follow a pattern: construct with parameters, then extract the result by upcasting to `MakeShape` and calling `shape()`.

```rust
use opencascade_sys::gp::{Pnt, Ax2};
use opencascade_sys::b_rep_prim_api::{MakeBox, MakeSphere, MakeCylinder};

// Overloaded constructors use parameter type suffixes
let mut make_box = MakeBox::new_real3(10.0, 20.0, 30.0);    // dx, dy, dz
let mut make_box = MakeBox::new_pnt_real3(&p, 10.0, 20.0, 30.0); // corner + size
let mut make_box = MakeBox::new_pnt2(&p1, &p2);             // two corners
let mut make_box = MakeBox::new_ax2_real3(&axes, 10.0, 20.0, 30.0); // local coords

// Extract the result via mutable upcast to MakeShape
let make_shape = make_box.as_b_rep_builder_api_make_shape_mut();
let result: &TopoDS_Shape = make_shape.shape();
```

### Boolean Operations

```rust
use opencascade_sys::b_rep_algo_api::{Fuse, Cut, Common};
use opencascade_sys::message::ProgressRange;

let progress = ProgressRange::new();
let mut fuse = Fuse::new_shape2_progressrange(&shape_a, &shape_b, &progress);
let make_shape = fuse.as_b_rep_builder_api_make_shape_mut();
let result = make_shape.shape();
```

### Static Methods

C++ static methods become associated functions on the type. These go through free function wrappers in the extern "C" layer.

```rust
use opencascade_sys::b_rep_bnd_lib::BRepBndLib;
use opencascade_sys::bnd::Box;

let mut bbox = Box::new();
// Static method: BRepBndLib::Add(shape, box, useTriangulation)
BRepBndLib::add(&shape, &mut bbox, true);
```

### Handle Types (Reference-Counted Smart Pointers)

OCCT uses `Handle<T>` (an intrusive reference-counted pointer) for objects inheriting from `Standard_Transient`. The generator produces `to_handle()` to wrap an object, and upcast methods to convert between Handle types in the inheritance hierarchy.

```rust
use opencascade_sys::geom::{BezierCurve, BSplineCurve};

// Wrap a geometry object in a Handle
let curve: OwnedPtr<BezierCurve> = /* ... */;
let handle = BezierCurve::to_handle(curve);
// handle is OwnedPtr<HandleGeomBezierCurve>

// Methods that return Handles
let copy = some_curve.copy();
// copy is OwnedPtr<HandleGeomGeometry>
```

### const char* String Conversions

Methods taking `const char*` accept `&str` in Rust; methods returning `const char*` return `String`. C++ wrappers pass through `const char*` directly.

```rust
use opencascade_sys::standard::Failure;

// const char* return -> String
let msg: String = failure.get_message_string();
```

### Nullable Pointer Parameters

Methods with `T* param = NULL` or `const T* param = NULL` use `Option<&mut T>` or `Option<&T>` in Rust. The C++ wrapper passes the raw pointer through — `NULL` for `None`, the underlying pointer for `Some`.

```rust
use opencascade_sys::bnd::OBB;

// theListOfTolerances is Option<&TColStd_Array1OfReal> (const T* = NULL in C++)
obb.re_build(&points, Some(&tolerances), true);
obb.re_build(&points, None, false);  // pass NULL for tolerances
```

### Non-Nullable Class Pointer Parameters

Methods with non-nullable `const T*` or `T*` parameters (where `T` is a known class type) are bound as `&T` or `&mut T` in Rust. The C++ wrapper passes the raw pointer through. This is safe because these parameters are documented as non-nullable in OCCT. Primitive pointer types (`int*`, `double*`) are NOT bound this way since they typically represent C-style arrays.

```rust
use opencascade_sys::adaptor3d::Surface;

// const Adaptor3d_Surface* in C++ → &Surface in Rust
fn example(surface: &Surface) { /* ... */ }
```

### Fixed-Size Array Reference Parameters

Methods taking fixed-size C++ arrays are now bound directly:

- `const T (&arr)[N]` → `&[T; N]`
- `T (&arr)[N]` → `&mut [T; N]`
- `const T arr[N]` → `&[T; N]` (array parameter decays to pointer in C++)
- `T arr[N]` → `&mut [T; N]` (array parameter decays to pointer in C++)

At the extern boundary, these lower to element pointers (`*const T` / `*mut T`). In C++ wrappers we reconstruct the array reference with `reinterpret_cast` back to `T (*)[N]` and pass `*...` into the original OCCT call.

```rust
use opencascade_sys::b_rep_mesh::MeshTool;

let mut edges = [0_i32; 3];
mesh_tool.add_triangle(p1, p2, p3, &mut edges);
```

This unblocks APIs like `BRepMesh_MeshTool::AddTriangle(Standard_Integer (&theEdges)[3])` without hand-written bindings, and also fixed-size non-reference array parameters declared as `T arr[N]`.


### Class Pointer Returns

Instance methods returning `const T*` or `T*` (where `T` is a known class type) are bound as `Option<&T>` or `Option<&mut T>` in Rust, with a null check. The C++ wrapper passes the raw pointer through — the Rust reexport checks for null and returns `None` or `Some(&ref)`. For const methods (`&self`), `T*` returns are downgraded to `Option<&T>` to avoid unsound `&self -> &mut T`. Static methods and free functions with class pointer returns are NOT bound this way (lifetime issues).

```rust
use opencascade_sys::geom::BSplineCurve;

// const TColStd_Array1OfReal* in C++ → Option<&TColStd_Array1OfReal> in Rust
let weights: Option<&_> = curve.weights();  // None if null, Some(&array) otherwise
```

### Unsafe Reference Returns (Ambiguous Lifetimes)

Some methods and free functions return a reference (`&T` or `&mut T`) while also taking reference parameters. In C++, the returned reference might borrow from `self`, from one of the parameters, or from neither (e.g., a global or class-static). Rust's lifetime elision rules assume the return borrows from `self` for methods, but this assumption may be incorrect — the return could actually borrow from a parameter, which might be dropped before `self`.

The generator marks these methods as `unsafe fn` with a `# Safety` doc comment explaining the ambiguity. This applies to:

- **Instance methods** returning `&T` or `&mut T` that also take reference parameters (other than `self`)
- **Free functions** returning a reference with 2+ reference parameters (no `self` to disambiguate)

For free functions, explicit lifetime annotations (`<'a>`) tie all reference parameters and the return type together, since Rust's elision rules cannot resolve the ambiguity without `self`.

```rust
use opencascade_sys::message::Msg;

// Method: returns &mut Msg, takes &str — could borrow from self or theString
let result: &mut Msg = unsafe { msg.arg_charptr("value") };

// Free function: explicit lifetime <'a> ties parameters and return together
use opencascade_sys::bin_tools;
let stream: &mut OStream = unsafe { bin_tools::put_real(&mut os, &value) };
```

Methods that return references but take no reference parameters (other than `self`) are **safe** — Rust's lifetime elision correctly binds the return to `self`.

### Collection Iterators

OCCT collection types (NCollection_List, NCollection_Sequence, NCollection_IndexedMap, etc.) get Rust iterator support. Each collection has a C++ iterator wrapper struct and Rust `Iterator` trait impl.

```rust
use opencascade_sys::top_tools::ListOfShape;
use opencascade_sys::topo_ds::Shape;

// Iterate over a list of shapes
for shape in list_of_shape.iter() {
    // shape: OwnedPtr<Shape>
    let x = shape.IsNull();
}

// Build a list from an iterator
let list = ListOfShape::from_iter(shapes.iter());
```

### Enums

OCCT enums are generated as `#[repr(i32)]` Rust enums with `From<EnumType> for i32` and `TryFrom<i32> for EnumType` conversions. Enum parameters in generated wrapper functions use typed Rust enums directly — the C++ wrapper handles the `i32` conversion at the FFI boundary.

```rust
use opencascade_sys::top_abs::ShapeEnum;

let shape_type = ShapeEnum::Edge;
let raw: i32 = shape_type.into();
let back = ShapeEnum::try_from(raw).unwrap();
```

---

## Architecture

### FFI Architecture

All types and functions are in a single `extern "C"` FFI module (`ffi.rs`), with per-module re-export files providing ergonomic short names:

```
crates/opencascade-sys/generated/
├── ffi.rs             # Single extern "C" block with ALL types (full C++ names)
├── wrappers.cpp       # Single C++ wrapper file (all includes + wrapper functions)
├── gp.rs              # Re-exports: `pub use crate::ffi::gp_Pnt as Pnt;` + impl blocks
├── topo_ds.rs         # Re-exports for topo_ds module + impl blocks
├── ... (per-module files)
└── lib.rs             # `pub(crate) mod ffi;` + `pub mod gp;` etc.
```

Users write `use opencascade_sys::gp::Pnt;` — the ffi module is `pub(crate)`.

### Generator Source

```
crates/opencascade-binding-generator/src/
├── main.rs           # CLI entry point
├── lib.rs            # Library API (for tests)
├── parser.rs         # libclang-based header parser
├── model.rs          # IR for parsed declarations (ParsedClass, Method, etc.)
├── resolver.rs       # Two-pass symbol table: resolves names, applies filters, builds SymbolTable
├── module_graph.rs   # Module dependency analysis
├── header_deps.rs    # Automatic header dependency resolution (--resolve-deps)
├── type_mapping.rs   # OCCT type -> Rust type mappings
└── codegen/
    ├── mod.rs
    ├── bindings.rs   # ClassBindings IR + emit functions for ffi/cpp/reexports
    ├── rust.rs       # Generates ffi.rs + per-module re-export files
    ├── cpp.rs        # Generates wrappers.cpp
    └── collections.rs # Generates collection type wrappers (iterators, accessors)
```

### Two-Pass Pipeline

1. **Parse**: libclang parses OCCT headers into `ParsedClass`, `Method`, etc. (`parser.rs`)
2. **Resolve**: `SymbolTable` built from parsed data — applies all filters, computes names, determines binding status (`resolver.rs`)
3. **Generate**: All binding decisions computed into `ClassBindings` structs (`codegen/bindings.rs`), then Rust and C++ code emitted from them (`codegen/rust.rs`, `codegen/cpp.rs`)

All method filtering (enum checks, lifetime issues, by-value params, etc.) is centralized in `resolver.rs` and applied consistently to both FFI and impl generation.

### Wrapper Functions

All methods use extern "C" C++ wrapper functions, since there is no direct Rust–C++ ABI bridge. The wrappers handle:

| C++ Pattern | Wrapper Approach |
|-------------|-----------------|
| Constructor | `new T(args)` returning `T*` |
| Return by value | `new T(obj.Method(args))` returning `T*` |
| Static method | Free function calling `ClassName::Method()` |
| `Handle<T>` | `typedef opencascade::handle<T> HandleT;` |
| Overloaded method | Suffix: `_real3`, `_pnt2`, etc. |
| `const char*` param | Pass-through as `const char*` |
| `const char*` return | Pass-through as `const char*` |
| Nullable `T*` param | Pass-through as `T*` (Rust uses `Option<&mut T>`) |
| Nullable `const T*` param | Pass-through as `const T*` (Rust uses `Option<&T>`) |
| Inherited method | Free function calling `self->Method()` |
| Upcast (const) | `Derived_as_Base(self) -> const Base*` |
| Upcast (mut) | `Derived_as_Base_mut(self) -> Base*` |
| By-value Handle param | Wrapper accepts `const Handle<T>&` |
| By-value class param | Wrapper accepts `const T&` |
| Const/mut return fix | `ConstMutReturnFix`: const-cast for non-const `self` |
| `const char*&` param (string ref) | Local `const char*` var, pass by ref, writeback to Rust-allocated string |
| `&mut` enum out-param | Local `int32_t` var + writeback |
| `&mut` enum return | `reinterpret_cast<int32_t*>(&obj.Method())` returning `*mut i32` |
| `const T*` return | Pass-through as pointer (Rust uses `Option<&T>`) |
| `T*` return (non-const method) | Pass-through as pointer (Rust uses `Option<&mut T>`) |
| `T*` return (const method) | Pass-through as pointer (Rust uses `Option<&T>`, downgraded for soundness) |

### Handle Support

Classes inheriting from `Standard_Transient` get:
- `ClassName::to_handle(obj)` — wrap in `Handle<T>`
- `handle.to_handle_base()` — upcast Handle to base type
- `handle.downcast_to_derived()` — type-checked downcast
- `handle.get()` / `handle.get_mut()` — dereference handle to contained object

### Collection Types

NCollection typedefs (e.g., `TopTools_ListOfShape`) get iterator wrappers:
- C++ iterator struct wrapping `const_iterator` or indexed access
- `TypeName_iter()` / `TypeNameIterator_next()` C++ functions
- Rust `Iterator` trait impl yielding `OwnedPtr<Element>`
- Impl methods: `iter()`, `from_iter()`, `append()`, etc.

### Standard Streams (iostream)

OCCT uses `Standard_OStream` (typedef for `std::ostream`) and `Standard_IStream` (typedef for `std::istream`) in many debug/dump methods. These are declared as `manual_types` in `bindings.toml` so the generator recognizes them as known types without generating class bindings. Namespace-scoped typedef aliases (for example `IMeshData::MapOfInteger`) are now resolved automatically by guarded parser logic that recognizes OCCT namespace typedef declarations.


Manual bindings in the `standard` module provide access to the global C++ stream objects:

```rust
use opencascade_sys::standard;
use opencascade_sys::b_rep_tools;

// Get a mutable reference to std::cout (valid for program lifetime)
let cout = standard::cout();

// Dump a shape's topological structure to stdout
let mut make_box = opencascade_sys::b_rep_prim_api::MakeBox::new_real3(10.0, 20.0, 30.0);
let shape = make_box.shape();
b_rep_tools::dump_shape_ostream(shape, cout);

// Also available: standard::cerr(), standard::clog(), standard::cin()
```

The `OStream` and `IStream` type aliases are re-exported from the `standard` module alongside the auto-generated Standard types.

### Naming Conventions

- **Types in ffi.rs**: Full C++ names (`gp_Pnt`, `TopoDS_Shape`, `BRepPrimAPI_MakeBox`)
- **Types in re-exports**: Short names (`Pnt`, `Shape`, `MakeBox`) via `pub use crate::ffi::X as Y;`
- **Methods**: snake_case (generated by the code generator)
- **Overloads**: Compressed parameter-type suffix (`_real3` not `_real_real_real`, `_pnt2` not `_pnt_pnt`)
- **Enums**: `TopAbs_ShapeEnum` -> `ShapeEnum`, variants `TopAbs_COMPOUND` -> `Compound`
- **Reserved names**: `Vec_` in ffi, re-exported as `Vec`

### Manual Bindings

Some C++ function signatures can't be auto-generated. Manual replacements live in `crates/opencascade-sys/manual/`:
- `<module>.rs` — `extern "C"` declarations + `impl` blocks
- `<module>_wrappers.cpp` — C++ wrapper functions

The generator appends `include!("../manual/<module>.rs");` (with a comment explaining why) to the generated module re-export file when a corresponding `manual/<module>.rs` exists. Because `include!()` is a textual insertion, the manual code has full access to the module's type aliases. The `extern "C"` declarations in manual files are not marked `pub`, so they are private to the module. `build.rs` globs `manual/*_wrappers.cpp` and compiles them alongside `generated/wrappers.cpp`. Since Rust allows multiple `impl` blocks for a type, manual methods appear seamlessly alongside the auto-generated ones.

Currently only `standard` iostream accessors (`cout()`, `cerr()`, etc.) require manual bindings in `crates/opencascade-sys/manual/`. `bindings.toml` `manual_types` remains useful for explicit opaque known types, while guarded namespace-typedef auto-resolution handles OCCT namespace aliases without per-type config entries.



---

## Skipped Symbols

The binding generator currently skips ~139 symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI. Abstract class constructors are silently omitted since that's expected behavior, not a binding limitation. Every other skipped symbol is documented in generated per-module `.rs` files as a `// SKIPPED:` comment block including:

- **Source location** (header file, line number, C++ symbol name)
- **Documentation comment** from the C++ header (first 3 lines)
- **Skip reason** explaining why the symbol was excluded
- **Commented-out Rust stub** showing the best-guess declaration

Example from `gp.rs`:
```rust
// SKIPPED: **Source:** `gp_XYZ.hxx`:109 - `gp_XYZ::GetData`
//   method: Returns a const ptr to coordinates location.
//   Reason: has unbindable types: return: raw pointer (const double*)
//   // pub fn get_data(&self) -> /* const double* */;
```

### Skip Reason Breakdown

| Count | % | Category | Description |
|------:|----:|----------|-------------|
| 66 | 48.9% | **Unknown/unresolved type** | Parameter or return type is not in the resolved known-type set |
| 42 | 31.1% | **Unknown Handle type** | `Handle(T)` where `T` is unresolved or excluded |
| 12 | 8.9% | **Rvalue reference** | C++ move semantics (`T&&`) — const-ref overloads usually exist |
| 6 | 4.4% | **C-style array** | Primarily incomplete arrays (`T[]`) and specialized pointer-array forms not yet mapped to safe Rust signatures |
| 5 | 3.7% | **Unresolved template type** | Template instantiations that can't be represented safely |
| 2 | 1.5% | **Excluded by bindings.toml** | Explicitly excluded in config |
| 1 | 0.7% | **Ambiguous overload** | C++ overload would produce conflicting wrapper signatures |
| 1 | 0.7% | **Not CppDeletable** | Return type has no destructor in the binding set |

Combined unresolved coverage (`Unknown/unresolved type` + `Unknown Handle type`) is **108 / 139 (77.7%)**.

Fixed-size arrays (`T (&)[N]`, `const T (&)[N]`, and `T arr[N]` parameter syntax) are now bindable; remaining array-related skips are mostly incomplete arrays and specialized pointer-array forms.

### Most Common Unknown Types

| Count | Type | How to Unblock |
|------:|------|----------------|
| 12 | `Handle(ShapePersistent_Geom::geometryBase<Geom_Surface>)` | Protected nested template class — not exposed as a bindable type |
| 10 | `Handle(ShapePersistent_Geom::geometryBase<Geom_Curve>)` | Protected nested template class — not exposed as a bindable type |
| 10 | `Handle(ShapePersistent_Geom::geometryBase<Geom2d_Curve>)` | Protected nested template class — not exposed as a bindable type |
| 5 | `WNT_HIDSpaceMouse const` | Windows-only type, WNT module excluded |
| 5 | `AVStream const` | FFmpeg media type — external dependency |
| 5 | `RWGltf_GltfOStreamWriter` | RapidJSON-backed writer type — external dependency |
| 4 | `GLXFBConfig` | X11/Linux display type — platform-specific |
| 3 | `Aspect_XDisplay` | X11/Linux display type — platform-specific |
| 3 | `IMeshData_Edge *const const` | Internal mesh pointer signature in specialized APIs |
| 3 | `IMeshData_Face *const const` | Internal mesh pointer signature in specialized APIs |

### Important Skipped Symbols

Most skipped symbols are in specialized or platform-specific areas. Current hotspots:

**Data Exchange (9 symbols)** — `rw_gltf` (5), `iges_control` (1), `step_control` (1), `xs_control` (1), `rw_stl` (1). Predominantly external/third-party types, plus a few `T&&` overloads where safe const-ref equivalents are already bound.

**Document Framework (1 symbol)** — `tdf` (1). Remaining unknown type is `TDF_LabelNode*` (internal raw pointer type not in binding set).

**Shape Meshing (14 symbols)** — `b_rep_mesh` (9), `i_mesh_data` (5). Namespace-scoped IMeshData typedef aliases are now auto-resolved; remaining skips are mostly C-style arrays, one unresolved template form, and internal pointer-heavy signatures.

**Shape Analysis/Fix (0 symbols)** — Fully bound for current header set.

**Geometry (0 symbols in gp/Geom/Geom2d)** — Fully bound for core geometry modules.

**Poly (2 symbols)** — one config exclusion and one unresolved type in low-level helper APIs.

### How Skipped Symbols Are Tracked

All filtering decisions happen in two places:
- `codegen/bindings.rs`: `is_method_bindable()`, `is_constructor_bindable()`, `is_static_method_bindable()` return `Result<(), String>` with a human-readable reason on `Err`
- `compute_class_bindings()` and `compute_all_function_bindings()` collect `SkippedSymbol` structs for every rejected symbol

The `emit_reexport_class()` and `emit_skipped_functions()` functions write the skip comments to the generated module files.

---

## Implementation Details

The generated code has two layers: a `pub(crate)` FFI module containing the raw extern "C" bindings, and public per-module re-export files that provide the user-facing API.

### Internal: `ffi.rs` (the FFI declarations)

A single `extern "C"` block declares all types and functions using full C++ names. This is `pub(crate)` — users never interact with it directly.

Types use their full C++ identifiers as Rust names to avoid collisions:

```rust
// generated/ffi.rs (pub(crate), not user-facing)
extern "C" {
    // Opaque C++ types -- full C++ names
    pub fn gp_Pnt_ctor_real3(theXp: f64, theYp: f64, theZp: f64) -> *mut gp_Pnt;
    pub fn gp_Pnt_destructor(self_: *mut gp_Pnt);
    pub fn gp_Pnt_x(self_: *const gp_Pnt) -> f64;
    pub fn gp_Pnt_set_x(self_: *mut gp_Pnt, theX: f64);
    pub fn gp_Pnt_distance(self_: *const gp_Pnt, theOther: *const gp_Pnt) -> f64;
    pub fn gp_Pnt_mirrored_pnt(self_: *const gp_Pnt, theP: *const gp_Pnt) -> *mut gp_Pnt;
    // ... thousands more
}
```

All methods go through extern "C" C++ wrapper functions. There is no ABI-level distinction between "direct" and "wrapped" methods — everything is wrapped for uniform handling.

### Internal: `wrappers.cpp` (C++ glue)

A single C++ file includes all needed OCCT headers and defines `extern "C"` wrapper functions for everything:

```cpp
// generated/wrappers.cpp
#include <gp_Pnt.hxx>
#include <TopoDS_Shape.hxx>
// ... OCCT headers

extern "C" gp_Pnt* gp_Pnt_ctor_real3(double x, double y, double z) {
    return new gp_Pnt(x, y, z);
}
extern "C" void gp_Pnt_destroy(gp_Pnt* self) {
    delete self;
}
extern "C" double gp_Pnt_x(const gp_Pnt* self) {
    return self->X();
}
// ...
```

### Internal: Per-Module Re-export Files

Each module file (e.g., `gp.rs`, `topo_ds.rs`) re-exports types from `ffi` with short names and provides `impl` blocks that wrap the internal FFI functions:

```rust
// generated/gp.rs

// Type re-export: full C++ name -> short Rust name
/// A 3D Cartesian point.
pub use crate::ffi::gp_Pnt as Pnt;

impl Pnt {
    pub fn new_real3(theXp: f64, theYp: f64, theZp: f64) -> crate::OwnedPtr<Self> {
        unsafe { crate::OwnedPtr::from_raw(crate::ffi::gp_Pnt_ctor_real3(theXp, theYp, theZp)) }
    }

    pub fn mirrored_pnt(&self, theP: &crate::ffi::gp_Pnt) -> crate::OwnedPtr<crate::ffi::gp_Pnt> {
        unsafe { crate::OwnedPtr::from_raw(crate::ffi::gp_Pnt_mirrored_pnt(self, theP)) }
    }

    pub fn to_owned(&self) -> crate::OwnedPtr<Self> {
        unsafe { crate::OwnedPtr::from_raw(crate::ffi::gp_Pnt_to_owned(self as *const Self)) }
    }
}
```

### Internal: `lib.rs` (Module Structure)

```rust
// generated/lib.rs
pub(crate) mod ffi;   // The FFI declarations -- internal only

pub mod gp;           // Re-exports gp_Pnt as Pnt, gp_Vec as Vec, etc.
pub mod topo_ds;      // Re-exports TopoDS_Shape as Shape, etc.
pub mod b_rep_prim_api;
pub mod b_rep_algo_api;
// ... modules
```

---

## Future Work

### Expanding to All OCCT Headers

Currently headers are selected via `bindings.toml`. OCCT ships 6,875 `.hxx` headers across ~349 modules. An experimental all-headers run produced 6,565 types and 90,295 functions in 333 modules, but surfaced issues:

1. **Fortran common blocks** (11 instances) — `AdvApp2Var_Data.hxx` defines structs like `maovpar_1_` that don't follow OCCT naming. The generator skips them (no bindable members).

2. **Non-type template parameters** (1 instance) — `BVH_Tree<T, int N>` has an `int N` template param that Rust can't represent. Filtered out.

3. **Raw pointer syntax in type names** (2 instances) — `IMeshData_Edge *const` leaking into names. Already filtered with a `contains('*')` check.

4. **Scale concerns** — ffi.rs would grow to 356K lines (6x). The entire extern "C" block is one compilation unit, causing long compile times. Would need per-module splitting or feature flags.

5. **Windows-only headers** — `OSD_WNT.hxx` includes `<windows.h>`, fails on macOS/Linux. Non-blocking.

6. **Nested C++ types** (SOLVED) — OCCT defines ~173 nested structs, enums, and typedefs inside classes (e.g., `Poly_CoherentTriangulation::TwoIntegers`, `AIS_PointCloud::DisplayMode`, `BOPTools_PairSelector::PairIDs`). The parser now detects parent class scope via clang's semantic parent and qualifies nested types as `Parent::Nested`. The generator flattens `::` to `_` for Rust FFI names (`Parent_Nested`) while keeping qualified names in C++ wrappers. Destructors are auto-generated for all nested opaque types. This unblocked 58 new types, 67 new methods, and 76 nested type destructors.

7. **Template instantiation Handle types** (SOLVED) — Methods returning or accepting `Handle(BVH_Builder<double, 3>)` or `Handle(NCollection_Shared<...>)` were skipped because the template instantiation isn't a named class. A `[template_instantiations]` section in `bindings.toml` declares specific template instantiations to bind. The generator creates C++ typedefs (e.g., `typedef BVH_Builder<double, 3> BVH_Builder_double_3;`), rewrites all Handle references in parsed data, and treats aliases as normal classes for Handle support. This resolved ~30 previously-skipped symbols.

8. **Nested class inheritance** (SOLVED) — Nested classes like `ShapePersistent_BRep::Curve3D` had unqualified base class names (e.g., `GCurve` instead of `ShapePersistent_BRep::GCurve`), breaking the transitive closure in `compute_handle_able_classes()`. The parser now qualifies sibling base class references when qualifying nested type names.

9. **Fixed-size arrays** (SOLVED) — OCCT APIs with parameters like `Standard_Integer (&theEdges)[3]` and `Standard_Integer theEdges[3]` were previously skipped as C-style arrays. The parser now models constant arrays explicitly (`Type::FixedArray`), Rust re-exports expose them as `&[T; N]` / `&mut [T; N]`, and wrappers bridge via element pointers at the extern C layer (with cast-back for array references in C++). This unblocked APIs such as `BRepMesh_MeshTool::AddTriangle`.


### System Include Path Auto-Detection

Currently `-I` path is passed manually. Could auto-detect from `occt-sys`.

### Explicit `bindings.toml` Config for Manual Bindings

An explicit `bindings.toml` section for declaring manual bindings would allow skipping problematic signatures without requiring code changes to the generator.

---

**Special-case/heuristic patterns in the codebase:**

1. **Short name convention (`split('_').skip(1)`)** — Used in enum variant name generation for converting OCCT enum variants (e.g., `TopAbs_COMPOUND` → `Compound`). This assumes a single module-prefix underscore. For class/type names, `short_name_for_module()` is used instead, which correctly handles the module prefix (e.g., `BRepOffset_Status` with module `BRepOffset` → `Status`). Handle upcast/downcast method names also use `short_name_for_module()` with proper module lookup.

**Previously problematic special cases (now resolved):**

- **Copy constructor detection for `to_owned()`**: Uses libclang's `is_copy_constructor()` to detect explicit copy constructors and `is_move_constructor()` to detect move constructors. Classes with an explicit public non-deleted copy constructor (`Some(true)`) always get `to_owned()`, those with an explicitly deleted/private copy constructor (`Some(false)`) never do. When no explicit copy constructor is present (`None`), falls back to a conservative module allowlist (`["TopoDS", "gp", "TopLoc", "Bnd", "GProp"]`) because implicit copy constructors can be silently deleted when a class has non-copyable members.

- **Handle upcast/downcast `split('_').skip(1)`**: Previously used `split('_').skip(1)` to derive short names for handle upcast/downcast methods, which broke for multi-underscore module prefixes like `DE_BREP_*`. Now uses `short_name_for_module()` with the proper module from the symbol table.

- **Handle type detection**: Unified through a single transitive closure algorithm (`compute_handle_able_classes()`) that walks the full inheritance graph starting from `Standard_Transient`. This replaces the old parser heuristic with hardcoded prefixes (`"Geom_*"`, `"Geom2d_*"`, `"Law_*"`) and fixes the inheritance graph by including `Standard_*` base classes.
- **Inheritance graph**: Fixed `extract_base_classes()` to include `Standard_*` classes, so the full inheritance hierarchy is now represented, enabling more accurate dependency analysis and upcasts.

## Known Issues

### UB crash in `clang` crate (v2.0.0) — worked around via `--release` build

The published `clang` crate (v2.0.0) has a bug in `SourceRange::tokenize()`: it
passes the pointer returned by `clang_tokenize` directly to
`slice::from_raw_parts` without a null check. When `clang_tokenize` returns a
null pointer (which happens for some macro-expanded expressions and compiler
builtins), this is undefined behavior. Starting with Rust 1.78, debug builds
include UB precondition checks that detect this and abort the process:

```
unsafe precondition(s) violated: slice::from_raw_parts requires the pointer to
be aligned and non-null
```

The fix was merged in [PR #58](https://github.com/KyleMayes/clang-rs/pull/58)
([issue #47](https://github.com/KyleMayes/clang-rs/issues/47)) but has not been
released to crates.io, and later commits in the git repository no longer compile
with current Rust. As a workaround, `scripts/regenerate-bindings.sh` builds the
generator in `--release` mode, which disables the debug UB checks. If/when a
v2.0.1 is released on crates.io, remove the `--release` flag from that script
and update `Cargo.toml` to use `version = "2.0"`.