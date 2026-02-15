# Porting C++ OCCT Code to Rust with opencascade-sys

This guide covers translating C++ code that uses OpenCASCADE Technology (OCCT)
into Rust using the `opencascade-sys` crate.

## Module Structure

OCCT C++ classes like `BRepBuilderAPI_MakeEdge` are re-exported in Rust modules
derived from the package prefix. The module name is the snake_case form of the
C++ package, and the type name is the suffix after the first underscore:

| C++ Class                      | Header                              | Rust Path                              |
|--------------------------------|-------------------------------------|----------------------------------------|
| `gp_Pnt`                       | `gp_Pnt.hxx`                       | `gp::Pnt`                             |
| `BRepBuilderAPI_MakeEdge`      | `BRepBuilderAPI_MakeEdge.hxx`       | `b_rep_builder_api::MakeEdge`          |
| `BRepOffsetAPI_MakeThickSolid` | `BRepOffsetAPI_MakeThickSolid.hxx`  | `b_rep_offset_api::MakeThickSolid`     |
| `BRepOffsetSimple_Status`      | `BRepOffset_MakeSimpleOffset.hxx`   | `b_rep_offset::SimpleStatus`           |
| `TopoDS_Shape`                 | `TopoDS_Shape.hxx`                  | `topo_ds::Shape`                       |
| `Geom_CylindricalSurface`      | `Geom_CylindricalSurface.hxx`      | `geom::CylindricalSurface`            |
| `Geom2d_Ellipse`               | `Geom2d_Ellipse.hxx`               | `geom2d::Ellipse`                     |
| `GC_MakeSegment`               | `GC_MakeSegment.hxx`               | `gc::MakeSegment`                     |

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

C++ constructors with trailing default parameters have convenience wrappers
that omit the defaulted arguments. You can use either the short or full form:

```cpp
// C++: BRepBuilderAPI_Transform(shape, trsf)  — copy defaults to false
BRepBuilderAPI_Transform aBRepTrsf(aShape, aTrsf);
```

```rust
// Rust: convenience wrapper fills in defaults
let mut brep_transform = b_rep_builder_api::Transform::new_shape_trsf(
    wire.pin_mut().shape(), &trsf,
);

// Or explicitly pass all args
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
generator creates Rust enums in the appropriate modules with `From<EnumType>
for i32` and `TryFrom<i32> for EnumType` conversions.

**Most enum parameters now use typed Rust enums directly.** The generated
wrapper functions handle the `i32` conversion at the FFI boundary, so you
pass and receive Rust enum values:

```cpp
// C++
TopExp_Explorer explorer(shape, TopAbs_EDGE, TopAbs_SHAPE);
BRepFilletAPI_MakeFillet fillet(shape, ChFi3d_Rational);
```

```rust
// Rust — pass enum values directly
let explorer = top_exp::Explorer::new_shape_shapeenum2(
    shape,
    top_abs::ShapeEnum::Edge,
    top_abs::ShapeEnum::Shape,
);
let fillet = b_rep_fillet_api::MakeFillet::new_shape_filletshape(
    shape,
    ch_fi3d::FilletShape::Rational,
);
```

Return values are also typed enums:

```rust
let orientation: top_abs::Orientation = shape.orientation();
let curve_type: geom_abs::CurveType = curve.get_type();
```

**Bitset enums** (those used as bitmasks, e.g. names containing "Flag" or
"Mask", or enums whose values are powers of 2) remain as `i32` and still
require manual conversion:

```rust
// Bitset enums still use i32
some_function(my_flags as i32);
```

Convert from `i32` back to enum with `TryFrom`:

```rust
let shape_enum = top_abs::ShapeEnum::try_from(raw_i32)
    .expect("Invalid ShapeEnum value");
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

`TopTools_IndexedMapOfShape` provides deduplicated, indexed access to shapes.
Use `top_exp::map_shapes()` to populate it with sub-shapes of a given type:

```cpp
// C++
TopTools_IndexedMapOfShape vertexMap;
TopExp::MapShapes(shape, TopAbs_VERTEX, vertexMap);
for (int i = 1; i <= vertexMap.Extent(); ++i) {
    const TopoDS_Vertex& v = TopoDS::Vertex(vertexMap.FindKey(i));
}
```

```rust
// Rust
let mut vertex_map = top_tools::IndexedMapOfShape::new();
top_exp::map_shapes(shape, top_abs::ShapeEnum::Vertex, vertex_map.pin_mut());
for i in 1..=vertex_map.size() {
    let v = topo_ds::vertex(vertex_map.find_key(i));
}
```

Note: prefer `map_shapes` over `TopExp_Explorer` when you need unique shapes.
`Explorer` may visit the same vertex/edge multiple times (once per adjacent
element), which can cause issues with operations like `MakeFillet2d::AddFillet`
that must receive each vertex exactly once.

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
```

```rust
// Rust
let a_surf = b_rep::Tool::surface_face(a_face);
```

## Utility Class Functions

OCCT "utility classes" (classes with only static methods and no instance
methods) are automatically converted to module-level free functions:

```cpp
// C++
BRepLib::BuildCurves3d(aWire);
BRepBndLib::Add(aShape, aBox, useTriangulation);
BRepGProp::SurfaceProperties(aShape, props);
gp_Pnt origin = gp::Origin();
gp_Ax1 ox = gp::OX();
```

```rust
// Rust
b_rep_lib::build_curves3d(a_wire.pin_mut().as_shape());
b_rep_bnd_lib::add(a_shape, a_box.pin_mut(), true);
b_rep_g_prop::surface_properties(a_shape, props.pin_mut(), false, false);
let origin = gp::origin();
let ox = gp::ox();
```

## Default-Argument Convenience Constructors

When a C++ constructor has trailing parameters with default values, convenience
wrappers are generated that omit those parameters:

```cpp
// C++ — defaults: Copy=Standard_False, CopyMesh=Standard_False
BRepBuilderAPI_Transform T(S, trsf);  // uses defaults
BRepBuilderAPI_Transform T(S, trsf, true, false);  // explicit
```

```rust
// Rust
let t = b_rep_builder_api::Transform::new_shape_trsf(&s, &trsf);    // uses defaults
let t = b_rep_builder_api::Transform::new_shape_trsf_bool2(&s, &trsf, true, false);  // explicit
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
    &gp::dz(),
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
  checks as a workaround (see Handle Downcasting section above). The
  `get_type_name()` method is available on all `Standard_Transient`-derived
  classes for RTTI type identification.
- **Bitset/flag enums remain `i32`** — most enum parameters now use typed Rust
  enums, but enums used as bitmasks (names containing "Flag" or "Mask", or
  values that are powers of 2) are still `i32`.
- **Some methods with `&mut` enum out-params use `&mut i32`** — when a method
  has an enum output parameter (e.g., `TopAbs_State& state`), the Rust
  signature uses `&mut i32`. Convert back with `TryFrom`.

## RTTI Type Identification

All classes inheriting from `Standard_Transient` have a `get_type_name()`
method that returns the C++ class name as a `String`. This is useful for
Handle downcasting type checks and debugging:

```rust
let surface_handle = b_rep::Tool::surface_face(face);
let type_name = surface_handle.get().get_type_name();
if type_name == "Geom_Plane" {
    // Safe to downcast
}
```

## Inherited Methods on TopoDS Subtypes

Most `TopoDS_Shape` methods are available directly on subtypes (`Face`, `Edge`,
`Wire`, `Shell`, `Solid`, `Compound`) as inherited methods — no `as_shape()`
needed:

```rust
// These work directly on Face, Edge, etc.
face.is_null()
face.location()
face.pin_mut().move_(&location, false)
face.pin_mut().reverse()
face.reversed()
face.nb_children()
face.is_same(other_shape)
face.orientation()           // i32 (TopAbs_Orientation)
face.shape_type()            // i32 (TopAbs_ShapeEnum)
face.oriented(0)             // returns UniquePtr<Shape>
face.pin_mut().compose(0)    // mutating: compose orientation
face.composed(0)             // returns UniquePtr<Shape>
// ... and: nullify, located, t_shape, free, locked, modified, checked,
//          orientable, closed, infinite, convex, moved, complement,
//          complemented, is_partner, is_equal, is_not_equal, empty_copy,
//          empty_copied, orientation_orientation
```

## Where Methods Live: Module Files vs ffi.rs

Methods in the generated bindings come from two sources:

1. **Wrapper methods** — defined in the per-module `.rs` files (e.g.,
   `generated/topo_ds.rs`, `generated/b_rep_builder_api.rs`). These appear as
   `impl` blocks and call into `crate::ffi::*` functions. You can browse them
   in the module files directly.

2. **CXX auto-methods** — methods with a `self:` receiver in `ffi.rs`. These
   are generated by CXX itself and do NOT appear in the module `.rs` files.
   They are part of the public API but are only visible in `generated/ffi.rs`.

To find all available methods on a type, check both:
- The `impl TypeName` block in the module file (wrapper methods + inherited methods)
- The CXX bridge section for that type in `ffi.rs` (auto-methods with `self:` receiver)

## Handle Chaining for Law Functions

Some OCCT patterns require chaining through multiple handle conversions:

```rust
// Create a Law_Interpol, convert to Handle(Law_Interpol), then to Handle(Law_Function)
let mut interpol = law::Interpol::new();
interpol.pin_mut().set_array1ofpnt2d_bool(&array, false);
let handle = law::Interpol::to_handle(interpol);  // Handle(Law_Interpol)
let law_handle = handle.to_handle_function();       // Handle(Law_Function)
```
