# FFI Mapping: Rust to OpenCASCADE Technology (OCCT)

This document provides a comprehensive mapping between the Rust FFI declarations in `opencascade-sys/src/lib.rs` and their corresponding C++ OpenCASCADE Technology (OCCT) types and functions.

## Enums

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `TopAbs_ShapeEnum` | `TopAbs_ShapeEnum` | Enumeration of shape types | `TopAbs_ShapeEnum.hxx` | `top_abs` |
| `TopAbs_Orientation` | `TopAbs_Orientation` | Enumeration of orientations | `TopAbs_Orientation.hxx` | `top_abs` |
| `IFSelect_ReturnStatus` | `IFSelect_ReturnStatus` | Return status for import/export operations | `IFSelect_ReturnStatus.hxx` | `if_select` |
| `BOPAlgo_GlueEnum` | `BOPAlgo_GlueEnum` | Boolean operation gluing modes | `BOPAlgo_GlueEnum.hxx` | `bop_algo` |
| `GeomAbs_CurveType` | `GeomAbs_CurveType` | Geometric curve type enumeration | `GeomAbs_CurveType.hxx` | `geom_abs` |
| `GeomAbs_JoinType` | `GeomAbs_JoinType` | Geometric join type enumeration | `GeomAbs_JoinType.hxx` | `geom_abs` |

## Core Runtime Types

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `Message_ProgressRange` | `Message_ProgressRange` | Progress indication for operations | `Message_ProgressRange.hxx` | `message` |

## Handle Types (Smart Pointers)

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `HandleStandardType` | `Handle(Standard_Type)` | Handle to type information | `Standard_Type.hxx` | `standard` |
| `HandleGeomCurve` | `Handle(Geom_Curve)` | Handle to geometric curve | `Geom_Curve.hxx` | `geom` |
| `HandleGeomBSplineCurve` | `Handle(Geom_BSplineCurve)` | Handle to B-spline curve | `Geom_BSplineCurve.hxx` | `geom` |
| `HandleGeomBezierCurve` | `Handle(Geom_BezierCurve)` | Handle to Bezier curve | `Geom_BezierCurve.hxx` | `geom` |
| `HandleGeomTrimmedCurve` | `Handle(Geom_TrimmedCurve)` | Handle to trimmed curve | `Geom_TrimmedCurve.hxx` | `geom` |
| `HandleGeomSurface` | `Handle(Geom_Surface)` | Handle to geometric surface | `Geom_Surface.hxx` | `geom` |
| `HandleGeomBezierSurface` | `Handle(Geom_BezierSurface)` | Handle to Bezier surface | `Geom_BezierSurface.hxx` | `geom` |
| `HandleGeomPlane` | `Handle(Geom_Plane)` | Handle to geometric plane | `Geom_Plane.hxx` | `geom` |
| `HandleGeom2d_Curve` | `Handle(Geom2d_Curve)` | Handle to 2D curve | `Geom2d_Curve.hxx` | `geom` |
| `HandleGeom2d_Ellipse` | `Handle(Geom2d_Ellipse)` | Handle to 2D ellipse | `Geom2d_Ellipse.hxx` | `geom` |
| `HandleGeom2d_TrimmedCurve` | `Handle(Geom2d_TrimmedCurve)` | Handle to 2D trimmed curve | `Geom2d_TrimmedCurve.hxx` | `geom` |
| `HandleGeom_CylindricalSurface` | `Handle(Geom_CylindricalSurface)` | Handle to cylindrical surface | `Geom_CylindricalSurface.hxx` | `geom` |
| `HandleTopTools_HSequenceOfShape` | `Handle(TopTools_HSequenceOfShape)` | Handle to sequence of shapes | `TopTools_HSequenceOfShape.hxx` | `top_tools` |
| `HandleLawFunction` | `Handle(Law_Function)` | Handle to law function | `Law_Function.hxx` | `law` |
| `Handle_TColgpHArray1OfPnt` | `Handle(TColgp_HArray1OfPnt)` | Handle to array of points | `TColgp_HArray1OfPnt.hxx` | `tcolgp` |
| `HandlePoly_Triangulation` | `Handle(Poly_Triangulation)` | Handle to triangulation | `Poly_Triangulation.hxx` | `poly` |

## Collection Types

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `TopTools_ListOfShape` | `TopTools_ListOfShape` | List of TopoDS shapes | `TopTools_ListOfShape.hxx` | `top_tools` |
| `TopTools_IndexedMapOfShape` | `TopTools_IndexedMapOfShape` | Indexed map of shapes | `TopTools_IndexedMapOfShape.hxx` | `top_tools` |
| `TopTools_IndexedDataMapOfShapeListOfShape` | `TopTools_IndexedDataMapOfShapeListOfShape` | Map from shape to list of shapes | `TopTools_IndexedDataMapOfShapeListOfShape.hxx` | `top_tools` |
| `TColgp_Array1OfDir` | `TColgp_Array1OfDir` | Array of directions | `TColgp_Array1OfDir.hxx` | `tcolgp` |
| `TColgp_Array1OfPnt2d` | `TColgp_Array1OfPnt2d` | Array of 2D points | `TColgp_Array1OfPnt2d.hxx` | `tcolgp` |
| `TColgp_Array2OfPnt` | `TColgp_Array2OfPnt` | 2D array of 3D points | `TColgp_Array2OfPnt.hxx` | `tcolgp` |
| `TColgp_HArray1OfPnt` | `TColgp_HArray1OfPnt` | Handle array of points | `TColgp_HArray1OfPnt.hxx` | `tcolgp` |
| `TopTools_HSequenceOfShape` | `TopTools_HSequenceOfShape` | Sequence of shapes | `TopTools_HSequenceOfShape.hxx` | `top_tools` |

## Geometric Types

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `gp_Pnt` | `gp_Pnt` | 3D point | `gp_Pnt.hxx` | `gp` |
| `gp_Pnt2d` | `gp_Pnt2d` | 2D point | `gp_Pnt2d.hxx` | `gp` |
| `gp_Vec` | `gp_Vec` | 3D vector | `gp_Vec.hxx` | `gp` |
| `gp_Dir` | `gp_Dir` | 3D direction | `gp_Dir.hxx` | `gp` |
| `gp_Dir2d` | `gp_Dir2d` | 2D direction | `gp_Dir2d.hxx` | `gp` |
| `gp_Lin` | `gp_Lin` | 3D line | `gp_Lin.hxx` | `gp` |
| `gp_Circ` | `gp_Circ` | 3D circle | `gp_Circ.hxx` | `gp` |
| `gp_Ax1` | `gp_Ax1` | 3D axis (point + direction) | `gp_Ax1.hxx` | `gp` |
| `gp_Ax2` | `gp_Ax2` | 3D coordinate system | `gp_Ax2.hxx` | `gp` |
| `gp_Ax3` | `gp_Ax3` | 3D coordinate system with handedness | `gp_Ax3.hxx` | `gp` |
| `gp_Ax2d` | `gp_Ax2d` | 2D coordinate system | `gp_Ax2d.hxx` | `gp` |
| `gp_Trsf` | `gp_Trsf` | 3D transformation | `gp_Trsf.hxx` | `gp` |
| `gp_GTrsf` | `gp_GTrsf` | 3D general transformation | `gp_GTrsf.hxx` | `gp` |

## Geometric Curve and Surface Types

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `Geom_TrimmedCurve` | `Geom_TrimmedCurve` | Trimmed 3D curve | `Geom_TrimmedCurve.hxx` | `geom` |
| `Geom_CylindricalSurface` | `Geom_CylindricalSurface` | Cylindrical surface | `Geom_CylindricalSurface.hxx` | `geom` |
| `Geom_BezierSurface` | `Geom_BezierSurface` | Bezier surface | `Geom_BezierSurface.hxx` | `geom` |
| `Geom_BezierCurve` | `Geom_BezierCurve` | Bezier curve | `Geom_BezierCurve.hxx` | `geom` |
| `Geom2d_Ellipse` | `Geom2d_Ellipse` | 2D ellipse | `Geom2d_Ellipse.hxx` | `geom` |
| `Geom2d_Curve` | `Geom2d_Curve` | 2D curve | `Geom2d_Curve.hxx` | `geom` |
| `Geom2d_TrimmedCurve` | `Geom2d_TrimmedCurve` | 2D trimmed curve | `Geom2d_TrimmedCurve.hxx` | `geom` |

## Topology Types

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `TopoDS_Vertex` | `TopoDS_Vertex` | Topological vertex | `TopoDS_Vertex.hxx` | `topods` |
| `TopoDS_Edge` | `TopoDS_Edge` | Topological edge | `TopoDS_Edge.hxx` | `topods` |
| `TopoDS_Wire` | `TopoDS_Wire` | Topological wire | `TopoDS_Wire.hxx` | `topods` |
| `TopoDS_Face` | `TopoDS_Face` | Topological face | `TopoDS_Face.hxx` | `topods` |
| `TopoDS_Shell` | `TopoDS_Shell` | Topological shell | `TopoDS_Shell.hxx` | `topods` |
| `TopoDS_Solid` | `TopoDS_Solid` | Topological solid | `TopoDS_Solid.hxx` | `topods` |
| `TopoDS_Shape` | `TopoDS_Shape` | Generic topological shape | `TopoDS_Shape.hxx` | `topods` |
| `TopoDS_Compound` | `TopoDS_Compound` | Compound shape | `TopoDS_Compound.hxx` | `topods` |

## Builder Types

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `BRep_Builder` | `BRep_Builder` | Boundary representation builder | `BRep_Builder.hxx` | `brep` |
| `TopoDS_Builder` | `TopoDS_Builder` | Topology builder | `TopoDS_Builder.hxx` | `topods` |
| `BRepBuilderAPI_MakeVertex` | `BRepBuilderAPI_MakeVertex` | Vertex builder | `BRepBuilderAPI_MakeVertex.hxx` | `brep_builder_api` |
| `BRepBuilderAPI_MakeEdge` | `BRepBuilderAPI_MakeEdge` | Edge builder | `BRepBuilderAPI_MakeEdge.hxx` | `brep_builder_api` |
| `BRepBuilderAPI_MakeWire` | `BRepBuilderAPI_MakeWire` | Wire builder | `BRepBuilderAPI_MakeWire.hxx` | `brep_builder_api` |
| `BRepBuilderAPI_MakeFace` | `BRepBuilderAPI_MakeFace` | Face builder | `BRepBuilderAPI_MakeFace.hxx` | `brep_builder_api` |
| `BRepBuilderAPI_MakeSolid` | `BRepBuilderAPI_MakeSolid` | Solid builder | `BRepBuilderAPI_MakeSolid.hxx` | `brep_builder_api` |
| `BRepBuilderAPI_MakeShapeOnMesh` | `BRepBuilderAPI_MakeShapeOnMesh` | Shape from mesh builder | `BRepBuilderAPI_MakeShapeOnMesh.hxx` | `brep_builder_api` |
| `BRepBuilderAPI_Transform` | `BRepBuilderAPI_Transform` | Shape transformation builder | `BRepBuilderAPI_Transform.hxx` | `brep_builder_api` |
| `BRepBuilderAPI_GTransform` | `BRepBuilderAPI_GTransform` | General transformation builder | `BRepBuilderAPI_GTransform.hxx` | `brep_builder_api` |

## Primitive Builders

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `BRepPrimAPI_MakePrism` | `BRepPrimAPI_MakePrism` | Prism builder | `BRepPrimAPI_MakePrism.hxx` | `brep_prim_api` |
| `BRepPrimAPI_MakeRevol` | `BRepPrimAPI_MakeRevol` | Revolution builder | `BRepPrimAPI_MakeRevol.hxx` | `brep_prim_api` |
| `BRepPrimAPI_MakeCylinder` | `BRepPrimAPI_MakeCylinder` | Cylinder builder | `BRepPrimAPI_MakeCylinder.hxx` | `brep_prim_api` |
| `BRepPrimAPI_MakeBox` | `BRepPrimAPI_MakeBox` | Box builder | `BRepPrimAPI_MakeBox.hxx` | `brep_prim_api` |
| `BRepPrimAPI_MakeSphere` | `BRepPrimAPI_MakeSphere` | Sphere builder | `BRepPrimAPI_MakeSphere.hxx` | `brep_prim_api` |
| `BRepPrimAPI_MakeCone` | `BRepPrimAPI_MakeCone` | Cone builder | `BRepPrimAPI_MakeCone.hxx` | `brep_prim_api` |
| `BRepPrimAPI_MakeTorus` | `BRepPrimAPI_MakeTorus` | Torus builder | `BRepPrimAPI_MakeTorus.hxx` | `brep_prim_api` |

## Feature Operations

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `BRepFeat_MakeDPrism` | `BRepFeat_MakeDPrism` | Draft prism feature | `BRepFeat_MakeDPrism.hxx` | `brep_feat` |
| `BRepFeat_MakeCylindricalHole` | `BRepFeat_MakeCylindricalHole` | Cylindrical hole feature | `BRepFeat_MakeCylindricalHole.hxx` | `brep_feat` |

## Fillet and Chamfer Operations

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `BRepFilletAPI_MakeFillet` | `BRepFilletAPI_MakeFillet` | 3D fillet builder | `BRepFilletAPI_MakeFillet.hxx` | `brep_fillet_api` |
| `BRepFilletAPI_MakeFillet2d` | `BRepFilletAPI_MakeFillet2d` | 2D fillet builder | `BRepFilletAPI_MakeFillet2d.hxx` | `brep_fillet_api` |
| `BRepFilletAPI_MakeChamfer` | `BRepFilletAPI_MakeChamfer` | Chamfer builder | `BRepFilletAPI_MakeChamfer.hxx` | `brep_fillet_api` |

## Offset Operations

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `BRepOffsetAPI_MakeOffset` | `BRepOffsetAPI_MakeOffset` | 2D offset builder | `BRepOffsetAPI_MakeOffset.hxx` | `brep_offset_api` |
| `BRepOffsetAPI_MakeThickSolid` | `BRepOffsetAPI_MakeThickSolid` | Thick solid builder | `BRepOffsetAPI_MakeThickSolid.hxx` | `brep_offset_api` |
| `BRepOffsetAPI_MakePipe` | `BRepOffsetAPI_MakePipe` | Pipe builder | `BRepOffsetAPI_MakePipe.hxx` | `brep_offset_api` |
| `BRepOffsetAPI_MakePipeShell` | `BRepOffsetAPI_MakePipeShell` | Pipe shell builder | `BRepOffsetAPI_MakePipeShell.hxx` | `brep_offset_api` |
| `BRepOffsetAPI_ThruSections` | `BRepOffsetAPI_ThruSections` | Lofting builder | `BRepOffsetAPI_ThruSections.hxx` | `brep_offset_api` |

## Boolean Operations

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `BRepAlgoAPI_BuilderAlgo` | `BRepAlgoAPI_BuilderAlgo` | Base boolean algorithm | `BRepAlgoAPI_BuilderAlgo.hxx` | `brep_algo_api` |
| `BRepAlgoAPI_Fuse` | `BRepAlgoAPI_Fuse` | Boolean union | `BRepAlgoAPI_Fuse.hxx` | `brep_algo_api` |
| `BRepAlgoAPI_Cut` | `BRepAlgoAPI_Cut` | Boolean subtraction | `BRepAlgoAPI_Cut.hxx` | `brep_algo_api` |
| `BRepAlgoAPI_Common` | `BRepAlgoAPI_Common` | Boolean intersection | `BRepAlgoAPI_Common.hxx` | `brep_algo_api` |
| `BRepAlgoAPI_Section` | `BRepAlgoAPI_Section` | Boolean section | `BRepAlgoAPI_Section.hxx` | `brep_algo_api` |

## Curve and Surface Builders

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `GC_MakeSegment` | `GC_MakeSegment` | 3D line segment builder | `GC_MakeSegment.hxx` | `gc` |
| `GCE2d_MakeSegment` | `GCE2d_MakeSegment` | 2D line segment builder | `GCE2d_MakeSegment.hxx` | `gce2d` |
| `GC_MakeArcOfCircle` | `GC_MakeArcOfCircle` | Circular arc builder | `GC_MakeArcOfCircle.hxx` | `gc` |

## Law Functions

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `Law_Function` | `Law_Function` | Law function base class | `Law_Function.hxx` | `law` |
| `Law_Interpol` | `Law_Interpol` | Interpolated law function | `Law_Interpol.hxx` | `law` |

## Curve Adaptation

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `BRepAdaptor_Curve` | `BRepAdaptor_Curve` | Curve adaptor for edges | `BRepAdaptor_Curve.hxx` | `brep_adaptor` |

## Geometric Interpolation and Projection

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `GeomAPI_Interpolate` | `GeomAPI_Interpolate` | Curve interpolation | `GeomAPI_Interpolate.hxx` | `geom_api` |
| `GeomAPI_ProjectPointOnSurf` | `GeomAPI_ProjectPointOnSurf` | Point projection on surface | `GeomAPI_ProjectPointOnSurf.hxx` | `geom_api` |

## Topology Exploration

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `TopExp_Explorer` | `TopExp_Explorer` | Topology explorer | `TopExp_Explorer.hxx` | `top_exp` |
| `TopLoc_Location` | `TopLoc_Location` | Topological location | `TopLoc_Location.hxx` | `top_loc` |

## Intersection Operations

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `BRepIntCurveSurface_Inter` | `BRepIntCurveSurface_Inter` | Curve-surface intersection | `BRepIntCurveSurface_Inter.hxx` | `brep_int_curve_surface` |

## Import/Export

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `STEPControl_Reader` | `STEPControl_Reader` | STEP file reader | `STEPControl_Reader.hxx` | `step_control` |
| `STEPControl_Writer` | `STEPControl_Writer` | STEP file writer | `STEPControl_Writer.hxx` | `step_control` |
| `IGESControl_Reader` | `IGESControl_Reader` | IGES file reader | `IGESControl_Reader.hxx` | `iges_control` |
| `IGESControl_Writer` | `IGESControl_Writer` | IGES file writer | `IGESControl_Writer.hxx` | `iges_control` |
| `StlAPI_Writer` | `StlAPI_Writer` | STL file writer | `StlAPI_Writer.hxx` | `stl_api` |

## Meshing

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `BRepMesh_IncrementalMesh` | `BRepMesh_IncrementalMesh` | Incremental meshing | `BRepMesh_IncrementalMesh.hxx` | `brep_mesh` |
| `Poly_Triangulation` | `Poly_Triangulation` | Triangular mesh | `Poly_Triangulation.hxx` | `poly` |
| `Poly_Triangle` | `Poly_Triangle` | Triangle in mesh | `Poly_Triangle.hxx` | `poly` |
| `Poly_Connect` | `Poly_Connect` | Mesh connectivity | `Poly_Connect.hxx` | `poly` |

## Edge Approximation

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `GCPnts_TangentialDeflection` | `GCPnts_TangentialDeflection` | Edge discretization | `GCPnts_TangentialDeflection.hxx` | `gcpnts` |

## Shape Properties

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `GProp_GProps` | `GProp_GProps` | Geometric properties | `GProp_GProps.hxx` | `gprop` |
| `BRepGProp_Face` | `BRepGProp_Face` | Face geometric properties | `BRepGProp_Face.hxx` | `brep_gprop` |

## Shape Cleaning

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `ShapeUpgrade_UnifySameDomain` | `ShapeUpgrade_UnifySameDomain` | Shape unification | `ShapeUpgrade_UnifySameDomain.hxx` | `shape_upgrade` |

## Bounding Box

| Rust Type | C++ Type | Description | Header File | Rust Module |
|-----------|----------|-------------|-------------|-------------|
| `Bnd_Box` | `Bnd_Box` | 3D bounding box | `Bnd_Box.hxx` | `bnd` |
| `BRepBndLib` | `BRepBndLib` | Bounding box computation | `BRepBndLib.hxx` | `brep_bnd_lib` |

## Notable Function Mappings

### Static Functions and Constants
| Rust Function | C++ Function/Constant | Description | Header File | Rust Module |
|---------------|----------------------|-------------|-------------|-------------|
| `gp_OX()` | `gp::OX()` | X-axis | `gp.hxx` | `gp` |
| `gp_OY()` | `gp::OY()` | Y-axis | `gp.hxx` | `gp` |
| `gp_OZ()` | `gp::OZ()` | Z-axis | `gp.hxx` | `gp` |
| `gp_DZ()` | `gp::DZ()` | Z-direction | `gp.hxx` | `gp` |
| `BRepLibBuildCurves3d()` | `BRepLib::BuildCurves3d()` | Build 3D curves on edges | `BRepLib.hxx` | `brep_lib` |

### Tool Functions
| Rust Function | C++ Function | Description | Header File | Rust Module |
|---------------|--------------|-------------|-------------|-------------|
| `BRep_Tool_Surface()` | `BRep_Tool::Surface()` | Get surface from face | `BRep_Tool.hxx` | `brep_tool` |
| `BRep_Tool_Curve()` | `BRep_Tool::Curve()` | Get curve from edge | `BRep_Tool.hxx` | `brep_tool` |
| `BRep_Tool_Pnt()` | `BRep_Tool::Pnt()` | Get point from vertex | `BRep_Tool.hxx` | `brep_tool` |
| `BRep_Tool_Triangulation()` | `BRep_Tool::Triangulation()` | Get triangulation from face | `BRep_Tool.hxx` | `brep_tool` |

### Property Functions
| Rust Function | C++ Function | Description | Header File | Rust Module |
|---------------|--------------|-------------|-------------|-------------|
| `BRepGProp_LinearProperties()` | `BRepGProp::LinearProperties()` | Compute linear properties | `BRepGProp.hxx` | `brep_gprop` |
| `BRepGProp_SurfaceProperties()` | `BRepGProp::SurfaceProperties()` | Compute surface properties | `BRepGProp.hxx` | `brep_gprop` |
| `BRepGProp_VolumeProperties()` | `BRepGProp::VolumeProperties()` | Compute volume properties | `BRepGProp.hxx` | `brep_gprop` |

### Explorer Functions
| Rust Function | C++ Function | Description | Header File | Rust Module |
|---------------|--------------|-------------|-------------|-------------|
| `TopExp_FirstVertex()` | `TopExp::FirstVertex()` | Get first vertex of edge | `TopExp.hxx` | `top_exp` |
| `TopExp_LastVertex()` | `TopExp::LastVertex()` | Get last vertex of edge | `TopExp.hxx` | `top_exp` |
| `TopExp_EdgeVertices()` | `TopExp::Vertices()` | Get edge vertices | `TopExp.hxx` | `top_exp` |
| `TopExp_WireVertices()` | `TopExp::Vertices()` | Get wire vertices | `TopExp.hxx` | `top_exp` |
| `TopExp_CommonVertex()` | `TopExp::CommonVertex()` | Find common vertex | `TopExp.hxx` | `top_exp` |

## Custom Wrapper Functions

Some functions in the FFI are custom wrappers defined in [`wrapper.hxx`](../crates/opencascade-sys/include/wrapper.hxx) to handle C++/Rust interfacing constraints. These include:

- Type casting functions (e.g., `cast_*_to_shape`)
- Handle conversion functions 
- Functions returning `UniquePtr` wrappers for stack-allocated returns
- Utility functions for collections and iterators

## Notes

1. **Handle Types**: OCCT uses reference-counted handles extensively. These are mapped to Rust `Handle*` types.

2. **Constructor Pattern**: Most constructors use the `construct_unique` pattern with `#[cxx_name = "construct_unique"]` to return `UniquePtr<T>`.

3. **Method vs Function**: Class methods use `self` parameter, while static/free functions do not.

4. **Mutability**: Non-const C++ methods require `Pin<&mut T>`, const methods use `&T`.

5. **Memory Management**: CXX handles memory safety between Rust and C++, with smart pointers (`UniquePtr`, handles) managing object lifetimes.

6. **Error Handling**: Most operations return status enums or boolean success indicators rather than throwing exceptions.

This mapping enables Rust code to safely interact with the extensive OpenCASCADE Technology C++ library through a type-safe FFI layer.