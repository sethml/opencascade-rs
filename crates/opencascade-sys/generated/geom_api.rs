#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_geom_api.hxx");
        #[doc = "HArray1OfPnt from t_colgp module"]
        type TColgp_HArray1OfPnt = crate::t_colgp::ffi::HArray1OfPnt;
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
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TColStd_HArray1OfReal"]
        type TColStd_HArray1OfReal;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Extrema_ExtPC"]
        type Extrema_ExtPC;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Extrema_ExtPS"]
        type Extrema_ExtPS;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TColgp_Array1OfVec"]
        type TColgp_Array1OfVec;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TColgp_Array1OfPnt"]
        type TColgp_Array1OfPnt;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TColStd_HArray1OfBoolean"]
        type TColStd_HArray1OfBoolean;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TColStd_Array1OfReal"]
        type TColStd_Array1OfReal;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Approx_ParametrizationType"]
        type Approx_ParametrizationType;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "GeomAbs_Shape"]
        type GeomAbs_Shape;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Extrema_ExtAlgo"]
        type Extrema_ExtAlgo;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Extrema_ExtFlag"]
        type Extrema_ExtFlag;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleTColgpHArray1OfPnt"]
        type HandleTColgpHArray1OfPnt;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleGeomSurface"]
        type HandleGeomSurface;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleTColStdHArray1OfBoolean"]
        type HandleTColStdHArray1OfBoolean;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleTColStdHArray1OfReal"]
        type HandleTColStdHArray1OfReal;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleGeomBSplineCurve"]
        type HandleGeomBSplineCurve;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleGeomCurve"]
        type HandleGeomCurve;
        #[doc = " ======================== GeomAPI_Interpolate ========================"]
        #[doc = "This  class  is  used  to  interpolate a  BsplineCurve passing   through  an  array  of  points,  with  a  C2 Continuity if tangency is not requested at the point. If tangency is requested at the point the continuity will be C1.  If Perodicity is requested the curve will be closed and the junction will be the first point given. The curve will than be only C1 Describes functions for building a constrained 3D BSpline curve. The curve is defined by a table of points through which it passes, and if required: -   by a parallel table of reals which gives the value of the parameter of each point through which the resulting BSpline curve passes, and -   by vectors tangential to these points. An Interpolate object provides a framework for: -   defining the constraints of the BSpline curve, -   implementing the interpolation algorithm, and -   consulting the results."]
        #[cxx_name = "GeomAPI_Interpolate"]
        type Interpolate;
        #[doc = "Initializes an algorithm for constructing a constrained BSpline curve passing through the points of the table   Points. Tangential vectors can then be assigned, using the function Load. If PeriodicFlag is true, the constrained BSpline curve will be periodic and closed. In this case, the junction point is the first point of the table Points. The tolerance value Tolerance is used to check that: -   points are not too close to each other, or -   tangential vectors (defined using the function Load) are not too small. The resulting BSpline curve will be \"C2\" continuous, except where a tangency constraint is defined on a point through which the curve passes (by using the Load function). In this case, it will be only \"C1\" continuous. Once all the constraints are defined, use the function Perform to compute the curve. Warning -   There must be at least 2 points in the table Points. -   If PeriodicFlag is false, there must be as many parameters in the array Parameters as there are points in the array Points. -   If PeriodicFlag is true, there must be one more parameter in the table Parameters: this is used to give the parameter on the resulting BSpline curve of the junction point of the curve (which is also the first point of the table Points). Exceptions -   Standard_ConstructionError if the distance between two consecutive points in the table Points is less than or equal to Tolerance. -   Standard_OutOfRange if: -   there are less than two points in the table Points, or -   conditions relating to the respective number of elements in the parallel tables Points and Parameters are not respected."]
        #[cxx_name = "GeomAPI_Interpolate_ctor_handleharray1ofpnt_bool_real"]
        fn Interpolate_ctor_handleharray1ofpnt_bool_real(
            Points: &HandleTColgpHArray1OfPnt,
            PeriodicFlag: bool,
            Tolerance: f64,
        ) -> UniquePtr<Interpolate>;
        #[doc = "Initializes an algorithm for constructing a constrained BSpline curve passing through the points of the table Points, where the parameters of each of its points are given by the parallel table Parameters. Tangential vectors can then be assigned, using the function Load. If PeriodicFlag is true, the constrained BSpline curve will be periodic and closed. In this case, the junction point is the first point of the table Points. The tolerance value Tolerance is used to check that: -   points are not too close to each other, or -   tangential vectors (defined using the function Load) are not too small. The resulting BSpline curve will be \"C2\" continuous, except where a tangency constraint is defined on a point through which the curve passes (by using the Load function). In this case, it will be only \"C1\" continuous. Once all the constraints are defined, use the function Perform to compute the curve. Warning -   There must be at least 2 points in the table Points. -   If PeriodicFlag is false, there must be as many parameters in the array Parameters as there are points in the array Points. -   If PeriodicFlag is true, there must be one more parameter in the table Parameters: this is used to give the parameter on the resulting BSpline curve of the junction point of the curve (which is also the first point of the table Points). Exceptions -   Standard_ConstructionError if the distance between two consecutive points in the table Points is less than or equal to Tolerance. -   Standard_OutOfRange if: -   there are less than two points in the table Points, or -   conditions relating to the respective number of elements in the parallel tables Points and Parameters are not respected."]
        #[cxx_name = "GeomAPI_Interpolate_ctor_handleharray1ofpnt_handleharray1ofreal_bool_real"]
        fn Interpolate_ctor_handleharray1ofpnt_handleharray1ofreal_bool_real(
            Points: &HandleTColgpHArray1OfPnt,
            Parameters: &HandleTColStdHArray1OfReal,
            PeriodicFlag: bool,
            Tolerance: f64,
        ) -> UniquePtr<Interpolate>;
        #[doc = "Assigns this constrained BSpline curve to be tangential to vectors InitialTangent and FinalTangent at its first and last points respectively (i.e. the first and last points of the table of points through which the curve passes, as defined at the time of initialization)."]
        #[cxx_name = "Load"]
        fn loadvec(
            self: Pin<&mut Interpolate>,
            InitialTangent: &gp_Vec,
            FinalTangent: &gp_Vec,
            Scale: bool,
        );
        #[doc = "Assigns this constrained BSpline curve to be tangential to vectors defined in the table Tangents, which is parallel to the table of points through which the curve passes, as defined at the time of initialization. Vectors in the table Tangents are defined only if the flag given in the parallel table TangentFlags is true: only these vectors are set as tangency constraints."]
        #[cxx_name = "Load"]
        fn loadarray1ofvec_2(
            self: Pin<&mut Interpolate>,
            Tangents: &TColgp_Array1OfVec,
            TangentFlags: &HandleTColStdHArray1OfBoolean,
            Scale: bool,
        );
        #[doc = "Clears all tangency constraints on this constrained BSpline curve (as initialized by the function Load)."]
        #[cxx_name = "ClearTangents"]
        fn clear_tangents(self: Pin<&mut Interpolate>);
        #[doc = "Computes the constrained BSpline curve. Use the function IsDone to verify that the computation is successful, and then the function Curve to obtain the result."]
        #[cxx_name = "Perform"]
        fn perform(self: Pin<&mut Interpolate>);
        #[doc = "Returns the computed BSpline curve. Raises StdFail_NotDone if the interpolation fails."]
        #[cxx_name = "Curve"]
        fn curve(self: &Interpolate) -> &HandleGeomBSplineCurve;
        #[doc = "Returns true if the constrained BSpline curve is successfully constructed. Note: in this case, the result is given by the function Curve."]
        #[cxx_name = "IsDone"]
        fn is_done(self: &Interpolate) -> bool;
        #[doc = " ======================== GeomAPI_PointsToBSpline ========================"]
        #[doc = "This  class  is  used  to  approximate a  BsplineCurve passing  through an  array  of points,  with  a  given Continuity. Describes functions for building a 3D BSpline curve which approximates a set of points. A PointsToBSpline object provides a framework for: -   defining the data of the BSpline curve to be built, -   implementing the approximation algorithm, and consulting the results."]
        #[cxx_name = "GeomAPI_PointsToBSpline"]
        type PointsToBSpline;
        #[doc = "Constructs an empty approximation algorithm. Use an Init function to define and build the BSpline curve."]
        #[cxx_name = "GeomAPI_PointsToBSpline_ctor"]
        fn PointsToBSpline_ctor() -> UniquePtr<PointsToBSpline>;
        #[doc = "Returns the computed BSpline curve. Raises StdFail_NotDone if the curve is not built."]
        #[cxx_name = "Curve"]
        fn curve(self: &PointsToBSpline) -> &HandleGeomBSplineCurve;
        #[cxx_name = "IsDone"]
        fn is_done(self: &PointsToBSpline) -> bool;
        #[doc = " ======================== GeomAPI_ProjectPointOnCurve ========================"]
        #[doc = "This class implements methods for  computing all the orthogonal projections of a 3D point onto a  3D curve."]
        #[cxx_name = "GeomAPI_ProjectPointOnCurve"]
        type ProjectPointOnCurve;
        #[doc = "Creates an empty object. Use an Init function for further initialization."]
        #[cxx_name = "GeomAPI_ProjectPointOnCurve_ctor"]
        fn ProjectPointOnCurve_ctor() -> UniquePtr<ProjectPointOnCurve>;
        #[doc = "Create the projection  of a  point  <P> on a curve <Curve>"]
        #[cxx_name = "GeomAPI_ProjectPointOnCurve_ctor_pnt_handlecurve"]
        fn ProjectPointOnCurve_ctor_pnt_handlecurve(
            P: &gp_Pnt,
            Curve: &HandleGeomCurve,
        ) -> UniquePtr<ProjectPointOnCurve>;
        #[doc = "Create  the projection  of a point <P>  on a curve <Curve> limited by the two points of parameter Umin and Usup."]
        #[cxx_name = "GeomAPI_ProjectPointOnCurve_ctor_pnt_handlecurve_real2"]
        fn ProjectPointOnCurve_ctor_pnt_handlecurve_real2(
            P: &gp_Pnt,
            Curve: &HandleGeomCurve,
            Umin: f64,
            Usup: f64,
        ) -> UniquePtr<ProjectPointOnCurve>;
        #[doc = "Init the projection  of a  point  <P> on a curve <Curve>"]
        #[cxx_name = "Init"]
        fn initpnt(self: Pin<&mut ProjectPointOnCurve>, P: &gp_Pnt, Curve: &HandleGeomCurve);
        #[doc = "Init  the  projection  of a  point <P>  on a curve <Curve> limited by the two points of parameter Umin and Usup."]
        #[cxx_name = "Init"]
        fn initpnt_2(
            self: Pin<&mut ProjectPointOnCurve>,
            P: &gp_Pnt,
            Curve: &HandleGeomCurve,
            Umin: f64,
            Usup: f64,
        );
        #[doc = "Init  the  projection  of a  point <P>  on a curve <Curve> limited by the two points of parameter Umin and Usup."]
        #[cxx_name = "Init"]
        fn inithandlecurve_3(
            self: Pin<&mut ProjectPointOnCurve>,
            Curve: &HandleGeomCurve,
            Umin: f64,
            Usup: f64,
        );
        #[doc = "Performs the projection of a point on the current curve."]
        #[cxx_name = "Perform"]
        fn perform(self: Pin<&mut ProjectPointOnCurve>, P: &gp_Pnt);
        #[doc = "Returns the number of computed orthogonal projection points. Note: if this algorithm fails, NbPoints returns 0."]
        #[cxx_name = "NbPoints"]
        fn nb_points(self: &ProjectPointOnCurve) -> i32;
        #[doc = "Returns the parameter on the curve of the point, which is the orthogonal projection. Index is a number of a computed point. Exceptions Standard_OutOfRange if Index is not in the range [ 1,NbPoints ], where NbPoints is the number of solution points."]
        #[cxx_name = "Parameter"]
        fn parameterint(self: &ProjectPointOnCurve, Index: i32) -> f64;
        #[doc = "Returns the parameter on the curve of the point, which is the orthogonal projection. Index is a number of a computed point. Exceptions Standard_OutOfRange if Index is not in the range [ 1,NbPoints ], where NbPoints is the number of solution points.-"]
        #[cxx_name = "Parameter"]
        fn parameterint_2(self: &ProjectPointOnCurve, Index: i32, U: &mut f64);
        #[doc = "Computes the distance between the point and its orthogonal projection on the curve. Index is a number of a computed point. Exceptions Standard_OutOfRange if Index is not in the range [ 1,NbPoints ], where NbPoints is the number of solution points."]
        #[cxx_name = "Distance"]
        fn distance(self: &ProjectPointOnCurve, Index: i32) -> f64;
        #[doc = "Returns the parameter on the curve of the nearest orthogonal projection of the point. Exceptions: StdFail_NotDone if this algorithm fails."]
        #[cxx_name = "LowerDistanceParameter"]
        fn lower_distance_parameter(self: &ProjectPointOnCurve) -> f64;
        #[doc = "Computes the distance between the point and its nearest orthogonal projection on the curve. Exceptions: StdFail_NotDone if this algorithm fails."]
        #[cxx_name = "LowerDistance"]
        fn lower_distance(self: &ProjectPointOnCurve) -> f64;
        #[doc = "return the algorithmic object from Extrema"]
        #[cxx_name = "Extrema"]
        fn extrema(self: &ProjectPointOnCurve) -> &Extrema_ExtPC;
        #[doc = "Returns the orthogonal projection on the curve. Index is a number of a computed point. Exceptions Standard_OutOfRange if Index is not in the range [ 1,NbPoints ], where NbPoints is the number of solution points."]
        #[cxx_name = "GeomAPI_ProjectPointOnCurve_Point"]
        fn ProjectPointOnCurve_point(self_: &ProjectPointOnCurve, Index: i32) -> UniquePtr<gp_Pnt>;
        #[doc = "Returns the nearest orthogonal projection of the point on the curve. Exceptions: StdFail_NotDone if this algorithm fails."]
        #[cxx_name = "GeomAPI_ProjectPointOnCurve_NearestPoint"]
        fn ProjectPointOnCurve_nearest_point(self_: &ProjectPointOnCurve) -> UniquePtr<gp_Pnt>;
        #[doc = " ======================== GeomAPI_ProjectPointOnSurf ========================"]
        #[doc = "This class implements methods for  computing all the orthogonal projections of a point onto a  surface."]
        #[cxx_name = "GeomAPI_ProjectPointOnSurf"]
        type ProjectPointOnSurf;
        #[doc = "Creates an empty object. Use the Init function for further initialization."]
        #[cxx_name = "GeomAPI_ProjectPointOnSurf_ctor"]
        fn ProjectPointOnSurf_ctor() -> UniquePtr<ProjectPointOnSurf>;
        #[doc = "Performs the projection of a point on the current surface."]
        #[cxx_name = "Perform"]
        fn perform(self: Pin<&mut ProjectPointOnSurf>, P: &gp_Pnt);
        #[cxx_name = "IsDone"]
        fn is_done(self: &ProjectPointOnSurf) -> bool;
        #[doc = "Returns the number of computed orthogonal projection points. Note: if projection fails, NbPoints returns 0."]
        #[cxx_name = "NbPoints"]
        fn nb_points(self: &ProjectPointOnSurf) -> i32;
        #[doc = "Returns the parameters (U,V) on the surface of the orthogonal projection. Index is a number of a computed point. Exceptions Standard_OutOfRange if Index is not in the range [ 1,NbPoints ], where NbPoints is the number of solution points."]
        #[cxx_name = "Parameters"]
        fn parameters(self: &ProjectPointOnSurf, Index: i32, U: &mut f64, V: &mut f64);
        #[doc = "Computes the distance between the point and its orthogonal projection on the surface. Index is a number of a computed point. Exceptions Standard_OutOfRange if Index is not in the range [ 1,NbPoints ], where NbPoints is the number of solution points."]
        #[cxx_name = "Distance"]
        fn distance(self: &ProjectPointOnSurf, Index: i32) -> f64;
        #[doc = "Returns the parameters (U,V) on the surface of the nearest computed orthogonal projection of the point. Exceptions StdFail_NotDone if projection fails."]
        #[cxx_name = "LowerDistanceParameters"]
        fn lower_distance_parameters(self: &ProjectPointOnSurf, U: &mut f64, V: &mut f64);
        #[doc = "Computes the distance between the point and its nearest orthogonal projection on the surface. Exceptions StdFail_NotDone if projection fails."]
        #[cxx_name = "LowerDistance"]
        fn lower_distance(self: &ProjectPointOnSurf) -> f64;
        #[doc = "return the algorithmic object from Extrema"]
        #[cxx_name = "Extrema"]
        fn extrema(self: &ProjectPointOnSurf) -> &Extrema_ExtPS;
        #[doc = "Returns the orthogonal projection on the surface. Index is a number of a computed point. Exceptions Standard_OutOfRange if Index is not in the range [ 1,NbPoints ], where NbPoints is the number of solution points."]
        #[cxx_name = "GeomAPI_ProjectPointOnSurf_Point"]
        fn ProjectPointOnSurf_point(self_: &ProjectPointOnSurf, Index: i32) -> UniquePtr<gp_Pnt>;
        #[doc = "Returns the nearest orthogonal projection of the point on the surface. Exceptions StdFail_NotDone if projection fails."]
        #[cxx_name = "GeomAPI_ProjectPointOnSurf_NearestPoint"]
        fn ProjectPointOnSurf_nearest_point(self_: &ProjectPointOnSurf) -> UniquePtr<gp_Pnt>;
    }
    impl UniquePtr<Interpolate> {}
    impl UniquePtr<PointsToBSpline> {}
    impl UniquePtr<ProjectPointOnCurve> {}
    impl UniquePtr<ProjectPointOnSurf> {}
}
pub use ffi::Interpolate;
impl Interpolate {
    #[doc = "Initializes an algorithm for constructing a constrained BSpline curve passing through the points of the table   Points. Tangential vectors can then be assigned, using the function Load. If PeriodicFlag is true, the constrained BSpline curve will be periodic and closed. In this case, the junction point is the first point of the table Points. The tolerance value Tolerance is used to check that: -   points are not too close to each other, or -   tangential vectors (defined using the function Load) are not too small. The resulting BSpline curve will be \"C2\" continuous, except where a tangency constraint is defined on a point through which the curve passes (by using the Load function). In this case, it will be only \"C1\" continuous. Once all the constraints are defined, use the function Perform to compute the curve. Warning -   There must be at least 2 points in the table Points. -   If PeriodicFlag is false, there must be as many parameters in the array Parameters as there are points in the array Points. -   If PeriodicFlag is true, there must be one more parameter in the table Parameters: this is used to give the parameter on the resulting BSpline curve of the junction point of the curve (which is also the first point of the table Points). Exceptions -   Standard_ConstructionError if the distance between two consecutive points in the table Points is less than or equal to Tolerance. -   Standard_OutOfRange if: -   there are less than two points in the table Points, or -   conditions relating to the respective number of elements in the parallel tables Points and Parameters are not respected."]
    pub fn new_handleharray1ofpnt_bool_real(
        Points: &ffi::HandleTColgpHArray1OfPnt,
        PeriodicFlag: bool,
        Tolerance: f64,
    ) -> cxx::UniquePtr<Self> {
        ffi::Interpolate_ctor_handleharray1ofpnt_bool_real(Points, PeriodicFlag, Tolerance)
    }

    #[doc = "Initializes an algorithm for constructing a constrained BSpline curve passing through the points of the table Points, where the parameters of each of its points are given by the parallel table Parameters. Tangential vectors can then be assigned, using the function Load. If PeriodicFlag is true, the constrained BSpline curve will be periodic and closed. In this case, the junction point is the first point of the table Points. The tolerance value Tolerance is used to check that: -   points are not too close to each other, or -   tangential vectors (defined using the function Load) are not too small. The resulting BSpline curve will be \"C2\" continuous, except where a tangency constraint is defined on a point through which the curve passes (by using the Load function). In this case, it will be only \"C1\" continuous. Once all the constraints are defined, use the function Perform to compute the curve. Warning -   There must be at least 2 points in the table Points. -   If PeriodicFlag is false, there must be as many parameters in the array Parameters as there are points in the array Points. -   If PeriodicFlag is true, there must be one more parameter in the table Parameters: this is used to give the parameter on the resulting BSpline curve of the junction point of the curve (which is also the first point of the table Points). Exceptions -   Standard_ConstructionError if the distance between two consecutive points in the table Points is less than or equal to Tolerance. -   Standard_OutOfRange if: -   there are less than two points in the table Points, or -   conditions relating to the respective number of elements in the parallel tables Points and Parameters are not respected."]
    pub fn new_handleharray1ofpnt_handleharray1ofreal_bool_real(
        Points: &ffi::HandleTColgpHArray1OfPnt,
        Parameters: &ffi::HandleTColStdHArray1OfReal,
        PeriodicFlag: bool,
        Tolerance: f64,
    ) -> cxx::UniquePtr<Self> {
        ffi::Interpolate_ctor_handleharray1ofpnt_handleharray1ofreal_bool_real(
            Points,
            Parameters,
            PeriodicFlag,
            Tolerance,
        )
    }
}
pub use ffi::PointsToBSpline;
impl PointsToBSpline {
    #[doc = "Constructs an empty approximation algorithm. Use an Init function to define and build the BSpline curve."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::PointsToBSpline_ctor()
    }
}
pub use ffi::ProjectPointOnCurve;
impl ProjectPointOnCurve {
    #[doc = "Creates an empty object. Use an Init function for further initialization."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::ProjectPointOnCurve_ctor()
    }

    #[doc = "Create the projection  of a  point  <P> on a curve <Curve>"]
    pub fn new_pnt_handlecurve(
        P: &ffi::gp_Pnt,
        Curve: &ffi::HandleGeomCurve,
    ) -> cxx::UniquePtr<Self> {
        ffi::ProjectPointOnCurve_ctor_pnt_handlecurve(P, Curve)
    }

    #[doc = "Create  the projection  of a point <P>  on a curve <Curve> limited by the two points of parameter Umin and Usup."]
    pub fn new_pnt_handlecurve_real2(
        P: &ffi::gp_Pnt,
        Curve: &ffi::HandleGeomCurve,
        Umin: f64,
        Usup: f64,
    ) -> cxx::UniquePtr<Self> {
        ffi::ProjectPointOnCurve_ctor_pnt_handlecurve_real2(P, Curve, Umin, Usup)
    }
}
pub use ffi::ProjectPointOnSurf;
impl ProjectPointOnSurf {
    #[doc = "Creates an empty object. Use the Init function for further initialization."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::ProjectPointOnSurf_ctor()
    }
}
