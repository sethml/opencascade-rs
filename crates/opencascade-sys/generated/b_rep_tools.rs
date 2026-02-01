#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_b_rep_tools.hxx");
        #[doc = "Shape from topo_ds module"]
        type TopoDS_Shape = crate::topo_ds::ffi::Shape;
        #[doc = "Vertex from topo_ds module"]
        type TopoDS_Vertex = crate::topo_ds::ffi::Vertex;
        #[doc = "Edge from topo_ds module"]
        type TopoDS_Edge = crate::topo_ds::ffi::Edge;
        #[doc = "Wire from topo_ds module"]
        type TopoDS_Wire = crate::topo_ds::ffi::Wire;
        #[doc = "Face from topo_ds module"]
        type TopoDS_Face = crate::topo_ds::ffi::Face;
        #[doc = "Shell from topo_ds module"]
        type TopoDS_Shell = crate::topo_ds::ffi::Shell;
        #[doc = "Solid from topo_ds module"]
        type TopoDS_Solid = crate::topo_ds::ffi::Solid;
        #[doc = "Compound from topo_ds module"]
        type TopoDS_Compound = crate::topo_ds::ffi::Compound;
        #[doc = "CompSolid from topo_ds module"]
        type TopoDS_CompSolid = crate::topo_ds::ffi::CompSolid;
        #[doc = "Builder from topo_ds module"]
        type TopoDS_Builder = crate::topo_ds::ffi::Builder;
        #[doc = "FormatVersion from top_tools module"]
        type TopTools_FormatVersion = crate::top_tools::ffi::TopTools_FormatVersion;
        #[doc = "ProgressRange from message module"]
        type Message_ProgressRange = crate::message::ffi::ProgressRange;
        #[doc = "Box from bnd module"]
        type Bnd_Box = crate::bnd::ffi::Box_;
        #[doc = "OBB from bnd module"]
        type Bnd_OBB = crate::bnd::ffi::OBB;
        #[doc = "ShapeEnum from top_abs module"]
        type TopAbs_ShapeEnum = crate::top_abs::ffi::TopAbs_ShapeEnum;
        #[doc = "Orientation from top_abs module"]
        type TopAbs_Orientation = crate::top_abs::ffi::TopAbs_Orientation;
        #[doc = "Curve from geom2d module"]
        type Geom2d_Curve = crate::geom2d::ffi::Curve;
        #[doc = "Ellipse from geom2d module"]
        type Geom2d_Ellipse = crate::geom2d::ffi::Ellipse;
        #[doc = "TrimmedCurve from geom2d module"]
        type Geom2d_TrimmedCurve = crate::geom2d::ffi::TrimmedCurve;
        #[doc = "BezierCurve from geom module"]
        type Geom_BezierCurve = crate::geom::ffi::BezierCurve;
        #[doc = "BSplineCurve from geom module"]
        type Geom_BSplineCurve = crate::geom::ffi::BSplineCurve;
        #[doc = "Curve from geom module"]
        type Geom_Curve = crate::geom::ffi::Curve;
        #[doc = "CylindricalSurface from geom module"]
        type Geom_CylindricalSurface = crate::geom::ffi::CylindricalSurface;
        #[doc = "Plane from geom module"]
        type Geom_Plane = crate::geom::ffi::Plane;
        #[doc = "Surface from geom module"]
        type Geom_Surface = crate::geom::ffi::Surface;
        #[doc = "TrimmedCurve from geom module"]
        type Geom_TrimmedCurve = crate::geom::ffi::TrimmedCurve;
        #[doc = "Tool from b_rep module"]
        type BRep_Tool = crate::b_rep::ffi::Tool;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TopTools_ListOfShape"]
        type TopTools_ListOfShape;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TopTools_IndexedMapOfShape"]
        type TopTools_IndexedMapOfShape;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "OSD_FileSystem"]
        type OSD_FileSystem;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "BRep_Builder"]
        type BRep_Builder;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Bnd_Box2d"]
        type Bnd_Box2d;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleGeomSurface"]
        type HandleGeomSurface;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleGeom2dCurve"]
        type HandleGeom2dCurve;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleGeomCurve"]
        type HandleGeomCurve;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleOSDFileSystem"]
        type HandleOSDFileSystem;
        #[doc = " ======================== BRepTools ========================"]
        #[doc = "The BRepTools package provides  utilities for BRep data structures. * WireExplorer : A tool to explore the topology of a wire in the order of the edges. * ShapeSet :  Tools used for  dumping, writing and reading. * UVBounds : Methods to compute the  limits of the boundary  of a  face,  a wire or   an edge in  the parametric space of a face. *  Update : Methods  to call when   a topology has been created to compute all missing data. * UpdateFaceUVPoints: Method to update the UV points stored with the edges on a face. * Compare : Method to compare two vertices. * Compare : Method to compare two edges. * OuterWire : A method to find the outer wire of a face. * Map3DEdges : A method to map all the 3D Edges of a Shape. * Dump : A method to dump a BRep object."]
        #[cxx_name = "BRepTools"]
        type BRepTools;
        #[doc = "Returns in UMin,  UMax, VMin,  VMax  the  bounding values in the parametric space of F."]
        #[cxx_name = "BRepTools_UVBounds_face_real4"]
        fn BRepTools_uv_bounds_face_real4(
            F: &TopoDS_Face,
            UMin: &mut f64,
            UMax: &mut f64,
            VMin: &mut f64,
            VMax: &mut f64,
        );
        #[doc = "Returns in UMin,  UMax, VMin,  VMax  the  bounding values of the wire in the parametric space of F."]
        #[cxx_name = "BRepTools_UVBounds_face_wire_real4"]
        fn BRepTools_uv_bounds_face_wire_real4(
            F: &TopoDS_Face,
            W: &TopoDS_Wire,
            UMin: &mut f64,
            UMax: &mut f64,
            VMin: &mut f64,
            VMax: &mut f64,
        );
        #[doc = "Returns in UMin,  UMax, VMin,  VMax  the  bounding values of the edge in the parametric space of F."]
        #[cxx_name = "BRepTools_UVBounds_face_edge_real4"]
        fn BRepTools_uv_bounds_face_edge_real4(
            F: &TopoDS_Face,
            E: &TopoDS_Edge,
            UMin: &mut f64,
            UMax: &mut f64,
            VMin: &mut f64,
            VMax: &mut f64,
        );
        #[doc = "Adds  to  the box <B>  the bounding values in  the parametric space of F."]
        #[cxx_name = "BRepTools_AddUVBounds_face_box2d"]
        fn BRepTools_add_uv_bounds_face_box2d(F: &TopoDS_Face, B: Pin<&mut Bnd_Box2d>);
        #[doc = "Adds  to the box  <B>  the bounding  values of the wire in the parametric space of F."]
        #[cxx_name = "BRepTools_AddUVBounds_face_wire_box2d"]
        fn BRepTools_add_uv_bounds_face_wire_box2d(
            F: &TopoDS_Face,
            W: &TopoDS_Wire,
            B: Pin<&mut Bnd_Box2d>,
        );
        #[doc = "Adds to  the box <B>  the  bounding values  of the edge in the parametric space of F."]
        #[cxx_name = "BRepTools_AddUVBounds_face_edge_box2d"]
        fn BRepTools_add_uv_bounds_face_edge_box2d(
            F: &TopoDS_Face,
            E: &TopoDS_Edge,
            B: Pin<&mut Bnd_Box2d>,
        );
        #[doc = "Update a vertex (nothing is done)"]
        #[cxx_name = "BRepTools_Update_vertex"]
        fn BRepTools_update_vertex(V: &TopoDS_Vertex);
        #[doc = "Update an edge, compute 2d bounding boxes."]
        #[cxx_name = "BRepTools_Update_edge"]
        fn BRepTools_update_edge(E: &TopoDS_Edge);
        #[doc = "Update a wire (nothing is done)"]
        #[cxx_name = "BRepTools_Update_wire"]
        fn BRepTools_update_wire(W: &TopoDS_Wire);
        #[doc = "Update a Face, update UV points."]
        #[cxx_name = "BRepTools_Update_face"]
        fn BRepTools_update_face(F: &TopoDS_Face);
        #[doc = "Update a shell (nothing is done)"]
        #[cxx_name = "BRepTools_Update_shell"]
        fn BRepTools_update_shell(S: &TopoDS_Shell);
        #[doc = "Update a solid (nothing is done)"]
        #[cxx_name = "BRepTools_Update_solid"]
        fn BRepTools_update_solid(S: &TopoDS_Solid);
        #[doc = "Update a composite solid (nothing is done)"]
        #[cxx_name = "BRepTools_Update_compsolid"]
        fn BRepTools_update_compsolid(C: &TopoDS_CompSolid);
        #[doc = "Update a compound (nothing is done)"]
        #[cxx_name = "BRepTools_Update_compound"]
        fn BRepTools_update_compound(C: &TopoDS_Compound);
        #[doc = "Update a shape, call the correct update."]
        #[cxx_name = "BRepTools_Update_shape"]
        fn BRepTools_update_shape(S: &TopoDS_Shape);
        #[doc = "For each edge of the face <F> reset the UV points to the bounding points of the parametric curve of the edge on the face."]
        #[cxx_name = "BRepTools_UpdateFaceUVPoints"]
        fn BRepTools_update_face_uv_points(theF: &TopoDS_Face);
        #[doc = "Removes all cached polygonal representation of the shape, i.e. the triangulations of the faces of <S> and polygons on triangulations and polygons 3d of the edges. In case polygonal representation is the only available representation for the shape (shape does not have geometry) it is not removed. @param[in] theShape   the shape to clean @param[in] theForce   allows removing all polygonal representations from the shape, including polygons on triangulations irrelevant for the faces of the given shape."]
        #[cxx_name = "BRepTools_Clean"]
        fn BRepTools_clean(theShape: &TopoDS_Shape, theForce: bool);
        #[doc = "Removes geometry (curves and surfaces) from all edges and faces of the shape"]
        #[cxx_name = "BRepTools_CleanGeometry"]
        fn BRepTools_clean_geometry(theShape: &TopoDS_Shape);
        #[doc = "Removes all the pcurves of the edges of <S> that refer to surfaces not belonging to any face of <S>"]
        #[cxx_name = "BRepTools_RemoveUnusedPCurves"]
        fn BRepTools_remove_unused_p_curves(S: &TopoDS_Shape);
        #[doc = "Verifies that each Face from the shape has got a triangulation with a deflection smaller or equal to specified one and the Edges a discretization on this triangulation. @param[in] theShape    shape to verify @param[in] theLinDefl  maximum allowed linear deflection @param[in] theToCheckFreeEdges  if TRUE, then free Edges are required to have 3D polygon @return FALSE if input Shape contains Faces without triangulation, or that triangulation has worse (greater) deflection than specified one, or Edges in Shape lack polygons on triangulation or free Edges in Shape lack 3D polygons"]
        #[cxx_name = "BRepTools_Triangulation"]
        fn BRepTools_triangulation(
            theShape: &TopoDS_Shape,
            theLinDefl: f64,
            theToCheckFreeEdges: bool,
        ) -> bool;
        #[doc = "Loads triangulation data for each face of the shape from some deferred storage using specified shared input file system @param[in] theShape             shape to load triangulations @param[in] theTriangulationIdx  index defining what triangulation should be loaded. Starts from 0. -1 is used in specific case to load currently already active triangulation. If some face doesn't contain triangulation with this index, nothing will be loaded for it. Exception will be thrown in case of invalid negative index @param[in] theToSetAsActive     flag to activate triangulation after its loading @param[in] theFileSystem        shared file system @return TRUE if at least one triangulation is loaded."]
        #[cxx_name = "BRepTools_LoadTriangulation"]
        fn BRepTools_load_triangulation(
            theShape: &TopoDS_Shape,
            theTriangulationIdx: i32,
            theToSetAsActive: bool,
            theFileSystem: &HandleOSDFileSystem,
        ) -> bool;
        #[doc = "Releases triangulation data for each face of the shape if there is deferred storage to load it later @param[in] theShape             shape to unload triangulations @param[in] theTriangulationIdx  index defining what triangulation should be unloaded. Starts from 0. -1 is used in specific case to unload currently already active triangulation. If some face doesn't contain triangulation with this index, nothing will be unloaded for it. Exception will be thrown in case of invalid negative index @return TRUE if at least one triangulation is unloaded."]
        #[cxx_name = "BRepTools_UnloadTriangulation"]
        fn BRepTools_unload_triangulation(
            theShape: &TopoDS_Shape,
            theTriangulationIdx: i32,
        ) -> bool;
        #[doc = "Activates triangulation data for each face of the shape from some deferred storage using specified shared input file system @param[in] theShape               shape to activate triangulations @param[in] theTriangulationIdx    index defining what triangulation should be activated. Starts from 0. Exception will be thrown in case of invalid negative index @param[in] theToActivateStrictly  flag to activate exactly triangulation with defined theTriangulationIdx index. In TRUE case if some face doesn't contain triangulation with this index, active triangulation will not be changed for it. Else the last available triangulation will be activated. @return TRUE if at least one active triangulation was changed."]
        #[cxx_name = "BRepTools_ActivateTriangulation"]
        fn BRepTools_activate_triangulation(
            theShape: &TopoDS_Shape,
            theTriangulationIdx: i32,
            theToActivateStrictly: bool,
        ) -> bool;
        #[doc = "Loads all available triangulations for each face of the shape from some deferred storage using specified shared input file system @param[in] theShape       shape to load triangulations @param[in] theFileSystem  shared file system @return TRUE if at least one triangulation is loaded."]
        #[cxx_name = "BRepTools_LoadAllTriangulations"]
        fn BRepTools_load_all_triangulations(
            theShape: &TopoDS_Shape,
            theFileSystem: &HandleOSDFileSystem,
        ) -> bool;
        #[doc = "Releases all available triangulations for each face of the shape if there is deferred storage to load them later @param[in] theShape       shape to unload triangulations @return TRUE if at least one triangulation is unloaded."]
        #[cxx_name = "BRepTools_UnloadAllTriangulations"]
        fn BRepTools_unload_all_triangulations(theShape: &TopoDS_Shape) -> bool;
        #[doc = "Returns  True if  the    distance between the  two vertices is lower than their tolerance."]
        #[cxx_name = "BRepTools_Compare_vertex2"]
        fn BRepTools_compare_vertex2(V1: &TopoDS_Vertex, V2: &TopoDS_Vertex) -> bool;
        #[doc = "Returns  True if  the    distance between the  two edges is lower than their tolerance."]
        #[cxx_name = "BRepTools_Compare_edge2"]
        fn BRepTools_compare_edge2(E1: &TopoDS_Edge, E2: &TopoDS_Edge) -> bool;
        #[doc = "Returns the outer most wire of <F>. Returns a Null wire if <F> has no wires."]
        #[cxx_name = "BRepTools_OuterWire"]
        fn BRepTools_outer_wire(F: &TopoDS_Face) -> UniquePtr<TopoDS_Wire>;
        #[doc = "Stores in the map  <M> all the 3D topology edges of <S>."]
        #[cxx_name = "BRepTools_Map3DEdges"]
        fn BRepTools_map3_d_edges(S: &TopoDS_Shape, M: Pin<&mut TopTools_IndexedMapOfShape>);
        #[doc = "Verifies that the edge  <E> is found two  times on the face <F> before calling BRep_Tool::IsClosed."]
        #[cxx_name = "BRepTools_IsReallyClosed"]
        fn BRepTools_is_really_closed(E: &TopoDS_Edge, F: &TopoDS_Face) -> bool;
        #[doc = "Detect closedness of face in U and V directions"]
        #[cxx_name = "BRepTools_DetectClosedness"]
        fn BRepTools_detect_closedness(
            theFace: &TopoDS_Face,
            theUclosed: &mut bool,
            theVclosed: &mut bool,
        );
        #[doc = "Writes the shape to the file in an ASCII format TopTools_FormatVersion_VERSION_1. This alias writes shape with triangulation data. @param[in] theShape  the shape to write @param[in] theFile   the path to file to output shape into @param theProgress the range of progress indicator to fill in"]
        #[cxx_name = "BRepTools_Write_shape_charptr_progressrange"]
        fn BRepTools_write_shape_charptr_progressrange(
            theShape: &TopoDS_Shape,
            theFile: &str,
            theProgress: &Message_ProgressRange,
        ) -> bool;
        #[doc = "Writes the shape to the file in an ASCII format of specified version. @param[in] theShape          the shape to write @param[in] theFile           the path to file to output shape into @param[in] theWithTriangles  flag which specifies whether to save shape with (TRUE) or without (FALSE) triangles; has no effect on triangulation-only geometry @param[in] theWithNormals    flag which specifies whether to save triangulation with (TRUE) or without (FALSE) normals; has no effect on triangulation-only geometry @param[in] theVersion        the TopTools format version @param theProgress the range of progress indicator to fill in"]
        #[cxx_name = "BRepTools_Write_shape_charptr_bool2_formatversion_progressrange"]
        fn BRepTools_write_shape_charptr_bool2_formatversion_progressrange(
            theShape: &TopoDS_Shape,
            theFile: &str,
            theWithTriangles: bool,
            theWithNormals: bool,
            theVersion: TopTools_FormatVersion,
            theProgress: &Message_ProgressRange,
        ) -> bool;
        #[doc = "Reads a Shape  from <File>,  returns it in  <Sh>. <B> is used to build the shape."]
        #[cxx_name = "BRepTools_Read_shape_charptr_builder_progressrange"]
        fn BRepTools_read_shape_charptr_builder_progressrange(
            Sh: Pin<&mut TopoDS_Shape>,
            File: &str,
            B: &BRep_Builder,
            theProgress: &Message_ProgressRange,
        ) -> bool;
        #[doc = "Evals real tolerance of edge  <theE>. <theC3d>, <theC2d>, <theS>, <theF>, <theL> are correspondently 3d curve of edge, 2d curve on surface <theS> and rang of edge If calculated tolerance is more then current edge tolerance, edge is updated. Method returns actual tolerance of edge"]
        #[cxx_name = "BRepTools_EvalAndUpdateTol"]
        fn BRepTools_eval_and_update_tol(
            theE: &TopoDS_Edge,
            theC3d: &HandleGeomCurve,
            theC2d: &HandleGeom2dCurve,
            theS: &HandleGeomSurface,
            theF: f64,
            theL: f64,
        ) -> f64;
        #[doc = "returns the cumul  of the orientation  of <Edge> and thc containing wire in <Face>"]
        #[cxx_name = "BRepTools_OriEdgeInFace"]
        fn BRepTools_ori_edge_in_face(
            theEdge: &TopoDS_Edge,
            theFace: &TopoDS_Face,
        ) -> TopAbs_Orientation;
        #[doc = "Removes internal sub-shapes from the shape. The check on internal status is based on orientation of sub-shapes, classification is not performed. Before removal of internal sub-shapes the algorithm checks if such removal is not going to break topological connectivity between sub-shapes. The flag <theForce> if set to true disables the connectivity check and clears the given shape from all sub-shapes with internal orientation."]
        #[cxx_name = "BRepTools_RemoveInternals"]
        fn BRepTools_remove_internals(theS: Pin<&mut TopoDS_Shape>, theForce: bool);
        #[doc = "Check all locations of shape according criterium: aTrsf.IsNegative() || (Abs(Abs(aTrsf.ScaleFactor()) - 1.) > TopLoc_Location::ScalePrec()) All sub-shapes having such locations are put in list theProblemShapes"]
        #[cxx_name = "BRepTools_CheckLocations"]
        fn BRepTools_check_locations(
            theS: &TopoDS_Shape,
            theProblemShapes: Pin<&mut TopTools_ListOfShape>,
        );
    }
    impl UniquePtr<BRepTools> {}
}
pub use ffi::BRepTools;
