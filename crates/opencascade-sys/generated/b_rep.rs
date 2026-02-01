#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_b_rep.hxx");
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
        #[doc = "Curve from geom2d module"]
        type Geom2d_Curve = crate::geom2d::ffi::Curve;
        #[doc = "Ellipse from geom2d module"]
        type Geom2d_Ellipse = crate::geom2d::ffi::Ellipse;
        #[doc = "TrimmedCurve from geom2d module"]
        type Geom2d_TrimmedCurve = crate::geom2d::ffi::TrimmedCurve;
        #[doc = "Pnt from gp module"]
        type gp_Pnt = crate::gp::ffi::Pnt;
        #[doc = "Pnt2d from gp module"]
        type gp_Pnt2d = crate::gp::ffi::Pnt2d;
        #[doc = "Vec from gp module"]
        type gp_Vec = crate::gp::ffi::Vec_;
        #[doc = "Vec2d from gp module"]
        type gp_Vec2d = crate::gp::ffi::Vec2d;
        #[doc = "Dir from gp module"]
        type gp_Dir = crate::gp::ffi::Dir;
        #[doc = "Dir2d from gp module"]
        type gp_Dir2d = crate::gp::ffi::Dir2d;
        #[doc = "XYZ from gp module"]
        type gp_XYZ = crate::gp::ffi::XYZ;
        #[doc = "Ax1 from gp module"]
        type gp_Ax1 = crate::gp::ffi::Ax1;
        #[doc = "Ax2 from gp module"]
        type gp_Ax2 = crate::gp::ffi::Ax2;
        #[doc = "Ax2d from gp module"]
        type gp_Ax2d = crate::gp::ffi::Ax2d;
        #[doc = "Ax3 from gp module"]
        type gp_Ax3 = crate::gp::ffi::Ax3;
        #[doc = "Trsf from gp module"]
        type gp_Trsf = crate::gp::ffi::Trsf;
        #[doc = "Trsf2d from gp module"]
        type gp_Trsf2d = crate::gp::ffi::Trsf2d;
        #[doc = "GTrsf from gp module"]
        type gp_GTrsf = crate::gp::ffi::GTrsf;
        #[doc = "GTrsf2d from gp module"]
        type gp_GTrsf2d = crate::gp::ffi::GTrsf2d;
        #[doc = "Lin from gp module"]
        type gp_Lin = crate::gp::ffi::Lin;
        #[doc = "Circ from gp module"]
        type gp_Circ = crate::gp::ffi::Circ;
        #[doc = "Pln from gp module"]
        type gp_Pln = crate::gp::ffi::Pln;
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
        #[doc = "Triangulation from poly module"]
        type Poly_Triangulation = crate::poly::ffi::Triangulation;
        #[doc = "ShapeEnum from top_abs module"]
        type TopAbs_ShapeEnum = crate::top_abs::ffi::TopAbs_ShapeEnum;
        #[doc = "Orientation from top_abs module"]
        type TopAbs_Orientation = crate::top_abs::ffi::TopAbs_Orientation;
        #[doc = "Location from top_loc module"]
        type TopLoc_Location = crate::top_loc::ffi::Location;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Poly_PolygonOnTriangulation"]
        type Poly_PolygonOnTriangulation;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Poly_Polygon2D"]
        type Poly_Polygon2D;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "GeomAbs_Shape"]
        type GeomAbs_Shape;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Poly_ListOfTriangulation"]
        type Poly_ListOfTriangulation;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Poly_Polygon3D"]
        type Poly_Polygon3D;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Poly_MeshPurpose"]
        type Poly_MeshPurpose;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleGeomSurface"]
        type HandleGeomSurface;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandlePolyPolygon2D"]
        type HandlePolyPolygon2D;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleGeom2dCurve"]
        type HandleGeom2dCurve;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandlePolyPolygon3D"]
        type HandlePolyPolygon3D;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandlePolyTriangulation"]
        type HandlePolyTriangulation;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandlePolyPolygonOnTriangulation"]
        type HandlePolyPolygonOnTriangulation;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleGeomCurve"]
        type HandleGeomCurve;
        #[doc = " ======================== BRep_Tool ========================"]
        #[doc = "Provides class methods  to  access to the geometry of BRep shapes."]
        #[cxx_name = "BRep_Tool"]
        type Tool;
        #[doc = "If S is Shell, returns True if it has no free boundaries (edges). If S is Wire, returns True if it has no free ends (vertices). (Internal and External sub-shepes are ignored in these checks) If S is Edge, returns True if its vertices are the same. For other shape types returns S.Closed()."]
        #[cxx_name = "BRep_Tool_IsClosed_shape"]
        fn Tool_is_closed_shape(S: &TopoDS_Shape) -> bool;
        #[doc = "Returns the geometric  surface of the face. It can be a copy if there is a Location."]
        #[cxx_name = "BRep_Tool_Surface_face"]
        fn Tool_surface_face(F: &TopoDS_Face) -> UniquePtr<HandleGeomSurface>;
        #[doc = "Returns the tolerance of the face."]
        #[cxx_name = "BRep_Tool_Tolerance_face"]
        fn Tool_tolerance_face(F: &TopoDS_Face) -> f64;
        #[doc = "Returns the  NaturalRestriction  flag of the  face."]
        #[cxx_name = "BRep_Tool_NaturalRestriction"]
        fn Tool_natural_restriction(F: &TopoDS_Face) -> bool;
        #[doc = "Returns True if <F> has a surface, false otherwise."]
        #[cxx_name = "BRep_Tool_IsGeometric_face"]
        fn Tool_is_geometric_face(F: &TopoDS_Face) -> bool;
        #[doc = "Returns True if <E> is a 3d curve or a curve on surface."]
        #[cxx_name = "BRep_Tool_IsGeometric_edge"]
        fn Tool_is_geometric_edge(E: &TopoDS_Edge) -> bool;
        #[doc = "Returns the 3D curve  of the edge. May be a Null handle. In <First> and <Last> the parameter range. It can be a copy if there is a Location."]
        #[cxx_name = "BRep_Tool_Curve_edge_real2"]
        fn Tool_curve_edge_real2(
            E: &TopoDS_Edge,
            First: &mut f64,
            Last: &mut f64,
        ) -> UniquePtr<HandleGeomCurve>;
        #[doc = "For the planar surface builds the 2d curve for the edge by projection of the edge on plane. Returns a NULL handle if the surface is not planar or the projection failed."]
        #[cxx_name = "BRep_Tool_CurveOnPlane"]
        fn Tool_curve_on_plane(
            E: &TopoDS_Edge,
            S: &HandleGeomSurface,
            L: &TopLoc_Location,
            First: &mut f64,
            Last: &mut f64,
        ) -> UniquePtr<HandleGeom2dCurve>;
        #[doc = "Returns in <C>, <S>, <L> a 2d curve, a surface and a location for the edge <E>. <C> and <S>  are null if the  edge has no curve on  surface.  Returns in <First> and <Last> the parameter range."]
        #[cxx_name = "BRep_Tool_CurveOnSurface_edge_handlecurve_handlesurface_location_real2"]
        fn Tool_curve_on_surface_edge_handlecurve_handlesurface_location_real2(
            E: &TopoDS_Edge,
            C: Pin<&mut HandleGeom2dCurve>,
            S: Pin<&mut HandleGeomSurface>,
            L: Pin<&mut TopLoc_Location>,
            First: &mut f64,
            Last: &mut f64,
        );
        #[doc = "Returns in <C>, <S>, <L> the 2d curve, the surface and the location for the edge <E> of rank <Index>. <C> and <S> are null if the index is out of range. Returns in <First> and <Last> the parameter range."]
        #[cxx_name = "BRep_Tool_CurveOnSurface_edge_handlecurve_handlesurface_location_real2_int"]
        fn Tool_curve_on_surface_edge_handlecurve_handlesurface_location_real2_int(
            E: &TopoDS_Edge,
            C: Pin<&mut HandleGeom2dCurve>,
            S: Pin<&mut HandleGeomSurface>,
            L: Pin<&mut TopLoc_Location>,
            First: &mut f64,
            Last: &mut f64,
            Index: i32,
        );
        #[doc = "Returns the polygon associated to the  edge in  the parametric  space of  the  face.  Returns   a NULL handle  if this polygon  does not exist."]
        #[cxx_name = "BRep_Tool_PolygonOnSurface_edge_face"]
        fn Tool_polygon_on_surface_edge_face(
            E: &TopoDS_Edge,
            F: &TopoDS_Face,
        ) -> UniquePtr<HandlePolyPolygon2D>;
        #[doc = "Returns the polygon associated to the  edge in  the parametric  space of  the surface. Returns   a NULL handle  if this polygon  does not exist."]
        #[cxx_name = "BRep_Tool_PolygonOnSurface_edge_handlesurface_location"]
        fn Tool_polygon_on_surface_edge_handlesurface_location(
            E: &TopoDS_Edge,
            S: &HandleGeomSurface,
            L: &TopLoc_Location,
        ) -> UniquePtr<HandlePolyPolygon2D>;
        #[doc = "Returns in <C>, <S>, <L> a 2d curve, a surface and a location for the edge <E>. <C> and <S>  are null if the  edge has no polygon on  surface."]
        #[cxx_name = "BRep_Tool_PolygonOnSurface_edge_handlepolygon2d_handlesurface_location"]
        fn Tool_polygon_on_surface_edge_handlepolygon2d_handlesurface_location(
            E: &TopoDS_Edge,
            C: Pin<&mut HandlePolyPolygon2D>,
            S: Pin<&mut HandleGeomSurface>,
            L: Pin<&mut TopLoc_Location>,
        );
        #[doc = "Returns in <C>, <S>, <L> the 2d curve, the surface and the location for the edge <E> of rank <Index>. <C> and <S> are null if the index is out of range."]
        #[cxx_name = "BRep_Tool_PolygonOnSurface_edge_handlepolygon2d_handlesurface_location_int"]
        fn Tool_polygon_on_surface_edge_handlepolygon2d_handlesurface_location_int(
            E: &TopoDS_Edge,
            C: Pin<&mut HandlePolyPolygon2D>,
            S: Pin<&mut HandleGeomSurface>,
            L: Pin<&mut TopLoc_Location>,
            Index: i32,
        );
        #[doc = "Returns in <P>, <T>, <L> a polygon on triangulation, a triangulation and a location for the edge <E>. <P>  and  <T>  are null  if  the  edge has no polygon on  triangulation."]
        #[cxx_name = "BRep_Tool_PolygonOnTriangulation_edge_handlepolygonontriangulation_handletriangulation_location"]
        fn Tool_polygon_on_triangulation_edge_handlepolygonontriangulation_handletriangulation_location(
            E: &TopoDS_Edge,
            P: Pin<&mut HandlePolyPolygonOnTriangulation>,
            T: Pin<&mut HandlePolyTriangulation>,
            L: Pin<&mut TopLoc_Location>,
        );
        #[doc = "Returns   in   <P>,  <T>,    <L> a     polygon  on triangulation,   a triangulation  and a  location for the edge <E> for the range index.  <C> and <S> are null if the edge has no polygon on triangulation."]
        #[cxx_name = "BRep_Tool_PolygonOnTriangulation_edge_handlepolygonontriangulation_handletriangulation_location_int"]
        fn Tool_polygon_on_triangulation_edge_handlepolygonontriangulation_handletriangulation_location_int(
            E: &TopoDS_Edge,
            P: Pin<&mut HandlePolyPolygonOnTriangulation>,
            T: Pin<&mut HandlePolyTriangulation>,
            L: Pin<&mut TopLoc_Location>,
            Index: i32,
        );
        #[doc = "Returns  True  if  <E>  has  two  PCurves  in  the parametric space of <F>. i.e.  <F>  is on a closed surface and <E> is on the closing curve."]
        #[cxx_name = "BRep_Tool_IsClosed_edge_face"]
        fn Tool_is_closed_edge_face(E: &TopoDS_Edge, F: &TopoDS_Face) -> bool;
        #[doc = "Returns  True  if  <E>  has  two  PCurves  in  the parametric space  of <S>.  i.e.   <S>  is a closed surface and <E> is on the closing curve."]
        #[cxx_name = "BRep_Tool_IsClosed_edge_handlesurface_location"]
        fn Tool_is_closed_edge_handlesurface_location(
            E: &TopoDS_Edge,
            S: &HandleGeomSurface,
            L: &TopLoc_Location,
        ) -> bool;
        #[doc = "Returns  True  if <E> has two arrays of indices in the triangulation <T>."]
        #[cxx_name = "BRep_Tool_IsClosed_edge_handletriangulation_location"]
        fn Tool_is_closed_edge_handletriangulation_location(
            E: &TopoDS_Edge,
            T: &HandlePolyTriangulation,
            L: &TopLoc_Location,
        ) -> bool;
        #[doc = "Returns the tolerance for <E>."]
        #[cxx_name = "BRep_Tool_Tolerance_edge"]
        fn Tool_tolerance_edge(E: &TopoDS_Edge) -> f64;
        #[doc = "Returns the SameParameter flag for the edge."]
        #[cxx_name = "BRep_Tool_SameParameter"]
        fn Tool_same_parameter(E: &TopoDS_Edge) -> bool;
        #[doc = "Returns the SameRange flag for the edge."]
        #[cxx_name = "BRep_Tool_SameRange"]
        fn Tool_same_range(E: &TopoDS_Edge) -> bool;
        #[doc = "Returns True  if the edge is degenerated."]
        #[cxx_name = "BRep_Tool_Degenerated"]
        fn Tool_degenerated(E: &TopoDS_Edge) -> bool;
        #[doc = "Gets the range of the 3d curve."]
        #[cxx_name = "BRep_Tool_Range_edge_real2"]
        fn Tool_range_edge_real2(E: &TopoDS_Edge, First: &mut f64, Last: &mut f64);
        #[doc = "Gets the range  of the edge  on the pcurve on  the surface."]
        #[cxx_name = "BRep_Tool_Range_edge_handlesurface_location_real2"]
        fn Tool_range_edge_handlesurface_location_real2(
            E: &TopoDS_Edge,
            S: &HandleGeomSurface,
            L: &TopLoc_Location,
            First: &mut f64,
            Last: &mut f64,
        );
        #[doc = "Gets the range of the edge on the pcurve on the face."]
        #[cxx_name = "BRep_Tool_Range_edge_face_real2"]
        fn Tool_range_edge_face_real2(
            E: &TopoDS_Edge,
            F: &TopoDS_Face,
            First: &mut f64,
            Last: &mut f64,
        );
        #[doc = "Gets the UV locations of the extremities of the edge."]
        #[cxx_name = "BRep_Tool_UVPoints_edge_handlesurface_location_pnt2d2"]
        fn Tool_uv_points_edge_handlesurface_location_pnt2d2(
            E: &TopoDS_Edge,
            S: &HandleGeomSurface,
            L: &TopLoc_Location,
            PFirst: Pin<&mut gp_Pnt2d>,
            PLast: Pin<&mut gp_Pnt2d>,
        );
        #[doc = "Gets the UV locations of the extremities of the edge."]
        #[cxx_name = "BRep_Tool_UVPoints_edge_face_pnt2d2"]
        fn Tool_uv_points_edge_face_pnt2d2(
            E: &TopoDS_Edge,
            F: &TopoDS_Face,
            PFirst: Pin<&mut gp_Pnt2d>,
            PLast: Pin<&mut gp_Pnt2d>,
        );
        #[doc = "Sets the UV locations of the extremities of the edge."]
        #[cxx_name = "BRep_Tool_SetUVPoints_edge_handlesurface_location_pnt2d2"]
        fn Tool_set_uv_points_edge_handlesurface_location_pnt2d2(
            E: &TopoDS_Edge,
            S: &HandleGeomSurface,
            L: &TopLoc_Location,
            PFirst: &gp_Pnt2d,
            PLast: &gp_Pnt2d,
        );
        #[doc = "Sets the UV locations of the extremities of the edge."]
        #[cxx_name = "BRep_Tool_SetUVPoints_edge_face_pnt2d2"]
        fn Tool_set_uv_points_edge_face_pnt2d2(
            E: &TopoDS_Edge,
            F: &TopoDS_Face,
            PFirst: &gp_Pnt2d,
            PLast: &gp_Pnt2d,
        );
        #[doc = "Returns True if the edge is on the surfaces of the two faces."]
        #[cxx_name = "BRep_Tool_HasContinuity_edge_face2"]
        fn Tool_has_continuity_edge_face2(
            E: &TopoDS_Edge,
            F1: &TopoDS_Face,
            F2: &TopoDS_Face,
        ) -> bool;
        #[doc = "Returns the continuity."]
        #[cxx_name = "BRep_Tool_Continuity_edge_face2"]
        fn Tool_continuity_edge_face2(
            E: &TopoDS_Edge,
            F1: &TopoDS_Face,
            F2: &TopoDS_Face,
        ) -> UniquePtr<GeomAbs_Shape>;
        #[doc = "Returns True if the edge is on the surfaces."]
        #[cxx_name = "BRep_Tool_HasContinuity_edge_handlesurface2_location2"]
        fn Tool_has_continuity_edge_handlesurface2_location2(
            E: &TopoDS_Edge,
            S1: &HandleGeomSurface,
            S2: &HandleGeomSurface,
            L1: &TopLoc_Location,
            L2: &TopLoc_Location,
        ) -> bool;
        #[doc = "Returns the continuity."]
        #[cxx_name = "BRep_Tool_Continuity_edge_handlesurface2_location2"]
        fn Tool_continuity_edge_handlesurface2_location2(
            E: &TopoDS_Edge,
            S1: &HandleGeomSurface,
            S2: &HandleGeomSurface,
            L1: &TopLoc_Location,
            L2: &TopLoc_Location,
        ) -> UniquePtr<GeomAbs_Shape>;
        #[doc = "Returns True if the edge has regularity on some two surfaces"]
        #[cxx_name = "BRep_Tool_HasContinuity_edge"]
        fn Tool_has_continuity_edge(E: &TopoDS_Edge) -> bool;
        #[doc = "Returns the max continuity of edge between some surfaces or GeomAbs_C0 if there no such surfaces."]
        #[cxx_name = "BRep_Tool_MaxContinuity"]
        fn Tool_max_continuity(theEdge: &TopoDS_Edge) -> UniquePtr<GeomAbs_Shape>;
        #[doc = "Returns the 3d point."]
        #[cxx_name = "BRep_Tool_Pnt"]
        fn Tool_pnt(V: &TopoDS_Vertex) -> UniquePtr<gp_Pnt>;
        #[doc = "Returns the tolerance."]
        #[cxx_name = "BRep_Tool_Tolerance_vertex"]
        fn Tool_tolerance_vertex(V: &TopoDS_Vertex) -> f64;
        #[doc = "Finds the parameter of <theV> on <theE>. @param[in] theV  input vertex @param[in] theE  input edge @param[out] theParam   calculated parameter on the curve @return TRUE if done"]
        #[cxx_name = "BRep_Tool_Parameter_vertex_edge_real"]
        fn Tool_parameter_vertex_edge_real(
            theV: &TopoDS_Vertex,
            theE: &TopoDS_Edge,
            theParam: &mut f64,
        ) -> bool;
        #[doc = "Returns the parameter of <V> on <E>. Throws Standard_NoSuchObject if no parameter on edge"]
        #[cxx_name = "BRep_Tool_Parameter_vertex_edge"]
        fn Tool_parameter_vertex_edge(V: &TopoDS_Vertex, E: &TopoDS_Edge) -> f64;
        #[doc = "Returns the  parameters  of   the  vertex   on the pcurve of the edge on the face."]
        #[cxx_name = "BRep_Tool_Parameter_vertex_edge_face"]
        fn Tool_parameter_vertex_edge_face(
            V: &TopoDS_Vertex,
            E: &TopoDS_Edge,
            F: &TopoDS_Face,
        ) -> f64;
        #[doc = "Returns the  parameters  of   the  vertex   on the pcurve of the edge on the surface."]
        #[cxx_name = "BRep_Tool_Parameter_vertex_edge_handlesurface_location"]
        fn Tool_parameter_vertex_edge_handlesurface_location(
            V: &TopoDS_Vertex,
            E: &TopoDS_Edge,
            S: &HandleGeomSurface,
            L: &TopLoc_Location,
        ) -> f64;
        #[doc = "Returns the parameters of the vertex on the face."]
        #[cxx_name = "BRep_Tool_Parameters"]
        fn Tool_parameters(V: &TopoDS_Vertex, F: &TopoDS_Face) -> UniquePtr<gp_Pnt2d>;
        #[cxx_name = "BRep_Tool_MaxTolerance"]
        fn Tool_max_tolerance(theShape: &TopoDS_Shape, theSubShape: TopAbs_ShapeEnum) -> f64;
    }
    impl UniquePtr<Tool> {}
}
pub use ffi::Tool;
