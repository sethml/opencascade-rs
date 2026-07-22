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

// Clone into a new OwnedPtr
let p_copy = p.clone();

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
let shape_copy = shape.clone();

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

OCCT uses `Handle<T>` (an intrusive reference-counted pointer) for objects inheriting from `Standard_Transient`. The generator produces `to_handle()` to wrap an object, upcast methods to convert between Handle types in the inheritance hierarchy, `Clone` / `ToOwned` support for copying handles, and `is_null()` to check for null handles.

```rust
use opencascade_sys::geom::{BezierCurve, BSplineCurve};

// Wrap a geometry object in a Handle
let curve: OwnedPtr<BezierCurve> = /* ... */;
let handle = BezierCurve::to_handle(curve);
// handle is OwnedPtr<Handle_Geom_BezierCurve>

// Clone a handle (invokes C++ copy constructor)
let handle2 = handle.clone();

// Check for null before dereferencing
if !handle.is_null() {
    let obj = handle.get();
}

// Methods that return Handles
let copy = some_curve.copy();
// copy is OwnedPtr<Handle_Geom_Geometry>
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

### Collections

Every `NCollection_*<T>` instantiation referenced by the API is materialized under a
canonical name in the `n_collection` module (`List_TopoDS_Shape`, `Array1_gp_Pnt`,
`HArray1_gp_Pnt`, `DataMap_TopoDS_Shape_int`, `IndexedMap_TopoDS_Shape_TopTools_ShapeMapHasher`,
…) with full methods (`new`, `append`/`value`/`set_value`/`add`/`bind`/`find`, …), a real
`Iterator` impl, and — for handle kinds (`HArray1`/`HSequence`) — `to_handle`. Collections
of primitive scalars (`List_int`, `Sequence_double`) are supported too; their scalar
accessors take/return the value directly.

Because the iterator implements `Iterator`, `iter()` works directly in `for` loops and
iterator adapters:

```rust
use opencascade_sys::n_collection::List_TopoDS_Shape;

let mut list = List_TopoDS_Shape::new();
list.append(a_shape);

for shape in list.iter() {
    // shape: OwnedPtr<Shape>
    let _ = shape.is_null();
}
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

Type definitions live in one file; the `extern "C"` function declarations and their C++
wrapper glue are split per OCCT toolkit (TKernel, TKMath, TKBRep, …) so the C++ and Rust
FFI compile in parallel. Per-module re-export files layer ergonomic short names on top:

```
crates/opencascade-sys/generated/
├── ffi_types.rs         # All opaque/POD type definitions (full C++ names) — pub(crate)
├── ffi_extern_TKMath.rs # extern "C" fn declarations for the TKMath toolkit — pub(crate)
├── ffi_extern_TKBRep.rs # ... one ffi_extern_TK<toolkit>.rs per toolkit (~47 files)
├── wrappers_TKMath.cpp  # C++ wrapper definitions for the TKMath toolkit
├── wrappers_TKBRep.cpp  # ... one wrappers_TK<toolkit>.cpp per toolkit (~47 files)
├── gp.rs                # Re-exports: `pub use crate::ffi_types::gp_Pnt as Pnt;` + impl blocks
├── topo_ds.rs           # Re-exports for topo_ds module + impl blocks
├── ... (per-module files)
└── lib.rs               # `pub(crate) mod ffi_types;` + per-toolkit `mod ffi_extern_TK*;` + `pub mod gp;` etc.
```

Users write `use opencascade_sys::gp::Pnt;` — the `ffi_types` and `ffi_extern_*` modules are
`pub(crate)`. (Internally the generator emits a `crate::ffi::` placeholder for every FFI
reference; the toolkit-split pass rewrites each into `crate::ffi_types::` for a type or
`crate::ffi_extern_TK<toolkit>::` for a function.)

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
    ├── rust.rs       # Generates ffi_types.rs + per-toolkit ffi_extern_TK*.rs + per-module re-exports
    ├── cpp.rs        # Generates per-toolkit wrappers_TK*.cpp
    └── collections.rs # Discovers NCollection instantiations + generates their wrappers
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
| `Handle<T>` | `typedef opencascade::handle<T> Handle_T;` |
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
- `handle.clone()` — copy the handle (via C++ copy constructor)
- `handle.is_null()` — check if the handle is null
- `handle.to_handle_base()` — upcast Handle to base type
- `handle.downcast_to_derived()` — type-checked downcast
- `handle.get()` / `handle.get_mut()` — dereference handle to contained object

The `NCollection_H*` templates (`HArray1`, `HArray2`, `HSequence`) also inherit
`Standard_Transient`, so their instantiations are auto-registered as handle-able. This
lets `Handle(NCollection_HArray1<T>)` parameters/returns resolve rather than being skipped
— previously the single largest source of skipped symbols.

### Collection Types

Collections are **auto-discovered** from the canonical `NCollection_*<...>` template
spellings the parsed API references (`collections.rs`: `discover_collections`). Each
instantiation whose element/value types are bindable is materialized in the `n_collection`
module under a canonical mangled name (`NCollection_List<TopoDS_Shape>` →
`n_collection::List_TopoDS_Shape`) with:
- a C++ `typedef <spelling> <mangled>;` bridge, so the wrappers use the short mangled name
- full element methods per kind (list/sequence/array/map/data-map), primitive-aware
  (scalar elements/values are passed by value rather than pointer)
- a C++ iterator struct + `TypeNameIterator_next()` function and a real `Iterator` impl
  (`iter()` — usable in `for` loops and adapters, via a blanket `Iterator for OwnedPtr<I>`)
- for handle kinds (`HArray1`/`HSequence`): `to_handle` plus `get`/`get_mut` and the
  handle lifetime machinery on the generated `Handle_*` type

The canonical names are the same ones method signatures resolve to, so a returned
`&NCollection_List<TopoDS_Shape>` and the collection's own methods refer to one type.

In overload suffixes, collection parameters use a short registered token (`imap_shape`,
`arr1_pnt`, `harr1_pnt`, `dmap_shape_int` — dropping the hasher argument) rather than the
full mangled type, so an overloaded `TopExp::MapShapes` reads
`map_shapes_shape_shapeenum_imap_shape` instead of
`map_shapes_shape_shapeenum_indexedmap_topods_shape_toptools_shapemaphasher`.

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

- **Types in `ffi_types.rs`**: Full C++ names (`gp_Pnt`, `TopoDS_Shape`, `BRepPrimAPI_MakeBox`)
- **Types in re-exports**: Short names (`Pnt`, `Shape`, `MakeBox`) via `pub use crate::ffi_types::X as Y;`
- **Handle types**: `Handle_<C++name>` (`Handle_Geom_Curve`, `Handle_NCollection_HArray1_gp_Pnt`)
- **Methods**: snake_case (generated by the code generator)
- **Overloads**: Compressed parameter-type suffix (`_real3` not `_real_real_real`, `_pnt2` not `_pnt_pnt`)
- **Enums**: `TopAbs_ShapeEnum` -> `ShapeEnum`, variants `TopAbs_COMPOUND` -> `Compound`
- **Reserved names**: `Vec_` in ffi, re-exported as `Vec`

### Manual Bindings

Some C++ function signatures can't be auto-generated. Manual replacements live in `crates/opencascade-sys/manual/`:
- `<module>.rs` — `extern "C"` declarations + `impl` blocks
- `<module>_wrappers.cpp` — C++ wrapper functions

The generator appends `include!("../manual/<module>.rs");` (with a comment explaining why) to the generated module re-export file when a corresponding `manual/<module>.rs` exists. Because `include!()` is a textual insertion, the manual code has full access to the module's type aliases. The `extern "C"` declarations in manual files are not marked `pub`, so they are private to the module. `build.rs` globs `manual/*_wrappers.cpp` and compiles them alongside the generated `wrappers_TK*.cpp`. Since Rust allows multiple `impl` blocks for a type, manual methods appear seamlessly alongside the auto-generated ones.

Currently only `standard` iostream accessors (`cout()`, `cerr()`, etc.) require manual bindings in `crates/opencascade-sys/manual/`. `bindings.toml` `manual_types` remains useful for explicit opaque known types, while guarded namespace-typedef auto-resolution handles OCCT namespace aliases without per-type config entries.



---

## Deprecated Symbols

The generator does **not** emit bindings for OCCT deprecated symbols. OCCT 8.0.0 moved
its backward-compatibility alias headers (e.g. `TColgp_HArray1OfPnt.hxx`, now just
`typedef NCollection_HArray1<gp_Pnt>`) into `src/Deprecated/` and pivoted its own API to
the canonical `NCollection_...<T>` template names. These alias headers are excluded
automatically by the `exclude_deprecated_headers` config option (on by default), which
matches the standard deprecation banner in header content — so bindings target the
canonical types rather than the deprecated aliases.

## Skipped Symbols

The binding generator currently skips **~720** symbols (methods, constructors, static methods, and free functions) that it cannot safely represent in Rust FFI — out of ~144,000 bound functions. Abstract class constructors are silently omitted since that's expected behavior, not a binding limitation. Every other skipped symbol is documented in generated per-module `.rs` files as a `// SKIPPED:` comment block including:

- **Source location** (header file, line number, C++ symbol name)
- **Documentation comment** from the C++ header (first 3 lines)
- **Skip reason** explaining why the symbol was excluded
- **Commented-out Rust stub** showing the best-guess declaration

Example from `select_mgr.rs`:
```rust
// SKIPPED: **Source:** `SelectMgr_BVHThreadPool.hxx`:101 - `SelectMgr_BVHThreadPool::Threads`
//   method: Returns array of threads
//   Reason: has unbindable types: return: unresolved template type (NCollection_Array1<BVHThread>&)
//   // pub fn threads(&mut self) -> /* NCollection_Array1<BVHThread>& */;
```

### Skip Reason Breakdown

| Approx. count | Category | Description |
|------:|----------|-------------|
| ~460 | **Unbindable types** | A parameter/return type isn't in the resolved known-type set — internal helper types, unresolved templates, or platform/external-dependency types (X11 `GLXFBConfig`/`Aspect_XDisplay*`, macOS `NSOpenGLContext*`, FFmpeg/glTF writer types, etc.). |
| ~120 | **`BRepGraphInc` graph API** | New-in-8.0 experimental graph package with private nested types. |
| ~33 | **`std::` types** | e.g. `std::string_view` parameters. |
| ~14 | **Rvalue reference** | C++ move semantics (`T&&`) — const-ref overloads usually exist. |

Handle-wrapped `NCollection_H*<T>` instantiations (`Handle(NCollection_HArray1<T>)`, etc.)
used to dominate this list; they are now auto-registered as handle-able (see the
[handle collections](#handle-support) note below), leaving only a handful of skips.

Fixed-size arrays (`T (&)[N]`, `const T (&)[N]`, and `T arr[N]` parameter syntax) are bindable; there are **no** remaining skips categorized as C-style arrays. Core geometry (`gp`/`Geom`/`Geom2d`) and topology/modeling modules are essentially fully bound.

### How Skipped Symbols Are Tracked

All filtering decisions happen in two places:
- `codegen/bindings.rs`: `is_method_bindable()`, `is_constructor_bindable()`, `is_static_method_bindable()` return `Result<(), String>` with a human-readable reason on `Err`
- `compute_class_bindings()` and `compute_all_function_bindings()` collect `SkippedSymbol` structs for every rejected symbol

The `emit_reexport_class()` and `emit_skipped_functions()` functions write the skip comments to the generated module files.

---

## Implementation Details

The generated code has two layers: a `pub(crate)` FFI layer (type definitions plus
per-toolkit `extern "C"` blocks) containing the raw bindings, and public per-module
re-export files that provide the user-facing API.

### Internal: `ffi_types.rs` + `ffi_extern_TK*.rs` (the FFI declarations)

`ffi_types.rs` declares all types (full C++ names, to avoid collisions), and each
`ffi_extern_TK<toolkit>.rs` declares the `extern "C"` functions for one OCCT toolkit. Both
are `pub(crate)` — users never interact with them directly.

```rust
// generated/ffi_types.rs (types) + generated/ffi_extern_TKMath.rs (functions) — pub(crate)
unsafe extern "C" {
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

### Internal: `wrappers_TK*.cpp` (C++ glue)

Each `wrappers_TK<toolkit>.cpp` includes the OCCT headers for its toolkit and defines the
`extern "C"` wrapper functions:

```cpp
// generated/wrappers_TKMath.cpp
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

Each module file (e.g., `gp.rs`, `topo_ds.rs`) re-exports types from `ffi_types` with short
names and provides `impl` blocks that call the toolkit's `ffi_extern_TK*` functions:

```rust
// generated/gp.rs

// Type re-export: full C++ name -> short Rust name
/// A 3D Cartesian point.
pub use crate::ffi_types::gp_Pnt as Pnt;

impl Pnt {
    pub fn new_real3(theXp: f64, theYp: f64, theZp: f64) -> crate::OwnedPtr<Self> {
        unsafe { crate::OwnedPtr::from_raw(crate::ffi_extern_TKMath::gp_Pnt_ctor_real3(theXp, theYp, theZp)) }
    }

    pub fn mirrored_pnt(&self, theP: &crate::ffi_types::gp_Pnt) -> crate::OwnedPtr<crate::ffi_types::gp_Pnt> {
        unsafe { crate::OwnedPtr::from_raw(crate::ffi_extern_TKMath::gp_Pnt_mirrored_pnt(self, theP)) }
    }
}
```

### Internal: `lib.rs` (Module Structure)

```rust
// generated/lib.rs
pub(crate) mod ffi_types;            // Type definitions -- internal only
pub(crate) mod ffi_extern_TKMath;    // Per-toolkit extern "C" blocks -- internal only
pub(crate) mod ffi_extern_TKBRep;
// ... one ffi_extern_TK<toolkit> per toolkit

pub mod gp;           // Re-exports gp_Pnt as Pnt, gp_Vec as Vec, etc.
pub mod topo_ds;      // Re-exports TopoDS_Shape as Shape, etc.
pub mod b_rep_prim_api;
// ... modules
```

---

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