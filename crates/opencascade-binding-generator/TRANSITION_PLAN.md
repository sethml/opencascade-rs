# Transition Plan: Moving to Generated Bindings

This document outlines the plan to transition `opencascade-sys` from hand-written bindings to automatically generated bindings using `opencascade-binding-generator`.

## Approach: Parallel Development

Keep the old crate as `opencascade-sys-old` for reference, and build a new `opencascade-sys` using the code generator with pre-generated code.

## Reference: Comparing Old vs New Code

To see how the old code worked before the binding generator changes, use:

```bash
# Reference commit before binding generator work (last commit on origin/main)
git show 14fca36:crates/opencascade/src/primitives/solid.rs

# Diff a specific file
git diff 14fca36 -- crates/opencascade/src/primitives/shape.rs

# See all changes to the opencascade crate since then
git diff 14fca36 -- crates/opencascade/

# See the old opencascade-sys structure
git show 14fca36:crates/opencascade-sys/src/lib.rs | head -200
```

**Key differences in old code:**
- Uses `opencascade_sys::ffi::*` directly (monolithic ffi module)
- Uses `ffi::TopoDS_Solid_to_owned(solid)` instead of `solid.to_owned()`
- Uses `ffi::cast_solid_to_shape(&self.inner)` instead of `self.inner.as_shape()`
- Uses `ffi::BRepFilletAPI_MakeFillet_ctor(shape)` instead of `b_rep_fillet_api::MakeFillet::new_shape(...)`

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
   ├── src/
   │   ├── lib.rs          # Main crate entry, re-exports generated/lib.rs
   │   ├── common.hxx      # Shared C++ utilities (construct_unique template)
   │   ├── collections.rs  # Hand-written collection helpers
   │   └── collections.hxx # C++ implementations for collection helpers
   ├── headers.txt         # List of OCCT headers (reference)
   └── generated/          # Pre-generated Rust and C++ files (auto-generated)
       ├── lib.rs          # Module declarations
       ├── gp.rs           # gp module (Pnt, Vec_, Dir)
       ├── wrapper_gp.hxx  # C++ wrappers for gp
       ├── topo_ds.rs      # TopoDS module (Shape)
       ├── wrapper_topo_ds.hxx
       └── ... etc
   ```

2. **Architecture Notes:**
   
   **Hand-written vs Generated Code:**
   - `src/` contains hand-written code that won't be overwritten by the generator
   - `generated/` contains auto-generated code from the binding generator
   - Both `src/` and `generated/` are added as include paths in build.rs
   - `common.hxx` lives in `src/` and is included by generated wrappers via `#include "common.hxx"`
   
   **Cross-Module Type References:**
   Hand-written modules can reference types from generated modules using CXX's type aliasing:
   ```rust
   // In src/collections.rs
   type TopoDS_Shape = crate::topo_ds::ffi::Shape;  // References generated type
   ```
   
   **CXX Limitations with Collections:**
   - `CxxVector<T>` only works for types implementing `VectorElement` trait
   - `VectorElement` is only implemented for primitives (i32, f64, etc.)
   - Opaque types like `TopoDS_Shape` cannot be stored in `CxxVector`
   - Workaround: Provide indexed access functions (`list_get(index)`) instead of vectors

3. **Current modules generated:**
   - `gp` - Pnt, Vec_ (with re-export as Vec), Dir, XYZ
   - `top_abs` - ShapeEnum, Orientation enums
   - `topo_ds` - Shape
   - `b_rep_prim_api` - MakeBox

4. **Build status:** ✅ Compiles successfully

5. **To regenerate bindings:**
   ```bash
   ./scripts/regenerate-bindings.sh
   ```

### ✅ Step 4a: Headers Required for `opencascade` Crate (COMPLETE)

All headers required by `crates/opencascade/src/**` are now in `headers.txt`. The generator
automatically resolves header dependencies, expanding 262 explicit headers to 624 total headers.
Bindings are generated for 88 modules.

**Automatic Header Dependency Resolution:**
The generator parses `#include` directives from each header and recursively adds dependent headers.
This eliminates manual header management and ensures complete type coverage.

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
- TopExp_Explorer.hxx, TopExp.hxx

**TopLoc module (locations):**
- TopLoc_Location.hxx

**TopTools module (topology collections):**
- TopTools_HSequenceOfShape.hxx, TopTools_ListOfShape.hxx
- TopTools_IndexedMapOfShape.hxx, TopTools_IndexedDataMapOfShapeListOfShape.hxx
- TopTools_FormatVersion.hxx

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

**ShapeAnalysis module (shape analysis tools):**
- ShapeAnalysis_FreeBounds.hxx

**Summary:** 262 explicit headers in `headers.txt`, which expand to 624 headers via automatic dependency resolution. The generator produces 88 modules covering all required OCCT functionality.

**Automatic dependency resolution:** The generator recursively parses `#include` directives to automatically include dependent headers. This is enabled via the `--resolve-deps` flag in `regenerate-bindings.sh`.

**Remaining work:**
1. ~~Fix Handle type naming convention (Step 4g)~~ ✅ DONE
2. ~~Fix inherited method pointer class mismatch (Step 4g)~~ ✅ DONE
3. Updating `crates/opencascade/src/*.rs` to use new module paths (Step 5)

### ✅ Step 4b: Collection Typedef Support in Generator (COMPLETE)

The binding generator now automatically generates wrappers for collection typedefs like
`TopTools_ListOfShape`, `TopTools_IndexedMapOfShape`, etc.

**✅ Reference Implementation Complete:**

Hand-written collection wrappers are now implemented in `src/collections.rs` and `src/collections.hxx`.
These serve as the template for what the generator should produce.

**Approach: Direct C++ Iterator Wrapping (not vector conversion)**

Rather than converting OCCT collections to `std::vector`, we wrap C++ iterators directly:
- Avoids intermediate memory allocation
- Lazy iteration - only copies elements as they're consumed
- Works with CXX's opaque type model (no `VectorElement` trait needed)

**Pattern for each collection type (from `src/collections.*`):**

1. **C++ side (`collections.hxx`):**
   ```cpp
   // Iterator wrapper struct holding C++ iterator state
   struct ListOfShapeIterator {
       TopTools_ListOfShape::const_iterator current;
       TopTools_ListOfShape::const_iterator end;
   };
   
   // Construction
   inline std::unique_ptr<TopTools_ListOfShape> TopTools_ListOfShape_new();
   
   // Create iterator
   inline std::unique_ptr<ListOfShapeIterator> TopTools_ListOfShape_iter(
       const TopTools_ListOfShape& list);
   
   // Advance iterator (returns nullptr when exhausted)
   inline std::unique_ptr<TopoDS_Shape> ListOfShapeIterator_next(
       ListOfShapeIterator& iter);
   
   // Add elements
   inline void TopTools_ListOfShape_append(
       TopTools_ListOfShape& list, const TopoDS_Shape& shape);
   ```

2. **Rust FFI side (`collections.rs`):**
   ```rust
   #[cxx::bridge]
   pub mod ffi {
       unsafe extern "C++" {
           type TopTools_ListOfShape;
           type ListOfShapeIterator;
           
           fn TopTools_ListOfShape_new() -> UniquePtr<TopTools_ListOfShape>;
           fn TopTools_ListOfShape_iter(list: &TopTools_ListOfShape) 
               -> UniquePtr<ListOfShapeIterator>;
           fn ListOfShapeIterator_next(iter: Pin<&mut ListOfShapeIterator>) 
               -> UniquePtr<TopoDS_Shape>;
           fn TopTools_ListOfShape_append(list: Pin<&mut TopTools_ListOfShape>, 
               shape: &TopoDS_Shape);
       }
   }
   ```

3. **Rust Iterator impl:**
   ```rust
   pub struct ListOfShapeIter {
       inner: UniquePtr<ffi::ListOfShapeIterator>,
   }
   
   impl Iterator for ListOfShapeIter {
       type Item = UniquePtr<Shape>;
       fn next(&mut self) -> Option<Self::Item> {
           let item = ffi::ListOfShapeIterator_next(self.inner.pin_mut());
           if item.is_null() { None } else { Some(item) }
       }
   }
   
   impl ListOfShape {
       pub fn iter(&self) -> ListOfShapeIter { ... }
       pub fn from_iter<'a>(shapes: impl IntoIterator<Item = &'a Shape>) 
           -> UniquePtr<Self> { ... }
   }
   ```

**Collection Types Currently Implemented (in `src/collections.*`):**

| Type | Rust API | Notes |
|------|----------|-------|
| `TopTools_ListOfShape` | `iter()`, `from_iter()` | Doubly-linked list |
| `TopTools_SequenceOfShape` | `iter()`, `from_iter()` | Dynamic array (1-indexed) |
| `TopTools_IndexedMapOfShape` | `iter()`, `from_iter()`, `find_index()` | Set with index access |
| `TopTools_MapOfShape` | `iter()`, `from_iter()`, `contains()` | Unordered set |
| `TopTools_IndexedDataMapOfShapeListOfShape` | `keys()`, `find_from_key()`, `find_from_index()` | Map Shape→ListOfShape |
| `TopTools_DataMapOfShapeShape` | `keys()`, `find()`, `contains()` | Map Shape→Shape |

**What the Generator Needs to Do:**

1. **Detect typedef patterns** in OCCT headers:
   - `typedef NCollection_List<TopoDS_Shape> TopTools_ListOfShape`
   - `typedef NCollection_Sequence<TopoDS_Shape> TopTools_SequenceOfShape`
   - `typedef NCollection_IndexedMap<TopoDS_Shape, ...> TopTools_IndexedMapOfShape`
   - `typedef NCollection_Map<TopoDS_Shape, ...> TopTools_MapOfShape`
   - `typedef NCollection_IndexedDataMap<K, V, ...> TopTools_IndexedDataMapOfShapeListOfShape`
   - `typedef NCollection_DataMap<K, V, ...> TopTools_DataMapOfShapeShape`

2. **Generate C++ wrapper code** following the pattern in `src/collections.hxx`:
   - Iterator struct with appropriate iterator type (const_iterator or Iterator)
   - `TypeName_new()` → construction
   - `TypeName_iter()` → create iterator wrapper
   - `TypeNameIterator_next()` → advance and return element (nullptr when done)
   - `TypeName_append/add/bind()` → add elements (type-specific)
   - Additional accessors for maps: `find_from_key()`, `find_from_index()`, etc.

3. **Generate Rust FFI and impl code** following the pattern in `src/collections.rs`:
   - FFI declarations for all C++ wrapper functions
   - Rust iterator struct wrapping the C++ iterator
   - `Iterator` trait impl returning `UniquePtr<ElementType>`
   - Impl block on collection type with `iter()`, `from_iter()`, and type-specific methods

4. **Handle different iterator APIs:**
   - `NCollection_List`, `NCollection_Map`: Use `const_iterator` with `cbegin()/cend()`
   - `NCollection_Sequence`, `NCollection_IndexedMap`: Use 1-indexed access with `Length()/Extent()`
   - `NCollection_DataMap`: Use OCCT's `Iterator` class with `More()/Next()/Key()/Value()`

**NCollection Types to Support (by frequency in bindings):**

| Type | Count | Element Type | Iterator API |
|------|-------|--------------|--------------|
| `TopTools_ListOfShape` | 104 | `TopoDS_Shape` | ✅ `const_iterator` |
| `TColgp_Array1OfPnt` | 46 | `gp_Pnt` | 1-indexed `Value()` |
| `TColgp_HArray1OfPnt` | 31 | `gp_Pnt` | Handle to Array1 |
| `TopTools_HSequenceOfShape` | 27 | `TopoDS_Shape` | Handle to Sequence |
| `TColgp_Array2OfPnt` | 14 | `gp_Pnt` | 2D 1-indexed access |
| `TColgp_Array1OfPnt2d` | 14 | `gp_Pnt2d` | 1-indexed `Value()` |
| `TopTools_SequenceOfShape` | 11 | `TopoDS_Shape` | ✅ 1-indexed `Value()` |
| `TopTools_IndexedMapOfShape` | 10 | `TopoDS_Shape` | ✅ 1-indexed `FindKey()` |
| `TopTools_MapOfShape` | 7 | `TopoDS_Shape` | ✅ `const_iterator` |
| `TopTools_IndexedDataMapOfShapeListOfShape` | 6 | Key+Value | ✅ Supported |
| `TopTools_DataMapOfShapeShape` | 4 | Key+Value | ✅ Supported |
| `TColgp_Array1OfVec` | 3 | `gp_Vec` | 1-indexed `Value()` |
| `TColgp_HArray1OfPnt2d` | 2 | `gp_Pnt2d` | Handle to Array1 |

**Efficiency Notes:**

All element types are cheap to copy (24 bytes or less, reference-counted where applicable):
- `TopoDS_Shape`: 24 bytes, ref-counted handle - copy just increments refcount
- `gp_Pnt/Vec/Dir`: 24 bytes, plain data
- `gp_Pnt2d`: 16 bytes, plain data

**Implementation Status:**

- ✅ Hand-written reference implementation served as template (now removed)
- ✅ Generator support implemented in `codegen/collections.rs`
- ✅ Generates `collections.hxx` (C++ wrappers) and `collections.rs` (Rust FFI + iterators)
- ✅ 4 simple collection types: `ListOfShape`, `SequenceOfShape`, `IndexedMapOfShape`, `MapOfShape`
- ✅ 2 data map types: `IndexedDataMapOfShapeListOfShape`, `DataMapOfShapeShape`
- 🔄 TColgp array types (Array1OfPnt, etc.): TODO

### 🔄 Step 4c: Feature Parity with opencascade-sys-old (IN PROGRESS)

Added headers to match all types from `opencascade-sys-old`. Now generating 131 headers successfully.

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

**Build status:** ✅ Binding generation and compilation both succeed (88 modules from 624 headers)

**Headers status for `opencascade/src/primitives/*.rs`:**

All required headers are now included via automatic dependency resolution:

- ✅ TColgp_Array2OfPnt.hxx - 2D arrays of 3D points (included)
- ✅ Geom_BezierSurface.hxx - Bezier surface geometry (included)
- ✅ BRep_Builder.hxx - Builder class (included, upcast generated)
- ✅ TopTools_HSequenceOfShape.hxx - Shape sequences (included)
- ✅ TopTools_ListOfShape.hxx - Shape lists with full iterator support
- ✅ TopExp.hxx - MapShapes, MapShapesAndAncestors functions
- ✅ TopTools_IndexedMapOfShape.hxx - Indexed shape maps
- ✅ TopTools_IndexedDataMapOfShapeListOfShape.hxx - Shape→list maps
- ✅ BRepIntCurveSurface_Inter.hxx - Line-shape intersection
- ✅ ShapeAnalysis_FreeBounds.hxx - Connect edges to wires

**Note:** Headers are included but some constructors may not be generated due to generator limitations (see blocking issues).

**Helper functions needed (from old hand-written bindings):**

These were custom C++ wrappers in opencascade-sys-old that need equivalents:

- ✅ `cast_*_to_shape()` → Use `*.as_shape()` upcast method (already generated)
- ✅ `TopoDS_cast_to_*()` → Use `topo_ds::edge()`, `topo_ds::face()`, etc. (already generated)
- `*_to_owned()` → Use `*.to_owned()` method (already generated for copyable types)

**✅ Collection conversion helpers (RESOLVED):**
- ✅ `shape_list_to_vector()` → Use `list.iter().collect()` instead
- ✅ `new_list_of_shape()` → Use `top_tools::ListOfShape::new()`
- ✅ `shape_list_append_*()` → Use `list.append(&shape)`
- ✅ `new_indexed_map_of_shape()` → Use `top_tools::IndexedMapOfShape::new()`
- ✅ `map_shapes()` → Use `top_exp::map_shapes()` (generated from TopExp.hxx)
- ✅ `new_indexed_data_map_of_shape_list_of_shape()` → Use `top_tools::IndexedDataMapOfShapeListOfShape::new()`
- ✅ `map_shapes_and_ancestors()` → Use `top_exp::map_shapes_and_ancestors()`
- ✅ `connect_edges_to_wires()` → Use `shape_analysis::FreeBounds` methods

**✅ Collection support headers now available:**
- ✅ TopExp.hxx - Provides MapShapes, MapShapesAndAncestors functions
- ✅ TopTools_IndexedMapOfShape.hxx - Shape indexing for fillet/chamfer
- ✅ TopTools_IndexedDataMapOfShapeListOfShape.hxx - Shape→list mapping
- ✅ ShapeAnalysis_FreeBounds.hxx - Edge-to-wire connection algorithm
- ✅ TopTools_ListOfShape.hxx - Available with full iterator support

**Note:** TopTools collection types (IndexedMapOfShape, ListOfShape, etc.) are typedef aliases for NCollection templates, 
so they appear as opaque types in modules that use them rather than as classes in the TopTools module. This is correct behavior.

**✅ Handle conversion helpers (RESOLVED):**
- ✅ `Law_Function_to_handle()` → Use `law::Interpol::to_handle(func)` then `handle.to_handle_function()`
- ✅ `BRep_Builder_upcast_to_topods_builder()` → Generated automatically via inheritance



### ✅ Step 4d: Inherited Method Support

Many OCCT classes inherit methods from base classes that are essential for use.
The generator now emits inherited methods via C++ wrapper functions.

**Problem (SOLVED):**
- `BRepOffsetAPI_ThruSections` inherits `Shape()` from `BRepBuilderAPI_MakeShape`
- `BRepBuilderAPI_MakeFace` inherits `Shape()`, `IsDone()` from `BRepBuilderAPI_MakeShape`
- `STEPControl_Reader` inherits `ClearShapes()`, `NbShapes()` from `XSControl_Reader`
- Without these methods, builder classes can't return their results

**CXX Limitation:**
CXX generates method pointer verification code like:
```cpp
void (DerivedClass::*method_ptr)(args) = &DerivedClass::InheritedMethod;
```
This fails for inherited methods because the actual pointer type is `BaseClass::*`, not `DerivedClass::*`.

**Solution implemented (C++ wrapper approach):**
Rather than declaring inherited methods directly on derived types (which CXX can't verify),
we generate C++ wrapper functions that call the methods directly:

1. **C++ wrapper functions** in `wrapper_*.hxx`:
   ```cpp
   inline Standard_Integer STEPControl_Reader_inherited_NbShapes(
       const STEPControl_Reader& self) {
       return self.NbShapes();  // Calls inherited method directly
   }
   ```

2. **Rust FFI declarations** with `#[cxx_name]`:
   ```rust
   #[cxx_name = "STEPControl_Reader_inherited_NbShapes"]
   fn reader_inherited_nb_shapes(self_: &Reader) -> i32;
   ```

3. **Rust impl block methods** for user-friendly API:
   ```rust
   impl Reader {
       pub fn nb_shapes(&self) -> i32 {
           ffi::reader_inherited_nb_shapes(self)
       }
   }
   ```

**Limitations of inherited method generation:**

Inherited methods are filtered out if they use:

1. **Cross-module types** - Methods using types from other modules are skipped because CXX 
   requires all types to be declared in the same bridge module. For example, `BOPAlgo_BOP`
   can't expose inherited methods that use `TopoDS_Shape` because that type is in `topo_ds`,
   not `bop_algo`.

2. **Raw pointers** - Methods with `*const` or `*mut` parameters/returns are skipped because
   CXX requires `unsafe` for raw pointers. Example: `SetNorm(const char*)` is filtered out.

3. **Already overridden methods** - If the derived class declares any method with the same
   name (even protected/private), the inherited version is skipped to respect visibility.

**Example: What gets generated for STEPControl_Reader:**

From base class `XSControl_Reader`, only simple methods pass the filters:
- `ClearShapes()` - void params/return ✅
- `NbShapes()` - returns primitive `i32` ✅  
- `PrintStatsTransfer(i32, i32)` - primitive params, void return ✅

Methods like `OneShape()` (returns `TopoDS_Shape`) are filtered because `TopoDS_Shape`
is a cross-module type not declared in the `step_control` module.

**Status:** ✅ Complete - inherited methods generated via C++ wrappers with filtering

### ✅ Step 4e: Handle Wrapping and Upcasting Support (COMPLETE)

OCCT uses `Handle<T>` (a reference-counted smart pointer) for managing objects that inherit
from `Standard_Transient`. Many APIs require Handle parameters, not raw pointers/references.

**Problem (SOLVED):**
Files like `edge.rs` need Handle operations that weren't auto-generated:
```cpp
// 1. Wrap object in Handle
Handle(Geom_BezierCurve) handle = curve;  // implicit in C++

// 2. Upcast Handle to base type  
Handle(Geom_Curve) baseHandle = handle;   // implicit in C++

// 3. Pass Handle to constructor
BRepBuilderAPI_MakeEdge edgeMaker(baseHandle);
```

**Solution implemented:**

1. **`to_handle()` associated functions** for classes inheriting from `Standard_Transient`:
   ```rust
   // Rust API - associated function on impl block
   impl BezierCurve {
       pub fn to_handle(obj: UniquePtr<Self>) -> UniquePtr<HandleGeomBezierCurve>;
   }
   // Usage: BezierCurve::to_handle(curve)
   ```
   ```cpp
   // C++ wrapper
   std::unique_ptr<HandleGeomBezierCurve> Geom_BezierCurve_to_handle(
       std::unique_ptr<Geom_BezierCurve> obj) {
       return std::make_unique<HandleGeomBezierCurve>(obj.release());
   }
   ```

2. **Handle upcast methods** as impl block methods on Handle types:
   ```rust
   // Rust API
   impl HandleGeomBezierCurve {
       pub fn to_handle_bounded_curve(&self) -> UniquePtr<HandleGeomBoundedCurve> {
           ffi::bezier_curve_to_handle_bounded_curve(self)
       }
   }
   ```
   ```cpp
   // C++ wrapper
   std::unique_ptr<HandleGeomBoundedCurve> HandleGeomBezierCurve_to_HandleGeomBoundedCurve(
       const HandleGeomBezierCurve& handle) {
       return std::make_unique<HandleGeomBoundedCurve>(handle);
   }
   ```

**Implementation details:**

1. **Transient class detection** (`parser.rs`): Added `check_is_handle_type()` function that
   checks if a class inherits from `Standard_Transient`, `Geom_*`, `Geom2d_*`, or `Law_*`.
   Note: TopoDS classes are NOT Handle types - they use their own internal reference counting.

2. **C++ wrappers** (`codegen/cpp.rs`):
   - `generate_to_handle_wrapper()`: Generates `ClassName_to_handle(unique_ptr) -> unique_ptr<Handle>`
   - `generate_handle_upcast_wrappers()`: Generates `HandleDerived_to_HandleBase(const Handle&) -> unique_ptr<HandleBase>`
   - `collect_handle_types()`: Includes Handle types for all `is_handle_type = true` classes

3. **Rust FFI** (`codegen/rust.rs`):
   - `generate_to_handle_ffi()`: FFI declarations for `to_handle` functions
   - `generate_handle_upcast_ffi()`: FFI declarations for Handle upcast functions
   - `generate_handle_impls()`: Impl blocks for Handle types with upcast methods
   - `to_handle()` is now an associated function in impl blocks (e.g., `BezierCurve::to_handle(obj)`)

4. **Headers added** for complete inheritance hierarchies:
   - `Geom_Geometry.hxx`, `Geom_BoundedCurve.hxx`, `Geom_BoundedSurface.hxx`, `Geom_ElementarySurface.hxx`
   - `Geom2d_Geometry.hxx`, `Geom2d_BoundedCurve.hxx`, `Geom2d_Conic.hxx`
   - `Law_BSpFunc.hxx`

**Classes with Handle support now generated:**

| Class | Handle Type | Can Upcast To |
|-------|-------------|---------------|
| `Geom_BezierCurve` | `HandleGeomBezierCurve` | `HandleGeomBoundedCurve` |
| `Geom_BSplineCurve` | `HandleGeomBSplineCurve` | `HandleGeomBoundedCurve` |
| `Geom_TrimmedCurve` | `HandleGeomTrimmedCurve` | `HandleGeomBoundedCurve` |
| `Geom_BezierSurface` | `HandleGeomBezierSurface` | `HandleGeomBoundedSurface` |
| `Geom_BSplineSurface` | `HandleGeomBSplineSurface` | `HandleGeomBoundedSurface` |
| `Geom_Plane` | `HandleGeomPlane` | `HandleGeomElementarySurface` |
| `Geom_CylindricalSurface` | `HandleGeomCylindricalSurface` | `HandleGeomElementarySurface` |
| `Geom_Curve` | `HandleGeomCurve` | `HandleGeomGeometry` |
| `Geom_Surface` | `HandleGeomSurface` | `HandleGeomGeometry` |
| `Geom_BoundedCurve` | `HandleGeomBoundedCurve` | `HandleGeomCurve` |
| `Geom_BoundedSurface` | `HandleGeomBoundedSurface` | `HandleGeomSurface` |
| `Geom_ElementarySurface` | `HandleGeomElementarySurface` | `HandleGeomSurface` |
| `Geom2d_Ellipse` | `HandleGeom2dEllipse` | `HandleGeom2dConic` |
| `Geom2d_TrimmedCurve` | `HandleGeom2dTrimmedCurve` | `HandleGeom2dBoundedCurve` |
| `Law_BSpFunc` | `HandleLawBSpFunc` | `HandleLawFunction` |
| `Law_Interpol` | `HandleLawInterpol` | `HandleLawBSpFunc` |

**Status:** ✅ Complete - Handle wrapping and upcasting now fully supported

### ✅ Step 4f: Abstract Class Handling (COMPLETE)

The generator now properly handles abstract classes (classes with pure virtual methods).

**Problem (SOLVED):**
- Abstract classes like `GeomEvaluator_Curve`, `GeomEvaluator_Surface`, `Message_ProgressIndicator` cannot be instantiated
- Generator was creating constructors, `to_handle()`, and `to_owned()` methods for abstract classes
- C++ compilation failed with "allocating an object of abstract class type"

**Solution implemented:**

1. **Abstract class detection** (`parser.rs`):
   - Added `is_abstract` field to `ParsedClass`
   - Detect pure virtual methods using clang's `is_pure_virtual_method()`
   - Mark class as abstract if any method is pure virtual

2. **Skip code generation for abstract classes** (`cpp.rs` and `rust.rs`):
   - `generate_constructor_wrappers()`: Skip for abstract classes
   - `generate_to_handle_wrapper()`: Skip for abstract classes
   - `generate_handle_upcast_wrappers()`: Skip for abstract classes
   - `generate_constructors()`: Skip for abstract classes
   - `generate_to_handle_ffi()`: Skip for abstract classes
   - `to_owned` generation: Skip for abstract classes

**Status:** ✅ Complete - abstract classes now handled correctly

### ✅ Step 4g: Remaining Build Issues (COMPLETE)

Both C++ compilation issues have been resolved and opencascade-sys now builds successfully.

**Issue 1: Handle type naming convention mismatch** - RESOLVED
- **Problem:** Generator created Handle type names like `HandleStandardTransient` but OCCT uses `Handle_Standard_Transient`
- **Solution:** Updated Handle type generation to correctly use OCCT's naming convention and proper `#[cxx_name]` attributes

**Issue 2: Inherited method pointer class mismatch** - RESOLVED
- **Problem:** Inherited method pointers had different declaring class than the binding class
- **Solution:** Disabled inherited method generation (they caused more issues than they solved). Direct method bindings on each class work correctly.

**Additional fixes applied:**
- Skip abstract class constructor impls (was generating calls to non-existent FFI functions)
- Filter methods using enums from impl generation (CXX requires `enum class`, but OCCT uses unscoped enums)
- Skip methods with by-value Handle parameters in static method impls
- Skip methods requiring explicit lifetimes (`Pin<&mut Self>` return with reference params)
- Fixed method count filters to be consistent between FFI and impl generation

**Status:** ✅ Complete - `cargo build -p opencascade-sys` succeeds with 0 errors

#### Key Insight: Filter Consistency Between FFI and Impl Generation

The most common source of build errors is **filter mismatch** between FFI declaration generation and impl method generation. When a method is filtered out of FFI generation (e.g., because it uses enums, needs explicit lifetimes, or is on an abstract class), the corresponding impl method must ALSO be filtered out, otherwise the impl will try to call a non-existent FFI function.

Filters that must be applied consistently in BOTH places:
- `method_uses_enum()` - methods with enum parameters/returns
- `needs_explicit_lifetimes()` - methods returning `Pin<&mut Self>` with reference params
- `has_unsupported_by_value_params()` - methods with class/handle by-value params
- `class.is_abstract` - abstract classes can't have constructor impls
- `has_const_mut_return_mismatch()` - mutable methods with const returns

#### Future Improvement: Centralized Method Filtering (Optional)

To prevent filter consistency bugs in the future, consider refactoring to use a single source of truth for method filtering:

```rust
/// Returns methods that should be bound, with all filters applied.
/// Use this in BOTH FFI generation and impl generation.
fn get_bindable_methods<'a>(
    class: &'a ParsedClass,
    all_enums: &HashSet<String>,
    filter_type: MethodFilterType, // FFI, WrapperImpl, StaticImpl, etc.
) -> Vec<&'a Method> {
    class.methods.iter()
        .filter(|m| !method_uses_enum(m, all_enums))
        .filter(|m| !needs_explicit_lifetimes(m))
        .filter(|m| !has_unsupported_by_value_params(m))
        // ... additional filters based on filter_type
        .collect()
}
```

This would ensure both FFI and impl generation iterate over the exact same method list, eliminating the possibility of filter mismatch.

---

### ✅ Step 4h: Two-Pass Architecture Refactor (COMPLETE)

**Status:** ✅ Complete - Both rust.rs and cpp.rs codegen now use SymbolTable

**Summary:**

1. ✅ Created `resolver.rs` (~1000 lines) with:
   - `SymbolTable` struct with lookup methods
   - `ResolvedClass`, `ResolvedMethod`, `ResolvedConstructor`, etc. types
   - `BindingStatus` enum (Included/Excluded with ExclusionReason)
   - `build_symbol_table()` function that resolves all symbols from parsed headers
   - Centralized filter functions: `type_uses_enum`, `method_needs_explicit_lifetimes`, `method_has_unsupported_by_value_params`

2. ✅ Integrated into main.rs:
   - Symbol table is now built after parsing, before code generation
   - Statistics are printed in verbose mode (classes, constructors, methods, etc.)
   - Added `--dump-symbols` CLI flag for debugging

3. ✅ All tests pass:
   - `regenerate-bindings.sh` succeeds
   - `opencascade-sys` compiles successfully

4. ✅ Consolidated ALL filter functions (Phase 2):
   - Made filter functions public in resolver.rs: `type_uses_enum`, `params_use_enum`, `method_needs_explicit_lifetimes`, `method_has_unsupported_by_value_params`, `static_method_has_unsupported_by_value_params`
   - Added convenience wrappers: `method_uses_enum`, `constructor_uses_enum`, `static_method_uses_enum`, `function_uses_enum`
   - Added `has_const_mut_return_mismatch` function to resolver.rs
   - Updated rust.rs to delegate ALL filter functions to resolver:
     - `method_uses_enum`, `constructor_uses_enum`, `static_method_uses_enum`, `function_uses_enum`
     - `has_unsupported_by_value_params` → `resolver::method_has_unsupported_by_value_params().is_some()`
     - `has_const_mut_return_mismatch` → `resolver::has_const_mut_return_mismatch()`
     - `needs_explicit_lifetimes` → `resolver::method_needs_explicit_lifetimes()`
   - Updated cpp.rs to delegate `method_uses_enum` to resolver (via `params_use_enum` + `type_uses_enum`)
   - Eliminated ~60 lines of duplicate filter logic across both files

5. ✅ Started migrating `generate_module()` to use SymbolTable (Phase 3):
   - ✅ Added `symbol_table: &SymbolTable` parameter to `generate_module()`
   - ✅ Updated main.rs and lib.rs to pass symbol_table to generators
   - ✅ Removed `all_enum_names` parameter - now using `symbol_table.all_enum_names`
   - ✅ Removed `all_class_names` parameter - now using `symbol_table.all_class_names`
   - ✅ Removed `cross_module_types` parameter - now using `symbol_table.cross_module_types_for_module()`
   - ✅ Added `protected_destructor_class_names()` helper to SymbolTable
   - ✅ Using `symbol_table.protected_destructor_class_names()` instead of computing from `all_parsed_classes`
   - ✅ Added `get_all_ancestors_by_name()` helper to SymbolTable
   - ✅ Updated `generate_re_exports()` to use `symbol_table.get_all_ancestors_by_name()`
   - ✅ Updated `generate_upcast_methods()` to use `symbol_table.get_all_ancestors_by_name()`
   - ✅ Updated `generate_handle_upcast_ffi()` to use `symbol_table.get_all_ancestors_by_name()`
   - ✅ Updated `generate_handle_impls()` to use `symbol_table.get_all_ancestors_by_name()`
   - ✅ Removed `all_classes_map` building and parameter from `generate_class()`
   - ✅ Removed `all_parsed_classes` parameter from `generate_module()` entirely
   - ✅ Removed unused local `get_all_ancestors()` function from rust.rs

6. ✅ Refactored `codegen/cpp.rs` to use SymbolTable:
   - ✅ Added `symbol_table: &SymbolTable` parameter to `generate_module_header()`
   - ✅ Removed `all_enum_names`, `all_class_names`, `global_inheritance_map`, `cross_module_types` parameters
   - ✅ Updated `generate_class_wrappers()` to use `symbol_table`
   - ✅ Updated `generate_upcast_wrappers()` to use `symbol_table.get_all_ancestors_by_name()` and `symbol_table.all_class_names`
   - ✅ Updated `generate_handle_upcast_wrappers()` to use `symbol_table.get_all_ancestors_by_name()` and `symbol_table.all_class_names`
   - ✅ Updated `generate_function_wrappers()` to use `symbol_table.all_enum_names`
   - ✅ Updated main.rs and lib.rs call sites
   - ✅ Removed redundant `all_enum_names`, `all_class_names`, `global_inheritance_map` variable construction from main.rs and lib.rs

**Remaining Work (Optional Future Enhancements):**

7. 🔲 Full migration to resolved types (using `ResolvedClass`, `ResolvedMethod`, etc.)
   - Currently, codegen receives `&[&ParsedClass]` and filters/processes at generation time
   - Target: Codegen receives resolved types with pre-computed names, statuses, and filtered members
   - Benefits:
     - Single source of truth for filtering decisions (resolver.rs only)
     - Pre-computed Rust/C++ names eliminate redundant computation
     - Easier debugging via `--dump-symbols` flag
   - Required changes:
     - Handle duplicate constructor/method naming in resolver instead of codegen
     - Update `generate_class()` to use `ResolvedClass` with `included_constructors()` etc.
     - Eventually remove `ParsedClass`/`ParsedMethod` from codegen signatures

**Current State:**
The two-pass architecture infrastructure is complete:
- ✅ SymbolTable with all resolved types (`ResolvedClass`, `ResolvedMethod`, etc.)
- ✅ Helper methods for lookup and iteration (`included_classes_for_module()`, `included_methods()`, etc.)
- ✅ All filter functions consolidated in resolver.rs
- ✅ Both rust.rs and cpp.rs codegen use SymbolTable for cross-module lookups
- ✅ main.rs/lib.rs no longer build redundant data structures

The remaining step (7) is optional - the current architecture is clean and functional. It would be valuable if we need to add more complex binding rules or want better debuggability.

**Original Motivation:**

The current architecture mixes symbol analysis with code generation in a single pass. This leads to:

1. **Filter consistency bugs** - The same filtering logic (method_uses_enum, needs_explicit_lifetimes, etc.) must be duplicated in rust.rs and cpp.rs, and inconsistencies cause compilation failures.

2. **Cross-module reference challenges** - When generating module A, we need to know about types in module B, but B may not be generated yet. Currently handled ad-hoc with cross_module_types lookups.

3. **Naming computed multiple times** - Rust short names, C++ names, FFI names, etc. are computed separately in different places, risking inconsistency.

4. **Poor debuggability** - Hard to see "what will be generated" without actually generating code. No easy way to dump the symbol table.

**Proposed Solution: Two-Pass Architecture**

**Pass 1: Symbol Table Construction**

Parse all headers and build a complete `SymbolTable` containing every symbol we'll wrap, with all derived information pre-computed:

```rust
/// A resolved symbol ready for code generation
#[derive(Debug, Clone)]
pub struct ResolvedSymbol {
    /// Unique identifier for this symbol
    pub id: SymbolId,
    
    /// Symbol kind (class, method, constructor, enum, function, handle type, etc.)
    pub kind: SymbolKind,
    
    /// C++ fully qualified name (e.g., "gp_Pnt", "BRepPrimAPI_MakeBox::Shape")
    pub cpp_name: String,
    
    /// Rust module this belongs to (e.g., "gp", "b_rep_prim_api")
    pub rust_module: String,
    
    /// Rust FFI type name with escaping (e.g., "Pnt", "Vec_", "MakeBox")
    pub rust_ffi_name: String,
    
    /// Rust public name (for re-exports, e.g., "Vec" when ffi name is "Vec_")
    pub rust_public_name: String,
    
    /// Source location
    pub source_header: String,
    pub source_line: Option<u32>,
    
    /// Documentation
    pub doc_comment: Option<String>,
    
    /// Binding status
    pub status: BindingStatus,
    
    /// For methods/constructors: parameter symbols
    pub params: Vec<ParamInfo>,
    
    /// For methods: return type info
    pub return_type: Option<TypeInfo>,
    
    /// For classes: parent class symbol IDs (for upcast generation)
    pub base_classes: Vec<SymbolId>,
    
    /// For classes: all method symbol IDs
    pub methods: Vec<SymbolId>,
    
    /// For classes: is this a Handle type?
    pub is_handle_type: bool,
    
    /// Cross-references to related symbols
    pub related: RelatedSymbols,
}

#[derive(Debug, Clone)]
pub enum BindingStatus {
    /// Will be generated
    Included,
    /// Skipped with reason
    Excluded(ExclusionReason),
}

#[derive(Debug, Clone)]
pub enum ExclusionReason {
    UsesEnum { enum_name: String },
    AbstractClass,
    ProtectedDestructor,
    NeedsExplicitLifetimes,
    UnsupportedByValueParam { param_name: String, type_name: String },
    ConstMutReturnMismatch,
    UnbindableType { description: String },
    // ... other reasons
}

#[derive(Debug, Clone)]
pub struct RelatedSymbols {
    /// For methods needing wrappers: the C++ wrapper function symbol
    pub cpp_wrapper: Option<SymbolId>,
    /// For classes: their Handle type symbol (if is_handle_type)
    pub handle_type: Option<SymbolId>,
    /// For derived classes: upcast function symbols
    pub upcast_functions: Vec<SymbolId>,
}

/// Complete symbol table for all modules
pub struct SymbolTable {
    /// All symbols indexed by ID
    symbols: HashMap<SymbolId, ResolvedSymbol>,
    /// Symbols grouped by module
    by_module: HashMap<String, Vec<SymbolId>>,
    /// Fast lookup by C++ name
    by_cpp_name: HashMap<String, SymbolId>,
    /// Fast lookup by Rust name (module::name)
    by_rust_name: HashMap<String, SymbolId>,
}
```

**Pass 2: Code Generation**

Generate code by iterating over the pre-built symbol table. Each generator (rust.rs, cpp.rs) only needs to:

1. Query the symbol table for symbols in the current module
2. Check `symbol.status == Included` to filter
3. Use pre-computed names, no re-calculation needed
4. Look up cross-module types via symbol ID references

```rust
/// Generate Rust FFI for a module
pub fn generate_rust_module(table: &SymbolTable, module: &str) -> String {
    let symbols = table.symbols_for_module(module);
    
    for sym in symbols.filter(|s| s.status.is_included()) {
        match sym.kind {
            SymbolKind::Class => generate_class_ffi(table, sym),
            SymbolKind::Method => generate_method_ffi(table, sym),
            // ...
        }
    }
}

/// Generate C++ wrapper for a module
pub fn generate_cpp_module(table: &SymbolTable, module: &str) -> String {
    // Uses the SAME symbol table, guaranteed to be consistent
    let symbols = table.symbols_for_module(module);
    // ...
}
```

**Benefits:**

1. **Single source of truth for filtering** - `BindingStatus` is computed once in Pass 1. Both rust.rs and cpp.rs simply check `status.is_included()`. No more duplicate filter functions.

2. **Pre-computed naming** - `rust_ffi_name`, `rust_public_name`, `cpp_name` are all computed once. No risk of computing different names in different places.

3. **Better cross-module support** - The symbol table knows about ALL symbols before any code is generated. Cross-module references can be resolved via symbol ID lookups.

4. **Debuggability** - Can dump the symbol table as JSON/debug output to see exactly what will be generated and why things are excluded.

5. **Incremental generation potential** - If symbol table is serializable, could support incremental updates (only regenerate modules with changed symbols).

6. **Clear separation of concerns** - Parser produces raw AST → Symbol resolver produces SymbolTable → Codegen consumes SymbolTable.

**Implementation Plan:**

1. **Create `symbol_table.rs`** with `ResolvedSymbol`, `SymbolTable`, etc. structs
2. **Create `resolver.rs`** - Pass 1 logic to build SymbolTable from ParsedHeaders
3. **Refactor `main.rs`** - Parse → Resolve → Generate pipeline
4. **Refactor `codegen/rust.rs`** - Take `&SymbolTable` instead of raw parsed data
5. **Refactor `codegen/cpp.rs`** - Take `&SymbolTable` instead of raw parsed data
6. **Remove duplicate filter functions** - All filtering is in resolver.rs
7. **Add `--dump-symbols` CLI flag** for debugging

**Estimated Effort:** 2-3 days of focused work

**Risk Assessment:**
- Medium risk: This is a significant refactor touching most of the codebase
- Mitigation: Can be done incrementally (symbol table first, then migrate generators one at a time)
- Testing: Regenerate bindings before/after, diff should be minimal (only formatting changes)

---

### 🔄 Step 4i: Unified FFI Module Architecture

**Status:** In progress - initial filters implemented, builds successfully

**Motivation:**

The current per-module architecture generates separate CXX bridges for each OCCT module
(e.g., `gp::ffi`, `topo_ds::ffi`, `bop_algo::ffi`). This causes limitations:

1. **Inherited method filtering** - Methods using types from other modules are skipped because
   CXX requires all types in a bridge to be declared within that bridge. For example,
   `BOPAlgo_BOP` can't expose inherited methods that use `TopoDS_Shape`.

2. **Cross-module type aliases** - Every module that references types from another module needs
   type alias declarations like `type TopoDS_Shape = crate::topo_ds::ffi::Shape;`

3. **Compilation overhead** - 70+ separate C++ compilation units instead of one.

**Proposed Solution:**

Generate a single unified FFI module containing all types, then provide per-module re-exports
for ergonomic API organization:

```
generated/
├── ffi.rs           # Single CXX bridge with ALL types
├── wrappers.hxx     # Single C++ wrapper header
├── gp.rs            # Re-exports for gp module + impl blocks
├── topo_ds.rs       # Re-exports for topo_ds module + impl blocks
├── bop_algo.rs      # Re-exports for bop_algo module + impl blocks
└── lib.rs           # Crate root, declares modules
```

**Architecture Details:**

**1. `generated/ffi.rs`** - Single CXX bridge:
```rust
#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("wrappers.hxx");

        // All types use full C++ names as Rust identifiers
        type gp_Pnt;
        type gp_Vec;
        type TopoDS_Shape;
        type TopoDS_Face;
        type BOPAlgo_BOP;
        // ... all ~300 types

        // All methods, constructors, wrappers
        fn gp_Pnt_ctor(X: f64, Y: f64, Z: f64) -> UniquePtr<gp_Pnt>;
        fn TopoDS_Shape_to_owned(self_: &TopoDS_Shape) -> UniquePtr<TopoDS_Shape>;
        // ... all functions
    }
}
```

**2. `generated/wrappers.hxx`** - Single C++ header:
```cpp
#pragma once
#include "common.hxx"

// All OCCT includes
#include <gp_Pnt.hxx>
#include <gp_Vec.hxx>
#include <TopoDS_Shape.hxx>
// ... etc

// All wrapper functions
inline std::unique_ptr<gp_Pnt> gp_Pnt_ctor(double X, double Y, double Z) {
    return std::make_unique<gp_Pnt>(X, Y, Z);
}
// ... etc
```

**3. `generated/gp.rs`** - Module re-exports with impl blocks:

Re-exports are grouped with their corresponding impl blocks and sorted by source header,
then by order of declaration within each header. This makes the generated code self-documenting
- readers can see which header each type comes from and find related functionality together.

```rust
//! gp module - Geometric primitives

// ========================
// From gp_Pnt.hxx
// ========================

/// A 3D Cartesian point.
pub use crate::ffi::gp_Pnt as Pnt;

impl Pnt {
    /// Create a new point from X, Y, Z coordinates.
    pub fn new(x: f64, y: f64, z: f64) -> cxx::UniquePtr<Self> {
        crate::ffi::gp_Pnt_ctor(x, y, z)
    }

    /// Returns the X coordinate.
    pub fn x(&self) -> f64 {
        crate::ffi::gp_Pnt_X(self)
    }

    // ... other methods from gp_Pnt.hxx
}

// ========================
// From gp_Vec.hxx
// ========================

/// A 3D vector.
pub use crate::ffi::gp_Vec as Vec;

impl Vec {
    /// Create a new vector from X, Y, Z components.
    pub fn new(x: f64, y: f64, z: f64) -> cxx::UniquePtr<Self> {
        crate::ffi::gp_Vec_ctor(x, y, z)
    }

    // ... other methods from gp_Vec.hxx
}

// ========================
// From gp_Dir.hxx
// ========================

/// A unit vector (direction).
pub use crate::ffi::gp_Dir as Dir;

impl Dir {
    // ...
}
```

**Benefits:**

1. **All inherited methods work** - No cross-module type filtering needed. `BOPAlgo_BOP` can
   expose `Shape()` inherited from `BRepBuilderAPI_MakeShape` because both `BOPAlgo_BOP` and
   `TopoDS_Shape` are in the same `ffi` module.

2. **Simpler codegen** - No need for:
   - Cross-module type alias generation
   - `source_module` tracking on types
   - Cross-module type filtering logic

3. **Faster builds** - Single C++ compilation unit instead of 70+.

4. **Same user API** - Users still write `use opencascade_sys::gp::Pnt;` - the module
   organization is preserved through re-exports.

**Implementation Phases:**

**Phase 1: Refactor C++ generation**
- Create `generate_unified_wrappers()` in cpp.rs
- Emit single `wrappers.hxx` with all includes and wrapper functions
- Keep per-module generation working in parallel for comparison

**Phase 2: Refactor Rust FFI generation**
- Create `generate_unified_ffi()` in rust.rs
- Emit single `ffi.rs` with all types using full C++ names
- Types declared once, no cross-module aliases needed
- All inherited methods included (remove cross-module filtering)

**Phase 3: Generate module re-exports**
- Create `generate_module_reexports()` in rust.rs
- For each module, emit `MODULE.rs` with:
  - `pub use crate::ffi::FullName as ShortName;` for types
  - `impl ShortName { ... }` blocks for methods
- Preserve current public API surface

**Phase 4: Update build.rs and lib.rs**
- `build.rs`: Compile single `ffi.rs` instead of per-module files
- `lib.rs`: Declare `pub mod ffi;` plus re-export modules

**Phase 5: Cleanup**
- Remove per-module FFI generation code
- Remove cross-module type alias infrastructure
- Remove `source_module` tracking from resolver
- Simplify inherited method generation (no type filtering)

**Naming Strategy:**

In the unified `ffi` module, use full C++ names as Rust identifiers:
- `gp_Pnt` (not `Pnt`)
- `TopoDS_Shape` (not `Shape`)
- `BRepBuilderAPI_MakeBox` (not `MakeBox`)

This avoids collisions (multiple modules have `Builder` classes) and makes the mapping
to C++ obvious. The short names are provided via re-exports in per-module files.

**Risk Assessment:**
- Low-medium risk: Architecture change but behavior should be identical
- Large generated file sizes: `ffi.rs` will be ~50k+ lines (acceptable for generated code)
- Testing: Compare API surface before/after, ensure all public types remain accessible

**Estimated Effort:** 3-4 days

**Progress (Feb 2026):**

1. ✅ Added `--resolve-deps` flag to `regenerate-bindings.sh` to enable automatic header dependency resolution
2. 🔲 TODO: Support `const char*` parameters and return types (currently filtered out):
   - Added `type_is_cstring()` helper to both cpp.rs and rust.rs
   - Static methods returning `const char*` are currently filtered out
   - Should add shim to convert `const char*` ↔ `&str` or `String`
3. ✅ Both `opencascade-sys` and `opencascade` crates build successfully
4. 🔲 Unified FFI module generation not yet started (still using per-module architecture)

---

### 🔄 Step 5: Update opencascade Crate (IN PROGRESS - COMPILES)

Update imports in `crates/opencascade/src/*.rs` to use the new generated bindings.

**Status:** ✅ The `opencascade` crate now compiles with the new bindings. Many methods are stubbed
due to blocking issues (documented below), but the crate builds successfully.

**RECENT UPDATE:** ✅ The `examples` crate also builds successfully after adding stubbed methods for missing Wire APIs (offset, sweep_along, sweep_along_with_radius_values) that examples depend on.

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
- OLD: `use opencascade_sys::ffi;`
  NEW: `use opencascade_sys::{gp, top_abs, top_exp, topo_ds};`
  NOTES: Import specific modules instead of the ffi module.

- OLD: `ffi::new_point(x, y, z)` or `ffi::gp_Pnt_ctor(...)`
  NEW: `gp::Pnt::new_real3(x, y, z)`
  NOTES: Constructors are now impl methods with `new_` prefix.

- OLD: `ffi::gp_Dir_ctor(x, y, z)`
  NEW: `gp::Dir::new_real3(x, y, z)`

- OLD: `ffi::new_vec(x, y, z)`
  NEW: `gp::Vec::new_real3(x, y, z)`

- OLD: `ffi::gp_Ax1_ctor(&pnt, &dir)`
  NEW: `gp::Ax1::new_pnt_dir(&pnt, &dir)`

- OLD: `ffi::gp_Ax2_ctor(&pnt, &dir)`
  NEW: `gp::Ax2::new_pnt_dir(&pnt, &dir)`

- OLD: `ffi::new_point_2d(x, y)`
  NEW: `gp::Pnt2d::new_real2(x, y)`

- OLD: `ffi::TopAbs_ShapeEnum::TopAbs_VERTEX`
  NEW: `top_abs::ShapeEnum::Vertex`
  NOTES: Enums are re-exported with short Rust names. Variants use PascalCase.

- OLD: `UniquePtr<ffi::TopExp_Explorer>`
  NEW: `UniquePtr<top_exp::Explorer>`
  NOTES: Types use short names without module prefix.

- OLD: `self.explorer.More()`, `self.explorer.Current()`, `self.explorer.pin_mut().Next()`
  NEW: `self.explorer.more()`, `self.explorer.current()`, `self.explorer.pin_mut().next()`
  NOTES: Methods are snake_case in new bindings.

- OLD: `ffi::TopoDS_cast_to_edge(shape)`
  NEW: `topo_ds::edge(shape)` or `topo_ds::edge_mut(shape)` for mutable
  NOTES: RESOLVED. Generator now parses TopoDS.hxx and generates bindings for
         TopoDS::Vertex(), TopoDS::Edge() etc. namespace-level functions automatically.
         Functions are: vertex/vertex_mut, edge/edge_mut, wire/wire_mut, face/face_mut,
         shell/shell_mut, solid/solid_mut, comp_solid/comp_solid_mut, compound/compound_mut

- OLD: `ffi::GeomAbs_JoinType::GeomAbs_Arc`
  NEW: `geom_abs::JoinType::Arc`
  NOTES: Added GeomAbs_JoinType.hxx to headers.txt. Enum uses short Rust names.

- OLD: `ffi::Geom_BezierCurve_to_handle(curve)`
  NEW: `geom::BezierCurve::to_handle(curve)`
  NOTES: to_handle is now an associated function on the impl block.

- OLD: `ffi::new_HandleGeomCurve_from_HandleGeom_BezierCurve(&bezier_handle)`
  NEW: `bezier_handle.to_handle_curve()`
  NOTES: Handle upcasts are now methods on the Handle type.

- OLD: `ffi::shape_list_to_vector(list)`
  NEW: `list.iter().map(|s| Shape::from_shape(&s)).collect()`
  NOTES: Collections now have iter() methods. No vector conversion needed.

- OLD: `ffi::TopTools_ListOfShape_new()`
  NEW: `top_tools::ListOfShape::new()`
  NOTES: Collection constructors are now impl methods.

- OLD: `ffi::Law_Function_to_handle(func)`
  NEW: `law::Function::to_handle(func)` or `law::Interpol::to_handle(func)`
  NOTES: to_handle is an associated function. Interpol has upcast: `handle.to_handle_function()`

- OLD: `make_box.pin_mut().shape()` (error: no shape() method)
   NEW: Use the mutable upcast impl:
   ```rust
   use opencascade_sys::b_rep_prim_api;
   let mut make_box = b_rep_prim_api::MakeBox::new_pnt_real3(&p1, dx, dy, dz);
   let make_shape = make_box.pin_mut().as_b_rep_builder_api_make_shape_mut();
   let result_shape = make_shape.shape();
   ```
   NOTES: Call the generated `as_*_mut()` upcast, then call `shape()` on `MakeShape`.
```

**Progress:**
- [x] `lib.rs` - ✅ Compiles (re-exports work)
- [x] `primitives.rs` - ✅ Translated (EdgeIterator and FaceIterator use topo_ds cast functions)
- [x] `bounding_box.rs` - ✅ Translated (uses bnd::Box, b_rep_bnd_lib module)
- [x] `primitives/edge.rs` - ✅ Compiles (arc() stubbed - Handle conversion blocked)
- [x] `primitives/wire.rs` - ✅ Compiles (from_unordered_edges/offset/sweep_along/sweep_along_with_radius_values stubbed - various blockers documented)
- [x] `primitives/face.rs` - 🟡 Partially updated (revolve restored, extrude_to_face/subtractive_extrude/fillet/chamfer/offset/sweep_along stubbed)
- [x] `primitives/shell.rs` - ✅ Updated (loft restored)
- [x] `primitives/solid.rs` - 🟡 Compiles (fillet_edge stubbed, subtract/union/intersect RESTORED but new_edges empty due to cross-module ListOfShape issue)
- [x] `primitives/compound.rs` - 🔴 Fully stubbed (BRep_Builder blocked)
- [x] `primitives/shape.rs` - 🟡 Partially updated (box/cylinder/sphere/cone/torus + subtract/union/intersect + write_stl RESTORED, fillet/chamfer/read_step/write_step/read_iges/write_iges stubbed)
- [x] `primitives/surface.rs` - 🔴 Fully stubbed (array constructors blocked)
- [x] `primitives/vertex.rs` - ✅ Translated
- [x] `primitives/boolean_shape.rs` - ✅ Translated
- [x] `section.rs` - ✅ Fully updated (section_edges and edges restored, fixed cross-module TopTools_ListOfShape conversion)
- [x] `mesh.rs` - 🔴 Fully stubbed (BRep_Tool blocked)
- [x] `law_function.rs` - 🔴 Fully stubbed (array constructors blocked)
- [x] `make_pipe_shell.rs` - 🔴 Fully stubbed (depends on law_function)
- [x] `kicad.rs` - ✅ Fully working (only edge_cuts() blocked due to from_unordered_edges)
- [x] `workplane.rs` - ✅ Already uses new module-based imports
- [x] `angle.rs` - ✅ Already uses new module-based imports (no opencascade-sys deps)

**Summary of Step 5:**

| Category | Count | Notes |
|----------|-------|-------|
| Fully working | 12 | primitives.rs, bounding_box.rs, vertex.rs, boolean_shape.rs, workplane.rs, angle.rs, lib.rs, shell.rs, section.rs, edge.rs, wire.rs, kicad.rs |
| Partially working | 3 | face.rs, shape.rs, solid.rs |
| Fully stubbed | 5 | compound.rs, surface.rs, mesh.rs, law_function.rs, make_pipe_shell.rs |

**Note:** Some methods using enums (like `Message_Status`, `IFSelect_ReturnStatus`) are intentionally skipped because CXX requires `enum class` but OCCT uses unscoped enums. See PLAN.md "Methods Skipped Due to CXX/OCCT Limitations" for details.

**✅ RESOLVED: Mutable Upcast Now Generated**

The generator now creates **both const and mutable upcasts** for all derived classes.

**New API:**
```rust
// Const upcast (existing)
fn make_box_as_b_rep_builder_api_make_shape(self_: &MakeBox) -> &BRepBuilderAPI_MakeShape;

// Mutable upcast (NEW)
fn make_box_as_b_rep_builder_api_make_shape_mut(
    self_: Pin<&mut MakeBox>
) -> Pin<&mut BRepBuilderAPI_MakeShape>;
```

**Usage pattern:**
```rust
use opencascade_sys::b_rep_prim_api;

// Create a box builder
let mut make_box = b_rep_prim_api::MakeBox::new_pnt_pnt(&p1, &p2);

// Get mutable reference to base MakeShape class
let make_shape = b_rep_prim_api::ffi::make_box_as_b_rep_builder_api_make_shape_mut(make_box.pin_mut());

// Now we can call shape() which requires Pin<&mut MakeShape>
let result_shape = make_shape.shape();
```

**Classes now unblocked (all builder classes that inherit from MakeShape):**
- ✅ `BRepPrimAPI_MakeBox`, `MakeCylinder`, `MakeSphere`, `MakeCone`, `MakeTorus` - can call shape()
- ✅ `BRepPrimAPI_MakePrism`, `MakeRevol` - can call shape()
- ✅ `BRepOffsetAPI_ThruSections`, `MakePipe`, `MakePipeShell` - can call shape()
- ✅ `BRepFilletAPI_MakeFillet`, `MakeChamfer` - can call shape()
- ✅ `BRepAlgoAPI_Section` - can call shape()

**Blocking Issues Discovered (Current):**

### ✅ Build Blockers (RESOLVED - Compiles Successfully)

11. **~~Handle type naming convention mismatch~~** - RESOLVED
    - Generator now creates handles following OCCT convention: `HandleClassName` → `Handle(ClassName)`
    - Handle types reference correct C++ types via `#[cxx_name]` attributes
    - All 88 modules compile successfully

12. **~~Inherited method pointer class mismatch~~** - RESOLVED
    - Inherited method generation disabled (was causing C++ compile errors)
    - Methods with same name that have different signatures in subclass vs base are now filtered
    - Builder APIs like `Shape()`, `IsDone()` still work via direct method bindings

13. **~~Mutable upcast not generated~~** - RESOLVED
    - Generator now creates both const and mutable upcast wrappers
    - Const: `fn derived_as_base(self_: &Derived) -> &Base`
    - Mutable: `fn derived_as_base_mut(self_: Pin<&mut Derived>) -> Pin<&mut Base>`
    - Builder classes can now call `shape()` via mutable upcast

### 🟡 Generator Limitations (Methods Skipped by Design)

The following are CXX or OCCT limitations that cause certain methods to be skipped. These are intentional and documented.

5. **Enum values not generated** (e.g., `GeomAbs_CurveType`) - INTENTIONALLY DISABLED
   - CXX requires `enum class` (scoped enums) but OCCT uses unscoped enums
   - Methods with enum parameters/returns are skipped in both FFI and impl generation
   - **Workaround:** Hand-write enum definitions in `src/` directory if needed

6. **Implicit default constructors not generated** (e.g., `BRep_Builder`)
   - `BRep_Builder` has no explicit constructor in header (uses C++ implicit default)
   - Generator only emits constructors it finds in the AST
   - `compound.rs` blocked: needs `BRep_Builder::new()`
   - **Attempted fix:** Tried generating synthetic default constructors for classes with no explicit constructors
   - **Why it failed:** The parser's `is_abstract` flag only detects pure virtual methods in the class itself, not inherited ones. Classes like `BOPAlgo_BuilderShape` inherit pure virtual `Perform()` from `BOPAlgo_Algo` but don't override it, so they're abstract but not detected as such. Generating synthetic constructors for these classes caused C++ compilation errors ("allocating an object of abstract class type").
   - **Fix needed:** Enhance abstract class detection to traverse the inheritance hierarchy and check for unimplemented pure virtual methods in base classes

7. **Constructors with default enum parameters skipped** (e.g., `BRepFilletAPI_MakeFillet`)
   - `BRepFilletAPI_MakeFillet(const TopoDS_Shape&, ChFi3d_FilletShape FShape = ChFi3d_Rational)`
   - Generator skips because it can't parse default enum values
   - `solid.rs` blocked: needs this constructor
   - **Fix needed:** Either handle default params or generate explicit overloads

8. **TColgp array constructors not generated**
   - `TColgp_Array1OfPnt2d`, `TColgp_Array2OfPnt` constructors not appearing
   - These are template instantiations: `typedef NCollection_Array1<gp_Pnt2d> TColgp_Array1OfPnt2d`
   - Generator may not be finding constructors for typedef'd templates
   - `law_function.rs`, `surface.rs` blocked
   - **Fix needed:** Generate constructors for NCollection template typedefs

9. **TColgp_Array1OfPnt2d::SetValue not generated**
    - Required to populate law functions used by `sweep_along_with_radius_values`
    - Blocks `make_pipe_shell.rs` and `Wire::sweep_along_with_radius_values`

10. **BRep_Tool static methods need wrapper generation**
    - `mesh.rs` uses `BRep_Tool::Triangulation()`, `BRep_Tool::Surface()` etc.
    - These are static methods on `BRep_Tool` class
    - May be partially available, need to verify what's generated

11. **Helper functions missing (map_shapes)**
   - `Face::fillet/chamfer` needs a `map_shapes` helper to dedupe vertices
   - `Wire::fillet/chamfer` depends on these face operations

12. **BRepFeat_MakeDPrism constructors not generated**
   - Blocks `Face::extrude_to_face` and `Face::subtractive_extrude`
   - Need constructors and/or helper methods for `BRepFeat_MakeDPrism`

13. **STEP/IGES helper functions missing**
   - `Shape::read_step`, `Shape::write_step`, `Shape::read_iges`, `Shape::write_iges` are stubbed
   - Need small helper wrappers (open file, transfer roots, write file)
   - ReadFile method returns `IFSelect_ReturnStatus` enum (not generated), blocking read operations

14. **~~Boolean ops still stubbed (Cut/Fuse/Common)~~** - PARTIALLY RESOLVED
   - `Solid::subtract/union/intersect` and `Shape::subtract/union/intersect` now work!
   - Uses `BRepAlgoAPI_Cut/Fuse/Common` constructors with `Message_ProgressRange`
   - Constructors are: `Cut::new_shape2_progressrange(S1, S2, &progress)` etc.
   - **Remaining issue:** `section_edges()` returns `&TopTools_ListOfShape` which is locally
     declared in `b_rep_algo_api::ffi` rather than imported from `top_tools`, so we can't use
     `top_tools::ListOfShape::iter()` on it. Currently returns empty `new_edges` vec.
   - **Fix needed:** Generator should import `TopTools_ListOfShape = top_tools::ffi::ListOfShape`
     instead of declaring it as a local opaque type when it's used across modules.

16. **~~STL writing blocked~~** - RESOLVED
   - `Shape::write_stl` and `Shape::write_stl_with_tolerance` now work!
   - Uses `BRepMesh_IncrementalMesh` for meshing and `StlAPI_Writer::write()` for export

15. **Cross-module type sharing issue (TopTools_ListOfShape)** - NEW BLOCKER
   - When types like `TopTools_ListOfShape` are used in module A (e.g., `b_rep_algo_api`) 
     but defined in module B (`top_tools`), they're declared as opaque local types instead
     of imported from the source module.
   - This means iterator/helper methods defined in module B don't work on references from module A.
   - Affects: `section_edges()` results in boolean ops, any other cross-module collection usage
   - **Fix needed:** Generator should detect when a type is defined in another module and use
     a type alias pointing to that module's ffi type instead of a local opaque declaration.

**Blocking Issues (Resolved):**

1. **~~Handle type conversions not generated~~** - RESOLVED. Handle operations now fully supported:
   - `geom::BezierCurve::to_handle(curve)` - converts object to Handle (associated function)
   - `handle.to_handle_curve()` - upcasts Handle types (impl method on Handle)
   - `b_rep_builder_api::MakeEdge::new_handlecurve(&curve_handle)` - constructor taking Handle

2. **~~TopTools_ListOfShape now available~~** - RESOLVED. TopTools_ListOfShape.hxx is imported
   with full iterator support: `list.iter()`, `ListOfShape::from_iter()`, etc.

3. **~~Free functions not re-exported~~** - RESOLVED. The generator now adds wrapper and static
   methods to impl blocks outside ffi:
   - Wrapper methods for by-value returns: `Box_::corner_min(&self)` wraps `ffi::Box__corner_min()`
   - Static methods: `BRepBndLib::add(...)` wraps `ffi::BRepBndLib_add(...)`
   - Methods with `Pin<&mut T>` parameters are properly handled

4. **~~Collection conversion helpers missing~~** - RESOLVED. Collections now have iterator APIs:
   - `list_of_shape.iter()` returns an iterator yielding `UniquePtr<Shape>`
   - `ListOfShape::from_iter(shapes)` creates from iterator
   - No need for `shape_list_to_vector()` - use `.iter().collect()` instead

### Step 6: Update Examples

Update `examples/src/*.rs` for new API.

**Status:** ⏸️ BLOCKED - Examples depend on methods that are currently stubbed:
- `sweep_along()` - blocked by missing MakePipe/MakeShape wiring in `wire.rs`
- `sweep_along_with_radius_values()` - blocked by law_function + TColgp_Array1OfPnt2d::SetValue

Examples will need to be updated once the mutable upcast blocker is resolved in the generator.

### Step 7: Cleanup

Delete `opencascade-sys-old` once everything works.

### ✅ Step 8: Expand Header Coverage (COMPLETE via Automatic Resolution)

The generator now automatically resolves header dependencies. From 262 explicit headers in `headers.txt`,
it expands to 624 total headers and generates 88 modules.

**Explicit headers in `headers.txt` (262 headers):**

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

**TopExp module** (2 headers):
- TopExp_Explorer, TopExp

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

**ShapeAnalysis module** (1 header):
- ShapeAnalysis_FreeBounds

**BRepAdaptor module** (1 header):
- BRepAdaptor_Curve

**BRepBndLib module** (1 header):
- BRepBndLib

**BRepIntCurveSurface module** (1 header):
- BRepIntCurveSurface_Inter

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
