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

All methods previously stubbed in the opencascade crate have been unstubbed, except for two that are blocked by a generator limitation (see below).

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
| Fully working | All files |
| Remaining stubs | None |

**Methods unstubbed in this iteration:**
- `law_function.rs` — full implementation using TColgp_Array1OfPnt2d, Law_Interpol
- `make_pipe_shell.rs` — both solid and shell variants using MakePipeShell with law functions
- `edge.rs` — `arc()` using GC_MakeArcOfCircle, HandleGeomTrimmedCurve upcast
- `section.rs` — `section_edges()` using inherited method + ListOfShapeIterator
- `shape.rs` — all 12+ stubs: `variable_fillet_edge/edges`, `subtract/union/intersect` section edges, `read_iges`, `write_iges`, `clean`, `mesh`, `faces_along_line`, `drill_hole`
- `face.rs` — all 6 stubs: `normal_at`, `normal_at_center`, `workplane`, `surface_area`, `sweep_along_with_radius_values`, `CompoundFace::clean`
- `compound.rs` — `clean()` using ShapeUpgrade_UnifySameDomain
- `solid.rs` — section edges extraction in `subtract/union/intersect`
- `wire.rs` — `sweep_along_with_radius_values` using make_pipe_shell_with_law_function_shell
- `wire.rs` — `from_unordered_edges` using ShapeAnalysis_FreeBounds::ConnectEdgesToWires with Handle get() dereference
- `kicad.rs` — `edge_cuts()` using from_unordered_edges
- `mesh.rs` — full Mesher implementation using BRepMesh_IncrementalMesh, BRep_Tool::Triangulation, Poly_Triangulation node/triangle/normal iteration

### Step 6: Update Examples (COMPLETE)

All examples compile and work. No changes needed — the examples reference the public API which is now fully implemented.

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

### 3. ~~TColgp array constructors not generated~~ RESOLVED

`TColgp_Array1OfPnt2d`, `TColgp_Array2OfPnt` constructors are now generated. `law_function.rs` and `surface.rs` are unstubbed.

### 4. ~~BRep_Tool static methods~~ RESOLVED

46 of 50 static methods are generated, including `Triangulation()` and `Surface()`. Only 4 are filtered: `Continuity()` and `MaxContinuity()` (return unscoped enum `GeomAbs_Shape`), `MaxTolerance()` (takes unscoped enum `TopAbs_ShapeEnum`), and `Triangulations()` (returns `Poly_ListOfTriangulation`, not yet in `known_collections`). The actual blocker for `mesh.rs` is not BRep_Tool methods but missing Handle dereferencing and `Poly_Triangulation` accessor support, plus the TColgp array issue (#3). Adding `Poly_ListOfTriangulation` to `known_collections` would unblock `Triangulations()` since collection-aware filtering is now in place.

### 5. ~~BRepFeat_MakeDPrism constructors~~ RESOLVED

Both constructors are generated: `MakeDPrism::new()` (default) and `MakeDPrism::new_shape_face2_real_int_bool()` (parameterized). All parameters are by-reference or primitive, so no filters apply. `Face::extrude_to_face` and `Face::subtractive_extrude` have been unstubbed.

### 5a. ~~Methods with collection type params/returns filtered~~ FIXED

Methods taking or returning known collection types (e.g., `TopTools_ListOfShape`) were filtered out because collection typedefs weren't in `all_classes`. Fixed by merging collection typedef names into `all_class_names` in `compute_all_class_bindings()`. ~80 new methods unblocked, including `Generated()`, `Modified()`, `TopExp::MapShapes()`, `SectionEdges()`, and many BOP/fillet/offset methods. Collection types not yet in `known_collections` (e.g., `Poly_ListOfTriangulation`) still need to be added there first.

### 6. ~~STEP/IGES I/O~~ RESOLVED

`Shape::read_step` and `Shape::write_step` are unstubbed. `Shape::read_iges` and `Shape::write_iges` are also now unstubbed — `IGESControl_Reader` uses inherited methods from `XSControl_Reader` (`read_file`, `transfer_roots`, `one_shape`), and `IGESControl_Writer` uses `add_shape`, `compute_model`, and `write` (via FNES).

### 7. ~~Enum methods generally~~ RESOLVED

CXX requires `enum class` but OCCT uses unscoped enums. All methods with enum parameters/returns are skipped. This is a fundamental CXX limitation. Workaround: hand-write enum definitions if needed.

### 8. ~~Methods in FFI but not in module re-exports~~ RESOLVED

All 602 types declared in `ffi.rs` are now re-exported in per-module files. This was accomplished by:

1. Adding `extra_types` parameter to `generate_module_reexports()` in `codegen/rust.rs`
2. Computing unreexported types in `main.rs` across three categories: handle types for transient classes, opaque referenced types, and collection iterator types
3. Generating module files for modules not in the dependency graph (e.g., `Transfer`, `TopOpeBRepBuild`, `DE`, `IntTools`, `GCE`, `BOPDS`, `StepData`, `TColGeom`)

Methods listed as "missing re-exports" (e.g., `GProp_GProps::mass()`, `GC_MakeArcOfCircle::value()`) were actually always accessible — they use CXX's direct `self:` receiver syntax, which works automatically through `pub use` type aliases without needing explicit delegation in `impl` blocks.

### 9. ~~Cross-module type identity issues~~ RESOLVED

Previously, types were declared in both `ffi.rs` and module-specific bridge blocks. The unified FFI architecture (Step 4i) eliminated this — all types are now declared once in `ffi.rs` and re-exported via `pub use`. Cross-module type identity works correctly.

### 10. ~~Wire::from_unordered_edges — Handle type mismatch~~ RESOLVED

The unified FFI architecture (Step 4i) already ensured all Handle types are declared once in `ffi.rs` and re-exported via `pub use`. The type identity issue was already fixed. Additionally, `get()` and `get_mut()` methods were added to all Handle types to allow dereferencing handles back to their contained objects. `Wire::from_unordered_edges` and `KicadPcb::edge_cuts` are now fully implemented.

### 11. Standalone package-level functions not generated

OCCT packages define standalone (non-class) convenience functions such as `gp::OX()`, `gp::DZ()`, `gp::Origin()` that return pre-built geometric primitives. The binding generator currently only processes class constructors, methods, and inherited methods — it does not detect or bind free functions declared at the package/namespace level. Adding support would make the API more natural (e.g., `gp::OX()` instead of `gp::Ax1::new_pnt_dir(&gp::Pnt::new(), &gp::Dir::new_real3(1.0, 0.0, 0.0))`).

### 12. Helper functions for constructors with default arguments

When a C++ constructor has trailing parameters with default values, the generator currently only emits the fully-specified variant (e.g., `Transform::new_shape_trsf_bool2(shape, trsf, copy, copy_mesh)`). It should also generate convenience wrappers in the re-export module that omit trailing defaulted parameters and delegate to the full version with the C++ default values filled in. For example, `BRepBuilderAPI_Transform(S, T)` would become `Transform::new_shape_trsf(shape, trsf)` calling `new_shape_trsf_bool2(shape, trsf, false, false)`. These wrappers are purely Rust-side (no FFI changes needed) and would significantly improve ergonomics.
