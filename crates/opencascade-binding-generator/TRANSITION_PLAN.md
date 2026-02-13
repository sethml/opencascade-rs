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
| Fully working | primitives.rs, bounding_box.rs, vertex.rs, boolean_shape.rs, workplane.rs, angle.rs, lib.rs, shell.rs, solid.rs, surface.rs |
| Mostly working (few stubs) | edge.rs (arc blocked), wire.rs (sweep_with_radius blocked), compound.rs (clean blocked), section.rs (section_edges blocked) |
| Partially working | face.rs (~8 stubs), shape.rs (~10 stubs) |
| Fully stubbed | mesh.rs, law_function.rs, make_pipe_shell.rs |
| Not building | kicad.rs (edge_cuts depends on from_unordered_edges cross-module type issue) |

**Methods unstubbed in this pass:**
- `Compound::from_shapes` — using BRep_Builder + make_compound + add
- `Shape::empty` — BRep_Builder + Compound
- `Shape::read_step` — STEPControl_Reader (read_file_charptr returns i32, 0=success)
- `Shape::write_step` — STEPControl_Writer (transfer_shape/write returns i32, 1=success)
- `Shape::set_global_translation` — gp_Trsf + TopLoc_Location
- `Shape::hollow` / `Shape::offset_surface` — MakeThickSolid + ListOfShape
- `Face::sweep_along` / `Wire::sweep_along` — BRepOffsetAPI_MakePipe
- `Face::center_of_mass` — BRepGProp + GProp_GProps
- `Face::outer_wire` — BRepTools::outer_wire
- `Face::union/intersect/subtract` — BRepAlgoAPI Fuse/Common/Cut
- `CompoundFace` methods — extrude, revolve, booleans, set_global_translation, From<Face>
- `Edge::edge_type` — BRepAdaptor_Curve::get_type() + GeomAbs::CurveType::try_from()

### Step 6: Update Examples

Partially unblocked — `Wire::sweep_along` now works. Still blocked by `sweep_along_with_radius_values` (law_function).

### Step 7: Cleanup

Delete `opencascade-sys-old` once everything works.

### Step 9: Viewer Crate

The `viewer` crate fails to compile because opencascade types contain `UniquePtr` (with raw `*const cxx::void`) which does not implement `Send`, but wasmtime's `ResourceTable` requires `Send`. This requires either `unsafe impl Send` for the wrapper types or a redesign of the viewer's resource management. Pre-existing issue, not caused by binding migration.

---

## Generator Limitations (Remaining Blockers)

These are CXX or generator limitations causing methods to be stubbed in the `opencascade` crate:

### 1. ~~Implicit default constructors not generated~~ FIXED

Synthetic default constructors are now generated for classes that have no explicit constructor declarations and are not effectively abstract. Abstract detection walks the full inheritance hierarchy via `is_effectively_abstract()` in `bindings.rs`, collecting pure virtual methods from all ancestors and checking they are overridden. Classes with any explicit constructor (public, protected, or private) do not get synthetic constructors since C++ won't generate an implicit default in that case. `BRep_Builder`, `TopoDS_Builder`, and ~30 other classes now have constructors.

### 2. ~~Constructors with default enum parameters skipped~~ FIXED

The `Param` struct now tracks `has_default: bool` by inspecting libclang AST children of each ParmDecl — expression nodes (`DeclRefExpr`, `UnexposedExpr`, `IntegerLiteral`, etc.) indicate defaults, while `TypeRef`/`NamespaceRef`/`TemplateRef` are just type references. When a constructor is filtered out due to enum/class/handle params, `compute_constructor_bindings()` tries trimming defaulted params from the right until the remaining params pass all filters. The C++ wrapper omits those args, letting C++ fill in the defaults. Classes like `BRepFilletAPI_MakeFillet`, `BRepOffsetAPI_MakeOffset`, `GeomAPI_PointsToBSpline`, `Extrema_ExtPS`, and ~20 others now have constructors.

### 3. TColgp array constructors not generated

`TColgp_Array1OfPnt2d`, `TColgp_Array2OfPnt` constructors missing. These are template instantiations (`typedef NCollection_Array1<gp_Pnt2d>`). Blocks `law_function.rs`, `surface.rs`.

**Fix needed:** Generate constructors for NCollection template typedefs.

### 4. ~~BRep_Tool static methods~~ RESOLVED

46 of 50 static methods are generated, including `Triangulation()` and `Surface()`. Only 4 are filtered: `Continuity()` and `MaxContinuity()` (return unscoped enum `GeomAbs_Shape`), `MaxTolerance()` (takes unscoped enum `TopAbs_ShapeEnum`), and `Triangulations()` (returns `Poly_ListOfTriangulation`, not yet in `known_collections`). The actual blocker for `mesh.rs` is not BRep_Tool methods but missing Handle dereferencing and `Poly_Triangulation` accessor support, plus the TColgp array issue (#3). Adding `Poly_ListOfTriangulation` to `known_collections` would unblock `Triangulations()` since collection-aware filtering is now in place.

### 5. ~~BRepFeat_MakeDPrism constructors~~ RESOLVED

Both constructors are generated: `MakeDPrism::new()` (default) and `MakeDPrism::new_shape_face2_real_int_bool()` (parameterized). All parameters are by-reference or primitive, so no filters apply. `Face::extrude_to_face` and `Face::subtractive_extrude` have been unstubbed.

### 5a. ~~Methods with collection type params/returns filtered~~ FIXED

Methods taking or returning known collection types (e.g., `TopTools_ListOfShape`) were filtered out because collection typedefs weren't in `all_classes`. Fixed by merging collection typedef names into `all_class_names` in `compute_all_class_bindings()`. ~80 new methods unblocked, including `Generated()`, `Modified()`, `TopExp::MapShapes()`, `SectionEdges()`, and many BOP/fillet/offset methods. Collection types not yet in `known_collections` (e.g., `Poly_ListOfTriangulation`) still need to be added there first.

### 6. ~~STEP/IGES I/O~~ PARTIALLY RESOLVED

`Shape::read_step` and `Shape::write_step` are now unstubbed. The workaround uses `read_file_charptr` (returns `i32`, 0 = IFSelect_RetDone) and `transfer_roots`/`one_shape` for reading; `transfer_shape` and `write_charptr` for writing. `read_iges` and `write_iges` remain blocked:
- `IGESControl_Reader::ReadFile()` is inherited from `XSControl_Reader` and uses enum `IFSelect_ReturnStatus` — not re-exported
- `IGESControl_Writer::AddShape()` → re-exported as `add_shape()`, but `ComputeModel()` → not re-exported

### 7. Enum methods generally

CXX requires `enum class` but OCCT uses unscoped enums. All methods with enum parameters/returns are skipped. This is a fundamental CXX limitation. Workaround: hand-write enum definitions if needed.

### 8. Methods in FFI but not in module re-exports (NEW)

Many methods exist in `ffi.rs` but are not added to per-module re-export `impl` blocks, making them inaccessible from the `opencascade` crate (since `ffi` is `pub(crate)`). This is the **largest remaining blocker category**. Affected methods:

| Missing Re-export | Blocks | FFI Location |
|---|---|---|
| `GProp_GProps::mass()` | `Face::surface_area`, `mesh.rs` | ffi.rs |
| `GC_MakeArcOfCircle::value()` | `Edge::arc` | ffi.rs |
| `GeomAPI_ProjectPointOnSurf::lower_distance_parameters()` | `Face::normal_at` | ffi.rs |
| `BRepGProp_Face::normal()` | `Face::normal_at` | ffi.rs |
| `ShapeUpgrade_UnifySameDomain::build()/shape()/allow_internal_edges()` | `Shape/Compound/Face::clean` | ffi.rs |
| `BRepIntCurveSurface_Inter::init()/more()/next()/face()` | `Shape::faces_along_line` | ffi.rs |
| `BRepFeat_MakeCylindricalHole::init()/perform()` | `Shape::drill_hole` | ffi.rs |
| `Law_Interpol::set()` | `law_function`, `sweep_with_radius` | ffi.rs |
| `BRepOffsetAPI_MakePipeShell::set_law()/add()/set_mode_bool()/build()/make_solid()` | `make_pipe_shell` | ffi.rs |
| `HandleTopTools_HSequenceOfShape` | `Wire::from_unordered_edges`, `kicad.rs` | cross-module type mismatch |
| `TopTools_ListOfShape` in `b_rep_algo_api` | `Section::section_edges`, boolean `new_edges` | locally-declared vs module type |

**Root cause:** The generator's `emit_reexport_class()` only adds methods whose parent class matches the module's class. Inherited methods, wrapper functions for cross-class operations, and some method signatures don't get included. The fix would be to ensure all methods emitted in `ffi.rs` for a class also appear in the corresponding module re-export.

### 9. Cross-module type identity issues (NEW)

Some types are declared in both `ffi.rs` (as `TopTools_ListOfShape`) and locally in module-specific bridge blocks (e.g., `b_rep_algo_api::ffi::TopTools_ListOfShape`). CXX treats these as distinct types, so methods returning one can't be used where the other is expected. Affects `SectionEdges()` return type and `HandleTopTools_HSequenceOfShape` across modules.
