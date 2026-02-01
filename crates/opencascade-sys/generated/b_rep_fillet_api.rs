#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_b_rep_fillet_api.hxx");
        #[doc = "ProgressRange from message module"]
        type Message_ProgressRange = crate::message::ffi::ProgressRange;
        #[doc = "FormatVersion from top_tools module"]
        type TopTools_FormatVersion = crate::top_tools::ffi::TopTools_FormatVersion;
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
        #[doc = "Function from law module"]
        type Law_Function = crate::law::ffi::Function;
        #[doc = "Interpol from law module"]
        type Law_Interpol = crate::law::ffi::Interpol;
        #[doc = "HArray1OfPnt from t_colgp module"]
        type TColgp_HArray1OfPnt = crate::t_colgp::ffi::HArray1OfPnt;
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
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "ChFiDS_ErrorStatus"]
        type ChFiDS_ErrorStatus;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TopOpeBRepBuild_HBuilder"]
        type TopOpeBRepBuild_HBuilder;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TColgp_Array1OfPnt2d"]
        type TColgp_Array1OfPnt2d;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TopTools_ListOfShape"]
        type TopTools_ListOfShape;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "ChFiDS_SecHArray1"]
        type ChFiDS_SecHArray1;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "ChFi3d_FilletShape"]
        type ChFi3d_FilletShape;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "ChFiDS_ChamfMode"]
        type ChFiDS_ChamfMode;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "GeomAbs_Shape"]
        type GeomAbs_Shape;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "ChFi2d_ConstructionError"]
        type ChFi2d_ConstructionError;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TopTools_SequenceOfShape"]
        type TopTools_SequenceOfShape;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleTopOpeBRepBuildHBuilder"]
        type HandleTopOpeBRepBuildHBuilder;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleGeomSurface"]
        type HandleGeomSurface;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleChFiDSSecHArray1"]
        type HandleChFiDSSecHArray1;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleLawFunction"]
        type HandleLawFunction;
        #[doc = " ======================== BRepFilletAPI_MakeChamfer ========================"]
        #[doc = "Describes functions to build chamfers on edges of a shell or solid. Chamfered Edge of a Shell or Solid A MakeChamfer object provides a framework for: -   initializing the construction algorithm with a given shape, -   acquiring the data characterizing the chamfers, -   building the chamfers and constructing the resulting shape, and -   consulting the result."]
        #[cxx_name = "BRepFilletAPI_MakeChamfer"]
        type MakeChamfer;
        #[doc = "Initializes an algorithm for computing chamfers on the shape S. The edges on which chamfers are built are defined using the Add function."]
        #[cxx_name = "BRepFilletAPI_MakeChamfer_ctor_shape"]
        fn MakeChamfer_ctor_shape(S: &TopoDS_Shape) -> UniquePtr<MakeChamfer>;
        #[doc = "Adds edge E to the table of edges used by this algorithm to build chamfers, where the parameters of the chamfer must be set after the"]
        #[cxx_name = "Add"]
        fn addedge(self: Pin<&mut MakeChamfer>, E: &TopoDS_Edge);
        #[doc = "Adds edge E to the table of edges used by this algorithm to build chamfers, where the parameters of the chamfer are given by the distance Dis (symmetric chamfer). The Add function results in a contour being built by propagation from the edge E (i.e. the contour contains at least this edge). This contour is composed of edges of the shape which are tangential to one another and which delimit two series of tangential faces, with one series of faces being located on either side of the contour. Warning Nothing is done if edge E or the face F does not belong to the initial shape."]
        #[cxx_name = "Add"]
        fn addreal_2(self: Pin<&mut MakeChamfer>, Dis: f64, E: &TopoDS_Edge);
        #[doc = "Sets the distances Dis1 and Dis2 which give the parameters of the chamfer along the contour of index IC generated using the Add function in the internal data structure of this algorithm. The face F identifies the side where Dis1 is measured. Warning Nothing is done if either the edge E or the face F does not belong to the initial shape."]
        #[cxx_name = "SetDist"]
        fn set_dist(self: Pin<&mut MakeChamfer>, Dis: f64, IC: i32, F: &TopoDS_Face);
        #[cxx_name = "GetDist"]
        fn get_dist(self: &MakeChamfer, IC: i32, Dis: &mut f64);
        #[doc = "Adds edge E to the table of edges used by this algorithm to build chamfers, where the parameters of the chamfer are given by the two distances Dis1 and Dis2; the face F identifies the side where Dis1 is measured. The Add function results in a contour being built by propagation from the edge E (i.e. the contour contains at least this edge). This contour is composed of edges of the shape which are tangential to one another and which delimit two series of tangential faces, with one series of faces being located on either side of the contour. Warning Nothing is done if edge E or the face F does not belong to the initial shape."]
        #[cxx_name = "Add"]
        fn addreal_3(
            self: Pin<&mut MakeChamfer>,
            Dis1: f64,
            Dis2: f64,
            E: &TopoDS_Edge,
            F: &TopoDS_Face,
        );
        #[doc = "Sets the distances Dis1 and Dis2 which give the parameters of the chamfer along the contour of index IC generated using the Add function in the internal data structure of this algorithm. The face F identifies the side where Dis1 is measured. Warning Nothing is done if either the edge E or the face F does not belong to the initial shape."]
        #[cxx_name = "SetDists"]
        fn set_dists(self: Pin<&mut MakeChamfer>, Dis1: f64, Dis2: f64, IC: i32, F: &TopoDS_Face);
        #[doc = "Returns the distances Dis1 and Dis2 which give the parameters of the chamfer along the contour of index IC in the internal data structure of this algorithm. Warning -1. is returned if IC is outside the bounds of the table of contours."]
        #[cxx_name = "Dists"]
        fn dists(self: &MakeChamfer, IC: i32, Dis1: &mut f64, Dis2: &mut f64);
        #[doc = "Adds a  fillet contour in  the  builder  (builds a contour  of tangent edges to <E> and sets the distance <Dis1> and angle <Angle> ( parameters of the chamfer ) )."]
        #[cxx_name = "AddDA"]
        fn add_da(
            self: Pin<&mut MakeChamfer>,
            Dis: f64,
            Angle: f64,
            E: &TopoDS_Edge,
            F: &TopoDS_Face,
        );
        #[doc = "set the distance <Dis> and <Angle> of the fillet contour of index <IC> in the DS with <Dis> on <F>. if the face <F> is not one of common faces of an edge of the contour <IC>"]
        #[cxx_name = "SetDistAngle"]
        fn set_dist_angle(
            self: Pin<&mut MakeChamfer>,
            Dis: f64,
            Angle: f64,
            IC: i32,
            F: &TopoDS_Face,
        );
        #[doc = "gives the distances <Dis> and <Angle> of the fillet contour of index <IC> in the DS"]
        #[cxx_name = "GetDistAngle"]
        fn get_dist_angle(self: &MakeChamfer, IC: i32, Dis: &mut f64, Angle: &mut f64);
        #[doc = "return True if chamfer symmetric false else."]
        #[cxx_name = "IsSymetric"]
        fn is_symetric(self: &MakeChamfer, IC: i32) -> bool;
        #[doc = "return True if chamfer is made with two distances false else."]
        #[cxx_name = "IsTwoDistances"]
        fn is_two_distances(self: &MakeChamfer, IC: i32) -> bool;
        #[doc = "return True if chamfer is made with distance and angle false else."]
        #[cxx_name = "IsDistanceAngle"]
        fn is_distance_angle(self: &MakeChamfer, IC: i32) -> bool;
        #[doc = "Erases the chamfer parameters on the contour of index IC in the internal data structure of this algorithm. Use the SetDists function to reset this data. Warning Nothing is done if IC is outside the bounds of the table of contours."]
        #[cxx_name = "ResetContour"]
        fn reset_contour(self: Pin<&mut MakeChamfer>, IC: i32);
        #[doc = "Returns the number of contours generated using the Add function in the internal data structure of this algorithm."]
        #[cxx_name = "NbContours"]
        fn nb_contours(self: &MakeChamfer) -> i32;
        #[doc = "Returns the index of the contour in the internal data structure of this algorithm, which contains the edge E of the shape. This function returns 0 if the edge E does not belong to any contour. Warning This index can change if a contour is removed from the internal data structure of this algorithm using the function Remove."]
        #[cxx_name = "Contour"]
        fn contour(self: &MakeChamfer, E: &TopoDS_Edge) -> i32;
        #[doc = "Returns the number of edges in the contour of index I in the internal data structure of this algorithm. Warning Returns 0 if I is outside the bounds of the table of contours."]
        #[cxx_name = "NbEdges"]
        fn nb_edges(self: &MakeChamfer, I: i32) -> i32;
        #[doc = "Returns the edge of index J in the contour of index I in the internal data structure of this algorithm. Warning Returns a null shape if: -   I is outside the bounds of the table of contours, or -   J is outside the bounds of the table of edges of the contour of index I."]
        #[cxx_name = "Edge"]
        fn edge(self: &MakeChamfer, I: i32, J: i32) -> &TopoDS_Edge;
        #[doc = "Removes the contour in the internal data structure of this algorithm which contains the edge E of the shape. Warning Nothing is done if the edge E does not belong to the contour in the internal data structure of this algorithm."]
        #[cxx_name = "Remove"]
        fn remove(self: Pin<&mut MakeChamfer>, E: &TopoDS_Edge);
        #[doc = "Returns the length of the contour of index IC in the internal data structure of this algorithm. Warning Returns -1. if IC is outside the bounds of the table of contours."]
        #[cxx_name = "Length"]
        fn length(self: &MakeChamfer, IC: i32) -> f64;
        #[doc = "Returns the curvilinear abscissa of the vertex V on the contour of index IC in the internal data structure of this algorithm. Warning Returns -1. if: -   IC is outside the bounds of the table of contours, or -   V is not on the contour of index IC."]
        #[cxx_name = "Abscissa"]
        fn abscissa(self: &MakeChamfer, IC: i32, V: &TopoDS_Vertex) -> f64;
        #[doc = "Returns the relative curvilinear abscissa (i.e. between 0 and 1) of the vertex V on the contour of index IC in the internal data structure of this algorithm. Warning Returns -1. if: -   IC is outside the bounds of the table of contours, or -   V is not on the contour of index IC."]
        #[cxx_name = "RelativeAbscissa"]
        fn relative_abscissa(self: &MakeChamfer, IC: i32, V: &TopoDS_Vertex) -> f64;
        #[doc = "eturns true if the contour of index IC in the internal data structure of this algorithm is closed and tangential at the point of closure. Warning Returns false if IC is outside the bounds of the table of contours."]
        #[cxx_name = "ClosedAndTangent"]
        fn closed_and_tangent(self: &MakeChamfer, IC: i32) -> bool;
        #[doc = "Returns true if the contour of index IC in the internal data structure of this algorithm is closed. Warning Returns false if IC is outside the bounds of the table of contours."]
        #[cxx_name = "Closed"]
        fn closed(self: &MakeChamfer, IC: i32) -> bool;
        #[doc = "Builds the chamfers on all the contours in the internal data structure of this algorithm and constructs the resulting shape. Use the function IsDone to verify that the chamfered shape is built. Use the function Shape to retrieve the chamfered shape. Warning The construction of chamfers implements highly complex construction algorithms. Consequently, there may be instances where the algorithm fails, for example if the data defining the parameters of the chamfer is not compatible with the geometry of the initial shape. There is no initial analysis of errors and these only become evident at the construction stage. Additionally, in the current software release, the following cases are not handled: -   the end point of the contour is the point of intersection of 4 or more edges of the shape, or -   the intersection of the chamfer with a face which limits the contour is not fully contained in this face."]
        #[cxx_name = "Build"]
        fn build(self: Pin<&mut MakeChamfer>, theRange: &Message_ProgressRange);
        #[doc = "Reinitializes this algorithm, thus canceling the effects of the Build function. This function allows modifications to be made to the contours and chamfer parameters in order to rebuild the shape."]
        #[cxx_name = "Reset"]
        fn reset(self: Pin<&mut MakeChamfer>);
        #[doc = "Returns the  list   of shapes generated   from the shape <EorV>."]
        #[cxx_name = "Generated"]
        fn generated(self: Pin<&mut MakeChamfer>, EorV: &TopoDS_Shape) -> &TopTools_ListOfShape;
        #[doc = "Returns the list  of shapes modified from the shape <F>."]
        #[cxx_name = "Modified"]
        fn modified(self: Pin<&mut MakeChamfer>, F: &TopoDS_Shape) -> &TopTools_ListOfShape;
        #[cxx_name = "IsDeleted"]
        fn is_deleted(self: Pin<&mut MakeChamfer>, F: &TopoDS_Shape) -> bool;
        #[cxx_name = "Simulate"]
        fn simulate(self: Pin<&mut MakeChamfer>, IC: i32);
        #[cxx_name = "NbSurf"]
        fn nb_surf(self: &MakeChamfer, IC: i32) -> i32;
        #[doc = "Returns the first vertex of the contour of index IC in the internal data structure of this algorithm. Warning Returns a null shape if IC is outside the bounds of the table of contours."]
        #[cxx_name = "BRepFilletAPI_MakeChamfer_FirstVertex"]
        fn MakeChamfer_first_vertex(self_: &MakeChamfer, IC: i32) -> UniquePtr<TopoDS_Vertex>;
        #[doc = "Returns the last vertex of the contour of index IC in the internal data structure of this algorithm. Warning Returns a null shape if IC is outside the bounds of the table of contours."]
        #[cxx_name = "BRepFilletAPI_MakeChamfer_LastVertex"]
        fn MakeChamfer_last_vertex(self_: &MakeChamfer, IC: i32) -> UniquePtr<TopoDS_Vertex>;
        #[doc = "Returns the internal filleting algorithm."]
        #[cxx_name = "BRepFilletAPI_MakeChamfer_Builder"]
        fn MakeChamfer_builder(self_: &MakeChamfer) -> UniquePtr<HandleTopOpeBRepBuildHBuilder>;
        #[cxx_name = "BRepFilletAPI_MakeChamfer_Sect"]
        fn MakeChamfer_sect(
            self_: &MakeChamfer,
            IC: i32,
            IS: i32,
        ) -> UniquePtr<HandleChFiDSSecHArray1>;
        #[doc = " ======================== BRepFilletAPI_MakeFillet ========================"]
        #[doc = "Describes functions to build fillets on the broken edges of a shell or solid. A MakeFillet object provides a framework for: -   initializing the construction algorithm with a given shape, -   acquiring the data characterizing the fillets, -   building the fillets and constructing the resulting shape, and -   consulting the result."]
        #[cxx_name = "BRepFilletAPI_MakeFillet"]
        type MakeFillet;
        #[cxx_name = "SetParams"]
        fn set_params(
            self: Pin<&mut MakeFillet>,
            Tang: f64,
            Tesp: f64,
            T2d: f64,
            TApp3d: f64,
            TolApp2d: f64,
            Fleche: f64,
        );
        #[doc = "Adds a  fillet contour in  the  builder  (builds a contour  of tangent edges). The Radius must be set after."]
        #[cxx_name = "Add"]
        fn addedge(self: Pin<&mut MakeFillet>, E: &TopoDS_Edge);
        #[doc = "Adds a  fillet description in  the  builder - builds a contour  of tangent edges, - sets the radius."]
        #[cxx_name = "Add"]
        fn addreal_2(self: Pin<&mut MakeFillet>, Radius: f64, E: &TopoDS_Edge);
        #[doc = "Adds a  fillet description in  the  builder - builds a contour  of tangent edges, - sets a linear radius evolution law between the first and last vertex of the spine."]
        #[cxx_name = "Add"]
        fn addreal_3(self: Pin<&mut MakeFillet>, R1: f64, R2: f64, E: &TopoDS_Edge);
        #[doc = "Adds a  fillet description in  the  builder - builds a contour  of tangent edges, - sest the radius evolution law."]
        #[cxx_name = "Add"]
        fn addhandlefunction_4(self: Pin<&mut MakeFillet>, L: &HandleLawFunction, E: &TopoDS_Edge);
        #[doc = "Adds a  fillet description in  the  builder - builds a contour  of tangent edges, - sets the radius evolution law interpolating the values given in the array UandR : p2d.X() = relative parameter on the spine [0,1] p2d.Y() = value of the radius."]
        #[cxx_name = "Add"]
        fn addarray1ofpnt2d_5(
            self: Pin<&mut MakeFillet>,
            UandR: &TColgp_Array1OfPnt2d,
            E: &TopoDS_Edge,
        );
        #[doc = "Sets the parameters of the fillet along the contour of index IC generated using the Add function in the internal data structure of this algorithm, where Radius is the radius of the fillet."]
        #[cxx_name = "SetRadius"]
        fn set_radiusreal(self: Pin<&mut MakeFillet>, Radius: f64, IC: i32, IinC: i32);
        #[doc = "Sets the parameters of the fillet along the contour of index IC generated using the Add function in the internal data structure of this algorithm, where the radius of the fillet evolves according to a linear evolution law defined from R1 to R2, between the first and last vertices of the contour of index IC."]
        #[cxx_name = "SetRadius"]
        fn set_radiusreal_2(self: Pin<&mut MakeFillet>, R1: f64, R2: f64, IC: i32, IinC: i32);
        #[doc = "Sets the parameters of the fillet along the contour of index IC generated using the Add function in the internal data structure of this algorithm, where the radius of the fillet evolves according to the evolution law L, between the first and last vertices of the contour of index IC."]
        #[cxx_name = "SetRadius"]
        fn set_radiushandlefunction_3(
            self: Pin<&mut MakeFillet>,
            L: &HandleLawFunction,
            IC: i32,
            IinC: i32,
        );
        #[doc = "Sets the parameters of the fillet along the contour of index IC generated using the Add function in the internal data structure of this algorithm, where the radius of the fillet evolves according to the evolution law which interpolates the set of parameter and radius pairs given in the array UandR as follows: -   the X coordinate of a point in UandR defines a relative parameter on the contour (i.e. a parameter between 0 and 1), -          the Y coordinate of a point in UandR gives the corresponding value of the radius, and the radius evolves between the first and last vertices of the contour of index IC."]
        #[cxx_name = "SetRadius"]
        fn set_radiusarray1ofpnt2d_4(
            self: Pin<&mut MakeFillet>,
            UandR: &TColgp_Array1OfPnt2d,
            IC: i32,
            IinC: i32,
        );
        #[doc = "Erases the radius information on the contour of index IC in the internal data structure of this algorithm. Use the SetRadius function to reset this data. Warning Nothing is done if IC is outside the bounds of the table of contours."]
        #[cxx_name = "ResetContour"]
        fn reset_contour(self: Pin<&mut MakeFillet>, IC: i32);
        #[doc = "Returns true if the radius of the fillet along the contour of index IC in the internal data structure of this algorithm is constant, Warning False is returned if IC is outside the bounds of the table of contours or if E does not belong to the contour of index IC."]
        #[cxx_name = "IsConstant"]
        fn is_constantint(self: Pin<&mut MakeFillet>, IC: i32) -> bool;
        #[doc = "Returns the radius of the fillet along the contour of index IC in the internal data structure of this algorithm Warning -   Use this function only if the radius is constant. -   -1. is returned if IC is outside the bounds of the table of contours or if E does not belong to the contour of index IC."]
        #[cxx_name = "Radius"]
        fn radiusint(self: Pin<&mut MakeFillet>, IC: i32) -> f64;
        #[doc = "Returns true if the radius of the fillet along the edge E of the contour of index IC in the internal data structure of this algorithm is constant. Warning False is returned if IC is outside the bounds of the table of contours or if E does not belong to the contour of index IC."]
        #[cxx_name = "IsConstant"]
        fn is_constantint_2(self: Pin<&mut MakeFillet>, IC: i32, E: &TopoDS_Edge) -> bool;
        #[doc = "Returns the radius of the fillet along the edge E of the contour of index IC in the internal data structure of this algorithm. Warning -   Use this function only if the radius is constant. -   -1 is returned if IC is outside the bounds of the table of contours or if E does not belong to the contour of index IC."]
        #[cxx_name = "Radius"]
        fn radiusint_2(self: Pin<&mut MakeFillet>, IC: i32, E: &TopoDS_Edge) -> f64;
        #[doc = "Assigns Radius as the radius of the fillet on the edge E"]
        #[cxx_name = "SetRadius"]
        fn set_radiusreal_5(self: Pin<&mut MakeFillet>, Radius: f64, IC: i32, E: &TopoDS_Edge);
        #[cxx_name = "SetRadius"]
        fn set_radiusreal_6(self: Pin<&mut MakeFillet>, Radius: f64, IC: i32, V: &TopoDS_Vertex);
        #[cxx_name = "GetBounds"]
        fn get_bounds(
            self: Pin<&mut MakeFillet>,
            IC: i32,
            E: &TopoDS_Edge,
            F: &mut f64,
            L: &mut f64,
        ) -> bool;
        #[cxx_name = "SetLaw"]
        fn set_law(self: Pin<&mut MakeFillet>, IC: i32, E: &TopoDS_Edge, L: &HandleLawFunction);
        #[doc = "Returns the number of contours generated using the Add function in the internal data structure of this algorithm."]
        #[cxx_name = "NbContours"]
        fn nb_contours(self: &MakeFillet) -> i32;
        #[doc = "Returns the index of the contour in the internal data structure of this algorithm which contains the edge E of the shape. This function returns 0 if the edge E does not belong to any contour. Warning This index can change if a contour is removed from the internal data structure of this algorithm using the function Remove."]
        #[cxx_name = "Contour"]
        fn contour(self: &MakeFillet, E: &TopoDS_Edge) -> i32;
        #[doc = "Returns the number of edges in the contour of index I in the internal data structure of this algorithm. Warning Returns 0 if I is outside the bounds of the table of contours."]
        #[cxx_name = "NbEdges"]
        fn nb_edges(self: &MakeFillet, I: i32) -> i32;
        #[doc = "Returns the edge of index J in the contour of index I in the internal data structure of this algorithm. Warning Returns a null shape if: -   I is outside the bounds of the table of contours, or -   J is outside the bounds of the table of edges of the index I contour."]
        #[cxx_name = "Edge"]
        fn edge(self: &MakeFillet, I: i32, J: i32) -> &TopoDS_Edge;
        #[doc = "Removes the contour in the internal data structure of this algorithm which contains the edge E of the shape. Warning Nothing is done if the edge E does not belong to the contour in the internal data structure of this algorithm."]
        #[cxx_name = "Remove"]
        fn remove(self: Pin<&mut MakeFillet>, E: &TopoDS_Edge);
        #[doc = "Returns the length of the contour of index IC in the internal data structure of this algorithm. Warning Returns -1. if IC is outside the bounds of the table of contours."]
        #[cxx_name = "Length"]
        fn length(self: &MakeFillet, IC: i32) -> f64;
        #[doc = "Returns the curvilinear abscissa of the vertex V on the contour of index IC in the internal data structure of this algorithm. Warning Returns -1. if: -   IC is outside the bounds of the table of contours, or -   V is not on the contour of index IC."]
        #[cxx_name = "Abscissa"]
        fn abscissa(self: &MakeFillet, IC: i32, V: &TopoDS_Vertex) -> f64;
        #[doc = "Returns the relative curvilinear abscissa (i.e. between 0 and 1) of the vertex V on the contour of index IC in the internal data structure of this algorithm. Warning Returns -1. if: -   IC is outside the bounds of the table of contours, or -   V is not on the contour of index IC."]
        #[cxx_name = "RelativeAbscissa"]
        fn relative_abscissa(self: &MakeFillet, IC: i32, V: &TopoDS_Vertex) -> f64;
        #[doc = "Returns true if the contour of index IC in the internal data structure of this algorithm is closed and tangential at the point of closure. Warning Returns false if IC is outside the bounds of the table of contours."]
        #[cxx_name = "ClosedAndTangent"]
        fn closed_and_tangent(self: &MakeFillet, IC: i32) -> bool;
        #[doc = "Returns true if the contour of index IC in the internal data structure of this algorithm is closed. Warning Returns false if IC is outside the bounds of the table of contours."]
        #[cxx_name = "Closed"]
        fn closed(self: &MakeFillet, IC: i32) -> bool;
        #[doc = "Builds the fillets on all the contours in the internal data structure of this algorithm and constructs the resulting shape. Use the function IsDone to verify that the filleted shape is built. Use the function Shape to retrieve the filleted shape. Warning The construction of fillets implements highly complex construction algorithms. Consequently, there may be instances where the algorithm fails, for example if the data defining the radius of the fillet is not compatible with the geometry of the initial shape. There is no initial analysis of errors and they only become evident at the construction stage. Additionally, in the current software release, the following cases are not handled: -   the end point of the contour is the point of intersection of 4 or more edges of the shape, or -   the intersection of the fillet with a face which limits the contour is not fully contained in this face."]
        #[cxx_name = "Build"]
        fn build(self: Pin<&mut MakeFillet>, theRange: &Message_ProgressRange);
        #[doc = "Reinitializes this algorithm, thus canceling the effects of the Build function. This function allows modifications to be made to the contours and fillet parameters in order to rebuild the shape."]
        #[cxx_name = "Reset"]
        fn reset(self: Pin<&mut MakeFillet>);
        #[doc = "Returns the  list   of shapes generated   from the shape <EorV>."]
        #[cxx_name = "Generated"]
        fn generated(self: Pin<&mut MakeFillet>, EorV: &TopoDS_Shape) -> &TopTools_ListOfShape;
        #[doc = "Returns the list  of shapes modified from the shape <F>."]
        #[cxx_name = "Modified"]
        fn modified(self: Pin<&mut MakeFillet>, F: &TopoDS_Shape) -> &TopTools_ListOfShape;
        #[cxx_name = "IsDeleted"]
        fn is_deleted(self: Pin<&mut MakeFillet>, F: &TopoDS_Shape) -> bool;
        #[doc = "returns the number of surfaces after the shape creation."]
        #[cxx_name = "NbSurfaces"]
        fn nb_surfaces(self: &MakeFillet) -> i32;
        #[doc = "Return the faces created for surface <I>."]
        #[cxx_name = "NewFaces"]
        fn new_faces(self: Pin<&mut MakeFillet>, I: i32) -> &TopTools_ListOfShape;
        #[cxx_name = "Simulate"]
        fn simulate(self: Pin<&mut MakeFillet>, IC: i32);
        #[cxx_name = "NbSurf"]
        fn nb_surf(self: &MakeFillet, IC: i32) -> i32;
        #[doc = "Returns the number of contours where the computation of the fillet failed"]
        #[cxx_name = "NbFaultyContours"]
        fn nb_faulty_contours(self: &MakeFillet) -> i32;
        #[doc = "for each I in [1.. NbFaultyContours] returns the index IC of the contour where the computation of the fillet failed. the method NbEdges(IC) gives the number of edges in the contour IC the method Edge(IC,ie) gives the edge number ie of the contour IC"]
        #[cxx_name = "FaultyContour"]
        fn faulty_contour(self: &MakeFillet, I: i32) -> i32;
        #[doc = "returns the number of surfaces which have been computed on the contour IC"]
        #[cxx_name = "NbComputedSurfaces"]
        fn nb_computed_surfaces(self: &MakeFillet, IC: i32) -> i32;
        #[doc = "returns the number of vertices where the computation failed"]
        #[cxx_name = "NbFaultyVertices"]
        fn nb_faulty_vertices(self: &MakeFillet) -> i32;
        #[doc = "returns true if a part of the result has been computed if the filling in a corner failed a shape with a hole is returned"]
        #[cxx_name = "HasResult"]
        fn has_result(self: &MakeFillet) -> bool;
        #[cxx_name = "BRepFilletAPI_MakeFillet_GetLaw"]
        fn MakeFillet_get_law(
            self_: Pin<&mut MakeFillet>,
            IC: i32,
            E: &TopoDS_Edge,
        ) -> UniquePtr<HandleLawFunction>;
        #[doc = "Returns the type of fillet shape built by this algorithm."]
        #[cxx_name = "BRepFilletAPI_MakeFillet_GetFilletShape"]
        fn MakeFillet_get_fillet_shape(self_: &MakeFillet) -> UniquePtr<ChFi3d_FilletShape>;
        #[doc = "Returns the first vertex of the contour of index IC in the internal data structure of this algorithm. Warning Returns a null shape if IC is outside the bounds of the table of contours."]
        #[cxx_name = "BRepFilletAPI_MakeFillet_FirstVertex"]
        fn MakeFillet_first_vertex(self_: &MakeFillet, IC: i32) -> UniquePtr<TopoDS_Vertex>;
        #[doc = "Returns the  last vertex of the contour of index IC in the internal data structure of this algorithm. Warning Returns a null shape if IC is outside the bounds of the table of contours."]
        #[cxx_name = "BRepFilletAPI_MakeFillet_LastVertex"]
        fn MakeFillet_last_vertex(self_: &MakeFillet, IC: i32) -> UniquePtr<TopoDS_Vertex>;
        #[doc = "Returns the internal topology building algorithm."]
        #[cxx_name = "BRepFilletAPI_MakeFillet_Builder"]
        fn MakeFillet_builder(self_: &MakeFillet) -> UniquePtr<HandleTopOpeBRepBuildHBuilder>;
        #[cxx_name = "BRepFilletAPI_MakeFillet_Sect"]
        fn MakeFillet_sect(
            self_: &MakeFillet,
            IC: i32,
            IS: i32,
        ) -> UniquePtr<HandleChFiDSSecHArray1>;
        #[doc = "returns the surface number IS concerning the contour IC"]
        #[cxx_name = "BRepFilletAPI_MakeFillet_ComputedSurface"]
        fn MakeFillet_computed_surface(
            self_: &MakeFillet,
            IC: i32,
            IS: i32,
        ) -> UniquePtr<HandleGeomSurface>;
        #[doc = "returns the vertex where the computation failed"]
        #[cxx_name = "BRepFilletAPI_MakeFillet_FaultyVertex"]
        fn MakeFillet_faulty_vertex(self_: &MakeFillet, IV: i32) -> UniquePtr<TopoDS_Vertex>;
        #[doc = "if (HasResult()) returns the partial result"]
        #[cxx_name = "BRepFilletAPI_MakeFillet_BadShape"]
        fn MakeFillet_bad_shape(self_: &MakeFillet) -> UniquePtr<TopoDS_Shape>;
        #[doc = "returns the status concerning the contour IC in case of error ChFiDS_Ok : the computation is Ok ChFiDS_StartsolFailure : the computation can't start, perhaps the the radius is too big ChFiDS_TwistedSurface : the computation failed because of a twisted surface ChFiDS_WalkingFailure : there is a problem in the walking ChFiDS_Error:  other error different from above"]
        #[cxx_name = "BRepFilletAPI_MakeFillet_StripeStatus"]
        fn MakeFillet_stripe_status(self_: &MakeFillet, IC: i32) -> UniquePtr<ChFiDS_ErrorStatus>;
        #[doc = " ======================== BRepFilletAPI_MakeFillet2d ========================"]
        #[doc = "Describes functions to build fillets and chamfers on the vertices of a planar face. Fillets and Chamfers on the Vertices of a Planar Face A MakeFillet2d object provides a framework for: - initializing the construction algorithm with a given face, - acquiring the data characterizing the fillets and chamfers, -   building the fillets and chamfers, and constructing the resulting shape, and -   consulting the result. Warning Only segments of straight lines and arcs of circles are treated. BSplines are not processed."]
        #[cxx_name = "BRepFilletAPI_MakeFillet2d"]
        type MakeFillet2d;
        #[doc = "Initializes an empty algorithm for computing fillets and chamfers. The face on which the fillets and chamfers are built is defined using the Init function. The vertices on which fillets or chamfers are built are defined using the AddFillet or AddChamfer function. Warning The status of the initialization, as given by the Status function, can be one of the following: -   ChFi2d_Ready if the initialization is correct, -   ChFi2d_NotPlanar if F is not planar, -   ChFi2d_NoFace if F is a null face."]
        #[cxx_name = "BRepFilletAPI_MakeFillet2d_ctor"]
        fn MakeFillet2d_ctor() -> UniquePtr<MakeFillet2d>;
        #[doc = "Initializes an algorithm for computing fillets and chamfers on the face F. The vertices on which fillets or chamfers are built are defined using the AddFillet or AddChamfer function. Warning The status of the initialization, as given by the Status function, can be one of the following: -   ChFi2d_Ready if the initialization is correct, -   ChFi2d_NotPlanar if F is not planar, -   ChFi2d_NoFace if F is a null face."]
        #[cxx_name = "BRepFilletAPI_MakeFillet2d_ctor_face"]
        fn MakeFillet2d_ctor_face(F: &TopoDS_Face) -> UniquePtr<MakeFillet2d>;
        #[doc = "Initializes this algorithm for constructing fillets or chamfers with the face F. Warning The status of the initialization, as given by the Status function, can be one of the following: -   ChFi2d_Ready if the initialization is correct, -   ChFi2d_NotPlanar if F is not planar, -   ChFi2d_NoFace if F is a null face."]
        #[cxx_name = "Init"]
        fn initface(self: Pin<&mut MakeFillet2d>, F: &TopoDS_Face);
        #[doc = "This initialize method allow to init the builder from a face RefFace and another face ModFace which derive from RefFace. This  is useful to modify a fillet or a chamfer already created on ModFace."]
        #[cxx_name = "Init"]
        fn initface_2(self: Pin<&mut MakeFillet2d>, RefFace: &TopoDS_Face, ModFace: &TopoDS_Face);
        #[doc = "Returns true if the edge E on the face modified by this algorithm is chamfered or filleted. Warning Returns false if E does not belong to the face modified by this algorithm."]
        #[cxx_name = "IsModified"]
        fn is_modified(self: &MakeFillet2d, E: &TopoDS_Edge) -> bool;
        #[doc = "Returns the table of fillets on the face modified by this algorithm."]
        #[cxx_name = "FilletEdges"]
        fn fillet_edges(self: &MakeFillet2d) -> &TopTools_SequenceOfShape;
        #[doc = "Returns the number of fillets on the face modified by this algorithm."]
        #[cxx_name = "NbFillet"]
        fn nb_fillet(self: &MakeFillet2d) -> i32;
        #[doc = "Returns the table of chamfers on the face modified by this algorithm."]
        #[cxx_name = "ChamferEdges"]
        fn chamfer_edges(self: &MakeFillet2d) -> &TopTools_SequenceOfShape;
        #[doc = "Returns the number of chamfers on the face modified by this algorithm."]
        #[cxx_name = "NbChamfer"]
        fn nb_chamfer(self: &MakeFillet2d) -> i32;
        #[doc = "Returns the list  of shapes modified from the shape <S>."]
        #[cxx_name = "Modified"]
        fn modified(self: Pin<&mut MakeFillet2d>, S: &TopoDS_Shape) -> &TopTools_ListOfShape;
        #[doc = "returns the number of new curves after the shape creation."]
        #[cxx_name = "NbCurves"]
        fn nb_curves(self: &MakeFillet2d) -> i32;
        #[doc = "Return the Edges created for curve I."]
        #[cxx_name = "NewEdges"]
        fn new_edges(self: Pin<&mut MakeFillet2d>, I: i32) -> &TopTools_ListOfShape;
        #[cxx_name = "HasDescendant"]
        fn has_descendant(self: &MakeFillet2d, E: &TopoDS_Edge) -> bool;
        #[doc = "Returns the chamfered or filleted edge built from the edge E on the face modified by this algorithm. If E has not been modified, this function returns E. Exceptions Standard_NoSuchObject if the edge E does not belong to the initial face."]
        #[cxx_name = "DescendantEdge"]
        fn descendant_edge(self: &MakeFillet2d, E: &TopoDS_Edge) -> &TopoDS_Edge;
        #[doc = "Returns the basis edge on the face modified by this algorithm from which the chamfered or filleted edge E is built. If E has not been modified, this function returns E. Warning E is returned if it does not belong to the initial face."]
        #[cxx_name = "BasisEdge"]
        fn basis_edge(self: &MakeFillet2d, E: &TopoDS_Edge) -> &TopoDS_Edge;
        #[doc = "Update the result and set the Done flag"]
        #[cxx_name = "Build"]
        fn build(self: Pin<&mut MakeFillet2d>, theRange: &Message_ProgressRange);
        #[doc = "Adds a fillet of radius Radius between the two edges adjacent to the vertex V on the face modified by this algorithm. The two edges do not need to be rectilinear. This function returns the fillet and builds the resulting face. Warning The status of the construction, as given by the Status function, can be one of the following: - ChFi2d_IsDone if the fillet is built, - ChFi2d_ConnexionError if V does not belong to the initial face, -   ChFi2d_ComputationError if Radius is too large to build a fillet between the two adjacent edges, -   ChFi2d_NotAuthorized -   if one of the two edges connected to V is a fillet or chamfer, or -   if a curve other than a straight line or an arc of a circle is used as E, E1 or E2. Do not use the returned fillet if the status of the construction is not ChFi2d_IsDone. Exceptions Standard_NegativeValue if Radius is less than or equal to zero."]
        #[cxx_name = "BRepFilletAPI_MakeFillet2d_AddFillet"]
        fn MakeFillet2d_add_fillet(
            self_: Pin<&mut MakeFillet2d>,
            V: &TopoDS_Vertex,
            Radius: f64,
        ) -> UniquePtr<TopoDS_Edge>;
        #[doc = "Assigns the radius Radius to the fillet Fillet already built on the face modified by this algorithm. This function returns the new fillet and modifies the existing face. Warning The status of the construction, as given by the Status function, can be one of the following: -   ChFi2d_IsDone if the new fillet is built, -   ChFi2d_ConnexionError if Fillet does not belong to the existing face, -   ChFi2d_ComputationError if Radius is too large to build a fillet between the two adjacent edges. Do not use the returned fillet if the status of the construction is not ChFi2d_IsDone. Exceptions Standard_NegativeValue if Radius is less than or equal to zero."]
        #[cxx_name = "BRepFilletAPI_MakeFillet2d_ModifyFillet"]
        fn MakeFillet2d_modify_fillet(
            self_: Pin<&mut MakeFillet2d>,
            Fillet: &TopoDS_Edge,
            Radius: f64,
        ) -> UniquePtr<TopoDS_Edge>;
        #[doc = "Removes the fillet Fillet already built on the face modified by this algorithm. This function returns the vertex connecting the two adjacent edges of Fillet and modifies the existing face. Warning -   The returned vertex is only valid if the Status function returns ChFi2d_IsDone. -   A null vertex is returned if the edge Fillet does not belong to the initial face."]
        #[cxx_name = "BRepFilletAPI_MakeFillet2d_RemoveFillet"]
        fn MakeFillet2d_remove_fillet(
            self_: Pin<&mut MakeFillet2d>,
            Fillet: &TopoDS_Edge,
        ) -> UniquePtr<TopoDS_Vertex>;
        #[doc = "Adds a chamfer on the face modified by this algorithm between the two adjacent edges E1 and E2, where the extremities of the chamfer are on E1 and E2 at distances D1 and D2 respectively In cases where the edges are not rectilinear, distances are measured using the curvilinear abscissa of the edges and the angle is measured with respect to the tangent at the corresponding point. The angle Ang is given in radians. This function returns the chamfer and builds the resulting face."]
        #[cxx_name = "BRepFilletAPI_MakeFillet2d_AddChamfer"]
        fn MakeFillet2d_add_chamferedge(
            self_: Pin<&mut MakeFillet2d>,
            E1: &TopoDS_Edge,
            E2: &TopoDS_Edge,
            D1: f64,
            D2: f64,
        ) -> UniquePtr<TopoDS_Edge>;
        #[doc = "Adds a chamfer on the face modified by this algorithm between the two edges connected by the vertex V, where E is one of the two edges. The chamfer makes an angle Ang with E and one of its extremities is on E at distance D from V. In cases where the edges are not rectilinear, distances are measured using the curvilinear abscissa of the edges and the angle is measured with respect to the tangent at the corresponding point. The angle Ang is given in radians. This function returns the chamfer and builds the resulting face. Warning The status of the construction, as given by the Status function, can be one of the following: -          ChFi2d_IsDone if the chamfer is built, -  ChFi2d_ParametersError if D1, D2, D or Ang is less than or equal to zero, -          ChFi2d_ConnexionError if: - the edge E, E1 or E2 does not belong to the initial face, or -  the edges E1 and E2 are not adjacent, or -  the vertex V is not one of the limit points of the edge E, -          ChFi2d_ComputationError if the parameters of the chamfer are too large to build a chamfer between the two adjacent edges, -          ChFi2d_NotAuthorized if: - the edge E1, E2 or one of the two edges connected to V is a fillet or chamfer, or - a curve other than a straight line or an arc of a circle is used as E, E1 or E2. Do not use the returned chamfer if the status of the construction is not ChFi2d_IsDone."]
        #[cxx_name = "BRepFilletAPI_MakeFillet2d_AddChamfer"]
        fn MakeFillet2d_add_chamferedge_2(
            self_: Pin<&mut MakeFillet2d>,
            E: &TopoDS_Edge,
            V: &TopoDS_Vertex,
            D: f64,
            Ang: f64,
        ) -> UniquePtr<TopoDS_Edge>;
        #[doc = "Modifies the chamfer Chamfer on the face modified by this algorithm, where: E1 and E2 are the two adjacent edges on which Chamfer is already built; the extremities of the new chamfer are on E1 and E2 at distances D1 and D2 respectively."]
        #[cxx_name = "BRepFilletAPI_MakeFillet2d_ModifyChamfer"]
        fn MakeFillet2d_modify_chamferedge(
            self_: Pin<&mut MakeFillet2d>,
            Chamfer: &TopoDS_Edge,
            E1: &TopoDS_Edge,
            E2: &TopoDS_Edge,
            D1: f64,
            D2: f64,
        ) -> UniquePtr<TopoDS_Edge>;
        #[doc = "Modifies the chamfer Chamfer on the face modified by this algorithm, where: E is one of the two adjacent edges on which Chamfer is already built; the new chamfer makes an angle Ang with E and one of its extremities is on E at distance D from the vertex on which the chamfer is built. In cases where the edges are not rectilinear, the distances are measured using the curvilinear abscissa of the edges and the angle is measured with respect to the tangent at the corresponding point. The angle Ang is given in radians. This function returns the new chamfer and modifies the existing face. Warning The status of the construction, as given by the Status function, can be one of the following: -   ChFi2d_IsDone if the chamfer is built, -   ChFi2d_ParametersError if D1, D2, D or Ang is less than or equal to zero, -   ChFi2d_ConnexionError if: -   the edge E, E1, E2 or Chamfer does not belong to the existing face, or -   the edges E1 and E2 are not adjacent, -   ChFi2d_ComputationError if the parameters of the chamfer are too large to build a chamfer between the two adjacent edges, -   ChFi2d_NotAuthorized if E1 or E2 is a fillet or chamfer. Do not use the returned chamfer if the status of the construction is not ChFi2d_IsDone."]
        #[cxx_name = "BRepFilletAPI_MakeFillet2d_ModifyChamfer"]
        fn MakeFillet2d_modify_chamferedge_2(
            self_: Pin<&mut MakeFillet2d>,
            Chamfer: &TopoDS_Edge,
            E: &TopoDS_Edge,
            D: f64,
            Ang: f64,
        ) -> UniquePtr<TopoDS_Edge>;
        #[doc = "Removes the chamfer Chamfer already built on the face modified by this algorithm. This function returns the vertex connecting the two adjacent edges of Chamfer and modifies the existing face. Warning -   The returned vertex is only valid if the Status function returns ChFi2d_IsDone. -   A null vertex is returned if the edge Chamfer does not belong to the initial face."]
        #[cxx_name = "BRepFilletAPI_MakeFillet2d_RemoveChamfer"]
        fn MakeFillet2d_remove_chamfer(
            self_: Pin<&mut MakeFillet2d>,
            Chamfer: &TopoDS_Edge,
        ) -> UniquePtr<TopoDS_Vertex>;
        #[cxx_name = "BRepFilletAPI_MakeFillet2d_Status"]
        fn MakeFillet2d_status(self_: &MakeFillet2d) -> UniquePtr<ChFi2d_ConstructionError>;
    }
    impl UniquePtr<MakeChamfer> {}
    impl UniquePtr<MakeFillet> {}
    impl UniquePtr<MakeFillet2d> {}
}
pub use ffi::MakeChamfer;
impl MakeChamfer {
    #[doc = "Initializes an algorithm for computing chamfers on the shape S. The edges on which chamfers are built are defined using the Add function."]
    pub fn new_shape(S: &ffi::TopoDS_Shape) -> cxx::UniquePtr<Self> {
        ffi::MakeChamfer_ctor_shape(S)
    }
}
pub use ffi::MakeFillet;
impl MakeFillet {}
pub use ffi::MakeFillet2d;
impl MakeFillet2d {
    #[doc = "Initializes an empty algorithm for computing fillets and chamfers. The face on which the fillets and chamfers are built is defined using the Init function. The vertices on which fillets or chamfers are built are defined using the AddFillet or AddChamfer function. Warning The status of the initialization, as given by the Status function, can be one of the following: -   ChFi2d_Ready if the initialization is correct, -   ChFi2d_NotPlanar if F is not planar, -   ChFi2d_NoFace if F is a null face."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::MakeFillet2d_ctor()
    }

    #[doc = "Initializes an algorithm for computing fillets and chamfers on the face F. The vertices on which fillets or chamfers are built are defined using the AddFillet or AddChamfer function. Warning The status of the initialization, as given by the Status function, can be one of the following: -   ChFi2d_Ready if the initialization is correct, -   ChFi2d_NotPlanar if F is not planar, -   ChFi2d_NoFace if F is a null face."]
    pub fn new_face(F: &ffi::TopoDS_Face) -> cxx::UniquePtr<Self> {
        ffi::MakeFillet2d_ctor_face(F)
    }
}
