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
| `&mut` enum out-param | Local `int32_t` var + writeback |

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

### Naming Conventions

- **Types in ffi.rs**: Full C++ names (`gp_Pnt`, `TopoDS_Shape`, `BRepPrimAPI_MakeBox`)
- **Types in re-exports**: Short names (`Pnt`, `Shape`, `MakeBox`) via `pub use crate::ffi::X as Y;`
- **Methods**: snake_case (generated by the code generator)
- **Overloads**: Compressed parameter-type suffix (`_real3` not `_real_real_real`, `_pnt2` not `_pnt_pnt`)
- **Enums**: `TopAbs_ShapeEnum` -> `ShapeEnum`, variants `TopAbs_COMPOUND` -> `Compound`
- **Reserved names**: `Vec_` in ffi, re-exported as `Vec`

### Manual Bindings

Some C++ function signatures can't be auto-generated — for example, methods with `const char*&` or `const char* const&` parameters (a reference to a `const char*`). The generator detects these (`ExclusionReason::StringRefParam` in `resolver.rs`) and skips them automatically.

Manual replacements live in `crates/opencascade-sys/manual/`:
- `<module>.rs` — `extern "C"` declarations + `impl` blocks
- `<module>_wrappers.cpp` — C++ wrapper functions

The generator appends `include!("../manual/<module>.rs");` (with a comment explaining why) to the generated module re-export file when a corresponding `manual/<module>.rs` exists. Because `include!()` is a textual insertion, the manual code has full access to the module's type aliases (e.g., `AdvancedEvolved`, `Finder`). The `extern "C"` declarations in manual files are not marked `pub`, so they are private to the module and not exposed as part of the public API. `build.rs` globs `manual/*_wrappers.cpp` and compiles them alongside `generated/wrappers.cpp`. Since Rust allows multiple `impl` blocks for a type, manual methods appear seamlessly alongside the auto-generated ones.

See `crates/opencascade-sys/manual/` and the comments in `bindings.toml` for the two existing examples (`Transfer_Finder::GetStringAttribute` and `BRepFill_AdvancedEvolved::SetTemporaryDirectory`).

---

## Methods Skipped Due to Limitations

The following patterns cause methods to be intentionally skipped during binding generation:

1. **Methods with ambiguous lifetimes** — Methods returning mutable references when there are also reference parameters. The lifetime of the returned reference is ambiguous.

2. **Abstract class constructors** — Abstract classes cannot be instantiated, so constructor wrappers and `to_handle()` functions are not generated. Abstract detection walks the full inheritance hierarchy to catch classes that inherit unimplemented pure virtual methods from ancestors.

3. **Classes with protected destructors** — Excluded from type declarations entirely since the FFI layer auto-generates destructor code.

4. **Raw pointer parameters** — Methods with `T*` / `const T*` parameters are excluded unless the pointer has a default value (i.e., nullable). Nullable pointer params are bound as `Option<&mut T>` / `Option<&T>` (see "Nullable Pointer Parameters" above). Non-nullable raw pointer methods are concentrated in BSplCLib and BSplSLib (internal B-spline evaluation routines). The few useful cases (e.g., `gp_XYZ::GetData()`) can be handled with handwritten wrappers if needed.

### Filter Consistency

All filter functions are centralized in `resolver.rs`. When any method is filtered out of FFI generation, it is automatically filtered out of impl generation too, since both use the same `SymbolTable`.

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

### System Include Path Auto-Detection

Currently `-I` path is passed manually. Could auto-detect from `occt-sys`.

### Explicit `bindings.toml` Config for Manual Bindings

The current `StringRefParam` detection automatically catches `const char*&` cases. An explicit `bindings.toml` section for declaring manual bindings would allow skipping other problematic signatures beyond string refs without requiring code changes to the generator.
