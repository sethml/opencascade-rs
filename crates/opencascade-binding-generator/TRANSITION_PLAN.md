# Transition Plan: Moving to Generated Bindings

This document tracks the migration of `opencascade-sys` from hand-written bindings to auto-generated bindings, and the `opencascade` crate's adaptation to the new API.

## Reference: Old vs New Code

The old hand-written bindings are preserved in `crates/opencascade-sys-old/` for reference. To compare:

```bash
# Reference commit before binding generator work
git show 14fca36:crates/opencascade/src/primitives/solid.rs

# Diff a specific file
git diff 14fca36 -- crates/opencascade/src/primitives/shape.rs
```

## Completed Steps

### Steps 1-3: Generator, Old Crate Preservation, New opencascade-sys

The generator was built, old bindings moved to `opencascade-sys-old`, and a new `opencascade-sys` created with generated code in `generated/`.

### Step 4a: Header Coverage

262 explicit headers in `headers.txt` expand to 378 via automatic dependency resolution (`--resolve-deps`). The generator produces 79 modules.

### Step 4b: Collection Support

NCollection typedefs (ListOfShape, SequenceOfShape, IndexedMapOfShape, etc.) get auto-generated iterator wrappers in `codegen/collections.rs`. Hand-written collection code has been removed.

### Step 4c: Feature Parity Headers

All headers needed by `crates/opencascade/src/` are included.

### Step 4d: Inherited Methods

Inherited methods are emitted via C++ wrapper functions (not direct CXX method declarations, which fail due to method pointer type mismatches).

### Step 4e: Handle Wrapping and Upcasting

`to_handle()` associated functions and Handle upcast methods are generated for all `Standard_Transient`-derived classes.

### Step 4f: Abstract Class Handling

Abstract classes detected via `is_pure_virtual_method()` -- no constructors, `to_handle()`, or `to_owned()` generated for them.

### Step 4g: Build Issue Resolution

Handle naming, inherited method pointer issues, and filter consistency between FFI and impl generation all resolved.

### Step 4h: Two-Pass Architecture (SymbolTable)

`resolver.rs` builds a `SymbolTable` with all filter decisions centralized. Both `rust.rs` and `cpp.rs` consume it. `--dump-symbols` flag available for debugging.

### Step 4i: Unified FFI Architecture

Single `ffi.rs` with all types (full C++ names), single `wrappers.hxx`, per-module re-export files. Now the default via `--unified` in `regenerate-bindings.sh`.

### Step 8: Header Coverage Expansion

Automatic dependency resolution handles this. 262 explicit headers -> 378 total.

---

## Current Work

### Step 5: Update opencascade Crate (IN PROGRESS -- COMPILES)

The `opencascade` crate compiles with the new bindings. Many methods work fully, some are stubbed due to generator limitations.

**Translation rules:**

| Old Pattern | New Pattern |
|-------------|-------------|
| `use opencascade_sys::ffi::gp_Pnt` | `use opencascade_sys::gp::Pnt` |
| `ffi::gp_Pnt::new(...)` | `gp::Pnt::new_real3(...)` |
| `ffi::TopAbs_ShapeEnum::TopAbs_VERTEX` | `top_abs::ShapeEnum::Vertex` |
| `ffi::TopoDS_cast_to_edge(shape)` | `topo_ds::edge(shape)` |
| `ffi::Geom_BezierCurve_to_handle(c)` | `geom::BezierCurve::to_handle(c)` |
| `ffi::new_HandleGeomCurve_from_...(&h)` | `h.to_handle_curve()` |
| `ffi::shape_list_to_vector(list)` | `list.iter().collect()` |
| `ffi::TopTools_ListOfShape_new()` | `top_tools::ListOfShape::new()` |

**File status:**

| Status | Files |
|--------|-------|
| Fully working | primitives.rs, bounding_box.rs, vertex.rs, boolean_shape.rs, workplane.rs, angle.rs, lib.rs, shell.rs, section.rs, edge.rs, wire.rs, kicad.rs |
| Partially working | face.rs, shape.rs, solid.rs |
| Fully stubbed | compound.rs, surface.rs, mesh.rs, law_function.rs, make_pipe_shell.rs |

### Step 6: Update Examples

Blocked on stubbed methods in wire.rs (`sweep_along`, `sweep_along_with_radius_values`).

### Step 7: Cleanup

Delete `opencascade-sys-old` once everything works.

---

## Generator Limitations (Remaining Blockers)

These are CXX or generator limitations causing methods to be stubbed in the `opencascade` crate:

### 1. Implicit default constructors not generated

`BRep_Builder` has no explicit constructor (uses C++ implicit default). The generator only emits constructors found in the AST. Blocks `compound.rs`.

**Attempted fix:** Tried generating synthetic default constructors for classes with no explicit constructors. Failed because abstract class detection doesn't traverse inheritance -- classes like `BOPAlgo_BuilderShape` inherit unimplemented pure virtual methods but aren't detected as abstract.

**Fix needed:** Enhance abstract class detection to walk the inheritance hierarchy.

### 2. Constructors with default enum parameters skipped

`BRepFilletAPI_MakeFillet(const TopoDS_Shape&, ChFi3d_FilletShape FShape = ChFi3d_Rational)` -- generator skips because it can't handle default enum values. Blocks fillet operations in `solid.rs`.

**Fix needed:** Either handle default params or generate explicit overloads without the defaulted enum parameter.

### 3. TColgp array constructors not generated

`TColgp_Array1OfPnt2d`, `TColgp_Array2OfPnt` constructors missing. These are template instantiations (`typedef NCollection_Array1<gp_Pnt2d>`). Blocks `law_function.rs`, `surface.rs`.

**Fix needed:** Generate constructors for NCollection template typedefs.

### 4. BRep_Tool static methods

`mesh.rs` uses `BRep_Tool::Triangulation()`, `BRep_Tool::Surface()`, etc. These may need verification that they're being generated.

### 5. BRepFeat_MakeDPrism constructors

Blocks `Face::extrude_to_face` and `Face::subtractive_extrude`.

### 6. STEP/IGES I/O

`Shape::read_step/write_step/read_iges/write_iges` are stubbed. `ReadFile()` returns `IFSelect_ReturnStatus` enum (unscoped, not generated by CXX).

### 7. Enum methods generally

CXX requires `enum class` but OCCT uses unscoped enums. All methods with enum parameters/returns are skipped. This is a fundamental CXX limitation. Workaround: hand-write enum definitions if needed.
