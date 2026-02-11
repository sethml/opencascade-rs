# opencascade-binding-generator

Automatically generates Rust [CXX](https://cxx.rs/) bindings for the [OpenCASCADE](https://dev.opencascade.org/) C++ CAD kernel.

Given a set of OCCT header files, the generator produces a complete Rust FFI layer: type declarations, method bindings, constructor wrappers, Handle smart pointer support, collection iterators, inheritance upcasts, and per-module re-exports with ergonomic short names.

## Quick Start

```bash
# Regenerate all bindings (from repo root)
./scripts/regenerate-bindings.sh
```

This parses 262 OCCT headers (expanding to 378 with automatic dependency resolution), and writes generated code to `crates/opencascade-sys/generated/`.

## Generated Public API

Users interact with `opencascade-sys` through per-module re-exports. Each OCCT module (gp, TopoDS, BRepPrimAPI, etc.) becomes a Rust module with short type names and idiomatic method signatures.

### Geometry Primitives

Simple value types like points, vectors, and directions get constructors, accessors, and transformation methods. Methods that return primitives bind directly through CXX; methods that return class types by value go through C++ wrappers that return `UniquePtr<T>`.

```rust
use opencascade_sys::gp::{Pnt, Vec, Dir, Ax1, Ax2, Trsf};

// Constructors -- overloads distinguished by parameter type suffix
let origin = Pnt::new();                          // default constructor
let p = Pnt::new_real3(1.0, 2.0, 3.0);           // from coordinates
let p2 = Pnt::new_xyz(&xyz);                      // from XYZ

// Direct accessors (bound as CXX self methods, no wrapper needed)
let x: f64 = p.x();
let y: f64 = p.y();
p.pin_mut().set_x(10.0);

// Transformations return UniquePtr (by-value return needs C++ wrapper)
let mirrored: cxx::UniquePtr<Pnt> = p.mirrored_pnt(&origin);
let rotated = p.rotated(&axis, std::f64::consts::FRAC_PI_2);
let transformed = p.transformed(&trsf);

// Copy into a new UniquePtr
let p_copy = p.to_owned();
```

### Topology Types and Inheritance

Topological shapes form an inheritance hierarchy: `Shape` is the base, with subtypes `Vertex`, `Edge`, `Wire`, `Face`, `Shell`, `Solid`, `CompSolid`, and `Compound`. CXX treats these as unrelated opaque types, so the generator produces explicit upcast methods (both const and mutable) and downcast free functions.

```rust
use opencascade_sys::topo_ds::{Shape, Edge, Face, Solid, Wire};

// Upcasting -- every subtype can upcast to its ancestors
let edge = Edge::new();
let shape_ref: &Shape = edge.as_shape();                    // const upcast
let shape_mut = edge.pin_mut().as_shape_mut();              // mutable upcast

// Downcasting -- free functions in the topo_ds module
// (wraps OCCT's TopoDS::Edge(), TopoDS::Face(), etc.)
use opencascade_sys::topo_ds;
let edge: cxx::UniquePtr<Edge> = topo_ds::edge(&some_shape);       // const
let face: cxx::UniquePtr<Face> = topo_ds::face_mut(&mut shape);    // mutable

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
let make_shape = make_box.pin_mut().as_b_rep_builder_api_make_shape_mut();
let result: &TopoDS_Shape = make_shape.shape();
```

### Boolean Operations

```rust
use opencascade_sys::b_rep_algo_api::{Fuse, Cut, Common};
use opencascade_sys::message::ProgressRange;

let progress = ProgressRange::new();
let mut fuse = Fuse::new_shape2_progressrange(&shape_a, &shape_b, &progress);
let make_shape = fuse.pin_mut().as_b_rep_builder_api_make_shape_mut();
let result = make_shape.shape();
```

### Static Methods

C++ static methods become associated functions on the type. Since CXX has no concept of static methods, these go through free function wrappers.

```rust
use opencascade_sys::b_rep_bnd_lib::BRepBndLib;
use opencascade_sys::bnd::Box;

let mut bbox = Box::new();
// Static method: BRepBndLib::Add(shape, box, useTriangulation)
BRepBndLib::add(&shape, bbox.pin_mut(), true);
```

### Handle Types (Reference-Counted Smart Pointers)

OCCT uses `Handle<T>` (an intrusive reference-counted pointer) for objects inheriting from `Standard_Transient`. The generator produces `to_handle()` to wrap an object, and upcast methods to convert between Handle types in the inheritance hierarchy.

```rust
use opencascade_sys::geom::{BezierCurve, BSplineCurve};

// Wrap a geometry object in a Handle
let curve: cxx::UniquePtr<BezierCurve> = /* ... */;
let handle = BezierCurve::to_handle(curve);
// handle is UniquePtr<HandleGeomBezierCurve>

// Methods that return Handles
let copy = some_curve.copy();
// copy is UniquePtr<HandleGeomGeometry>
```

### const char* String Conversions

Methods taking `const char*` accept `&str` in Rust; methods returning `const char*` return `String`. C++ wrappers handle the conversion via `rust::Str` and `rust::String`.

```rust
use opencascade_sys::standard::Failure;

// const char* return -> String
let msg: String = failure.get_message_string();
```

### Collection Iterators

OCCT collection types (NCollection_List, NCollection_Sequence, NCollection_IndexedMap, etc.) get Rust iterator support. Each collection has a C++ iterator wrapper struct and Rust `Iterator` trait impl.

```rust
use opencascade_sys::top_tools::ListOfShape;
use opencascade_sys::topo_ds::Shape;

// Iterate over a list of shapes
for shape in list_of_shape.iter() {
    // shape: cxx::UniquePtr<Shape>
    let x = shape.IsNull();
}

// Build a list from an iterator
let list = ListOfShape::from_iter(shapes.iter());
```

### Enums

OCCT enums are not yet generated into the CXX bridge because OCCT uses unscoped C-style enums while CXX requires C++11 `enum class`. Methods that take or return enum types are skipped. This is the single largest gap in binding coverage.

## Implementation Details

The generated code has two layers: a `pub(crate)` FFI module containing the raw CXX bridge, and public per-module re-export files that provide the user-facing API.

### Internal: `ffi.rs` (the CXX bridge)

A single `#[cxx::bridge]` module declares all types and functions using full C++ names. This is `pub(crate)` -- users never interact with it directly.

Types use their full C++ identifiers as Rust names to avoid collisions:

```rust
// generated/ffi.rs (pub(crate), not user-facing)
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("wrappers.hxx");

        // Opaque C++ types -- full C++ names
        type gp_Pnt;
        type gp_Vec;
        type TopoDS_Shape;
        type TopoDS_Edge;
        type BRepPrimAPI_MakeBox;

        // Handle types
        type HandleGeomBezierCurve;
        type HandleGeomCurve;
        type HandleGeomGeometry;

        // Direct CXX method binding (simple accessors)
        // CXX generates the glue automatically -- no C++ wrapper needed
        #[cxx_name = "X"]
        fn x(self: &gp_Pnt) -> f64;
        #[cxx_name = "Y"]
        fn y(self: &gp_Pnt) -> f64;
        #[cxx_name = "Z"]
        fn z(self: &gp_Pnt) -> f64;
        #[cxx_name = "SetX"]
        fn set_x(self: Pin<&mut gp_Pnt>, theX: f64);

        // Distance between two points (direct binding, returns primitive)
        #[cxx_name = "Distance"]
        fn distance(self: &gp_Pnt, theOther: &gp_Pnt) -> f64;

        // Constructor wrapper (returns UniquePtr, needs C++ wrapper)
        fn gp_Pnt_ctor_real3(theXp: f64, theYp: f64, theZp: f64) -> UniquePtr<gp_Pnt>;

        // By-value return wrapper (C++ returns gp_Pnt, wrapper returns unique_ptr)
        fn gp_Pnt_mirrored_pnt(self_: &gp_Pnt, theP: &gp_Pnt) -> UniquePtr<gp_Pnt>;

        // Copy constructor wrapper
        fn gp_Pnt_to_owned(self_: &gp_Pnt) -> UniquePtr<gp_Pnt>;

        // Static method wrapper (no self parameter)
        fn BRepBndLib_add(S: &TopoDS_Shape, B: Pin<&mut Bnd_Box>, useTriangulation: bool);

        // Upcast wrapper (C++ implicit conversion, explicit in Rust)
        fn TopoDS_Edge_as_TopoDS_Shape(self_: &TopoDS_Edge) -> &TopoDS_Shape;
        fn TopoDS_Edge_as_TopoDS_Shape_mut(
            self_: Pin<&mut TopoDS_Edge>
        ) -> Pin<&mut TopoDS_Shape>;

        // Handle wrapping
        fn Geom_BezierCurve_to_handle(
            obj: UniquePtr<Geom_BezierCurve>
        ) -> UniquePtr<HandleGeomBezierCurve>;
    }
}
```

**What gets a direct CXX binding vs a C++ wrapper:**

| Pattern | Direct CXX binding | C++ wrapper |
|---------|:---:|:---:|
| Accessor returning primitive (`f64`, `bool`, `i32`) | Yes | |
| Mutator taking primitives | Yes | |
| Method returning `const T&` | Yes | |
| Method returning `T` by value (class type) | | Yes |
| Constructor | | Yes |
| Static method | | Yes |
| `const char*` parameter or return | | Yes |
| Copy (`to_owned`) | | Yes |
| Upcast | | Yes |

### Internal: `wrappers.hxx` (C++ glue)

A single C++ header includes all needed OCCT headers and defines inline wrapper functions for everything CXX can't bind directly:

```cpp
// generated/wrappers.hxx (included by ffi.rs via CXX)
#pragma once
#include "common.hxx"      // construct_unique<T> template

#include <gp_Pnt.hxx>
#include <gp_Vec.hxx>
#include <TopoDS_Shape.hxx>
// ... 560 OCCT headers

// Constructor wrappers
inline std::unique_ptr<gp_Pnt> gp_Pnt_ctor_real3(
    Standard_Real theXp, Standard_Real theYp, Standard_Real theZp) {
    return std::make_unique<gp_Pnt>(theXp, theYp, theZp);
}

// By-value return wrappers
inline std::unique_ptr<gp_Pnt> gp_Pnt_mirrored_pnt(
    const gp_Pnt& self_, const gp_Pnt& theP) {
    return std::make_unique<gp_Pnt>(self_.Mirrored(theP));
}

// Copy constructor wrapper
inline std::unique_ptr<gp_Pnt> gp_Pnt_to_owned(const gp_Pnt& self_) {
    return std::make_unique<gp_Pnt>(self_);
}

// Upcast wrappers (C++ implicit reference conversion)
inline const TopoDS_Shape& TopoDS_Edge_as_TopoDS_Shape(const TopoDS_Edge& self_) {
    return self_;
}
inline TopoDS_Shape& TopoDS_Edge_as_TopoDS_Shape_mut(TopoDS_Edge& self_) {
    return self_;
}

// Static method wrappers
inline void BRepBndLib_add(
    const TopoDS_Shape& S, Bnd_Box& B, Standard_Boolean useTriangulation) {
    BRepBndLib::Add(S, B, useTriangulation);
}

// Handle wrapping
inline std::unique_ptr<HandleGeomBezierCurve> Geom_BezierCurve_to_handle(
    std::unique_ptr<Geom_BezierCurve> obj) {
    return std::make_unique<HandleGeomBezierCurve>(obj.release());
}
```

### Internal: Per-Module Re-export Files

Each module file (e.g., `gp.rs`, `topo_ds.rs`) re-exports types from `ffi` with short names and provides `impl` blocks that wrap the internal FFI functions:

```rust
// generated/gp.rs

// Type re-export: full C++ name -> short Rust name
/// A 3D Cartesian point.
pub use crate::ffi::gp_Pnt as Pnt;

impl Pnt {
    // Constructor -- delegates to FFI wrapper function
    pub fn new_real3(theXp: f64, theYp: f64, theZp: f64) -> cxx::UniquePtr<Self> {
        crate::ffi::gp_Pnt_ctor_real3(theXp, theYp, theZp)
    }

    // By-value return method -- delegates to FFI wrapper function
    pub fn mirrored_pnt(&self, theP: &crate::ffi::gp_Pnt) -> cxx::UniquePtr<crate::ffi::gp_Pnt> {
        crate::ffi::gp_Pnt_mirrored_pnt(self, theP)
    }

    // Copy
    pub fn to_owned(&self) -> cxx::UniquePtr<Self> {
        crate::ffi::gp_Pnt_to_owned(self)
    }

    // Note: direct CXX methods (x(), y(), z(), set_x(), distance(), etc.)
    // are NOT listed here -- they're already bound as methods on the type
    // by CXX itself, since they use `self: &gp_Pnt` syntax in ffi.rs.
}
```

Methods that CXX binds directly (using `self: &Type` or `self: Pin<&mut Type>` syntax) are available on the type automatically and do not appear in the `impl` block. Only constructor wrappers, by-value return wrappers, static methods, upcasts, Handle operations, and copy constructors appear in `impl` blocks.

### Internal: `lib.rs` (Module Structure)

```rust
// generated/lib.rs
pub(crate) mod ffi;   // The CXX bridge -- internal only

pub mod gp;           // Re-exports gp_Pnt as Pnt, gp_Vec as Vec, etc.
pub mod topo_ds;      // Re-exports TopoDS_Shape as Shape, etc.
pub mod b_rep_prim_api;
pub mod b_rep_algo_api;
// ... 79 modules total
```
