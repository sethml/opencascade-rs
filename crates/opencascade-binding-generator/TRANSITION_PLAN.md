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

### 13. ~~By-value Handle and class parameters~~ RESOLVED

Methods taking `Handle<T>` or class types by value (not by reference) previously caused the method to be skipped entirely. The generator now emits C++ wrapper functions that accept `const Handle<T>&` or `const T&` respectively, converting by-value parameters to const-reference parameters. The wrapper body forwards to the original C++ method, which copies from the const reference — this matches C++ copy semantics. ~25 new methods unblocked across various modules.

### 14. ~~C-string returns for free functions and static methods~~ RESOLVED

Free functions (from utility classes) and static methods returning `const char*` were previously excluded. The generator now emits wrapper functions that convert the C string to `rust::String` and return it. This added 473 new FFI function declarations, including 462 `get_type_name()` RTTI methods on `Standard_Transient`-derived classes (useful for Handle downcasting type checks) plus 11 other string-returning functions.

### 15. ~~Const/mut return mismatches~~ RESOLVED

Methods returning `const T&` where the method is non-const were skipped because CXX's ownership model couldn't reconcile them. The generator now emits `ConstMutReturnFix` wrapper functions that take `Pin<&mut Self>` as the receiver, call the non-const C++ method, and cast the result to `const T&`. This unblocked 4 methods: `math_DoubleTab::Value`, `math_Matrix::Value`, `TopLoc_SListNodeOfItemLocation::Tail`, and `TopLoc_SListNodeOfItemLocation::Value`.

### 16. ~~`&mut` enum output parameters~~ RESOLVED

Methods with `&mut EnumType` output parameters (e.g., `IntCurveSurface_IntersectionPoint::Values` which has a `TopAbs_State& state` out-param) were excluded because CXX can't pass `&mut EnumType` across FFI. The generator now emits `MutRefEnumParam` wrapper functions that use a local `Standard_Integer` variable, call the method, and write back through the enum pointer. On the Rust side, the parameter is `&mut i32`. This unblocked 2 methods.

### 17. Raw pointer investigation — NOT WORTH GENERAL SUPPORT

190 methods are excluded because they have raw pointer parameters (`T*`, `const T*`). Investigation showed this is not worth implementing general support for:

- **87% concentrated in 2 classes**: BSplCLib (116 methods) and BSplSLib (49 methods) — internal B-spline evaluation routines not typically called from user code
- **Top pointed-to types**: `TColStd_Array1OfReal*` (104 occurrences), `TColStd_Array1OfInteger*` (40), `TColStd_Array2OfReal*` (34), `void*` (23)
- **NCollection allocators**: 6 classes × 3 methods each use `void*` memory allocator internals
- **Only 2-3 commonly useful cases** (see below)

#### Useful raw pointer cases (handle individually if needed)

| Method | Pointer Usage | Pattern |
|--------|--------------|---------|
| `BRep_Tool::CurveOnSurface` (2 overloads) | `bool* isStored` out-param | Nullable boolean output — caller passes null or `&mut bool` |
| `gp_XYZ::GetData()` / `ChangeData()` | Returns `const f64*` / `f64*` | Array access — returns pointer to 3 doubles |
| `BRepMesh_IncrementalMesh::Discret` | Static method with complex signature | Rarely needed from user code |

**Recommendation**: Handle the 2-3 useful cases with handwritten wrappers if/when needed, rather than building general raw pointer support in the generator.

### 18. Nullable pointer parameters (optional, low priority)

Some C++ methods have `T* param = NULL` parameters where NULL means "don't care about this output." The most notable example is `BRep_Tool::CurveOnSurface` with its `Standard_Boolean* theIsStored = NULL` parameter. CXX has no nullable pointer type, but a general pattern can handle these:

**FFI layer** (C++ wrapper + ffi.rs): Split into `(want: bool, out: &mut T)` — the C++ wrapper passes `want ? &out : NULL` to the original method.

**Public API** (module re-export): Wrap as `Option<&mut T>` — the impl method maps `Some(p) => (true, p)` and `None => (false, &mut dummy)`.

```cpp
// C++ wrapper
HandleGeom2dCurve BRep_Tool_CurveOnSurface_edge_face_real2_bool(
    const TopoDS_Edge& E, const TopoDS_Face& F,
    Standard_Real& First, Standard_Real& Last,
    bool wantIsStored, bool& isStoredOut) {
    Standard_Boolean stored = Standard_False;
    auto result = BRep_Tool::CurveOnSurface(
        E, F, First, Last, wantIsStored ? &stored : NULL);
    isStoredOut = stored;
    return result;
}
```

```rust
// Public API in module file
pub fn curve_on_surface(e: &Edge, f: &Face, first: &mut f64, last: &mut f64,
                        is_stored: Option<&mut bool>) -> UniquePtr<HandleGeom2dCurve> {
    let mut dummy = false;
    let (want, out) = match is_stored {
        Some(p) => (true, p),
        None => (false, &mut dummy),
    };
    crate::ffi::BRep_Tool_CurveOnSurface_edge_face_real2_bool(e, f, first, last, want, out)
}
```

This pattern could be automated in the generator for `T* param = NULL` trailing parameters, or applied manually for the 2-3 cases that matter. The generator would need to detect `Type::Ptr` parameters with default value `NULL`/`nullptr` and emit the split wrapper.

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

---

## Optional: Switching from CXX to `extern "C"` FFI

### Motivation

CXX provides safety guarantees at the FFI boundary (lifetime checking, `UniquePtr` ownership, `Pin` for move-prevention), but imposes significant constraints on what can be bound. The current generator already works around many CXX limitations with C++ wrapper functions — 12,108 of 19,552 FFI functions (62%) require a C++ wrapper to satisfy CXX. Only 7,444 (38%) are direct CXX method bindings. The question is whether CXX's safety benefits justify its constraints, given that the binding code is machine-generated and the safety layer could be provided by hand-written Rust wrappers in the `opencascade` crate instead.

### What CXX Provides

**Safety features:**
- `UniquePtr<T>` — RAII ownership of heap-allocated C++ objects with automatic destructor calls
- `Pin<&mut T>` — prevents Rust from moving C++ objects (which may have internal pointers)
- Lifetime checking — prevents returning references with ambiguous lifetimes
- Type identity — CXX ensures types are declared consistently across the bridge
- `rust::String`/`rust::Str` — safe bidirectional string conversion

**Convenience features:**
- Direct method binding with `self: &T` / `self: Pin<&mut T>` — 7,444 methods bound without C++ wrappers
- `#[cxx_name]` for Rust↔C++ name mapping
- Automatic destructor generation via `impl UniquePtr<T> {}`

### What CXX Costs

**Excluded functionality (CXX-specific limitations):**
- Classes with protected destructors — entirely excluded (CXX needs public dtor for `UniquePtr`)
- Methods returning `&mut T` with reference params — excluded (ambiguous lifetimes)
- Const methods returning `T&` (non-const result type mismatch) — need `ConstMutReturnFix` wrappers
- No template types in the bridge — 386 Handle typedefs needed (`HandleGeomCurve` instead of `Handle<Geom_Curve>`)
- No C++ enum support (only `enum class`) — all enums mapped to `i32` with `static_cast` (2,322 casts in wrappers.hxx)
- No namespace-scoped types (e.g., `IMeshData::ListOfPnt2d`)

**Structural constraints:**
- Single monolithic `#[cxx::bridge]` module — all 1,472 types and 19,552 functions in one compilation unit (125K-line ffi.rs, 39K-line wrappers.hxx)
- CXX cannot reference types across bridge modules — prevents splitting into per-module compilation
- 5 reserved name collisions (`Vec`, `Box`, `String`, `Result`, `Option`) require renaming
- `UniquePtr` wrapping forces heap allocation for every C++ object, even small value types like `gp_Pnt` (24 bytes)

**Generated code overhead:**
- 6,085 `UniquePtr<T>` annotations in ffi.rs
- 9,042 `Pin<&mut T>` annotations in ffi.rs
- 1,000 `impl UniquePtr<T> {}` blocks
- 4,809 `std::make_unique<T>()` calls in wrappers.hxx
- 1,025 `rust::String` conversions in wrappers.hxx
- 1,304 `rust::Str` conversions in wrappers.hxx

### What `extern "C"` Would Look Like

#### C++ side (wrappers.hxx → wrappers.cpp)

Every function becomes `extern "C"` with opaque pointer types:

```cpp
// Current (CXX):
inline std::unique_ptr<gp_Pnt> gp_Pnt_ctor_real3(
    Standard_Real theXp, Standard_Real theYp, Standard_Real theZp) {
    return std::make_unique<gp_Pnt>(theXp, theYp, theZp);
}

// extern "C" equivalent:
extern "C" gp_Pnt* gp_Pnt_ctor_real3(double x, double y, double z) {
    return new gp_Pnt(x, y, z);
}
extern "C" void gp_Pnt_destroy(gp_Pnt* self) {
    delete self;
}

// Current (CXX direct method — no C++ wrapper):
// In ffi.rs: fn x(self: &gp_Pnt) -> f64;
// CXX calls gp_Pnt::X() directly

// extern "C" equivalent (all methods need wrappers):
extern "C" double gp_Pnt_x(const gp_Pnt* self) {
    return self->X();
}
```

#### Rust side (ffi.rs)

```rust
// Current (CXX):
#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        type gp_Pnt;
        fn gp_Pnt_ctor_real3(x: f64, y: f64, z: f64) -> UniquePtr<gp_Pnt>;
        #[cxx_name = "X"]
        fn x(self: &gp_Pnt) -> f64;
    }
}

// extern "C" equivalent:
#[repr(C)]
pub struct gp_Pnt { _opaque: [u8; 0] }

extern "C" {
    fn gp_Pnt_ctor_real3(x: f64, y: f64, z: f64) -> *mut gp_Pnt;
    fn gp_Pnt_destroy(self_: *mut gp_Pnt);
    fn gp_Pnt_x(self_: *const gp_Pnt) -> f64;
}

// Safe wrapper (in per-module re-export file):
pub struct Pnt {
    ptr: *mut crate::ffi::gp_Pnt,
}
impl Pnt {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { ptr: unsafe { crate::ffi::gp_Pnt_ctor_real3(x, y, z) } }
    }
    pub fn x(&self) -> f64 {
        unsafe { crate::ffi::gp_Pnt_x(self.ptr) }
    }
}
impl Drop for Pnt {
    fn drop(&mut self) {
        unsafe { crate::ffi::gp_Pnt_destroy(self.ptr) }
    }
}
```

### Advantages of `extern "C"`

1. **Per-module compilation** — The module dependency graph is a clean DAG (zero cycles across 123 modules). Each module could be its own compilation unit: separate `.cpp` file compiled independently, separate `extern "C"` block in Rust. This would dramatically improve incremental compile times and enable parallel compilation.

2. **No excluded functionality** — Protected destructors, lifetime-ambiguous methods, const/mut mismatches, and namespace-scoped types all work fine with raw pointers. The only remaining exclusions would be genuinely unbindable patterns (rvalue references, complex templates).

3. **Simpler generated code** — No `UniquePtr`, `Pin`, `#[cxx_name]`, `impl UniquePtr` blocks, `rust::String`/`rust::Str`, `static_cast` for enums. The C++ wrapper is a straightforward `extern "C"` function. The Rust FFI is a straightforward `extern "C"` block.

4. **Smaller dependency footprint** — Remove the `cxx` and `cxx-build` crate dependencies.

5. **Value types** — Small types like `gp_Pnt` (24 bytes) could be `#[repr(C)]` structs passed by value instead of heap-allocated. This requires matching the C++ struct layout, but OCCT's `gp_*` types are simple POD types (just doubles), and the layout could be verified at build time.

6. **Enum support** — Enums could be `#[repr(i32)]` Rust enums used directly in function signatures, eliminating the i32-with-static_cast workaround.

### Disadvantages of `extern "C"`

1. **All 19,552 functions need C++ wrappers** — Currently 7,444 are direct CXX bindings. Switching to extern "C" means every single method, accessor, and mutator needs a C++ wrapper function. The generator already generates 12,108 wrappers, so this is a ~62% increase in C++ wrapper code.

2. **Manual memory management** — Every `new` needs a corresponding `delete` wrapper. The safe wrapper structs handle this, but it's more code to generate:
   - 1 destructor per non-abstract class (~600 classes)
   - Safe wrapper struct + `Drop` impl per class

3. **No compiler-enforced safety at FFI boundary** — CXX catches certain misuse at compile time (e.g., using `&T` where `Pin<&mut T>` is needed). With extern "C", the raw pointer layer is all `unsafe` — safety enforcement moves to the generated safe wrapper layer.

4. **Significant generator rewrite** — `codegen/rust.rs`, `codegen/cpp.rs`, `type_mapping.rs`, `resolver.rs`, and `build.rs` all need substantial changes. Estimate: 2,000-3,000 lines of codegen changes.

5. **`opencascade` crate migration** — The crate currently uses `UniquePtr<T>`, `pin_mut()`, etc. extensively (~188 occurrences). All would need to change to use the new safe wrapper types.

6. **Handle reference counting** — OCCT's `Handle<T>` (based on `opencascade::handle<T>`, an intrusive ref-counted smart pointer) would need explicit ref-count management wrappers (`IncrementRefCounter`/`DecrementRefCounter`) rather than relying on CXX's `UniquePtr<HandleT>` RAII.

### Per-Module Compilation Architecture

The key structural benefit of extern "C" is enabling per-module compilation. Here's how it would work:

```
crates/opencascade-sys/generated/
├── gp_ffi.rs          # extern "C" { fn gp_Pnt_ctor_real3(...) -> *mut gp_Pnt; ... }
├── gp_wrappers.cpp    # extern "C" gp_Pnt* gp_Pnt_ctor_real3(...) { ... }
├── gp.rs              # pub struct Pnt { ptr: *mut gp_ffi::gp_Pnt } + safe wrappers
├── topo_ds_ffi.rs     # extern "C" { ... }
├── topo_ds_wrappers.cpp
├── topo_ds.rs          # safe wrappers, can reference gp::Pnt
├── ...
└── lib.rs             # pub mod gp; pub mod topo_ds; ...
```

**Module dependency feasibility:**
- 123 modules, zero circular dependencies (verified by module_graph.rs analysis)
- 72 modules (59%) are leaf modules with no cross-module deps
- Deepest dependency chain is 3 levels
- Most-depended-on modules: `geom_abs` (28 deps), `top_abs` (19 deps), `gp` (4 deps)

**Build system changes:**
- `build.rs` compiles each `*_wrappers.cpp` as a separate translation unit
- Each `.cpp` file only includes the headers it needs (currently wrappers.hxx includes all 1,027 headers)
- The cc crate's `Build::files()` handles parallel C++ compilation natively
- Incremental builds: changing one module only recompiles that module's `.cpp`

**Cross-module type references in Rust:**
- Each module's `_ffi.rs` declares its own opaque types
- Cross-module references use Rust's type system: `gp_ffi::gp_Pnt` can be referenced from `topo_ds_ffi.rs` as an opaque pointer
- The safe wrapper layer handles conversions: `impl From<&gp::Pnt> for *const gp_ffi::gp_Pnt`
- No C++ cross-module issues since each `.cpp` includes its own OCCT headers directly

### Exploration Needed to Validate

Before committing to this transition, the following should be investigated:

1. **Prototype one module** — Convert `gp` to extern "C" as a proof-of-concept. This is the simplest module (no cross-module deps, well-understood POD types). Verify:
   - Compile time improvement (currently the monolithic build takes significant time)
   - That `#[repr(C)]` for simple types like `gp_Pnt` matches C++ layout
   - That the safe wrapper API is ergonomic enough

2. **Handle<T> reference counting** — Prototype the explicit ref-count pattern for one Handle type. Verify that `IncrementRefCounter`/`DecrementRefCounter` works correctly, especially for Handle upcasting and the `to_handle()` pattern.

3. **Protected destructors** — Verify that classes with protected destructors can be handled (e.g., via a destructor wrapper that's a friend function, or by only allowing Handle-based ownership for those types).

4. **Value types feasibility** — For `gp_*` types (Pnt, Vec, Dir, Trsf, etc.), verify that `#[repr(C)]` structs with known field layout match the C++ ABI. Could use `static_assert(sizeof(gp_Pnt) == 24)` and `offsetof` checks in the generated C++ code.

5. **Compile time measurement** — Measure current monolithic CXX build time vs projected per-module extern "C" build time. The current build compiles 39K lines of C++ as one unit; splitting into ~123 units of ~300 lines each with parallel compilation should be substantially faster.

6. **Incremental migration path** — Determine whether CXX and extern "C" can coexist during a transition period. A module could be migrated one at a time if the extern "C" types can be used alongside CXX `UniquePtr` types. This may require type-erased pointer conversions at the boundary.

### Assessment

**Is it worthwhile?** Probably yes, but not urgently.

The strongest argument for switching is **per-module compilation**. The monolithic 125K-line ffi.rs / 39K-line wrappers.hxx is already the largest compilation bottleneck, and it will grow 6x if we expand to all OCCT headers (future work item #16). CXX's inability to split across bridge modules means this will only get worse. With extern "C", each module compiles independently, enabling both parallel compilation and incremental rebuilds.

The second strongest argument is **removing exclusions**. Protected destructors, lifetime-ambiguous methods, and const/mut mismatches are all CXX-specific limitations that would disappear.

The main cost is **generator rewrite effort** (estimated 2-3K lines) and **opencascade crate migration**. However, the generator is already designed with clean separation between binding computation (resolver.rs, bindings.rs) and code emission (codegen/), so the rewrite would primarily affect the codegen layer. The binding computation layer would be simplified (fewer exclusion reasons).

**Recommendation:** Prototype the `gp` module first (exploration item #1). If it demonstrates clear compile-time benefits and acceptable ergonomics, plan the full transition as a future major step.
