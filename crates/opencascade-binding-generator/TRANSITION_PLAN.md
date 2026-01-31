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
   - `gp` - Pnt, Vec_ (with re-export as Vec), Dir
   - `topo_ds` - Shape
   - `top_abs` - ShapeEnum, Orientation enums
   - `b_rep_prim_api` - MakeBox

3. **Build status:** ✅ Compiles successfully

4. **To regenerate bindings:**
   ```bash
   DYLD_LIBRARY_PATH=/Library/Developer/CommandLineTools/usr/lib cargo run -p opencascade-binding-generator -- \
     -I target/OCCT/include \
     -o crates/opencascade-sys/generated \
     target/OCCT/include/gp_Pnt.hxx \
     target/OCCT/include/gp_Vec.hxx \
     target/OCCT/include/gp_Dir.hxx \
     target/OCCT/include/TopAbs_ShapeEnum.hxx \
     target/OCCT/include/TopAbs_Orientation.hxx \
     target/OCCT/include/TopoDS_Shape.hxx \
     target/OCCT/include/BRepPrimAPI_MakeBox.hxx
   ```

### 🔄 Step 4: Feature Parity (IN PROGRESS)

Expand headers to cover all modules from `opencascade-sys-old`:

**Headers to add (derived from opencascade-sys-old wrappers):**

From `include/wrappers/gp.hxx`:
- gp_Pnt.hxx, gp_Pnt2d.hxx, gp_Vec.hxx, gp_Vec2d.hxx, gp_Dir.hxx, gp_Dir2d.hxx
- gp_Ax1.hxx, gp_Ax2.hxx, gp_Ax2d.hxx, gp_Ax3.hxx, gp_Trsf.hxx, gp_Trsf2d.hxx
- gp_GTrsf.hxx, gp_GTrsf2d.hxx, gp_Pln.hxx, gp_Circ.hxx

From `include/wrappers/Bnd.hxx`:
- Bnd_Box.hxx

From `include/wrappers/TopAbs.hxx`:
- TopAbs_ShapeEnum.hxx, TopAbs_Orientation.hxx

From `include/wrappers/TopoDS.hxx`:
- TopoDS_Shape.hxx, TopoDS_Vertex.hxx, TopoDS_Edge.hxx, TopoDS_Wire.hxx
- TopoDS_Face.hxx, TopoDS_Shell.hxx, TopoDS_Solid.hxx, TopoDS_Compound.hxx
- TopoDS_CompSolid.hxx, TopoDS.hxx, TopoDS_Builder.hxx

From `include/wrappers/TopLoc.hxx`:
- TopLoc_Location.hxx

From `include/wrappers/TopExp.hxx`:
- TopExp_Explorer.hxx

From `include/wrappers/Geom.hxx`:
- Geom_Curve.hxx, Geom_Surface.hxx, Geom_BSplineCurve.hxx, Geom_TrimmedCurve.hxx
- Geom_Plane.hxx, Geom_CylindricalSurface.hxx, etc.

From `include/wrappers/Geom2d.hxx`:
- Geom2d_Curve.hxx, Geom2d_Ellipse.hxx, Geom2d_TrimmedCurve.hxx, etc.

From `include/wrappers/GC.hxx`:
- GC_MakeSegment.hxx, GC_MakeArcOfCircle.hxx, etc.

From `include/wrappers/BRepBuilderAPI.hxx`:
- BRepBuilderAPI_MakeEdge.hxx, BRepBuilderAPI_MakeWire.hxx
- BRepBuilderAPI_MakeFace.hxx, BRepBuilderAPI_MakeSolid.hxx
- BRepBuilderAPI_Transform.hxx, BRepBuilderAPI_Sewing.hxx

From `include/wrappers/BRepPrimAPI.hxx`:
- BRepPrimAPI_MakeBox.hxx, BRepPrimAPI_MakeCylinder.hxx
- BRepPrimAPI_MakeSphere.hxx, BRepPrimAPI_MakeCone.hxx
- BRepPrimAPI_MakePrism.hxx, BRepPrimAPI_MakeRevol.hxx

From `include/wrappers/BRepAlgoAPI.hxx`:
- BRepAlgoAPI_Fuse.hxx, BRepAlgoAPI_Cut.hxx, BRepAlgoAPI_Common.hxx
- BRepAlgoAPI_Section.hxx

From `include/wrappers/BRepFilletAPI.hxx`:
- BRepFilletAPI_MakeFillet.hxx, BRepFilletAPI_MakeChamfer.hxx

From `include/wrappers/BRepOffsetAPI.hxx`:
- BRepOffsetAPI_MakeThickSolid.hxx, BRepOffsetAPI_MakePipe.hxx
- BRepOffsetAPI_MakePipeShell.hxx, BRepOffsetAPI_MakeOffset.hxx
- BRepOffsetAPI_ThruSections.hxx

From `include/wrappers/BRepMesh.hxx`:
- BRepMesh_IncrementalMesh.hxx

From `include/wrappers/BRepTools.hxx`:
- BRepTools.hxx

From `include/wrappers/STEPControl.hxx`:
- STEPControl_Reader.hxx, STEPControl_Writer.hxx

From `include/wrappers/IGESControl.hxx`:
- IGESControl_Reader.hxx, IGESControl_Writer.hxx

From `include/wrappers/StlAPI.hxx`:
- StlAPI_Writer.hxx

### Step 5: Update opencascade Crate

Update imports in `crates/opencascade/src/*.rs`:
- Old: `use opencascade_sys::ffi::gp_Pnt`
- New: `use opencascade_sys::gp::Pnt` (short names, no ffi)

### Step 6: Update Examples

Update `examples/src/*.rs` for new API.

### Step 7: Cleanup

Delete `opencascade-sys-old` once everything works.

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

### Generated API Example

```rust
use opencascade_sys::gp::{Pnt, Vec, Pnt_ctor_real_real_real};

// Create a point at (1, 2, 3)
let p = Pnt_ctor_real_real_real(1.0, 2.0, 3.0);

// Access coordinates
println!("x = {}", p.x());
```

### Build Configuration

- **Pre-generated code**: Bindings are pre-generated (not at build time) due to libclang DYLD_LIBRARY_PATH issues in build scripts
- **C++ standard**: C++14 (`-std=c++14`)
- **Include paths**: OCCT headers + generated directory for wrapper headers
