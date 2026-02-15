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

265 explicit headers (from `bindings.toml` — 2 full modules + 219 individual headers) expand to 628 via automatic dependency resolution. The generator produces 84 modules.

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

### Step 10: Module Derivation Centralization

Module assignment for types is now determined solely by their source header file name, centralized in `resolver.rs`. Previously, module names were derived from type name prefixes (splitting on the first `_`), which was 99.3% accurate but failed for the 34 types whose name-based module prefix doesn't match their source header's module (see item #13 below).

**Changes:**
- `SymbolTable` now contains a `type_to_module: HashMap<String, String>` built from parsed class/enum header data
- `module_graph.rs` second pass uses the HashMap lookup instead of `extract_module_from_type()`
- `Type::module()` removed from `model.rs` — no longer possible to derive module from a `Type` node alone
- `TypeContext` carries `type_to_module` for use during type mapping

Short names (the Rust re-export alias, e.g., `MakeBox` from `BRepPrimAPI_MakeBox`) are now computed module-relative via `short_name_for_module(cpp_name, module)` in `type_mapping.rs`. This strips the actual module prefix rather than splitting on the first `_`, preserving extra prefix text. For example, `BRepOffsetSimple_Status` in module `BRepOffset` becomes `SimpleStatus` instead of the incorrect `Status` (which would collide with `BRepOffset_Status`).

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

### 7a. ~~Free functions with enum parameters excluded~~ FIXED

Free functions (from utility classes converted to module-level functions) with
enum parameters were excluded because `function_uses_unknown_handle()` in the
resolver did not recognize enum types. `Type::Class("TopAbs_ShapeEnum")` was
checked against `all_class_names` only, not `all_enum_names`, causing functions
like `TopExp::MapShapes(S, T, M)` (where `T` is `TopAbs_ShapeEnum`) to be
filtered out as `UnknownHandleType`. Fixed by adding `all_enum_names` to the
check — enum types by value or const-ref are now recognized as known types.
This unblocked ~27 functions including `TopExp::MapShapes`,
`TopExp::MapShapesAndAncestors`, and others across multiple modules.

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

### 11. ~~Standalone package-level functions not generated~~ RESOLVED

OCCT utility classes (classes with only static methods, no instance methods, and no meaningful constructors) are now automatically detected and converted to module-level free functions. Examples include `gp`, `TopoDS`, `TopExp`, `BRepBndLib`, `BRepGProp`, `BRepTools`, `BRepLib`, `BSplCLib`, `Message`, and `Precision`. The conversion handles:
- `ConstRef` return types with no ref params are converted to by-value (UniquePtr-wrapped for class/Handle types)
- Handle return types are wrapped in `std::unique_ptr` / `UniquePtr`
- Function-only modules (where the utility class was the only class) are generated automatically
- The `opencascade` crate has been updated to use the new free function syntax (e.g., `b_rep_tools::outer_wire(...)` instead of `b_rep_tools::BRepTools::outer_wire(...)`)

### 12. ~~Helper functions for constructors with default arguments~~ RESOLVED

When a C++ constructor has trailing parameters with default values, the generator emits convenience wrappers in the re-export module that omit trailing defaulted parameters and delegate to the full-argument version with the C++ default values filled in. For example, `BRepBuilderAPI_Transform(S, T, copy=false, copyMesh=false)` generates:
- `new_shape_trsf_bool2(S, T, copy, copyMesh)` — full version with C++ wrapper
- `new_shape_trsf_bool(S, T, copy)` — Rust-only, calls `Self::new_shape_trsf_bool2(S, T, copy, false)`
- `new_shape_trsf(S, T)` — Rust-only, calls `Self::new_shape_trsf_bool2(S, T, false, false)`

These convenience wrappers are purely Rust-side (no ffi.rs or wrappers.hxx entries generated). Default values are extracted from the C++ AST via `IntegerLiteral`, `FloatingLiteral`, `BoolLiteralExpr`, and `NullPtrLiteralExpr` cursor kinds, with a fallback that tokenizes the parent `ParmDecl` for macro-expanded defaults like `Standard_False`. The `adapt_default_for_rust_type()` function ensures default values are properly cast for Rust (e.g., C++ integer `0` becomes `0.0` for `f64` parameters). Convenience wrappers are only generated when all trimmed parameters have extractable defaults that can be expressed as valid Rust literals.

---

## Future Work: Expanding to All OCCT Headers

Currently 265 explicit headers are configured in `bindings.toml` (2 full modules + 219 individual headers). OCCT ships 6,875 `.hxx` headers across ~349 modules. An experimental run generating bindings for all headers produced 6,565 types and 90,295 functions in 333 modules, but surfaced several issues that need fixing first.

### 13. Invalid Rust identifiers from Fortran common blocks (11 instances)

**Problem:** `AdvApp2Var_Data.hxx` defines Fortran COMMON block wrapper structs with names like `maovpar_1_`, `mmapgs0_1_`, `mdnombr_1_`. These don't follow OCCT's `Module_Class` naming convention. The generator's `extract_module_from_name()` splits on the first `_`, producing module `maovpar` and short name `1_` — not a valid Rust identifier.

**Affected:** 11 structs (maovpar, maovpch, mdnombr, minombr, mlgdrtl, mmapgs0, mmapgs1, mmapgs2, mmapgss, mmcmcnp, mmjcobi). These appear as dependency-resolved types, not explicit headers.

**Analysis:** A scan of all 6,875 OCCT headers found 4,920 top-level class/struct definitions, of which 4,886 (99.3%) have matching module names. Only 34 (0.7%) are mismatches, in three categories:

- **11 Fortran common blocks** — `maovpar_1_`, `mmapgs0_1_`, etc. in `AdvApp2Var_Data.hxx`. Pure data structs, no methods. Skipped by the parser (no bindable members) but pulled in as opaque referenced types.
- **21 no-underscore helper classes** — e.g., `FilletPoint` in `ChFi2d_FilletAlgo.hxx`, `Alert` in `Message_Alert.hxx`, `parser`/`scanner` in `exptocas.tab.hxx`. These are file-local helper types defined alongside the main class.
- **2 genuinely different modules** — `Persistence_` in `TObj_Persistence.hxx`, `PSO_Particle` in `math_PSOParticlesPool.hxx`.

The module derivation from class names is 99.3% accurate. The mismatches are edge cases.

**Fix options:**
- (a) In `safe_short_name()` (type_mapping.rs), prefix short names starting with a digit with `_` (so `1_` becomes `_1_`)
- (b) Exclude these structs entirely in the parser — they're C wrappers for Fortran internals with no useful public API
- (c) Improve module derivation: detect that `maovpar_1_` doesn't correspond to any known OCCT module prefix and either skip it or assign it to its source header's module (AdvApp2Var)

Option (b) is probably best since these types have no methods and aren't useful from Rust. The no-underscore helpers are typically skipped by the parser already (no bindable members or inner classes). The 2 genuine mismatches (`Persistence_`, `PSO_Particle`) would need either a hardcoded map or source-header-based module derivation as a fallback.

### 14. Non-type template parameter in opaque type declaration (1 instance)

**Problem:** `BVH_Tree` is a C++ template with `template <class T, int N>` — the `int N` is a non-type template parameter. The generator emits `type BVH_Tree<Standard_Real, 3>;` which is invalid Rust/CXX syntax (CXX generics only support type parameters, not value parameters).

**Fix:** In `generate_unified_opaque_declarations()` (codegen/rust.rs) or `is_unbindable()` (model.rs), filter out type names containing `<..., numeric_literal>`. The existing `is_nested_type()` checks for `<` but this type bypasses it because it enters as an opaque reference collected from function signatures. Add a regex check for numeric template arguments.

### 15. Raw pointer syntax leaking into type names (2 instances)

**Problem:** `IMeshData_Edge *const` and `IMeshData_Face *const` appear as `type IMeshData_Edge *const;` in ffi.rs. The C++ pointer syntax leaked into the class name string instead of being parsed as `Type::ConstPtr(Type::Class("IMeshData_Edge"))`.

**Fix:** Either fix the parser to properly decompose these pointer types, or add validation in `generate_unified_opaque_declarations()` to reject type name strings containing `*`.

### 16. Scale concerns for all-headers build

**Problem:** Going from 267 to 6,875 headers produces:
- ffi.rs: 356K lines (vs current 57K) — 6.3x growth
- wrappers.hxx: 201K lines (vs current 17K) — 12x growth
- 90K wrapper functions, 6.4K includes

CXX processes the entire bridge as one compilation unit. This will likely cause extreme compile times (potentially 10-30+ minutes) and large binary sizes.

**Fix options:**
- (a) Split the CXX bridge into multiple modules (per OCCT toolkit — there are ~15 toolkits in OCCT)
- (b) Use Cargo feature flags to enable header subsets on demand
- (c) Accept the compile time if it stays under ~5 minutes; only split if proved necessary
- (d) Generate but don't compile all wrappers — only compile wrapper functions that are actually called

### 17. Windows-only header parse failure (non-blocking)

`OSD_WNT.hxx` includes `<windows.h>` which doesn't exist on macOS/Linux. Clang emits a fatal error but the generator continues. No types are lost. Could suppress the warning by detecting known Windows-only headers.
