# Porting C++ OCCT Code to Rust with opencascade-sys

This guide covers translating C++ code that uses OpenCASCADE Technology (OCCT)
into Rust using the `opencascade-sys` crate.

## Module Structure

OCCT C++ classes like `BRepBuilderAPI_MakeEdge` are re-exported in Rust modules
derived from the package prefix. The module name is the snake_case form of the
C++ package, and the type name is the suffix after the first underscore:

| C++ Class                      | Rust Path                              |
|--------------------------------|----------------------------------------|
| `gp_Pnt`                       | `gp::Pnt`                             |
| `BRepBuilderAPI_MakeEdge`      | `b_rep_builder_api::MakeEdge`          |
| `BRepOffsetAPI_MakeThickSolid` | `b_rep_offset_api::MakeThickSolid`     |
| `TopoDS_Shape`                 | `topo_ds::Shape`                       |
| `Geom_CylindricalSurface`      | `geom::CylindricalSurface`            |
| `Geom2d_Ellipse`               | `geom2d::Ellipse`                     |
| `GC_MakeSegment`               | `gc::MakeSegment`                     |

## Constructors

C++ constructors map to `new_*` associated functions. The suffix encodes the
parameter types to disambiguate overloads:

```cpp
// C++
gp_Pnt aPnt(1.0, 2.0, 3.0);
GC_MakeSegment aSegment(aPnt1, aPnt2);
BRepBuilderAPI_MakeWire aWire(edge1, edge2, edge3);
```

```rust
// Rust
let a_pnt = gp::Pnt::new_real3(1.0, 2.0, 3.0);
let a_segment = gc::MakeSegment::new_pnt2(&a_pnt1, &a_pnt2);
let mut a_wire = b_rep_builder_api::MakeWire::new_edge3(
    edge1.pin_mut().edge(),
    edge2.pin_mut().edge(),
    edge3.pin_mut().edge(),
);
```

The default (no-argument) constructor is just `::new()`:

```rust
let mut trsf = gp::Trsf::new();
let mut compound = topo_ds::Compound::new();
```

### Constructors with Default Arguments

C++ constructors often have trailing default parameters. The Rust API currently
only exposes the fully-specified variant. You must pass all arguments explicitly:

```cpp
// C++: BRepBuilderAPI_Transform(shape, trsf)  — copy defaults to false
BRepBuilderAPI_Transform aBRepTrsf(aShape, aTrsf);
```

```rust
// Rust: must pass all args including defaults
let mut brep_transform = b_rep_builder_api::Transform::new_shape_trsf_bool2(
    wire.pin_mut().shape(), &trsf, false, false,
);
```

## Ownership and Pinning

All OCCT objects are returned as `cxx::UniquePtr<T>`. Calling methods that
mutate the object requires `pin_mut()`:

```rust
let mut trsf = gp::Trsf::new();
trsf.pin_mut().set_mirror_ax1(&x_axis);  // mutating method needs pin_mut()

let name = type_obj.name();  // non-mutating method works on &self
```

Methods that return references to internal state (like `.shape()`, `.wire()`,
`.edge()`, `.face()`) are CXX auto-methods that work through `pin_mut()`:

```rust
let shape = body.pin_mut().shape();     // returns &TopoDS_Shape
let wire = make_wire.pin_mut().wire();  // returns &TopoDS_Wire
```

## Handle Types (Reference-Counted Smart Pointers)

OCCT's `Handle(T)` maps to `UniquePtr<HandleT>` in Rust (e.g.,
`Handle(Geom_Curve)` → `UniquePtr<HandleGeomCurve>`).

### Creating Handles

`to_handle()` is an associated function (not a method) that consumes a
`UniquePtr<T>` and returns `UniquePtr<HandleT>`. This is because
`self: UniquePtr<Self>` receivers require Rust's unstable `arbitrary_self_types`
feature.

```cpp
// C++: Handle is created implicitly with new
Handle(Geom_CylindricalSurface) aCyl = new Geom_CylindricalSurface(anAx3, r);
```

```rust
// Rust: create object, then convert to handle (consumes the object)
let cyl = geom::CylindricalSurface::new_ax3_real(&an_ax3, radius);
let handle_cyl = geom::CylindricalSurface::to_handle(cyl);
// `cyl` is now consumed and cannot be used
```

### Handle Upcasting

Use `to_handle_*()` methods to upcast handles in the inheritance hierarchy:

```rust
// HandleGeomCylindricalSurface → HandleGeomSurface
let handle_surface = handle_cyl.to_handle_surface();

// HandleGeomTrimmedCurve → HandleGeomCurve
let handle_curve = segment.value().to_handle_curve();
```

These are `&self` methods, so the original handle remains usable.

### Handle Dereferencing

Use `.get()` to access the underlying object through a Handle:

```cpp
// C++: Handle auto-dereferences
gp_Pnt2d p = anEllipseHandle->Value(0);
```

```rust
// Rust: explicit .get() dereference
let p = handle_ellipse.get().value(0.0);
```

### Handle Downcasting

Handle downcasting (e.g., `Handle(Geom_Surface)` → `Handle(Geom_Plane)`) is
not yet supported by the binding generator. As a workaround, you can use an
unsafe pointer cast after confirming the dynamic type:

```rust
let surface_ref = surface_handle.get();
let dynamic_type = surface_ref.dynamic_type();
let type_obj = dynamic_type.get();
let name = type_obj.name();

if name == "Geom_Plane" {
    // TODO: Use proper Handle downcast once the generator supports it.
    // This is safe because DynamicType() confirms the concrete type,
    // and the types share object layout via inheritance.
    let plane: &geom::Plane =
        unsafe { &*(surface_ref as *const geom::Surface as *const geom::Plane) };
    let location = plane.location();
}
```

## Enums

OCCT uses unscoped C++ enums, which CXX cannot bind directly. The binding
generator creates Rust enums in the appropriate modules and you cast to `i32`
when calling FFI functions:

```cpp
// C++
TopAbs_ShapeEnum::TopAbs_EDGE
ChFi3d_Rational
```

```rust
// Rust
top_abs::ShapeEnum::Edge as i32
ch_fi3d::FilletShape::Rational as i32
```

## Collections

Collection types like `TopTools_ListOfShape` have `new()`, `append()`,
`prepend()`, `size()`, and `clear()` methods:

```cpp
// C++
TopTools_ListOfShape aList;
aList.Append(aFace);
```

```rust
// Rust
let mut faces = top_tools::ListOfShape::new();
faces.pin_mut().append(a_face.as_shape());
```

## TopoDS Shape Casting

OCCT uses free functions to downcast `TopoDS_Shape` references into specific
subtypes. These are available as module-level functions:

```cpp
// C++
const TopoDS_Edge& anEdge = TopoDS::Edge(aShape);
const TopoDS_Wire& aWire = TopoDS::Wire(aShape);
const TopoDS_Face& aFace = TopoDS::Face(aShape);
```

```rust
// Rust
let an_edge = topo_ds::edge(a_shape);         // returns &TopoDS_Edge
let a_wire = topo_ds::wire(a_shape);           // returns &TopoDS_Wire
let a_face = topo_ds::face(a_shape);           // returns &TopoDS_Face
```

## Shape Inheritance (as_shape / as_shape_mut)

TopoDS subtypes (Edge, Wire, Face, Solid, Compound, etc.) inherit from
`TopoDS_Shape`. Use `as_shape()` and `as_shape_mut()` to get a Shape reference:

```rust
let shape_ref: &topo_ds::Shape = compound.as_shape();
let shape_mut: Pin<&mut topo_ds::Shape> = compound.pin_mut().as_shape_mut();
```

## Static Methods

C++ static methods become associated functions on the Rust type:

```cpp
// C++
Handle(Geom_Surface) aSurf = BRep_Tool::Surface(aFace);
BRepLib::BuildCurves3d(aWire);
```

```rust
// Rust
let a_surf = b_rep::Tool::surface_face(a_face);
b_rep_lib::BRepLib::build_curves3d_shape(a_wire.pin_mut().shape());
```

## Message_ProgressRange

Several OCCT operations (Boolean operations, meshing, thick solid) require a
`Message_ProgressRange` parameter that doesn't exist in older tutorials. Create
a default one:

```rust
let progress = message::ProgressRange::new();
let mut fuse = b_rep_algo_api::Fuse::new_shape2_progressrange(
    body_shape, cylinder_shape, &progress,
);
```

## Complete Translation Example

Here is a side-by-side comparison of a typical sequence:

```cpp
// C++ — Create a cylinder and fuse it with a body
gp_Ax2 neckAx2(gp_Pnt(0, 0, height), gp::DZ());
BRepPrimAPI_MakeCylinder MKCyl(neckAx2, neckR, neckH);
BRepAlgoAPI_Fuse mkFuse(myBody, MKCyl.Shape());
TopoDS_Shape myBody = mkFuse.Shape();
```

```rust
// Rust
let neck_ax2 = gp::Ax2::new_pnt_dir(
    &gp::Pnt::new_real3(0.0, 0.0, height),
    &gp::Dir::new_real3(0.0, 0.0, 1.0),
);
let mut mk_cyl = b_rep_prim_api::MakeCylinder::new_ax2_real2(
    &neck_ax2, neck_r, neck_h,
);
let progress = message::ProgressRange::new();
let mut mk_fuse = b_rep_algo_api::Fuse::new_shape2_progressrange(
    my_body, mk_cyl.pin_mut().shape(), &progress,
);
let my_body = mk_fuse.pin_mut().shape();
```

## Known Limitations

- **`to_handle()` is a static function**, not a method — use
  `Type::to_handle(obj)` instead of `obj.to_handle()`. This is due to Rust's
  `arbitrary_self_types` feature not being stable yet.
- **No Handle downcasting** — use unsafe pointer casts after dynamic type
  checks as a workaround.
- **No standalone package functions** — `gp::OX()`, `gp::DZ()`, etc. must be
  constructed manually (e.g., `gp::Dir::new_real3(0.0, 0.0, 1.0)` for DZ).
- **No default-argument shorthand** — all constructor parameters must be
  specified explicitly, including those with C++ defaults.
- **Enum parameters are `i32`** — cast Rust enums with `as i32`.
