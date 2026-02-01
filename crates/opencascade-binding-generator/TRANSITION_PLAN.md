# Transition Plan: Moving to Generated Bindings

This document outlines the plan to transition `opencascade-sys` from hand-written bindings to automatically generated bindings using `opencascade-binding-generator`.

## Approach: Parallel Development

Keep the old crate as `opencascade-sys-old` for reference, and build a new `opencascade-sys` using the code generator with pre-generated code.

## Progress

### ✅ Step 1: Generator Enhancements (COMPLETE)

The generator now supports:

1. **Handle<T> support** - Generates typedefs and opaque types for `opencascade::handle<T>`
2. **Enum parsing** - Generates CXX shared enums with `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`
3. **Static functions** - Generates wrapper functions and CXX bindings
4. **Cross-module types** - Generates type aliases like `type gp_Vec = crate::gp::ffi::Vec_;`
5. **Short names** - Types use Rust-friendly names (`Pnt` not `gp_Pnt`) with `#[cxx_name]` mapping
6. **Reserved name handling** - Names like `Vec`, `Box` use trailing underscore (`Vec_`) with re-export aliases
7. **Wrapper functions** - Methods returning by value generate free functions calling C++ wrappers

### ✅ Step 2: Preserve Old Crate (COMPLETE)

Moved `opencascade-sys` to `opencascade-sys-old`:

- Located at `crates/opencascade-sys-old/`
- Package name changed to `opencascade-sys-old` in Cargo.toml
- Workspace Cargo.toml updated to include both crates

### ✅ Step 3: Create New opencascade-sys (COMPLETE)

Created new `opencascade-sys` with generated bindings:

1. **Crate structure:**
   ```
   crates/opencascade-sys/
   ├── Cargo.toml          # Dependencies on cxx, cxx-build
   ├── build.rs            # Compiles pre-generated code with cxx_build
   ├── src/lib.rs          # Includes generated/lib.rs
   ├── headers.txt         # List of OCCT headers (reference)
   └── generated/          # Pre-generated Rust and C++ files
       ├── lib.rs          # Module declarations
       ├── gp.rs           # gp module (Pnt, Vec_, Dir)
       ├── wrapper_gp.hxx  # C++ wrappers for gp
       ├── topo_ds.rs      # TopoDS module (Shape)
       ├── wrapper_topo_ds.hxx
       ├── top_abs.rs      # TopAbs enums
       ├── wrapper_top_abs.hxx
       ├── b_rep_prim_api.rs  # BRepPrimAPI (MakeBox)
       ├── wrapper_b_rep_prim_api.hxx
       └── common.hxx      # Shared utilities
   ```

2. **Current modules generated:**
   - `gp` - Pnt, Vec_ (with re-export as Vec), Dir, XYZ
   - `top_abs` - ShapeEnum, Orientation enums
   - `topo_ds` - Shape
   - `b_rep_prim_api` - MakeBox

3. **Build status:** ✅ Compiles successfully

4. **To regenerate bindings:**
   ```bash
   ./scripts/regenerate-bindings.sh
   ```

### 🔄 Step 4a: Headers Required for `opencascade` Crate

Analysis of `crates/opencascade/src/**` shows these OCCT types are used.
Generate bindings for these headers to enable building `opencascade` crate:

**gp module (geometry primitives):**
- gp_Pnt.hxx, gp_Pnt2d.hxx, gp_Vec.hxx, gp_Dir.hxx, gp_XYZ.hxx
- gp_Ax1.hxx, gp_Ax2.hxx, gp_Trsf.hxx, gp_Lin.hxx, gp_Circ.hxx

**TopAbs module (enums):**
- TopAbs_ShapeEnum.hxx, TopAbs_Orientation.hxx

**TopoDS module (topology):**
- TopoDS_Shape.hxx, TopoDS_Vertex.hxx, TopoDS_Edge.hxx, TopoDS_Wire.hxx
- TopoDS_Face.hxx, TopoDS_Shell.hxx, TopoDS_Solid.hxx, TopoDS_Compound.hxx
- TopoDS.hxx, TopoDS_Builder.hxx

**TopExp module (topology exploration):**
- TopExp_Explorer.hxx

**TopLoc module (locations):**
- TopLoc_Location.hxx

**TopTools module (topology collections):**
- TopTools_HSequenceOfShape.hxx

**Geom/GeomAbs module (geometry):**
- Geom_BezierCurve.hxx, Geom_BezierSurface.hxx
- GeomAbs_JoinType.hxx

**GeomAPI module (geometry algorithms):**
- GeomAPI_Interpolate.hxx, GeomAPI_ProjectPointOnSurf.hxx

**GC module (geometry construction):**
- GC_MakeArcOfCircle.hxx

**GCPnts module (geometry points):**
- GCPnts_TangentialDeflection.hxx

**Bnd module (bounding boxes):**
- Bnd_Box.hxx

**BRep module (boundary representation):**
- BRep_Builder.hxx, BRep_Tool.hxx

**BRepBndLib module:**
- BRepBndLib.hxx

**BRepAdaptor module:**
- BRepAdaptor_Curve.hxx

**BRepBuilderAPI module (shape construction):**
- BRepBuilderAPI_MakeEdge.hxx, BRepBuilderAPI_MakeFace.hxx
- BRepBuilderAPI_MakeVertex.hxx, BRepBuilderAPI_MakeWire.hxx
- BRepBuilderAPI_Transform.hxx

**BRepPrimAPI module (primitive shapes):**
- BRepPrimAPI_MakeBox.hxx, BRepPrimAPI_MakeCone.hxx
- BRepPrimAPI_MakeCylinder.hxx, BRepPrimAPI_MakePrism.hxx
- BRepPrimAPI_MakeRevol.hxx, BRepPrimAPI_MakeSphere.hxx
- BRepPrimAPI_MakeTorus.hxx

**BRepAlgoAPI module (boolean operations):**
- BRepAlgoAPI_Common.hxx, BRepAlgoAPI_Cut.hxx
- BRepAlgoAPI_Fuse.hxx, BRepAlgoAPI_Section.hxx

**BRepFilletAPI module (fillets/chamfers):**
- BRepFilletAPI_MakeChamfer.hxx, BRepFilletAPI_MakeFillet.hxx
- BRepFilletAPI_MakeFillet2d.hxx

**BRepOffsetAPI module (offsets/sweeps):**
- BRepOffsetAPI_MakeOffset.hxx, BRepOffsetAPI_MakePipe.hxx
- BRepOffsetAPI_MakePipeShell.hxx, BRepOffsetAPI_MakeThickSolid.hxx
- BRepOffsetAPI_ThruSections.hxx

**BRepFeat module (features):**
- BRepFeat_MakeCylindricalHole.hxx, BRepFeat_MakeDPrism.hxx

**BRepGProp module (geometric properties):**
- BRepGProp.hxx, BRepGProp_Face.hxx

**BRepIntCurveSurface module (intersection):**
- BRepIntCurveSurface_Inter.hxx

**BRepMesh module (meshing):**
- BRepMesh_IncrementalMesh.hxx

**GProp module (properties):**
- GProp_GProps.hxx

**Law module (functions):**
- Law_Function.hxx, Law_Interpol.hxx

**Poly module (triangulation):**
- Poly_Triangulation.hxx

**ShapeUpgrade module:**
- ShapeUpgrade_UnifySameDomain.hxx

**TColgp module (collections):**
- TColgp_Array1OfDir.hxx, TColgp_Array1OfPnt2d.hxx
- TColgp_Array2OfPnt.hxx, TColgp_HArray1OfPnt.hxx

**Message module:**
- Message_ProgressRange.hxx

**STEPControl module (STEP I/O):**
- STEPControl_Reader.hxx, STEPControl_Writer.hxx

**IGESControl module (IGES I/O):**
- IGESControl_Reader.hxx, IGESControl_Writer.hxx

**StlAPI module (STL I/O):**
- StlAPI_Writer.hxx

**IFSelect module (enum):**
- IFSelect_ReturnStatus.hxx

**Summary:** ~70 headers across ~25 modules. This will require:
1. Expanding the binding generator to handle all these class types
2. Updating `crates/opencascade/src/*.rs` to use new module paths
3. Adding helper functions/casts that the old hand-written bindings provided

### ✅ Step 4b: Feature Parity with opencascade-sys-old (COMPLETE)

Added headers to match all types from `opencascade-sys-old`. Now generating 100 headers:

**Added gp geometry types:**
- gp_Vec2d.hxx, gp_Dir2d.hxx, gp_Ax2d.hxx, gp_Ax3.hxx
- gp_Trsf2d.hxx, gp_GTrsf.hxx, gp_GTrsf2d.hxx, gp_Pln.hxx

**Added TopoDS types:**
- TopoDS_CompSolid.hxx

**Added BRepBuilderAPI types:**
- BRepBuilderAPI_MakeSolid.hxx, BRepBuilderAPI_Sewing.hxx

**Added BRepTools:**
- BRepTools.hxx, TopTools_FormatVersion.hxx

**Added Geom types:**
- Geom2d_Ellipse.hxx, Geom_CylindricalSurface.hxx

**Build status:** ✅ Compiles successfully

---

### Step 5: Update opencascade Crate

Update imports in `crates/opencascade/src/*.rs` to use the new generated bindings.

**Goals:**
- Change existing code as little as possible
- Only update identifiers and import paths
- If refactoring or more significant changes are required, STOP and ask the user

**Translation rules:**

| Old Pattern | New Pattern |
|-------------|-------------|
| `use opencascade_sys::ffi::gp_Pnt` | `use opencascade_sys::gp::Pnt` |
| `use opencascade_sys::ffi::TopAbs_ShapeEnum` | `use opencascade_sys::top_abs::ShapeEnum` |
| `ffi::gp_Pnt::new(...)` | `gp::Pnt::new(...)` |
| `ffi::BRepPrimAPI_MakeBox_ctor(...)` | `b_rep_prim_api::MakeBox::new(...)` (check exact name) |

**Approach:**
1. Start with one file at a time
2. Update imports at the top of the file
3. Update type references in the code
4. Build and fix errors iteratively
5. Document any patterns that aren't straightforward

**When to STOP and ask:**
- A type or method doesn't exist in the new bindings
- The API has changed in a way that requires logic changes
- Cast/conversion helpers from old bindings are missing
- Any change that isn't a simple identifier rename

**Translation learnings (update as you go):**

_Document patterns discovered during translation here. Refer back to this section when translation is unclear._

```
Example format:
- OLD: `ffi::some_helper_function(&shape)` 
  NEW: `shape.method()` or requires wrapper
  NOTES: Why this changed, what to watch for
```

**Progress:**
- [ ] `lib.rs`
- [ ] `primitives.rs`
- [ ] `edge.rs`
- [ ] `wire.rs`
- [ ] `face.rs`
- [ ] `shell.rs`
- [ ] `solid.rs`
- [ ] `compound.rs`
- [ ] `shape.rs`
- [ ] (other files as discovered)

### Step 6: Update Examples

Update `examples/src/*.rs` for new API.

### Step 7: Cleanup

Delete `opencascade-sys-old` once everything works.

### Step 8: Expand Header Coverage

Add more OCCT headers as needed. Current coverage (100 headers):

**gp module** (18 headers):
- gp_Pnt, gp_Pnt2d, gp_Vec, gp_Vec2d, gp_Dir, gp_Dir2d, gp_XYZ
- gp_Ax1, gp_Ax2, gp_Ax2d, gp_Ax3, gp_Trsf, gp_Trsf2d
- gp_GTrsf, gp_GTrsf2d, gp_Lin, gp_Circ, gp_Pln

**TopAbs module** (2 enums):
- TopAbs_ShapeEnum, TopAbs_Orientation

**TopoDS module** (10 headers):
- TopoDS_Shape, TopoDS_Vertex, TopoDS_Edge, TopoDS_Wire
- TopoDS_Face, TopoDS_Shell, TopoDS_Solid, TopoDS_Compound
- TopoDS_CompSolid, TopoDS_Builder

**TopExp module** (1 header):
- TopExp_Explorer

**TopLoc module** (1 header):
- TopLoc_Location

**BRepBuilderAPI module** (7 headers):
- BRepBuilderAPI_MakeEdge, BRepBuilderAPI_MakeFace, BRepBuilderAPI_MakeSolid
- BRepBuilderAPI_MakeVertex, BRepBuilderAPI_MakeWire
- BRepBuilderAPI_Sewing, BRepBuilderAPI_Transform

**BRepPrimAPI module** (7 headers):
- BRepPrimAPI_MakeBox, BRepPrimAPI_MakeCone, BRepPrimAPI_MakeCylinder
- BRepPrimAPI_MakePrism, BRepPrimAPI_MakeRevol
- BRepPrimAPI_MakeSphere, BRepPrimAPI_MakeTorus

**BRepAlgoAPI module** (4 headers):
- BRepAlgoAPI_Common, BRepAlgoAPI_Cut, BRepAlgoAPI_Fuse, BRepAlgoAPI_Section

**BRepFilletAPI module** (3 headers):
- BRepFilletAPI_MakeChamfer, BRepFilletAPI_MakeFillet, BRepFilletAPI_MakeFillet2d

**BRepOffsetAPI module** (5 headers):
- BRepOffsetAPI_MakeOffset, BRepOffsetAPI_MakePipe, BRepOffsetAPI_MakePipeShell
- BRepOffsetAPI_MakeThickSolid, BRepOffsetAPI_ThruSections

**BRepFeat module** (2 headers):
- BRepFeat_MakeCylindricalHole, BRepFeat_MakeDPrism

**BRepTools module** (2 headers):
- BRepTools, TopTools_FormatVersion

**Geom module** (7 headers):
- Geom_BezierCurve, Geom_BSplineCurve, Geom_Curve
- Geom_CylindricalSurface, Geom_Plane, Geom_Surface, Geom_TrimmedCurve

**Geom2d module** (3 headers):
- Geom2d_Curve, Geom2d_Ellipse, Geom2d_TrimmedCurve

**GeomAPI module** (4 headers):
- GeomAPI_Interpolate, GeomAPI_PointsToBSpline
- GeomAPI_ProjectPointOnCurve, GeomAPI_ProjectPointOnSurf

**GC module** (2 headers):
- GC_MakeArcOfCircle, GC_MakeSegment

**Bnd module** (2 headers):
- Bnd_Box, Bnd_OBB

**BRep module** (1 header):
- BRep_Tool

**BRepGProp module** (1 header):
- BRepGProp

**BRepMesh module** (1 header):
- BRepMesh_IncrementalMesh

**GProp module** (1 header):
- GProp_GProps

**Law module** (2 headers):
- Law_Function, Law_Interpol

**Poly module** (1 header):
- Poly_Triangulation

**ShapeUpgrade module** (1 header):
- ShapeUpgrade_UnifySameDomain

**TColgp module** (5 headers):
- TColgp_Array1OfDir, TColgp_Array1OfPnt, TColgp_Array1OfPnt2d
- TColgp_Array2OfPnt, TColgp_HArray1OfPnt

**Message module** (1 header):
- Message_ProgressRange

**STEPControl module** (2 headers):
- STEPControl_Reader, STEPControl_Writer

**IGESControl module** (2 headers):
- IGESControl_Reader, IGESControl_Writer

**StlAPI module** (1 header):
- StlAPI_Writer

**IFSelect module** (1 enum):
- IFSelect_ReturnStatus

## Module Priority

Generate in this order (based on dependencies):

1. **gp** - Points, vectors, transforms (no deps)
2. **TopAbs** - Enums (no deps)  
3. **TopoDS** - Shape types (deps: TopAbs)
4. **TopLoc** - Locations (deps: gp)
5. **TopExp** - Explorers (deps: TopoDS, TopAbs)
6. **Geom/Geom2d** - Curves, surfaces (deps: gp)
7. **GC/GCE2d** - Curve makers (deps: Geom, gp)
8. **BRepBuilderAPI** - Edge, wire, face makers (deps: TopoDS, Geom)
9. **BRepPrimAPI** - Primitives (deps: TopoDS, gp)
10. **BRepAlgoAPI** - Boolean ops (deps: TopoDS)
11. **BRepFilletAPI** - Fillets (deps: TopoDS)
12. **BRepOffsetAPI** - Offsets, thicken (deps: TopoDS)
13. **Remaining** - As needed

---

## Technical Notes (Step 3 Implementation)

### Key Generator Improvements

During Step 3 implementation, several generator improvements were made:

1. **Reserved name handling**: Rust reserved names (`Vec`, `Box`, `String`, `Result`, `Option`) get a trailing underscore in the ffi module (e.g., `Vec_`), then are re-exported with aliases:
   ```rust
   pub use ffi::Vec_ as Vec;
   ```

2. **Wrapper functions for by-value returns**: CXX cannot directly call C++ methods that return class types by value (return type mismatch). Solution: generate free functions that call C++ wrapper functions:
   ```rust
   // Instead of: fn mirrored(self: &Pnt, p: &Pnt) -> UniquePtr<Pnt>  // calls gp_Pnt::Mirrored
   // Generate:   fn Pnt_mirrored(self_: &Pnt, p: &Pnt) -> UniquePtr<Pnt>  // calls gp_Pnt_Mirrored wrapper
   ```

3. **C++ keyword avoidance**: Use `self_` instead of `this` for self parameters in wrapper functions.

4. **Header name collision avoidance**: Generated headers use `wrapper_` prefix (`wrapper_gp.hxx`) to avoid collision with OCCT's own headers (e.g., `gp.hxx`).

5. **C++14 standard**: Required for `std::make_unique` in generated wrapper functions.

6. **Cross-module type aliases**: When referencing types from other modules, use the safe internal name:
   ```rust
   type gp_Vec = crate::gp::ffi::Vec_;  // Not Vec (which is the re-export alias)
   ```

7. **Enum returns don't need wrappers**: Methods returning enum types (like `Orientation()` returning `TopAbs_Orientation`) don't need wrapper functions because CXX handles shared enums directly. The `needs_wrapper_function` check was updated to exclude enum return types.

### Generated API Example

```rust
use opencascade_sys::gp::Pnt;

// Create a point at (1, 2, 3)
let p = Pnt::new_real3(1.0, 2.0, 3.0);

// Access coordinates
println!("x = {}", p.x());
```

### Ported Example

A complete ported OCCT sample is available at `crates/opencascade-sys/examples/point_info_3d.rs`.
This is a literal translation of `GeometrySamples::PointInfo3dSample()` from the OCCT samples.

Run with: `cargo run -p opencascade-sys --example point_info_3d`

### Build Configuration

- **Pre-generated code**: Bindings are pre-generated (not at build time) due to libclang DYLD_LIBRARY_PATH issues in build scripts
- **C++ standard**: C++14 (`-std=c++14`)
- **Include paths**: OCCT headers + generated directory for wrapper headers
