#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_geom.hxx");
        #[doc = "HArray1OfPnt from t_colgp module"]
        type TColgp_HArray1OfPnt = crate::t_colgp::ffi::HArray1OfPnt;
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
        #[cxx_name = "GeomAbs_BSplKnotDistribution"]
        type GeomAbs_BSplKnotDistribution;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "gp_Cylinder"]
        type gp_Cylinder;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TColStd_Array1OfInteger"]
        type TColStd_Array1OfInteger;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Geom_Geometry"]
        type Geom_Geometry;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TColgp_Array1OfPnt"]
        type TColgp_Array1OfPnt;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TColStd_Array1OfReal"]
        type TColStd_Array1OfReal;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "GeomAbs_Shape"]
        type GeomAbs_Shape;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Standard_Type"]
        type Standard_Type;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleGeomSurface"]
        type HandleGeomSurface;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleStandardType"]
        type HandleStandardType;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleGeomCurve"]
        type HandleGeomCurve;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleGeomBSplineCurve"]
        type HandleGeomBSplineCurve;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleGeomGeometry"]
        type HandleGeomGeometry;
        #[doc = " ======================== Geom_BezierCurve ========================"]
        #[doc = "Describes a rational or non-rational Bezier curve - a non-rational Bezier curve is defined by a table of poles (also called control points), - a rational Bezier curve is defined by a table of poles with varying weights. These data are manipulated by two parallel arrays: - the poles table, which is an array of gp_Pnt points, and - the weights table, which is an array of reals. The bounds of these arrays are 1 and \"the number of \"poles\" of the curve. The poles of the curve are \"control points\" used to deform the curve. The first pole is the start point of the curve, and the last pole is the end point of the curve. The segment that joins the first pole to the second pole is the tangent to the curve at its start point, and the segment that joins the last pole to the second-from-last pole is the tangent to the curve at its end point. It is more difficult to give a geometric signification to the weights but they are useful for providing the exact representations of arcs of a circle or ellipse. Moreover, if the weights of all poles are equal, the curve is polynomial; it is therefore a non-rational curve. The non-rational curve is a special and frequently used case. The weights are defined and used only in the case of a rational curve. The degree of a Bezier curve is equal to the number of poles, minus 1. It must be greater than or equal to 1. However, the degree of a Geom_BezierCurve curve is limited to a value (25) which is defined and controlled by the system. This value is returned by the function MaxDegree. The parameter range for a Bezier curve is [ 0, 1 ]. If the first and last control points of the Bezier curve are the same point then the curve is closed. For example, to create a closed Bezier curve with four control points, you have to give the set of control points P1, P2, P3 and P1. The continuity of a Bezier curve is infinite. It is not possible to build a Bezier curve with negative weights. We consider that a weight value is zero if it is less than or equal to gp::Resolution(). We also consider that two weight values W1 and W2 are equal if: |W2 - W1| <= gp::Resolution(). Warning - When considering the continuity of a closed Bezier curve at the junction point, remember that a curve of this type is never periodic. This means that the derivatives for the parameter u = 0 have no reason to be the same as the derivatives for the parameter u = 1 even if the curve is closed. - The length of a Bezier curve can be null."]
        #[cxx_name = "Geom_BezierCurve"]
        type BezierCurve;
        #[doc = "Creates a non rational Bezier curve with a set of poles CurvePoles.  The weights are defaulted to all being 1. Raises ConstructionError if the number of poles is greater than MaxDegree + 1 or lower than 2."]
        #[cxx_name = "Geom_BezierCurve_ctor_array1ofpnt"]
        fn BezierCurve_ctor_array1ofpnt(CurvePoles: &TColgp_Array1OfPnt) -> UniquePtr<BezierCurve>;
        #[doc = "Creates a rational Bezier curve with the set of poles CurvePoles and the set of weights  PoleWeights . If all the weights are identical the curve is considered as non rational. Raises ConstructionError if the number of poles is greater than  MaxDegree + 1 or lower than 2 or CurvePoles and CurveWeights have not the same length or one weight value is lower or equal to Resolution from package gp."]
        #[cxx_name = "Geom_BezierCurve_ctor_array1ofpnt_array1ofreal"]
        fn BezierCurve_ctor_array1ofpnt_array1ofreal(
            CurvePoles: &TColgp_Array1OfPnt,
            PoleWeights: &TColStd_Array1OfReal,
        ) -> UniquePtr<BezierCurve>;
        #[doc = "Increases the degree of a bezier curve. Degree is the new degree of <me>. Raises ConstructionError if Degree is greater than MaxDegree or lower than 2 or lower than the initial degree of <me>."]
        #[cxx_name = "Increase"]
        fn increase(self: Pin<&mut BezierCurve>, Degree: i32);
        #[doc = "Inserts a pole P after the pole of range Index. If the curve <me> is rational the weight value for the new pole of range Index is 1.0. raised if Index is not in the range [1, NbPoles] raised if the resulting number of poles is greater than MaxDegree + 1."]
        #[cxx_name = "InsertPoleAfter"]
        fn insert_pole_afterint(self: Pin<&mut BezierCurve>, Index: i32, P: &gp_Pnt);
        #[doc = "Inserts a pole with its weight in the set of poles after the pole of range Index. If the curve was non rational it can become rational if all the weights are not identical. Raised if Index is not in the range [1, NbPoles] Raised if the resulting number of poles is greater than MaxDegree + 1. Raised if Weight is lower or equal to Resolution from package gp."]
        #[cxx_name = "InsertPoleAfter"]
        fn insert_pole_afterint_2(self: Pin<&mut BezierCurve>, Index: i32, P: &gp_Pnt, Weight: f64);
        #[doc = "Inserts a pole P before the pole of range Index. If the curve <me> is rational the weight value for the new pole of range Index is 1.0. Raised if Index is not in the range [1, NbPoles] Raised if the resulting number of poles is greater than MaxDegree + 1."]
        #[cxx_name = "InsertPoleBefore"]
        fn insert_pole_beforeint(self: Pin<&mut BezierCurve>, Index: i32, P: &gp_Pnt);
        #[doc = "Inserts a pole with its weight in the set of poles after the pole of range Index. If the curve was non rational it can become rational if all the weights are not identical. Raised if Index is not in the range [1, NbPoles] Raised if the resulting number of poles is greater than MaxDegree + 1. Raised if Weight is lower or equal to Resolution from package gp."]
        #[cxx_name = "InsertPoleBefore"]
        fn insert_pole_beforeint_2(
            self: Pin<&mut BezierCurve>,
            Index: i32,
            P: &gp_Pnt,
            Weight: f64,
        );
        #[doc = "Removes the pole of range Index. If the curve was rational it can become non rational. Raised if Index is not in the range [1, NbPoles] Raised if Degree is lower than 2."]
        #[cxx_name = "RemovePole"]
        fn remove_pole(self: Pin<&mut BezierCurve>, Index: i32);
        #[doc = "Reverses the direction of parametrization of <me> Value (NewU) =  Value (1 - OldU)"]
        #[cxx_name = "Reverse"]
        fn reverse(self: Pin<&mut BezierCurve>);
        #[doc = "Returns the  parameter on the  reversed  curve for the point of parameter U on <me>. returns 1-U"]
        #[cxx_name = "ReversedParameter"]
        fn reversed_parameter(self: &BezierCurve, U: f64) -> f64;
        #[doc = "Segments the curve between U1 and U2 which can be out of the bounds of the curve. The curve is oriented from U1 to U2. The control points are modified, the first and the last point are not the same but the parametrization range is [0, 1] else it could not be a Bezier curve. Warnings : Even if <me> is not closed it can become closed after the segmentation for example if U1 or U2 are out of the bounds of the curve <me> or if the curve makes loop. After the segmentation the length of a curve can be null."]
        #[cxx_name = "Segment"]
        fn segment(self: Pin<&mut BezierCurve>, U1: f64, U2: f64);
        #[doc = "Substitutes the pole of range index with P. If the curve <me> is rational the weight of range Index is not modified. raiseD if Index is not in the range [1, NbPoles]"]
        #[cxx_name = "SetPole"]
        fn set_poleint(self: Pin<&mut BezierCurve>, Index: i32, P: &gp_Pnt);
        #[doc = "Substitutes the pole and the weights of range Index. If the curve <me> is not rational it can become rational if all the weights are not identical. If the curve was rational it can become non rational if all the weights are identical. Raised if Index is not in the range [1, NbPoles] Raised if Weight <= Resolution from package gp"]
        #[cxx_name = "SetPole"]
        fn set_poleint_2(self: Pin<&mut BezierCurve>, Index: i32, P: &gp_Pnt, Weight: f64);
        #[doc = "Changes the weight of the pole of range Index. If the curve <me> is not rational it can become rational if all the weights are not identical. If the curve was rational it can become non rational if all the weights are identical. Raised if Index is not in the range [1, NbPoles] Raised if Weight <= Resolution from package gp"]
        #[cxx_name = "SetWeight"]
        fn set_weight(self: Pin<&mut BezierCurve>, Index: i32, Weight: f64);
        #[doc = "Returns True if the distance between the first point and the last point of the curve is lower or equal to the Resolution from package gp."]
        #[cxx_name = "IsClosed"]
        fn is_closed(self: &BezierCurve) -> bool;
        #[doc = "Continuity of the curve, returns True."]
        #[cxx_name = "IsCN"]
        fn is_cn(self: &BezierCurve, N: i32) -> bool;
        #[doc = "Returns True if the parametrization of a curve is periodic. (P(u) = P(u + T) T = constante)"]
        #[cxx_name = "IsPeriodic"]
        fn is_periodic(self: &BezierCurve) -> bool;
        #[doc = "Returns false if all the weights are identical. The tolerance criterion is Resolution from package gp."]
        #[cxx_name = "IsRational"]
        fn is_rational(self: &BezierCurve) -> bool;
        #[doc = "Returns the polynomial degree of the curve. it is the number of poles - 1 point P and derivatives (V1, V2, V3) computation The Bezier Curve has a Polynomial representation so the parameter U can be out of the bounds of the curve."]
        #[cxx_name = "Degree"]
        fn degree(self: &BezierCurve) -> i32;
        #[cxx_name = "D0"]
        fn d0(self: &BezierCurve, U: f64, P: Pin<&mut gp_Pnt>);
        #[cxx_name = "D1"]
        fn d1(self: &BezierCurve, U: f64, P: Pin<&mut gp_Pnt>, V1: Pin<&mut gp_Vec>);
        #[cxx_name = "D2"]
        fn d2(
            self: &BezierCurve,
            U: f64,
            P: Pin<&mut gp_Pnt>,
            V1: Pin<&mut gp_Vec>,
            V2: Pin<&mut gp_Vec>,
        );
        #[doc = "For this Bezier curve, computes - the point P of parameter U, or - the point P and one or more of the following values: - V1, the first derivative vector, - V2, the second derivative vector, - V3, the third derivative vector. Note: the parameter U can be outside the bounds of the curve."]
        #[cxx_name = "D3"]
        fn d3(
            self: &BezierCurve,
            U: f64,
            P: Pin<&mut gp_Pnt>,
            V1: Pin<&mut gp_Vec>,
            V2: Pin<&mut gp_Vec>,
            V3: Pin<&mut gp_Vec>,
        );
        #[doc = "Returns the value of the first  parameter of this Bezier curve. This is 0.0, which gives the start point of this Bezier curve"]
        #[cxx_name = "FirstParameter"]
        fn first_parameter(self: &BezierCurve) -> f64;
        #[doc = "Returns the value of the last parameter of this Bezier curve. This is  1.0, which gives the end point of this Bezier curve."]
        #[cxx_name = "LastParameter"]
        fn last_parameter(self: &BezierCurve) -> f64;
        #[doc = "Returns the number of poles of this Bezier curve."]
        #[cxx_name = "NbPoles"]
        fn nb_poles(self: &BezierCurve) -> i32;
        #[doc = "Returns the pole of range Index. Raised if Index is not in the range [1, NbPoles]"]
        #[cxx_name = "Pole"]
        fn pole(self: &BezierCurve, Index: i32) -> &gp_Pnt;
        #[doc = "Returns all the poles of the curve. Raised if the length of P is not equal to the number of poles."]
        #[cxx_name = "Poles"]
        fn polesarray1ofpnt(self: &BezierCurve, P: Pin<&mut TColgp_Array1OfPnt>);
        #[doc = "Returns all the poles of the curve."]
        #[cxx_name = "Poles"]
        fn poles2(self: &BezierCurve) -> &TColgp_Array1OfPnt;
        #[doc = "Returns the weight of range Index. Raised if Index is not in the range [1, NbPoles]"]
        #[cxx_name = "Weight"]
        fn weight(self: &BezierCurve, Index: i32) -> f64;
        #[doc = "Returns all the weights of the curve. Raised if the length of W is not equal to the number of poles."]
        #[cxx_name = "Weights"]
        fn weightsarray1ofreal(self: &BezierCurve, W: Pin<&mut TColStd_Array1OfReal>);
        #[doc = "Applies the transformation T to this Bezier curve."]
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut BezierCurve>, T: &gp_Trsf);
        #[doc = "Computes for this Bezier curve the parametric tolerance UTolerance for a given 3D tolerance Tolerance3D. If f(t) is the equation of this Bezier curve, UTolerance ensures that: |t1-t0| < UTolerance ===> |f(t1)-f(t0)| < Tolerance3D"]
        #[cxx_name = "Resolution"]
        fn resolution(self: Pin<&mut BezierCurve>, Tolerance3D: f64, UTolerance: &mut f64);
        #[cxx_name = "DynamicType"]
        fn dynamic_type(self: &BezierCurve) -> &HandleStandardType;
        #[doc = "a Bezier curve is CN"]
        #[cxx_name = "Geom_BezierCurve_Continuity"]
        fn BezierCurve_continuity(self_: &BezierCurve) -> UniquePtr<GeomAbs_Shape>;
        #[doc = "For the point of parameter U of this Bezier curve, computes the vector corresponding to the Nth derivative. Note: the parameter U can be outside the bounds of the curve. Exceptions Standard_RangeError if N is less than 1."]
        #[cxx_name = "Geom_BezierCurve_DN"]
        fn BezierCurve_dn(self_: &BezierCurve, U: f64, N: i32) -> UniquePtr<gp_Vec>;
        #[doc = "Returns Value (U=0.), it is the first control point of the curve."]
        #[cxx_name = "Geom_BezierCurve_StartPoint"]
        fn BezierCurve_start_point(self_: &BezierCurve) -> UniquePtr<gp_Pnt>;
        #[doc = "Returns Value (U=1.), it is the last control point of the Bezier curve."]
        #[cxx_name = "Geom_BezierCurve_EndPoint"]
        fn BezierCurve_end_point(self_: &BezierCurve) -> UniquePtr<gp_Pnt>;
        #[doc = "Creates a new object which is a copy of this Bezier curve."]
        #[cxx_name = "Geom_BezierCurve_Copy"]
        fn BezierCurve_copy(self_: &BezierCurve) -> UniquePtr<HandleGeomGeometry>;
        #[doc = "Returns the value of the maximum polynomial degree of any Geom_BezierCurve curve. This value is 25."]
        #[cxx_name = "Geom_BezierCurve_MaxDegree"]
        fn BezierCurve_max_degree() -> i32;
        #[cxx_name = "Geom_BezierCurve_get_type_name"]
        fn BezierCurve_get_type_name() -> String;
        #[doc = " ======================== Geom_BSplineCurve ========================"]
        #[doc = "Definition of the B_spline curve. A B-spline curve can be Uniform  or non-uniform Rational or non-rational Periodic or non-periodic a b-spline curve is defined by : its degree; the degree for a Geom_BSplineCurve is limited to a value (25) which is defined and controlled by the system. This value is returned by the function MaxDegree; - its periodic or non-periodic nature; - a table of poles (also called control points), with their associated weights if the BSpline curve is rational. The poles of the curve are \"control points\" used to deform the curve. If the curve is non-periodic, the first pole is the start point of the curve, and the last pole is the end point of the curve. The segment which joins the first pole to the second pole is the tangent to the curve at its start point, and the segment which joins the last pole to the second-from-last pole is the tangent to the curve at its end point. If the curve is periodic, these geometric properties are not verified. It is more difficult to give a geometric signification to the weights but are useful for providing exact representations of the arcs of a circle or ellipse. Moreover, if the weights of all the poles are equal, the curve has a polynomial equation; it is therefore a non-rational curve. - a table of knots with their multiplicities. For a Geom_BSplineCurve, the table of knots is an increasing sequence of reals without repetition; the multiplicities define the repetition of the knots. A BSpline curve is a piecewise polynomial or rational curve. The knots are the parameters of junction points between two pieces. The multiplicity Mult(i) of the knot Knot(i) of the BSpline curve is related to the degree of continuity of the curve at the knot Knot(i), which is equal to Degree - Mult(i) where Degree is the degree of the BSpline curve. If the knots are regularly spaced (i.e. the difference between two consecutive knots is a constant), three specific and frequently used cases of knot distribution can be identified: - \"uniform\" if all multiplicities are equal to 1, - \"quasi-uniform\" if all multiplicities are equal to 1, except the first and the last knot which have a multiplicity of Degree + 1, where Degree is the degree of the BSpline curve, - \"Piecewise Bezier\" if all multiplicities are equal to Degree except the first and last knot which have a multiplicity of Degree + 1, where Degree is the degree of the BSpline curve. A curve of this type is a concatenation of arcs of Bezier curves. If the BSpline curve is not periodic: - the bounds of the Poles and Weights tables are 1 and NbPoles, where NbPoles is the number of poles of the BSpline curve, - the bounds of the Knots and Multiplicities tables are 1 and NbKnots, where NbKnots is the number of knots of the BSpline curve. If the BSpline curve is periodic, and if there are k periodic knots and p periodic poles, the period is: period = Knot(k + 1) - Knot(1) and the poles and knots tables can be considered as infinite tables, verifying: - Knot(i+k) = Knot(i) + period - Pole(i+p) = Pole(i) Note: data structures of a periodic BSpline curve are more complex than those of a non-periodic one. Warning In this class, weight value is considered to be zero if the weight is less than or equal to gp::Resolution(). References : . A survey of curve and surface methods in CADG Wolfgang BOHM CAGD 1 (1984) . On de Boor-like algorithms and blossoming Wolfgang BOEHM cagd 5 (1988) . Blossoming and knot insertion algorithms for B-spline curves Ronald N. GOLDMAN . Modelisation des surfaces en CAO, Henri GIAUME Peugeot SA . Curves and Surfaces for Computer Aided Geometric Design, a practical guide Gerald Farin"]
        #[cxx_name = "Geom_BSplineCurve"]
        type BSplineCurve;
        #[doc = "Creates a  non-rational B_spline curve   on  the basis <Knots, Multiplicities> of degree <Degree>."]
        #[cxx_name = "Geom_BSplineCurve_ctor_array1ofpnt_array1ofreal_array1ofinteger_int_bool"]
        fn BSplineCurve_ctor_array1ofpnt_array1ofreal_array1ofinteger_int_bool(
            Poles: &TColgp_Array1OfPnt,
            Knots: &TColStd_Array1OfReal,
            Multiplicities: &TColStd_Array1OfInteger,
            Degree: i32,
            Periodic: bool,
        ) -> UniquePtr<BSplineCurve>;
        #[doc = "Creates  a rational B_spline  curve  on the basis <Knots, Multiplicities> of degree <Degree>. Raises ConstructionError subject to the following conditions 0 < Degree <= MaxDegree. Weights.Length() == Poles.Length() Knots.Length() == Mults.Length() >= 2 Knots(i) < Knots(i+1) (Knots are increasing) 1 <= Mults(i) <= Degree On a non periodic curve the first and last multiplicities may be Degree+1 (this is even recommended if you want the curve to start and finish on the first and last pole). On a periodic  curve the first  and  the last multicities must be the same. on non-periodic curves Poles.Length() == Sum(Mults(i)) - Degree - 1 >= 2 on periodic curves Poles.Length() == Sum(Mults(i)) except the first or last"]
        #[cxx_name = "Geom_BSplineCurve_ctor_array1ofpnt_array1ofreal2_array1ofinteger_int_bool2"]
        fn BSplineCurve_ctor_array1ofpnt_array1ofreal2_array1ofinteger_int_bool2(
            Poles: &TColgp_Array1OfPnt,
            Weights: &TColStd_Array1OfReal,
            Knots: &TColStd_Array1OfReal,
            Multiplicities: &TColStd_Array1OfInteger,
            Degree: i32,
            Periodic: bool,
            CheckRational: bool,
        ) -> UniquePtr<BSplineCurve>;
        #[doc = "Increases the degree of this BSpline curve to Degree. As a result, the poles, weights and multiplicities tables are modified; the knots table is not changed. Nothing is done if Degree is less than or equal to the current degree. Exceptions Standard_ConstructionError if Degree is greater than Geom_BSplineCurve::MaxDegree()."]
        #[cxx_name = "IncreaseDegree"]
        fn increase_degree(self: Pin<&mut BSplineCurve>, Degree: i32);
        #[doc = "Increases the multiplicity  of the knot <Index> to <M>. If   <M>   is   lower   or  equal   to  the current multiplicity nothing is done. If <M> is higher than the degree the degree is used. If <Index> is not in [FirstUKnotIndex, LastUKnotIndex]"]
        #[cxx_name = "IncreaseMultiplicity"]
        fn increase_multiplicityint(self: Pin<&mut BSplineCurve>, Index: i32, M: i32);
        #[doc = "Increases  the  multiplicities   of  the knots  in [I1,I2] to <M>. For each knot if  <M>  is  lower  or equal  to  the current multiplicity  nothing  is  done. If <M>  is higher than the degree the degree is used. If <I1,I2> are not in [FirstUKnotIndex, LastUKnotIndex]"]
        #[cxx_name = "IncreaseMultiplicity"]
        fn increase_multiplicityint_2(self: Pin<&mut BSplineCurve>, I1: i32, I2: i32, M: i32);
        #[doc = "Increment  the  multiplicities   of  the knots  in [I1,I2] by <M>. If <M> is not positive nithing is done. For   each  knot   the resulting   multiplicity  is limited to the Degree. If <I1,I2> are not in [FirstUKnotIndex, LastUKnotIndex]"]
        #[cxx_name = "IncrementMultiplicity"]
        fn increment_multiplicity(self: Pin<&mut BSplineCurve>, I1: i32, I2: i32, M: i32);
        #[doc = "Inserts a knot value in the sequence of knots.  If <U>  is an  existing knot     the multiplicity  is increased by <M>. If U  is  not  on the parameter  range  nothing is done. If the multiplicity is negative or null nothing is done. The  new   multiplicity  is limited  to  the degree. The  tolerance criterion  for  knots  equality  is the max of Epsilon(U) and ParametricTolerance."]
        #[cxx_name = "InsertKnot"]
        fn insert_knot(
            self: Pin<&mut BSplineCurve>,
            U: f64,
            M: i32,
            ParametricTolerance: f64,
            Add: bool,
        );
        #[doc = "Inserts a set of knots  values in  the sequence of knots. For each U = Knots(i), M = Mults(i) If <U>  is an existing  knot  the  multiplicity is increased by  <M> if  <Add>  is True, increased to <M> if <Add> is False. If U  is  not  on the parameter  range  nothing is done. If the multiplicity is negative or null nothing is done. The  new   multiplicity  is limited  to  the degree. The  tolerance criterion  for  knots  equality  is the max of Epsilon(U) and ParametricTolerance."]
        #[cxx_name = "InsertKnots"]
        fn insert_knots(
            self: Pin<&mut BSplineCurve>,
            Knots: &TColStd_Array1OfReal,
            Mults: &TColStd_Array1OfInteger,
            ParametricTolerance: f64,
            Add: bool,
        );
        #[doc = "Reduces the multiplicity of the knot of index Index to M. If M is equal to 0, the knot is removed. With a modification of this type, the array of poles is also modified. Two different algorithms are systematically used to compute the new poles of the curve. If, for each pole, the distance between the pole calculated using the first algorithm and the same pole calculated using the second algorithm, is less than Tolerance, this ensures that the curve is not modified by more than Tolerance. Under these conditions, true is returned; otherwise, false is returned. A low tolerance is used to prevent modification of the curve. A high tolerance is used to \"smooth\" the curve. Exceptions Standard_OutOfRange if Index is outside the bounds of the knots table. pole insertion and pole removing this operation is limited to the Uniform or QuasiUniform BSplineCurve. The knot values are modified . If the BSpline is NonUniform or Piecewise Bezier an exception Construction error is raised."]
        #[cxx_name = "RemoveKnot"]
        fn remove_knot(self: Pin<&mut BSplineCurve>, Index: i32, M: i32, Tolerance: f64) -> bool;
        #[doc = "Changes the direction of parametrization of <me>. The Knot sequence is modified, the FirstParameter and the LastParameter are not modified. The StartPoint of the initial curve becomes the EndPoint of the reversed curve and the EndPoint of the initial curve becomes the StartPoint of the reversed curve."]
        #[cxx_name = "Reverse"]
        fn reverse(self: Pin<&mut BSplineCurve>);
        #[doc = "Returns the  parameter on the  reversed  curve for the point of parameter U on <me>. returns UFirst + ULast - U"]
        #[cxx_name = "ReversedParameter"]
        fn reversed_parameter(self: &BSplineCurve, U: f64) -> f64;
        #[doc = "Modifies this BSpline curve by segmenting it between U1 and U2. Either of these values can be outside the bounds of the curve, but U2 must be greater than U1. All data structure tables of this BSpline curve are modified, but the knots located between U1 and U2 are retained. The degree of the curve is not modified. Parameter theTolerance defines the possible proximity of the segment boundaries and B-spline knots to treat them as equal. Warnings : Even if <me> is not closed it can become closed after the segmentation for example if U1 or U2 are out of the bounds of the curve <me> or if the curve makes loop. After the segmentation the length of a curve can be null. raises if U2 < U1. Standard_DomainError if U2 - U1 exceeds the period for periodic curves. i.e. ((U2 - U1) - Period) > Precision::PConfusion()."]
        #[cxx_name = "Segment"]
        fn segment(self: Pin<&mut BSplineCurve>, U1: f64, U2: f64, theTolerance: f64);
        #[doc = "Modifies this BSpline curve by assigning the value K to the knot of index Index in the knots table. This is a relatively local modification because K must be such that: Knots(Index - 1) < K < Knots(Index + 1) The second syntax allows you also to increase the multiplicity of the knot to M (but it is not possible to decrease the multiplicity of the knot with this function). Standard_ConstructionError if: - K is not such that: Knots(Index - 1) < K < Knots(Index + 1) - M is greater than the degree of this BSpline curve or lower than the previous multiplicity of knot of index Index in the knots table. Standard_OutOfRange if Index is outside the bounds of the knots table."]
        #[cxx_name = "SetKnot"]
        fn set_knotint(self: Pin<&mut BSplineCurve>, Index: i32, K: f64);
        #[doc = "Modifies this BSpline curve by assigning the array K to its knots table. The multiplicity of the knots is not modified. Exceptions Standard_ConstructionError if the values in the array K are not in ascending order. Standard_OutOfRange if the bounds of the array K are not respectively 1 and the number of knots of this BSpline curve."]
        #[cxx_name = "SetKnots"]
        fn set_knots(self: Pin<&mut BSplineCurve>, K: &TColStd_Array1OfReal);
        #[doc = "Changes the knot of range Index with its multiplicity. You can increase the multiplicity of a knot but it is not allowed to decrease the multiplicity of an existing knot. Raised if K >= Knots(Index+1) or K <= Knots(Index-1). Raised if M is greater than Degree or lower than the previous multiplicity of knot of range Index. Raised if Index < 1 || Index > NbKnots"]
        #[cxx_name = "SetKnot"]
        fn set_knotint_2(self: Pin<&mut BSplineCurve>, Index: i32, K: f64, M: i32);
        #[doc = "returns the parameter normalized within the period if the curve is periodic : otherwise does not do anything"]
        #[cxx_name = "PeriodicNormalization"]
        fn periodic_normalization(self: &BSplineCurve, U: &mut f64);
        #[doc = "Changes this BSpline curve into a periodic curve. To become periodic, the curve must first be closed. Next, the knot sequence must be periodic. For this, FirstUKnotIndex and LastUKnotIndex are used to compute I1 and I2, the indexes in the knots array of the knots corresponding to the first and last parameters of this BSpline curve. The period is therefore: Knots(I2) - Knots(I1). Consequently, the knots and poles tables are modified. Exceptions Standard_ConstructionError if this BSpline curve is not closed."]
        #[cxx_name = "SetPeriodic"]
        fn set_periodic(self: Pin<&mut BSplineCurve>);
        #[doc = "Assigns the knot of index Index in the knots table as the origin of this periodic BSpline curve. As a consequence, the knots and poles tables are modified. Exceptions Standard_NoSuchObject if this curve is not periodic. Standard_DomainError if Index is outside the bounds of the knots table."]
        #[cxx_name = "SetOrigin"]
        fn set_originint(self: Pin<&mut BSplineCurve>, Index: i32);
        #[doc = "Set the origin of a periodic curve at Knot U. If U is  not a  knot  of  the  BSpline  a  new knot  is inserted. KnotVector and poles are modified. Raised if the curve is not periodic"]
        #[cxx_name = "SetOrigin"]
        fn set_originreal_2(self: Pin<&mut BSplineCurve>, U: f64, Tol: f64);
        #[doc = "Changes this BSpline curve into a non-periodic curve. If this curve is already non-periodic, it is not modified. Note: the poles and knots tables are modified. Warning If this curve is periodic, as the multiplicity of the first and last knots is not modified, and is not equal to Degree + 1, where Degree is the degree of this BSpline curve, the start and end points of the curve are not its first and last poles."]
        #[cxx_name = "SetNotPeriodic"]
        fn set_not_periodic(self: Pin<&mut BSplineCurve>);
        #[doc = "Modifies this BSpline curve by assigning P to the pole of index Index in the poles table. Exceptions Standard_OutOfRange if Index is outside the bounds of the poles table. Standard_ConstructionError if Weight is negative or null."]
        #[cxx_name = "SetPole"]
        fn set_poleint(self: Pin<&mut BSplineCurve>, Index: i32, P: &gp_Pnt);
        #[doc = "Modifies this BSpline curve by assigning P to the pole of index Index in the poles table. This syntax also allows you to modify the weight of the modified pole, which becomes Weight. In this case, if this BSpline curve is non-rational, it can become rational and vice versa. Exceptions Standard_OutOfRange if Index is outside the bounds of the poles table. Standard_ConstructionError if Weight is negative or null."]
        #[cxx_name = "SetPole"]
        fn set_poleint_2(self: Pin<&mut BSplineCurve>, Index: i32, P: &gp_Pnt, Weight: f64);
        #[doc = "Changes the weight for the pole of range Index. If the curve was non rational it can become rational. If the curve was rational it can become non rational. Raised if Index < 1 || Index > NbPoles Raised if Weight <= 0.0"]
        #[cxx_name = "SetWeight"]
        fn set_weight(self: Pin<&mut BSplineCurve>, Index: i32, Weight: f64);
        #[doc = "Moves the point of parameter U of this BSpline curve to P. Index1 and Index2 are the indexes in the table of poles of this BSpline curve of the first and last poles designated to be moved. FirstModifiedPole and LastModifiedPole are the indexes of the first and last poles which are effectively modified. In the event of incompatibility between Index1, Index2 and the value U: - no change is made to this BSpline curve, and - the FirstModifiedPole and LastModifiedPole are returned null. Exceptions Standard_OutOfRange if: - Index1 is greater than or equal to Index2, or - Index1 or Index2 is less than 1 or greater than the number of poles of this BSpline curve."]
        #[cxx_name = "MovePoint"]
        fn move_point(
            self: Pin<&mut BSplineCurve>,
            U: f64,
            P: &gp_Pnt,
            Index1: i32,
            Index2: i32,
            FirstModifiedPole: &mut i32,
            LastModifiedPole: &mut i32,
        );
        #[doc = "Move a point with parameter U to P. and makes it tangent at U be Tangent. StartingCondition = -1 means first can move EndingCondition   = -1 means last point can move StartingCondition = 0 means the first point cannot move EndingCondition   = 0 means the last point cannot move StartingCondition = 1 means the first point and tangent cannot move EndingCondition   = 1 means the last point and tangent cannot move and so forth ErrorStatus != 0 means that there are not enough degree of freedom with the constrain to deform the curve accordingly"]
        #[cxx_name = "MovePointAndTangent"]
        fn move_point_and_tangent(
            self: Pin<&mut BSplineCurve>,
            U: f64,
            P: &gp_Pnt,
            Tangent: &gp_Vec,
            Tolerance: f64,
            StartingCondition: i32,
            EndingCondition: i32,
            ErrorStatus: &mut i32,
        );
        #[doc = "Returns the continuity of the curve, the curve is at least C0. Raised if N < 0."]
        #[cxx_name = "IsCN"]
        fn is_cn(self: &BSplineCurve, N: i32) -> bool;
        #[doc = "Check if curve has at least G1 continuity in interval [theTf, theTl] Returns true if IsCN(1) or angle between \"left\" and \"right\" first derivatives at knots with C0 continuity is less then theAngTol only knots in interval [theTf, theTl] is checked"]
        #[cxx_name = "IsG1"]
        fn is_g1(self: &BSplineCurve, theTf: f64, theTl: f64, theAngTol: f64) -> bool;
        #[doc = "Returns true if the distance between the first point and the last point of the curve is lower or equal to Resolution from package gp. Warnings : The first and the last point can be different from the first pole and the last pole of the curve."]
        #[cxx_name = "IsClosed"]
        fn is_closed(self: &BSplineCurve) -> bool;
        #[doc = "Returns True if the curve is periodic."]
        #[cxx_name = "IsPeriodic"]
        fn is_periodic(self: &BSplineCurve) -> bool;
        #[doc = "Returns True if the weights are not identical. The tolerance criterion is Epsilon of the class Real."]
        #[cxx_name = "IsRational"]
        fn is_rational(self: &BSplineCurve) -> bool;
        #[doc = "Returns the degree of this BSpline curve. The degree of a Geom_BSplineCurve curve cannot be greater than Geom_BSplineCurve::MaxDegree(). Computation of value and derivatives"]
        #[cxx_name = "Degree"]
        fn degree(self: &BSplineCurve) -> i32;
        #[doc = "Returns in P the point of parameter U."]
        #[cxx_name = "D0"]
        fn d0(self: &BSplineCurve, U: f64, P: Pin<&mut gp_Pnt>);
        #[doc = "Raised if the continuity of the curve is not C1."]
        #[cxx_name = "D1"]
        fn d1(self: &BSplineCurve, U: f64, P: Pin<&mut gp_Pnt>, V1: Pin<&mut gp_Vec>);
        #[doc = "Raised if the continuity of the curve is not C2."]
        #[cxx_name = "D2"]
        fn d2(
            self: &BSplineCurve,
            U: f64,
            P: Pin<&mut gp_Pnt>,
            V1: Pin<&mut gp_Vec>,
            V2: Pin<&mut gp_Vec>,
        );
        #[doc = "Raised if the continuity of the curve is not C3."]
        #[cxx_name = "D3"]
        fn d3(
            self: &BSplineCurve,
            U: f64,
            P: Pin<&mut gp_Pnt>,
            V1: Pin<&mut gp_Vec>,
            V2: Pin<&mut gp_Vec>,
            V3: Pin<&mut gp_Vec>,
        );
        #[doc = "Raised if FromK1 = ToK2."]
        #[cxx_name = "LocalD0"]
        fn local_d0(self: &BSplineCurve, U: f64, FromK1: i32, ToK2: i32, P: Pin<&mut gp_Pnt>);
        #[doc = "Raised if the local continuity of the curve is not C1 between the knot K1 and the knot K2. Raised if FromK1 = ToK2."]
        #[cxx_name = "LocalD1"]
        fn local_d1(
            self: &BSplineCurve,
            U: f64,
            FromK1: i32,
            ToK2: i32,
            P: Pin<&mut gp_Pnt>,
            V1: Pin<&mut gp_Vec>,
        );
        #[doc = "Raised if the local continuity of the curve is not C2 between the knot K1 and the knot K2. Raised if FromK1 = ToK2."]
        #[cxx_name = "LocalD2"]
        fn local_d2(
            self: &BSplineCurve,
            U: f64,
            FromK1: i32,
            ToK2: i32,
            P: Pin<&mut gp_Pnt>,
            V1: Pin<&mut gp_Vec>,
            V2: Pin<&mut gp_Vec>,
        );
        #[doc = "Raised if the local continuity of the curve is not C3 between the knot K1 and the knot K2. Raised if FromK1 = ToK2."]
        #[cxx_name = "LocalD3"]
        fn local_d3(
            self: &BSplineCurve,
            U: f64,
            FromK1: i32,
            ToK2: i32,
            P: Pin<&mut gp_Pnt>,
            V1: Pin<&mut gp_Vec>,
            V2: Pin<&mut gp_Vec>,
            V3: Pin<&mut gp_Vec>,
        );
        #[doc = "Returns the index in the knot array of the knot corresponding to the first or last parameter of this BSpline curve. For a BSpline curve, the first (or last) parameter (which gives the start (or end) point of the curve) is a knot value. However, if the multiplicity of the first (or last) knot is less than Degree + 1, where Degree is the degree of the curve, it is not the first (or last) knot of the curve."]
        #[cxx_name = "FirstUKnotIndex"]
        fn first_u_knot_index(self: &BSplineCurve) -> i32;
        #[doc = "Returns the value of the first parameter of this BSpline curve. This is a knot value. The first parameter is the one of the start point of the BSpline curve."]
        #[cxx_name = "FirstParameter"]
        fn first_parameter(self: &BSplineCurve) -> f64;
        #[doc = "Returns the knot of range Index. When there is a knot with a multiplicity greater than 1 the knot is not repeated. The method Multiplicity can be used to get the multiplicity of the Knot. Raised if Index < 1 or Index > NbKnots"]
        #[cxx_name = "Knot"]
        fn knot(self: &BSplineCurve, Index: i32) -> f64;
        #[doc = "returns the knot values of the B-spline curve; Warning A knot with a multiplicity greater than 1 is not repeated in the knot table. The Multiplicity function can be used to obtain the multiplicity of each knot. Raised K.Lower() is less than number of first knot or K.Upper() is more than number of last knot."]
        #[cxx_name = "Knots"]
        fn knotsarray1ofreal(self: &BSplineCurve, K: Pin<&mut TColStd_Array1OfReal>);
        #[doc = "returns the knot values of the B-spline curve; Warning A knot with a multiplicity greater than 1 is not repeated in the knot table. The Multiplicity function can be used to obtain the multiplicity of each knot."]
        #[cxx_name = "Knots"]
        fn knots2(self: &BSplineCurve) -> &TColStd_Array1OfReal;
        #[doc = "Returns K, the knots sequence of this BSpline curve. In this sequence, knots with a multiplicity greater than 1 are repeated. In the case of a non-periodic curve the length of the sequence must be equal to the sum of the NbKnots multiplicities of the knots of the curve (where NbKnots is the number of knots of this BSpline curve). This sum is also equal to : NbPoles + Degree + 1 where NbPoles is the number of poles and Degree the degree of this BSpline curve. In the case of a periodic curve, if there are k periodic knots, the period is Knot(k+1) - Knot(1). The initial sequence is built by writing knots 1 to k+1, which are repeated according to their corresponding multiplicities. If Degree is the degree of the curve, the degree of continuity of the curve at the knot of index 1 (or k+1) is equal to c = Degree + 1 - Mult(1). c knots are then inserted at the beginning and end of the initial sequence: - the c values of knots preceding the first item Knot(k+1) in the initial sequence are inserted at the beginning; the period is subtracted from these c values; - the c values of knots following the last item Knot(1) in the initial sequence are inserted at the end; the period is added to these c values. The length of the sequence must therefore be equal to: NbPoles + 2*Degree - Mult(1) + 2. Example For a non-periodic BSpline curve of degree 2 where: - the array of knots is: { k1 k2 k3 k4 }, - with associated multiplicities: { 3 1 2 3 }, the knot sequence is: K = { k1 k1 k1 k2 k3 k3 k4 k4 k4 } For a periodic BSpline curve of degree 4 , which is \"C1\" continuous at the first knot, and where : - the periodic knots are: { k1 k2 k3 (k4) } (3 periodic knots: the points of parameter k1 and k4 are identical, the period is p = k4 - k1), - with associated multiplicities: { 3 1 2 (3) }, the degree of continuity at knots k1 and k4 is: Degree + 1 - Mult(i) = 2. 2 supplementary knots are added at the beginning and end of the sequence: - at the beginning: the 2 knots preceding k4 minus the period; in this example, this is k3 - p both times; - at the end: the 2 knots following k1 plus the period; in this example, this is k2 + p and k3 + p. The knot sequence is therefore: K = { k3-p k3-p k1 k1 k1 k2 k3 k3 k4 k4 k4 k2+p k3+p } Exceptions Raised if K.Lower() is less than number of first knot in knot sequence with repetitions or K.Upper() is more than number of last knot in knot sequence with repetitions."]
        #[cxx_name = "KnotSequence"]
        fn knot_sequencearray1ofreal(self: &BSplineCurve, K: Pin<&mut TColStd_Array1OfReal>);
        #[doc = "returns the knots of the B-spline curve. Knots with multiplicit greater than 1 are repeated"]
        #[cxx_name = "KnotSequence"]
        fn knot_sequence2(self: &BSplineCurve) -> &TColStd_Array1OfReal;
        #[doc = "For a BSpline curve the last parameter (which gives the end point of the curve) is a knot value but if the multiplicity of the last knot index is lower than Degree + 1 it is not the last knot of the curve. This method computes the index of the knot corresponding to the last parameter."]
        #[cxx_name = "LastUKnotIndex"]
        fn last_u_knot_index(self: &BSplineCurve) -> i32;
        #[doc = "Computes the parametric value of the end point of the curve. It is a knot value."]
        #[cxx_name = "LastParameter"]
        fn last_parameter(self: &BSplineCurve) -> f64;
        #[doc = "Locates the parametric value U in the sequence of knots. If \"WithKnotRepetition\" is True we consider the knot's representation with repetition of multiple knot value, otherwise  we consider the knot's representation with no repetition of multiple knot values. Knots (I1) <= U <= Knots (I2) . if I1 = I2  U is a knot value (the tolerance criterion ParametricTolerance is used). . if I1 < 1  => U < Knots (1) - Abs(ParametricTolerance) . if I2 > NbKnots => U > Knots (NbKnots) + Abs(ParametricTolerance)"]
        #[cxx_name = "LocateU"]
        fn locate_u(
            self: &BSplineCurve,
            U: f64,
            ParametricTolerance: f64,
            I1: &mut i32,
            I2: &mut i32,
            WithKnotRepetition: bool,
        );
        #[doc = "Returns the multiplicity of the knots of range Index. Raised if Index < 1 or Index > NbKnots"]
        #[cxx_name = "Multiplicity"]
        fn multiplicity(self: &BSplineCurve, Index: i32) -> i32;
        #[doc = "Returns the multiplicity of the knots of the curve. Raised if the length of M is not equal to NbKnots."]
        #[cxx_name = "Multiplicities"]
        fn multiplicitiesarray1ofinteger(self: &BSplineCurve, M: Pin<&mut TColStd_Array1OfInteger>);
        #[doc = "returns the multiplicity of the knots of the curve."]
        #[cxx_name = "Multiplicities"]
        fn multiplicities2(self: &BSplineCurve) -> &TColStd_Array1OfInteger;
        #[doc = "Returns the number of knots. This method returns the number of knot without repetition of multiple knots."]
        #[cxx_name = "NbKnots"]
        fn nb_knots(self: &BSplineCurve) -> i32;
        #[doc = "Returns the number of poles"]
        #[cxx_name = "NbPoles"]
        fn nb_poles(self: &BSplineCurve) -> i32;
        #[doc = "Returns the pole of range Index. Raised if Index < 1 or Index > NbPoles."]
        #[cxx_name = "Pole"]
        fn pole(self: &BSplineCurve, Index: i32) -> &gp_Pnt;
        #[doc = "Returns the poles of the B-spline curve; Raised if the length of P is not equal to the number of poles."]
        #[cxx_name = "Poles"]
        fn polesarray1ofpnt(self: &BSplineCurve, P: Pin<&mut TColgp_Array1OfPnt>);
        #[doc = "Returns the poles of the B-spline curve;"]
        #[cxx_name = "Poles"]
        fn poles2(self: &BSplineCurve) -> &TColgp_Array1OfPnt;
        #[doc = "Returns the weight of the pole of range Index . Raised if Index < 1 or Index > NbPoles."]
        #[cxx_name = "Weight"]
        fn weight(self: &BSplineCurve, Index: i32) -> f64;
        #[doc = "Returns the weights of the B-spline curve; Raised if the length of W is not equal to NbPoles."]
        #[cxx_name = "Weights"]
        fn weightsarray1ofreal(self: &BSplineCurve, W: Pin<&mut TColStd_Array1OfReal>);
        #[doc = "Applies the transformation T to this BSpline curve."]
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut BSplineCurve>, T: &gp_Trsf);
        #[doc = "Computes for this BSpline curve the parametric tolerance UTolerance for a given 3D tolerance Tolerance3D. If f(t) is the equation of this BSpline curve, UTolerance ensures that: | t1 - t0| < Utolerance ===> |f(t1) - f(t0)| < Tolerance3D"]
        #[cxx_name = "Resolution"]
        fn resolution(self: Pin<&mut BSplineCurve>, Tolerance3D: f64, UTolerance: &mut f64);
        #[doc = "Compare two Bspline curve on identity;"]
        #[cxx_name = "IsEqual"]
        fn is_equal(self: &BSplineCurve, theOther: &HandleGeomBSplineCurve, thePreci: f64) -> bool;
        #[cxx_name = "DynamicType"]
        fn dynamic_type(self: &BSplineCurve) -> &HandleStandardType;
        #[doc = "Returns the global continuity of the curve : C0 : only geometric continuity, C1 : continuity of the first derivative all along the Curve, C2 : continuity of the second derivative all along the Curve, C3 : continuity of the third derivative all along the Curve, CN : the order of continuity is infinite. For a B-spline curve of degree d if a knot Ui has a multiplicity p the B-spline curve is only Cd-p continuous at Ui. So the global continuity of the curve can't be greater than Cd-p where p is the maximum multiplicity of the interior Knots. In the interior of a knot span the curve is infinitely continuously differentiable."]
        #[cxx_name = "Geom_BSplineCurve_Continuity"]
        fn BSplineCurve_continuity(self_: &BSplineCurve) -> UniquePtr<GeomAbs_Shape>;
        #[doc = "For the point of parameter U of this BSpline curve, computes the vector corresponding to the Nth derivative. Warning On a point where the continuity of the curve is not the one requested, this function impacts the part defined by the parameter with a value greater than U, i.e. the part of the curve to the \"right\" of the singularity. Exceptions Standard_RangeError if N is less than 1. The following functions compute the point of parameter U and the derivatives at this point on the B-spline curve arc defined between the knot FromK1 and the knot ToK2. U can be out of bounds [Knot (FromK1),  Knot (ToK2)] but for the computation we only use the definition of the curve between these two knots. This method is useful to compute local derivative, if the order of continuity of the whole curve is not greater enough.    Inside the parametric domain Knot (FromK1), Knot (ToK2) the evaluations are the same as if we consider the whole definition of the curve. Of course the evaluations are different outside this parametric domain."]
        #[cxx_name = "Geom_BSplineCurve_DN"]
        fn BSplineCurve_dn(self_: &BSplineCurve, U: f64, N: i32) -> UniquePtr<gp_Vec>;
        #[doc = "Raised if FromK1 = ToK2."]
        #[cxx_name = "Geom_BSplineCurve_LocalValue"]
        fn BSplineCurve_local_value(
            self_: &BSplineCurve,
            U: f64,
            FromK1: i32,
            ToK2: i32,
        ) -> UniquePtr<gp_Pnt>;
        #[doc = "Raised if the local continuity of the curve is not CN between the knot K1 and the knot K2. Raised if FromK1 = ToK2. Raised if N < 1."]
        #[cxx_name = "Geom_BSplineCurve_LocalDN"]
        fn BSplineCurve_local_dn(
            self_: &BSplineCurve,
            U: f64,
            FromK1: i32,
            ToK2: i32,
            N: i32,
        ) -> UniquePtr<gp_Vec>;
        #[doc = "Returns the last point of the curve. Warnings : The last point of the curve is different from the last pole of the curve if the multiplicity of the last knot is lower than Degree."]
        #[cxx_name = "Geom_BSplineCurve_EndPoint"]
        fn BSplineCurve_end_point(self_: &BSplineCurve) -> UniquePtr<gp_Pnt>;
        #[doc = "Returns NonUniform or Uniform or QuasiUniform or PiecewiseBezier. If all the knots differ by a positive constant from the preceding knot the BSpline Curve can be : - Uniform if all the knots are of multiplicity 1, - QuasiUniform if all the knots are of multiplicity 1 except for the first and last knot which are of multiplicity Degree + 1, - PiecewiseBezier if the first and last knots have multiplicity Degree + 1 and if interior knots have multiplicity Degree A piecewise Bezier with only two knots is a BezierCurve. else the curve is non uniform. The tolerance criterion is Epsilon from class Real."]
        #[cxx_name = "Geom_BSplineCurve_KnotDistribution"]
        fn BSplineCurve_knot_distribution(
            self_: &BSplineCurve,
        ) -> UniquePtr<GeomAbs_BSplKnotDistribution>;
        #[doc = "Returns the start point of the curve. Warnings : This point is different from the first pole of the curve if the multiplicity of the first knot is lower than Degree."]
        #[cxx_name = "Geom_BSplineCurve_StartPoint"]
        fn BSplineCurve_start_point(self_: &BSplineCurve) -> UniquePtr<gp_Pnt>;
        #[doc = "Creates a new object which is a copy of this BSpline curve."]
        #[cxx_name = "Geom_BSplineCurve_Copy"]
        fn BSplineCurve_copy(self_: &BSplineCurve) -> UniquePtr<HandleGeomGeometry>;
        #[doc = "Returns the value of the maximum degree of the normalized B-spline basis functions in this package."]
        #[cxx_name = "Geom_BSplineCurve_MaxDegree"]
        fn BSplineCurve_max_degree() -> i32;
        #[cxx_name = "Geom_BSplineCurve_get_type_name"]
        fn BSplineCurve_get_type_name() -> String;
        #[doc = " ======================== Geom_Curve ========================"]
        #[doc = "The abstract class Curve describes the common behavior of curves in 3D space. The Geom package provides numerous concrete classes of derived curves, including lines, circles, conics, Bezier or BSpline curves, etc. The main characteristic of these curves is that they are parameterized. The Geom_Curve class shows: - how to work with the parametric equation of a curve in order to calculate the point of parameter u, together with the vector tangent and the derivative vectors of order 2, 3,..., N at this point; - how to obtain general information about the curve (for example, level of continuity, closed characteristics, periodicity, bounds of the parameter field); - how the parameter changes when a geometric transformation is applied to the curve or when the orientation of the curve is inverted. All curves must have a geometric continuity: a curve is at least \"C0\". Generally, this property is checked at the time of construction or when the curve is edited. Where this is not the case, the documentation states so explicitly. Warning The Geom package does not prevent the construction of curves with null length or curves which self-intersect."]
        #[cxx_name = "Geom_Curve"]
        type Curve;
        #[doc = "Changes the direction of parametrization of <me>. The \"FirstParameter\" and the \"LastParameter\" are not changed but the orientation  of the curve is modified. If the curve is bounded the StartPoint of the initial curve becomes the EndPoint of the reversed curve  and the EndPoint of the initial curve becomes the StartPoint of the reversed curve."]
        #[cxx_name = "Reverse"]
        fn reverse(self: Pin<&mut Curve>);
        #[doc = "Returns the  parameter on the  reversed  curve for the point of parameter U on <me>. me->Reversed()->Value(me->ReversedParameter(U)) is the same point as me->Value(U)"]
        #[cxx_name = "ReversedParameter"]
        fn reversed_parameter(self: &Curve, U: f64) -> f64;
        #[doc = "Returns the  parameter on the  transformed  curve for the transform of the point of parameter U on <me>. me->Transformed(T)->Value(me->TransformedParameter(U,T)) is the same point as me->Value(U).Transformed(T) This methods returns <U> It can be redefined. For example on the Line."]
        #[cxx_name = "TransformedParameter"]
        fn transformed_parameter(self: &Curve, U: f64, T: &gp_Trsf) -> f64;
        #[doc = "Returns a  coefficient to compute the parameter on the transformed  curve  for  the transform  of the point on <me>. Transformed(T)->Value(U * ParametricTransformation(T)) is the same point as Value(U).Transformed(T) This methods returns 1. It can be redefined. For example on the Line."]
        #[cxx_name = "ParametricTransformation"]
        fn parametric_transformation(self: &Curve, T: &gp_Trsf) -> f64;
        #[doc = "Returns the value of the first parameter. Warnings : It can be RealFirst from package Standard if the curve is infinite"]
        #[cxx_name = "FirstParameter"]
        fn first_parameter(self: &Curve) -> f64;
        #[doc = "Returns the value of the last parameter. Warnings : It can be RealLast from package Standard if the curve is infinite"]
        #[cxx_name = "LastParameter"]
        fn last_parameter(self: &Curve) -> f64;
        #[doc = "Returns true if the curve is closed. Some curves such as circle are always closed, others such as line are never closed (by definition). Some Curves such as OffsetCurve can be closed or not. These curves are considered as closed if the distance between the first point and the last point of the curve is lower or equal to the Resolution from package gp which is a fixed criterion independent of the application."]
        #[cxx_name = "IsClosed"]
        fn is_closed(self: &Curve) -> bool;
        #[doc = "Is the parametrization of the curve periodic ? It is possible only if the curve is closed and if the following relation is satisfied : for each parametric value U the distance between the point P(u) and the point P (u + T) is lower or equal to Resolution from package gp, T is the period and must be a constant. There are three possibilities : . the curve is never periodic by definition (SegmentLine) . the curve is always periodic by definition (Circle) . the curve can be defined as periodic (BSpline). In this case a function SetPeriodic allows you to give the shape of the curve.  The general rule for this case is : if a curve can be periodic or not the default periodicity set is non periodic and you have to turn (explicitly) the curve into a periodic curve  if you want the curve to be periodic."]
        #[cxx_name = "IsPeriodic"]
        fn is_periodic(self: &Curve) -> bool;
        #[doc = "Returns the period of this curve. Exceptions Standard_NoSuchObject if this curve is not periodic."]
        #[cxx_name = "Period"]
        fn period(self: &Curve) -> f64;
        #[doc = "Returns true if the degree of continuity of this curve is at least N. Exceptions -  Standard_RangeError if N is less than 0."]
        #[cxx_name = "IsCN"]
        fn is_cn(self: &Curve, N: i32) -> bool;
        #[doc = "Returns in P the point of parameter U. If the curve is periodic  then the returned point is P(U) with U = Ustart + (U - Uend)  where Ustart and Uend are the parametric bounds of the curve. Raised only for the \"OffsetCurve\" if it is not possible to compute the current point. For example when the first derivative on the basis curve and the offset direction are parallel."]
        #[cxx_name = "D0"]
        fn d0(self: &Curve, U: f64, P: Pin<&mut gp_Pnt>);
        #[doc = "Returns the point P of parameter U and the first derivative V1. Raised if the continuity of the curve is not C1."]
        #[cxx_name = "D1"]
        fn d1(self: &Curve, U: f64, P: Pin<&mut gp_Pnt>, V1: Pin<&mut gp_Vec>);
        #[doc = "Returns the point P of parameter U, the first and second derivatives V1 and V2. Raised if the continuity of the curve is not C2."]
        #[cxx_name = "D2"]
        fn d2(
            self: &Curve,
            U: f64,
            P: Pin<&mut gp_Pnt>,
            V1: Pin<&mut gp_Vec>,
            V2: Pin<&mut gp_Vec>,
        );
        #[doc = "Returns the point P of parameter U, the first, the second and the third derivative. Raised if the continuity of the curve is not C3."]
        #[cxx_name = "D3"]
        fn d3(
            self: &Curve,
            U: f64,
            P: Pin<&mut gp_Pnt>,
            V1: Pin<&mut gp_Vec>,
            V2: Pin<&mut gp_Vec>,
            V3: Pin<&mut gp_Vec>,
        );
        #[cxx_name = "DynamicType"]
        fn dynamic_type(self: &Curve) -> &HandleStandardType;
        #[doc = "Returns a copy of <me> reversed."]
        #[cxx_name = "Geom_Curve_Reversed"]
        fn Curve_reversed(self_: &Curve) -> UniquePtr<HandleGeomCurve>;
        #[doc = "It is the global continuity of the curve C0 : only geometric continuity, C1 : continuity of the first derivative all along the Curve, C2 : continuity of the second derivative all along the Curve, C3 : continuity of the third derivative all along the Curve, G1 : tangency continuity all along the Curve, G2 : curvature continuity all along the Curve, CN : the order of continuity is infinite."]
        #[cxx_name = "Geom_Curve_Continuity"]
        fn Curve_continuity(self_: &Curve) -> UniquePtr<GeomAbs_Shape>;
        #[doc = "The returned vector gives the value of the derivative for the order of derivation N. Raised if the continuity of the curve is not CN. Raised if the   derivative  cannot  be  computed easily. e.g. rational bspline and n > 3. Raised if N < 1."]
        #[cxx_name = "Geom_Curve_DN"]
        fn Curve_dn(self_: &Curve, U: f64, N: i32) -> UniquePtr<gp_Vec>;
        #[doc = "Computes the point of parameter U on <me>. If the curve is periodic  then the returned point is P(U) with U = Ustart + (U - Uend)  where Ustart and Uend are the parametric bounds of the curve. it is implemented with D0. Raised only for the \"OffsetCurve\" if it is not possible to compute the current point. For example when the first derivative on the basis curve and the offset direction are parallel."]
        #[cxx_name = "Geom_Curve_Value"]
        fn Curve_value(self_: &Curve, U: f64) -> UniquePtr<gp_Pnt>;
        #[cxx_name = "Geom_Curve_get_type_name"]
        fn Curve_get_type_name() -> String;
        #[doc = " ======================== Geom_CylindricalSurface ========================"]
        #[doc = "This class defines the infinite cylindrical surface. Every cylindrical surface is set by the following equation: @code S(U,V) = Location + R*cos(U)*XAxis + R*sin(U)*YAxis + V*ZAxis, @endcode where R is cylinder radius. The local coordinate system of the CylindricalSurface is defined with an axis placement (see class ElementarySurface). The \"ZAxis\" is the symmetry axis of the CylindricalSurface, it gives the direction of increasing parametric value V. The parametrization range is : @code U [0, 2*PI],  V ]- infinite, + infinite[ @endcode The \"XAxis\" and the \"YAxis\" define the placement plane of the surface (Z = 0, and parametric value V = 0)  perpendicular to the symmetry axis. The \"XAxis\" defines the origin of the parameter U = 0.  The trigonometric sense gives the positive orientation for the parameter U. When you create a CylindricalSurface the U and V directions of parametrization are such that at each point of the surface the normal is oriented towards the \"outside region\". The methods UReverse VReverse change the orientation of the surface."]
        #[cxx_name = "Geom_CylindricalSurface"]
        type CylindricalSurface;
        #[doc = "A3 defines the local coordinate system of the cylindrical surface. The \"ZDirection\" of A3 defines the direction of the surface's axis of symmetry. At the creation the parametrization of the surface is defined such that the normal Vector (N = D1U ^ D1V) is oriented towards the \"outside region\" of the surface. Warnings: It is not forbidden to create a cylindrical surface with Radius = 0.0 Raised if Radius < 0.0"]
        #[cxx_name = "Geom_CylindricalSurface_ctor_ax3_real"]
        fn CylindricalSurface_ctor_ax3_real(
            A3: &gp_Ax3,
            Radius: f64,
        ) -> UniquePtr<CylindricalSurface>;
        #[doc = "Creates a CylindricalSurface from a non transient gp_Cylinder."]
        #[cxx_name = "Geom_CylindricalSurface_ctor_cylinder"]
        fn CylindricalSurface_ctor_cylinder(C: &gp_Cylinder) -> UniquePtr<CylindricalSurface>;
        #[doc = "Set <me> so that <me> has the same geometric properties as C."]
        #[cxx_name = "SetCylinder"]
        fn set_cylinder(self: Pin<&mut CylindricalSurface>, C: &gp_Cylinder);
        #[doc = "Changes the radius of the cylinder. Raised if R < 0.0"]
        #[cxx_name = "SetRadius"]
        fn set_radius(self: Pin<&mut CylindricalSurface>, R: f64);
        #[doc = "Return the  parameter on the  Ureversed surface for the point of parameter U on <me>. Return 2.PI - U."]
        #[cxx_name = "UReversedParameter"]
        fn u_reversed_parameter(self: &CylindricalSurface, U: f64) -> f64;
        #[doc = "Return the  parameter on the  Vreversed surface for the point of parameter V on <me>. Return -V"]
        #[cxx_name = "VReversedParameter"]
        fn v_reversed_parameter(self: &CylindricalSurface, V: f64) -> f64;
        #[doc = "Computes the  parameters on the  transformed  surface for the transform of the point of parameters U,V on <me>. @code me->Transformed(T)->Value(U',V') @endcode is the same point as @code me->Value(U,V).Transformed(T) @endcode Where U',V' are the new values of U,V after calling @code me->TransformParameters(U,V,T) @endcode This method multiplies V by T.ScaleFactor()"]
        #[cxx_name = "TransformParameters"]
        fn transform_parameters(self: &CylindricalSurface, U: &mut f64, V: &mut f64, T: &gp_Trsf);
        #[doc = "The CylindricalSurface is infinite in the V direction so V1 = Realfirst, V2 = RealLast from package Standard. U1 = 0 and U2 = 2*PI."]
        #[cxx_name = "Bounds"]
        fn bounds(
            self: &CylindricalSurface,
            U1: &mut f64,
            U2: &mut f64,
            V1: &mut f64,
            V2: &mut f64,
        );
        #[doc = "Returns the coefficients of the implicit equation of the quadric in the absolute cartesian coordinate system : These coefficients are normalized. @code A1.X**2 + A2.Y**2 + A3.Z**2 + 2.(B1.X.Y + B2.X.Z + B3.Y.Z) + 2.(C1.X + C2.Y + C3.Z) + D = 0.0 @endcode"]
        #[cxx_name = "Coefficients"]
        fn coefficients(
            self: &CylindricalSurface,
            A1: &mut f64,
            A2: &mut f64,
            A3: &mut f64,
            B1: &mut f64,
            B2: &mut f64,
            B3: &mut f64,
            C1: &mut f64,
            C2: &mut f64,
            C3: &mut f64,
            D: &mut f64,
        );
        #[doc = "Returns the radius of this cylinder."]
        #[cxx_name = "Radius"]
        fn radius(self: &CylindricalSurface) -> f64;
        #[doc = "Returns True."]
        #[cxx_name = "IsUClosed"]
        fn is_u_closed(self: &CylindricalSurface) -> bool;
        #[doc = "Returns False."]
        #[cxx_name = "IsVClosed"]
        fn is_v_closed(self: &CylindricalSurface) -> bool;
        #[doc = "Returns True."]
        #[cxx_name = "IsUPeriodic"]
        fn is_u_periodic(self: &CylindricalSurface) -> bool;
        #[doc = "Returns False."]
        #[cxx_name = "IsVPeriodic"]
        fn is_v_periodic(self: &CylindricalSurface) -> bool;
        #[doc = "Computes the  point P (U, V) on the surface. P (U, V) = Loc + Radius * (cos (U) * XDir + sin (U) * YDir) + V * ZDir where Loc is the origin of the placement plane (XAxis, YAxis) XDir is the direction of the XAxis and YDir the direction of the YAxis."]
        #[cxx_name = "D0"]
        fn d0(self: &CylindricalSurface, U: f64, V: f64, P: Pin<&mut gp_Pnt>);
        #[doc = "Computes the current point and the first derivatives in the directions U and V."]
        #[cxx_name = "D1"]
        fn d1(
            self: &CylindricalSurface,
            U: f64,
            V: f64,
            P: Pin<&mut gp_Pnt>,
            D1U: Pin<&mut gp_Vec>,
            D1V: Pin<&mut gp_Vec>,
        );
        #[doc = "Computes the current point, the first and the second derivatives in the directions U and V."]
        #[cxx_name = "D2"]
        fn d2(
            self: &CylindricalSurface,
            U: f64,
            V: f64,
            P: Pin<&mut gp_Pnt>,
            D1U: Pin<&mut gp_Vec>,
            D1V: Pin<&mut gp_Vec>,
            D2U: Pin<&mut gp_Vec>,
            D2V: Pin<&mut gp_Vec>,
            D2UV: Pin<&mut gp_Vec>,
        );
        #[doc = "Computes the current point, the first, the second and the third   derivatives in the directions U and V."]
        #[cxx_name = "D3"]
        fn d3(
            self: &CylindricalSurface,
            U: f64,
            V: f64,
            P: Pin<&mut gp_Pnt>,
            D1U: Pin<&mut gp_Vec>,
            D1V: Pin<&mut gp_Vec>,
            D2U: Pin<&mut gp_Vec>,
            D2V: Pin<&mut gp_Vec>,
            D2UV: Pin<&mut gp_Vec>,
            D3U: Pin<&mut gp_Vec>,
            D3V: Pin<&mut gp_Vec>,
            D3UUV: Pin<&mut gp_Vec>,
            D3UVV: Pin<&mut gp_Vec>,
        );
        #[doc = "Applies the transformation T to this cylinder."]
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut CylindricalSurface>, T: &gp_Trsf);
        #[cxx_name = "DynamicType"]
        fn dynamic_type(self: &CylindricalSurface) -> &HandleStandardType;
        #[doc = "returns a non transient cylinder with the same geometric properties as <me>."]
        #[cxx_name = "Geom_CylindricalSurface_Cylinder"]
        fn CylindricalSurface_cylinder(self_: &CylindricalSurface) -> UniquePtr<gp_Cylinder>;
        #[doc = "Returns a 2d transformation used to find the new parameters of a point on the transformed surface. @code me->Transformed(T)->Value(U',V') @endcode is the same point as @code me->Value(U,V).Transformed(T) @endcode Where U',V' are obtained by transforming U,V with the 2d transformation returned by @code me->ParametricTransformation(T) @endcode This method returns a scale centered on the U axis with T.ScaleFactor"]
        #[cxx_name = "Geom_CylindricalSurface_ParametricTransformation"]
        fn CylindricalSurface_parametric_transformation(
            self_: &CylindricalSurface,
            T: &gp_Trsf,
        ) -> UniquePtr<gp_GTrsf2d>;
        #[doc = "The UIso curve is a Line. The location point of this line is on the placement plane (XAxis, YAxis) of the surface. This line is parallel to the axis of symmetry of the surface."]
        #[cxx_name = "Geom_CylindricalSurface_UIso"]
        fn CylindricalSurface_u_iso(
            self_: &CylindricalSurface,
            U: f64,
        ) -> UniquePtr<HandleGeomCurve>;
        #[doc = "The VIso curve is a circle. The start point of this circle (U = 0) is defined with the \"XAxis\" of the surface. The center of the circle is on the symmetry axis."]
        #[cxx_name = "Geom_CylindricalSurface_VIso"]
        fn CylindricalSurface_v_iso(
            self_: &CylindricalSurface,
            V: f64,
        ) -> UniquePtr<HandleGeomCurve>;
        #[doc = "Computes the derivative of order Nu in the direction u and Nv in the direction v. Raised if Nu + Nv < 1 or Nu < 0 or Nv < 0."]
        #[cxx_name = "Geom_CylindricalSurface_DN"]
        fn CylindricalSurface_dn(
            self_: &CylindricalSurface,
            U: f64,
            V: f64,
            Nu: i32,
            Nv: i32,
        ) -> UniquePtr<gp_Vec>;
        #[doc = "Creates a new object which is a copy of this cylinder."]
        #[cxx_name = "Geom_CylindricalSurface_Copy"]
        fn CylindricalSurface_copy(self_: &CylindricalSurface) -> UniquePtr<HandleGeomGeometry>;
        #[cxx_name = "Geom_CylindricalSurface_get_type_name"]
        fn CylindricalSurface_get_type_name() -> String;
        #[doc = " ======================== Geom_Plane ========================"]
        #[doc = "Describes a plane in 3D space. A plane is positioned in space by a coordinate system (a gp_Ax3 object) such that the plane is defined by the origin, \"X Direction\" and \"Y Direction\" of this coordinate system. This coordinate system is the \"local coordinate system\" of the plane. The following apply: - Its \"X Direction\" and \"Y Direction\" are respectively the u and v parametric directions of the plane. - Its origin is the origin of the u and v parameters (also called the \"origin\" of the plane). - Its \"main Direction\" is a vector normal to the plane. This normal vector gives the orientation of the plane only if the local coordinate system is \"direct\". (The orientation of the plane is always defined by the \"X Direction\" and the \"Y Direction\" of its local coordinate system.) The parametric equation of the plane is: @code P(u, v) = O + u*XDir + v*YDir @endcode where O, XDir and YDir are respectively the origin, the \"X Direction\" and the \"Y Direction\" of the local coordinate system of the plane. The parametric range of the two parameters u and v is ] -infinity, +infinity [."]
        #[cxx_name = "Geom_Plane"]
        type Plane;
        #[doc = "Creates a plane located in 3D space with an axis placement three axis. The \"ZDirection\" of \"A3\" is the direction normal to the plane.  The \"Location\" point of \"A3\" is the origin of the plane. The \"XDirection\" and \"YDirection\" of \"A3\" define the directions of the U isoparametric and V isoparametric curves."]
        #[cxx_name = "Geom_Plane_ctor_ax3"]
        fn Plane_ctor_ax3(A3: &gp_Ax3) -> UniquePtr<Plane>;
        #[doc = "Creates a plane from a non transient plane from package gp."]
        #[cxx_name = "Geom_Plane_ctor_pln"]
        fn Plane_ctor_pln(Pl: &gp_Pln) -> UniquePtr<Plane>;
        #[doc = "P is the \"Location\" point or origin of the plane. V is the direction normal to the plane."]
        #[cxx_name = "Geom_Plane_ctor_pnt_dir"]
        fn Plane_ctor_pnt_dir(P: &gp_Pnt, V: &gp_Dir) -> UniquePtr<Plane>;
        #[doc = "Creates a plane from its cartesian equation: @code Ax + By + Cz + D = 0.0 @endcode Raised if Sqrt (A*A + B*B + C*C) <= Resolution from gp"]
        #[cxx_name = "Geom_Plane_ctor_real4"]
        fn Plane_ctor_real4(A: f64, B: f64, C: f64, D: f64) -> UniquePtr<Plane>;
        #[doc = "Set <me> so that <me> has the same geometric properties as Pl."]
        #[cxx_name = "SetPln"]
        fn set_pln(self: Pin<&mut Plane>, Pl: &gp_Pln);
        #[doc = "Changes the orientation of this plane in the u (or v) parametric direction. The bounds of the plane are not changed but the given parametric direction is reversed. Hence the orientation of the surface is reversed."]
        #[cxx_name = "UReverse"]
        fn u_reverse(self: Pin<&mut Plane>);
        #[doc = "Computes the u  parameter on the modified plane, produced when reversing the u parametric of this plane, for any point of u parameter U on this plane. In the case of a plane, these methods return - -U."]
        #[cxx_name = "UReversedParameter"]
        fn u_reversed_parameter(self: &Plane, U: f64) -> f64;
        #[doc = "Changes the orientation of this plane in the u (or v) parametric direction. The bounds of the plane are not changed but the given parametric direction is reversed. Hence the orientation of the surface is reversed."]
        #[cxx_name = "VReverse"]
        fn v_reverse(self: Pin<&mut Plane>);
        #[doc = "Computes the v parameter on the modified plane, produced when reversing the v parametric of this plane, for any point of v parameter V on this plane. In the case of a plane, these methods return -V."]
        #[cxx_name = "VReversedParameter"]
        fn v_reversed_parameter(self: &Plane, V: f64) -> f64;
        #[doc = "Computes the parameters on the transformed surface for the transform of the point of parameters U,V on <me>. @code me->Transformed(T)->Value(U',V') @endcode is the same point as @code me->Value(U,V).Transformed(T) @endcode Where U',V' are the new values of U,V after calling @code me->TransformParameters(U,V,T) @endcode This method multiplies U and V by T.ScaleFactor()"]
        #[cxx_name = "TransformParameters"]
        fn transform_parameters(self: &Plane, U: &mut f64, V: &mut f64, T: &gp_Trsf);
        #[doc = "Returns the parametric bounds U1, U2, V1 and V2 of this plane. Because a plane is an infinite surface, the following is always true: - U1 = V1 =   Standard_Real::RealFirst() - U2 = V2 =   Standard_Real::RealLast()."]
        #[cxx_name = "Bounds"]
        fn bounds(self: &Plane, U1: &mut f64, U2: &mut f64, V1: &mut f64, V2: &mut f64);
        #[doc = "Computes the normalized coefficients of the plane's cartesian equation: @code Ax + By + Cz + D = 0.0 @endcode"]
        #[cxx_name = "Coefficients"]
        fn coefficients(self: &Plane, A: &mut f64, B: &mut f64, C: &mut f64, D: &mut f64);
        #[doc = "return False"]
        #[cxx_name = "IsUClosed"]
        fn is_u_closed(self: &Plane) -> bool;
        #[doc = "return False"]
        #[cxx_name = "IsVClosed"]
        fn is_v_closed(self: &Plane) -> bool;
        #[doc = "return False."]
        #[cxx_name = "IsUPeriodic"]
        fn is_u_periodic(self: &Plane) -> bool;
        #[doc = "return False."]
        #[cxx_name = "IsVPeriodic"]
        fn is_v_periodic(self: &Plane) -> bool;
        #[doc = "Computes the point P (U, V) on <me>. @code P = O + U * XDir + V * YDir. @endcode where O is the \"Location\" point of the plane, XDir the \"XDirection\" and YDir the \"YDirection\" of the plane's local coordinate system."]
        #[cxx_name = "D0"]
        fn d0(self: &Plane, U: f64, V: f64, P: Pin<&mut gp_Pnt>);
        #[doc = "Computes the current point and the first derivatives in the directions U and V."]
        #[cxx_name = "D1"]
        fn d1(
            self: &Plane,
            U: f64,
            V: f64,
            P: Pin<&mut gp_Pnt>,
            D1U: Pin<&mut gp_Vec>,
            D1V: Pin<&mut gp_Vec>,
        );
        #[doc = "Computes the current point, the first and the second derivatives in the directions U and V."]
        #[cxx_name = "D2"]
        fn d2(
            self: &Plane,
            U: f64,
            V: f64,
            P: Pin<&mut gp_Pnt>,
            D1U: Pin<&mut gp_Vec>,
            D1V: Pin<&mut gp_Vec>,
            D2U: Pin<&mut gp_Vec>,
            D2V: Pin<&mut gp_Vec>,
            D2UV: Pin<&mut gp_Vec>,
        );
        #[doc = "Computes the current point, the first,the second and the third derivatives in the directions U and V."]
        #[cxx_name = "D3"]
        fn d3(
            self: &Plane,
            U: f64,
            V: f64,
            P: Pin<&mut gp_Pnt>,
            D1U: Pin<&mut gp_Vec>,
            D1V: Pin<&mut gp_Vec>,
            D2U: Pin<&mut gp_Vec>,
            D2V: Pin<&mut gp_Vec>,
            D2UV: Pin<&mut gp_Vec>,
            D3U: Pin<&mut gp_Vec>,
            D3V: Pin<&mut gp_Vec>,
            D3UUV: Pin<&mut gp_Vec>,
            D3UVV: Pin<&mut gp_Vec>,
        );
        #[doc = "Applies the transformation T to this plane."]
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Plane>, T: &gp_Trsf);
        #[cxx_name = "DynamicType"]
        fn dynamic_type(self: &Plane) -> &HandleStandardType;
        #[doc = "Converts this plane into a gp_Pln plane."]
        #[cxx_name = "Geom_Plane_Pln"]
        fn Plane_pln(self_: &Plane) -> UniquePtr<gp_Pln>;
        #[doc = "Returns a 2d transformation used to find the new parameters of a point on the transformed surface. @code me->Transformed(T)->Value(U',V') @endcode is the same point as @code me->Value(U,V).Transformed(T) @endcode Where U',V' are  obtained by transforming U,V with the 2d transformation returned by @code me->ParametricTransformation(T) @endcode This method returns a scale centered on the origin with T.ScaleFactor"]
        #[cxx_name = "Geom_Plane_ParametricTransformation"]
        fn Plane_parametric_transformation(self_: &Plane, T: &gp_Trsf) -> UniquePtr<gp_GTrsf2d>;
        #[doc = "Computes the U isoparametric curve. This is a Line parallel to the YAxis of the plane."]
        #[cxx_name = "Geom_Plane_UIso"]
        fn Plane_u_iso(self_: &Plane, U: f64) -> UniquePtr<HandleGeomCurve>;
        #[doc = "Computes the V isoparametric curve. This is a Line parallel to the XAxis of the plane."]
        #[cxx_name = "Geom_Plane_VIso"]
        fn Plane_v_iso(self_: &Plane, V: f64) -> UniquePtr<HandleGeomCurve>;
        #[doc = "Computes the derivative of order Nu in the direction u and Nv in the direction v. Raised if Nu + Nv < 1 or Nu < 0 or Nv < 0."]
        #[cxx_name = "Geom_Plane_DN"]
        fn Plane_dn(self_: &Plane, U: f64, V: f64, Nu: i32, Nv: i32) -> UniquePtr<gp_Vec>;
        #[doc = "Creates a new object which is a copy of this plane."]
        #[cxx_name = "Geom_Plane_Copy"]
        fn Plane_copy(self_: &Plane) -> UniquePtr<HandleGeomGeometry>;
        #[cxx_name = "Geom_Plane_get_type_name"]
        fn Plane_get_type_name() -> String;
        #[doc = " ======================== Geom_Surface ========================"]
        #[doc = "Describes the common behavior of surfaces in 3D space. The Geom package provides many implementations of concrete derived surfaces, such as planes, cylinders, cones, spheres and tori, surfaces of linear extrusion, surfaces of revolution, Bezier and BSpline surfaces, and so on. The key characteristic of these surfaces is that they are parameterized. Geom_Surface demonstrates: - how to work with the parametric equation of a surface to compute the point of parameters (u, v), and, at this point, the 1st, 2nd ... Nth derivative; - how to find global information about a surface in each parametric direction (for example, level of continuity, whether the surface is closed, its periodicity, the bounds of the parameters and so on); - how the parameters change when geometric transformations are applied to the surface, or the orientation is modified. Note that all surfaces must have a geometric continuity, and any surface is at least \"C0\". Generally, continuity is checked at construction time or when the curve is edited. Where this is not the case, the documentation makes this explicit. Warning The Geom package does not prevent the construction of surfaces with null areas, or surfaces which self-intersect."]
        #[cxx_name = "Geom_Surface"]
        type Surface;
        #[doc = "Reverses the U direction of parametrization of <me>. The bounds of the surface are not modified."]
        #[cxx_name = "UReverse"]
        fn u_reverse(self: Pin<&mut Surface>);
        #[doc = "Returns the  parameter on the  Ureversed surface for the point of parameter U on <me>. @code me->UReversed()->Value(me->UReversedParameter(U),V) @endcode is the same point as @code me->Value(U,V) @endcode"]
        #[cxx_name = "UReversedParameter"]
        fn u_reversed_parameter(self: &Surface, U: f64) -> f64;
        #[doc = "Reverses the V direction of parametrization of <me>. The bounds of the surface are not modified."]
        #[cxx_name = "VReverse"]
        fn v_reverse(self: Pin<&mut Surface>);
        #[doc = "Returns the  parameter on the  Vreversed surface for the point of parameter V on <me>. @code me->VReversed()->Value(U,me->VReversedParameter(V)) @endcode is the same point as @code me->Value(U,V) @endcode"]
        #[cxx_name = "VReversedParameter"]
        fn v_reversed_parameter(self: &Surface, V: f64) -> f64;
        #[doc = "Computes the  parameters on the  transformed  surface for the transform of the point of parameters U,V on <me>. @code me->Transformed(T)->Value(U',V') @endcode is the same point as @code me->Value(U,V).Transformed(T) @endcode Where U',V' are the new values of U,V after calling @code me->TransformParameters(U,V,T) @endcode This method does not change <U> and <V> It  can be redefined.  For  example on  the Plane, Cylinder, Cone, Revolved and Extruded surfaces."]
        #[cxx_name = "TransformParameters"]
        fn transform_parameters(self: &Surface, U: &mut f64, V: &mut f64, T: &gp_Trsf);
        #[doc = "Returns the parametric bounds U1, U2, V1 and V2 of this surface. If the surface is infinite, this function can return a value equal to Precision::Infinite: instead of Standard_Real::LastReal."]
        #[cxx_name = "Bounds"]
        fn bounds(self: &Surface, U1: &mut f64, U2: &mut f64, V1: &mut f64, V2: &mut f64);
        #[doc = "Checks whether this surface is closed in the u parametric direction. Returns true if, in the u parametric direction: taking uFirst and uLast as the parametric bounds in the u parametric direction, for each parameter v, the distance between the points P(uFirst, v) and P(uLast, v) is less than or equal to gp::Resolution()."]
        #[cxx_name = "IsUClosed"]
        fn is_u_closed(self: &Surface) -> bool;
        #[doc = "Checks whether this surface is closed in the u parametric direction. Returns true if, in the v parametric direction: taking vFirst and vLast as the parametric bounds in the v parametric direction, for each parameter u, the distance between the points P(u, vFirst) and P(u, vLast) is less than or equal to gp::Resolution()."]
        #[cxx_name = "IsVClosed"]
        fn is_v_closed(self: &Surface) -> bool;
        #[doc = "Checks if this surface is periodic in the u parametric direction. Returns true if: - this surface is closed in the u parametric direction, and - there is a constant T such that the distance between the points P (u, v) and P (u + T, v) (or the points P (u, v) and P (u, v + T)) is less than or equal to gp::Resolution(). Note: T is the parametric period in the u parametric direction."]
        #[cxx_name = "IsUPeriodic"]
        fn is_u_periodic(self: &Surface) -> bool;
        #[doc = "Returns the period of this surface in the u parametric direction. Raises if the surface is not uperiodic."]
        #[cxx_name = "UPeriod"]
        fn u_period(self: &Surface) -> f64;
        #[doc = "Checks if this surface is periodic in the v parametric direction. Returns true if: - this surface is closed in the v parametric direction, and - there is a constant T such that the distance between the points P (u, v) and P (u + T, v) (or the points P (u, v) and P (u, v + T)) is less than or equal to gp::Resolution(). Note: T is the parametric period in the v parametric direction."]
        #[cxx_name = "IsVPeriodic"]
        fn is_v_periodic(self: &Surface) -> bool;
        #[doc = "Returns the period of this surface in the v parametric direction. raises if the surface is not vperiodic."]
        #[cxx_name = "VPeriod"]
        fn v_period(self: &Surface) -> f64;
        #[doc = "Returns the order of continuity of the surface in the U parametric direction. Raised if N < 0."]
        #[cxx_name = "IsCNu"]
        fn is_c_nu(self: &Surface, N: i32) -> bool;
        #[doc = "Returns the order of continuity of the surface in the V parametric direction. Raised if N < 0."]
        #[cxx_name = "IsCNv"]
        fn is_c_nv(self: &Surface, N: i32) -> bool;
        #[doc = "Computes the point of parameter U,V on the surface. Raised only for an \"OffsetSurface\" if it is not possible to compute the current point."]
        #[cxx_name = "D0"]
        fn d0(self: &Surface, U: f64, V: f64, P: Pin<&mut gp_Pnt>);
        #[doc = "Computes the point P and the first derivatives in the directions U and V at this point. Raised if the continuity of the surface is not C1. Tip: use GeomLib::NormEstim() to calculate surface normal at specified (U, V) point."]
        #[cxx_name = "D1"]
        fn d1(
            self: &Surface,
            U: f64,
            V: f64,
            P: Pin<&mut gp_Pnt>,
            D1U: Pin<&mut gp_Vec>,
            D1V: Pin<&mut gp_Vec>,
        );
        #[doc = "Computes the point P, the first and the second derivatives in the directions U and V at this point. Raised if the continuity of the surface is not C2."]
        #[cxx_name = "D2"]
        fn d2(
            self: &Surface,
            U: f64,
            V: f64,
            P: Pin<&mut gp_Pnt>,
            D1U: Pin<&mut gp_Vec>,
            D1V: Pin<&mut gp_Vec>,
            D2U: Pin<&mut gp_Vec>,
            D2V: Pin<&mut gp_Vec>,
            D2UV: Pin<&mut gp_Vec>,
        );
        #[doc = "Computes the point P, the first,the second and the third derivatives in the directions U and V at this point. Raised if the continuity of the surface is not C2."]
        #[cxx_name = "D3"]
        fn d3(
            self: &Surface,
            U: f64,
            V: f64,
            P: Pin<&mut gp_Pnt>,
            D1U: Pin<&mut gp_Vec>,
            D1V: Pin<&mut gp_Vec>,
            D2U: Pin<&mut gp_Vec>,
            D2V: Pin<&mut gp_Vec>,
            D2UV: Pin<&mut gp_Vec>,
            D3U: Pin<&mut gp_Vec>,
            D3V: Pin<&mut gp_Vec>,
            D3UUV: Pin<&mut gp_Vec>,
            D3UVV: Pin<&mut gp_Vec>,
        );
        #[cxx_name = "DynamicType"]
        fn dynamic_type(self: &Surface) -> &HandleStandardType;
        #[doc = "Reverses the U direction of parametrization of <me>. The bounds of the surface are not modified. A copy of <me> is returned."]
        #[cxx_name = "Geom_Surface_UReversed"]
        fn Surface_u_reversed(self_: &Surface) -> UniquePtr<HandleGeomSurface>;
        #[doc = "Reverses the V direction of parametrization of <me>. The bounds of the surface are not modified. A copy of <me> is returned."]
        #[cxx_name = "Geom_Surface_VReversed"]
        fn Surface_v_reversed(self_: &Surface) -> UniquePtr<HandleGeomSurface>;
        #[doc = "Returns a 2d transformation  used to find the  new parameters of a point on the transformed surface. @code me->Transformed(T)->Value(U',V') @endcode is the same point as @code me->Value(U,V).Transformed(T) @endcode Where U',V' are  obtained by transforming U,V with the 2d transformation returned by @code me->ParametricTransformation(T) @endcode This method returns an identity transformation It  can be redefined.  For  example on  the Plane, Cylinder, Cone, Revolved and Extruded surfaces."]
        #[cxx_name = "Geom_Surface_ParametricTransformation"]
        fn Surface_parametric_transformation(self_: &Surface, T: &gp_Trsf)
            -> UniquePtr<gp_GTrsf2d>;
        #[doc = "Computes the U isoparametric curve."]
        #[cxx_name = "Geom_Surface_UIso"]
        fn Surface_u_iso(self_: &Surface, U: f64) -> UniquePtr<HandleGeomCurve>;
        #[doc = "Computes the V isoparametric curve."]
        #[cxx_name = "Geom_Surface_VIso"]
        fn Surface_v_iso(self_: &Surface, V: f64) -> UniquePtr<HandleGeomCurve>;
        #[doc = "Returns the Global Continuity of the surface in direction U and V : - C0: only geometric continuity, - C1: continuity of the first derivative all along the surface, - C2: continuity of the second derivative all along the surface, - C3: continuity of the third derivative all along the surface, - G1: tangency continuity all along the surface, - G2: curvature continuity all along the surface, - CN: the order of continuity is infinite. Example: If the surface is C1 in the V parametric direction and C2 in the U parametric direction Shape = C1."]
        #[cxx_name = "Geom_Surface_Continuity"]
        fn Surface_continuity(self_: &Surface) -> UniquePtr<GeomAbs_Shape>;
        #[doc = "Computes the derivative of order Nu in the direction U and Nv in the direction V at the point P(U, V). Raised if the continuity of the surface is not CNu in the U direction or not CNv in the V direction. Raised if Nu + Nv < 1 or Nu < 0 or Nv < 0."]
        #[cxx_name = "Geom_Surface_DN"]
        fn Surface_dn(self_: &Surface, U: f64, V: f64, Nu: i32, Nv: i32) -> UniquePtr<gp_Vec>;
        #[doc = "Computes the point of parameter (U, V) on the surface. It is implemented with D0. Tip: use GeomLib::NormEstim() to calculate surface normal at specified (U, V) point. Raised only for an \"OffsetSurface\" if it is not possible to compute the current point."]
        #[cxx_name = "Geom_Surface_Value"]
        fn Surface_value(self_: &Surface, U: f64, V: f64) -> UniquePtr<gp_Pnt>;
        #[cxx_name = "Geom_Surface_get_type_name"]
        fn Surface_get_type_name() -> String;
        #[doc = " ======================== Geom_TrimmedCurve ========================"]
        #[doc = "Describes a portion of a curve (termed the \"basis curve\") limited by two parameter values inside the parametric domain of the basis curve. The trimmed curve is defined by: - the basis curve, and - the two parameter values which limit it. The trimmed curve can either have the same orientation as the basis curve or the opposite orientation."]
        #[cxx_name = "Geom_TrimmedCurve"]
        type TrimmedCurve;
        #[doc = "Constructs a trimmed curve from the basis curve C which is limited between parameter values U1 and U2. Note: - U1 can be greater or less than U2; in both cases, the returned curve is oriented from U1 to U2. - If the basis curve C is periodic, there is an ambiguity because two parts are available. In this case, the trimmed curve has the same orientation as the basis curve if Sense is true (default value) or the opposite orientation if Sense is false. - If the curve is closed but not periodic, it is not possible to keep the part of the curve which includes the junction point (except if the junction point is at the beginning or at the end of the trimmed curve). If you tried to do this, you could alter the fundamental characteristics of the basis curve, which are used, for example, to compute the derivatives of the trimmed curve. The rules for a closed curve are therefore the same as those for an open curve. Warning: The trimmed curve is built from a copy of curve C. Therefore, when C is modified, the trimmed curve is not modified. - If the basis curve is periodic and theAdjustPeriodic is True, the bounds of the trimmed curve may be different from U1 and U2 if the parametric origin of the basis curve is within the arc of the trimmed curve. In this case, the modified parameter will be equal to U1 or U2 plus or minus the period. When theAdjustPeriodic is False, parameters U1 and U2 will be the same, without adjustment into the first period. Exceptions Standard_ConstructionError if: - C is not periodic and U1 or U2 is outside the bounds of C, or - U1 is equal to U2."]
        #[cxx_name = "Geom_TrimmedCurve_ctor_handlecurve_real2_bool2"]
        fn TrimmedCurve_ctor_handlecurve_real2_bool2(
            C: &HandleGeomCurve,
            U1: f64,
            U2: f64,
            Sense: bool,
            theAdjustPeriodic: bool,
        ) -> UniquePtr<TrimmedCurve>;
        #[doc = "Changes the orientation of this trimmed curve. As a result: - the basis curve is reversed, - the start point of the initial curve becomes the end point of the reversed curve, - the end point of the initial curve becomes the start point of the reversed curve, - the first and last parameters are recomputed. If the trimmed curve was defined by: - a basis curve whose parameter range is [ 0., 1. ], - the two trim values U1 (first parameter) and U2 (last parameter), the reversed trimmed curve is defined by: - the reversed basis curve, whose parameter range is still [ 0., 1. ], - the two trim values 1. - U2 (first parameter) and 1. - U1 (last parameter)."]
        #[cxx_name = "Reverse"]
        fn reverse(self: Pin<&mut TrimmedCurve>);
        #[doc = "Computes the parameter on the reversed curve for the point of parameter U on this trimmed curve."]
        #[cxx_name = "ReversedParameter"]
        fn reversed_parameter(self: &TrimmedCurve, U: f64) -> f64;
        #[doc = "Changes this trimmed curve, by redefining the parameter values U1 and U2 which limit its basis curve. Note: If the basis curve is periodic, the trimmed curve has the same orientation as the basis curve if Sense is true (default value) or the opposite orientation if Sense is false. Warning If the basis curve is periodic and theAdjustPeriodic is True, the bounds of the trimmed curve may be different from U1 and U2 if the parametric origin of the basis curve is within the arc of the trimmed curve. In this case, the modified parameter will be equal to U1 or U2 plus or minus the period. When theAdjustPeriodic is False, parameters U1 and U2 will be the same, without adjustment into the first period. Exceptions Standard_ConstructionError if: - the basis curve is not periodic, and either U1 or U2 are outside the bounds of the basis curve, or - U1 is equal to U2."]
        #[cxx_name = "SetTrim"]
        fn set_trim(
            self: Pin<&mut TrimmedCurve>,
            U1: f64,
            U2: f64,
            Sense: bool,
            theAdjustPeriodic: bool,
        );
        #[doc = "Returns true if the degree of continuity of the basis curve of this trimmed curve is at least N. A trimmed curve is at least \"C0\" continuous. Warnings : The continuity of the trimmed curve can be greater than the continuity of the basis curve because you consider only a part of the basis curve. Raised if N < 0."]
        #[cxx_name = "IsCN"]
        fn is_cn(self: &TrimmedCurve, N: i32) -> bool;
        #[doc = "Returns the value of the first parameter of <me>. The first parameter is the parameter of the \"StartPoint\" of the trimmed curve."]
        #[cxx_name = "FirstParameter"]
        fn first_parameter(self: &TrimmedCurve) -> f64;
        #[doc = "Returns True if the distance between the StartPoint and the EndPoint is lower or equal to Resolution from package gp."]
        #[cxx_name = "IsClosed"]
        fn is_closed(self: &TrimmedCurve) -> bool;
        #[doc = "Always returns FALSE (independently of the type of basis curve)."]
        #[cxx_name = "IsPeriodic"]
        fn is_periodic(self: &TrimmedCurve) -> bool;
        #[doc = "Returns the period of the basis curve of this trimmed curve. Exceptions Standard_NoSuchObject if the basis curve is not periodic."]
        #[cxx_name = "Period"]
        fn period(self: &TrimmedCurve) -> f64;
        #[doc = "Returns the value of the last parameter of <me>. The last parameter is the parameter of the \"EndPoint\" of the trimmed curve."]
        #[cxx_name = "LastParameter"]
        fn last_parameter(self: &TrimmedCurve) -> f64;
        #[doc = "Returns in P the point of parameter U. If the basis curve is an OffsetCurve sometimes it is not possible to do the evaluation of the curve at the parameter U (see class OffsetCurve)."]
        #[cxx_name = "D0"]
        fn d0(self: &TrimmedCurve, U: f64, P: Pin<&mut gp_Pnt>);
        #[doc = "Raised if the continuity of the curve is not C1."]
        #[cxx_name = "D1"]
        fn d1(self: &TrimmedCurve, U: f64, P: Pin<&mut gp_Pnt>, V1: Pin<&mut gp_Vec>);
        #[doc = "Raised if the continuity of the curve is not C2."]
        #[cxx_name = "D2"]
        fn d2(
            self: &TrimmedCurve,
            U: f64,
            P: Pin<&mut gp_Pnt>,
            V1: Pin<&mut gp_Vec>,
            V2: Pin<&mut gp_Vec>,
        );
        #[doc = "Raised if the continuity of the curve is not C3."]
        #[cxx_name = "D3"]
        fn d3(
            self: &TrimmedCurve,
            U: f64,
            P: Pin<&mut gp_Pnt>,
            V1: Pin<&mut gp_Vec>,
            V2: Pin<&mut gp_Vec>,
            V3: Pin<&mut gp_Vec>,
        );
        #[doc = "Applies the transformation T to this trimmed curve. Warning The basis curve is also modified."]
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut TrimmedCurve>, T: &gp_Trsf);
        #[doc = "Returns the  parameter on the  transformed  curve for the transform of the point of parameter U on <me>. me->Transformed(T)->Value(me->TransformedParameter(U,T)) is the same point as me->Value(U).Transformed(T) This methods calls the basis curve method."]
        #[cxx_name = "TransformedParameter"]
        fn transformed_parameter(self: &TrimmedCurve, U: f64, T: &gp_Trsf) -> f64;
        #[doc = "Returns a  coefficient to compute the parameter on the transformed  curve  for  the transform  of the point on <me>. Transformed(T)->Value(U * ParametricTransformation(T)) is the same point as Value(U).Transformed(T) This methods calls the basis curve method."]
        #[cxx_name = "ParametricTransformation"]
        fn parametric_transformation(self: &TrimmedCurve, T: &gp_Trsf) -> f64;
        #[cxx_name = "DynamicType"]
        fn dynamic_type(self: &TrimmedCurve) -> &HandleStandardType;
        #[doc = "Returns the basis curve. Warning This function does not return a constant reference. Consequently, any modification of the returned value directly modifies the trimmed curve."]
        #[cxx_name = "Geom_TrimmedCurve_BasisCurve"]
        fn TrimmedCurve_basis_curve(self_: &TrimmedCurve) -> UniquePtr<HandleGeomCurve>;
        #[doc = "Returns the continuity of the curve : C0 : only geometric continuity, C1 : continuity of the first derivative all along the Curve, C2 : continuity of the second derivative all along the Curve, C3 : continuity of the third derivative all along the Curve, CN : the order of continuity is infinite."]
        #[cxx_name = "Geom_TrimmedCurve_Continuity"]
        fn TrimmedCurve_continuity(self_: &TrimmedCurve) -> UniquePtr<GeomAbs_Shape>;
        #[doc = "Returns the end point of <me>. This point is the evaluation of the curve for the \"LastParameter\"."]
        #[cxx_name = "Geom_TrimmedCurve_EndPoint"]
        fn TrimmedCurve_end_point(self_: &TrimmedCurve) -> UniquePtr<gp_Pnt>;
        #[doc = "Returns the start point of <me>. This point is the evaluation of the curve from the \"FirstParameter\". value and derivatives Warnings : The returned derivatives have the same orientation as the derivatives of the basis curve even if the trimmed curve has not the same orientation as the basis curve."]
        #[cxx_name = "Geom_TrimmedCurve_StartPoint"]
        fn TrimmedCurve_start_point(self_: &TrimmedCurve) -> UniquePtr<gp_Pnt>;
        #[doc = "N is the order of derivation. Raised if the continuity of the curve is not CN. Raised if N < 1. geometric transformations"]
        #[cxx_name = "Geom_TrimmedCurve_DN"]
        fn TrimmedCurve_dn(self_: &TrimmedCurve, U: f64, N: i32) -> UniquePtr<gp_Vec>;
        #[doc = "Creates a new object which is a copy of this trimmed curve."]
        #[cxx_name = "Geom_TrimmedCurve_Copy"]
        fn TrimmedCurve_copy(self_: &TrimmedCurve) -> UniquePtr<HandleGeomGeometry>;
        #[cxx_name = "Geom_TrimmedCurve_get_type_name"]
        fn TrimmedCurve_get_type_name() -> String;
    }
    impl UniquePtr<BezierCurve> {}
    impl UniquePtr<BSplineCurve> {}
    impl UniquePtr<Curve> {}
    impl UniquePtr<CylindricalSurface> {}
    impl UniquePtr<Plane> {}
    impl UniquePtr<Surface> {}
    impl UniquePtr<TrimmedCurve> {}
}
pub use ffi::BezierCurve;
impl BezierCurve {
    #[doc = "Creates a non rational Bezier curve with a set of poles CurvePoles.  The weights are defaulted to all being 1. Raises ConstructionError if the number of poles is greater than MaxDegree + 1 or lower than 2."]
    pub fn new_array1ofpnt(CurvePoles: &ffi::TColgp_Array1OfPnt) -> cxx::UniquePtr<Self> {
        ffi::BezierCurve_ctor_array1ofpnt(CurvePoles)
    }

    #[doc = "Creates a rational Bezier curve with the set of poles CurvePoles and the set of weights  PoleWeights . If all the weights are identical the curve is considered as non rational. Raises ConstructionError if the number of poles is greater than  MaxDegree + 1 or lower than 2 or CurvePoles and CurveWeights have not the same length or one weight value is lower or equal to Resolution from package gp."]
    pub fn new_array1ofpnt_array1ofreal(
        CurvePoles: &ffi::TColgp_Array1OfPnt,
        PoleWeights: &ffi::TColStd_Array1OfReal,
    ) -> cxx::UniquePtr<Self> {
        ffi::BezierCurve_ctor_array1ofpnt_array1ofreal(CurvePoles, PoleWeights)
    }
}
pub use ffi::BSplineCurve;
impl BSplineCurve {
    #[doc = "Creates a  non-rational B_spline curve   on  the basis <Knots, Multiplicities> of degree <Degree>."]
    pub fn new_array1ofpnt_array1ofreal_array1ofinteger_int_bool(
        Poles: &ffi::TColgp_Array1OfPnt,
        Knots: &ffi::TColStd_Array1OfReal,
        Multiplicities: &ffi::TColStd_Array1OfInteger,
        Degree: i32,
        Periodic: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::BSplineCurve_ctor_array1ofpnt_array1ofreal_array1ofinteger_int_bool(
            Poles,
            Knots,
            Multiplicities,
            Degree,
            Periodic,
        )
    }

    #[doc = "Creates  a rational B_spline  curve  on the basis <Knots, Multiplicities> of degree <Degree>. Raises ConstructionError subject to the following conditions 0 < Degree <= MaxDegree. Weights.Length() == Poles.Length() Knots.Length() == Mults.Length() >= 2 Knots(i) < Knots(i+1) (Knots are increasing) 1 <= Mults(i) <= Degree On a non periodic curve the first and last multiplicities may be Degree+1 (this is even recommended if you want the curve to start and finish on the first and last pole). On a periodic  curve the first  and  the last multicities must be the same. on non-periodic curves Poles.Length() == Sum(Mults(i)) - Degree - 1 >= 2 on periodic curves Poles.Length() == Sum(Mults(i)) except the first or last"]
    pub fn new_array1ofpnt_array1ofreal2_array1ofinteger_int_bool2(
        Poles: &ffi::TColgp_Array1OfPnt,
        Weights: &ffi::TColStd_Array1OfReal,
        Knots: &ffi::TColStd_Array1OfReal,
        Multiplicities: &ffi::TColStd_Array1OfInteger,
        Degree: i32,
        Periodic: bool,
        CheckRational: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::BSplineCurve_ctor_array1ofpnt_array1ofreal2_array1ofinteger_int_bool2(
            Poles,
            Weights,
            Knots,
            Multiplicities,
            Degree,
            Periodic,
            CheckRational,
        )
    }
}
pub use ffi::{Curve, CylindricalSurface};
impl CylindricalSurface {
    #[doc = "A3 defines the local coordinate system of the cylindrical surface. The \"ZDirection\" of A3 defines the direction of the surface's axis of symmetry. At the creation the parametrization of the surface is defined such that the normal Vector (N = D1U ^ D1V) is oriented towards the \"outside region\" of the surface. Warnings: It is not forbidden to create a cylindrical surface with Radius = 0.0 Raised if Radius < 0.0"]
    pub fn new_ax3_real(A3: &ffi::gp_Ax3, Radius: f64) -> cxx::UniquePtr<Self> {
        ffi::CylindricalSurface_ctor_ax3_real(A3, Radius)
    }

    #[doc = "Creates a CylindricalSurface from a non transient gp_Cylinder."]
    pub fn new_cylinder(C: &ffi::gp_Cylinder) -> cxx::UniquePtr<Self> {
        ffi::CylindricalSurface_ctor_cylinder(C)
    }
}
pub use ffi::Plane;
impl Plane {
    #[doc = "Creates a plane located in 3D space with an axis placement three axis. The \"ZDirection\" of \"A3\" is the direction normal to the plane.  The \"Location\" point of \"A3\" is the origin of the plane. The \"XDirection\" and \"YDirection\" of \"A3\" define the directions of the U isoparametric and V isoparametric curves."]
    pub fn new_ax3(A3: &ffi::gp_Ax3) -> cxx::UniquePtr<Self> {
        ffi::Plane_ctor_ax3(A3)
    }

    #[doc = "Creates a plane from a non transient plane from package gp."]
    pub fn new_pln(Pl: &ffi::gp_Pln) -> cxx::UniquePtr<Self> {
        ffi::Plane_ctor_pln(Pl)
    }

    #[doc = "P is the \"Location\" point or origin of the plane. V is the direction normal to the plane."]
    pub fn new_pnt_dir(P: &ffi::gp_Pnt, V: &ffi::gp_Dir) -> cxx::UniquePtr<Self> {
        ffi::Plane_ctor_pnt_dir(P, V)
    }

    #[doc = "Creates a plane from its cartesian equation: @code Ax + By + Cz + D = 0.0 @endcode Raised if Sqrt (A*A + B*B + C*C) <= Resolution from gp"]
    pub fn new_real4(A: f64, B: f64, C: f64, D: f64) -> cxx::UniquePtr<Self> {
        ffi::Plane_ctor_real4(A, B, C, D)
    }
}
pub use ffi::{Surface, TrimmedCurve};
impl TrimmedCurve {
    #[doc = "Constructs a trimmed curve from the basis curve C which is limited between parameter values U1 and U2. Note: - U1 can be greater or less than U2; in both cases, the returned curve is oriented from U1 to U2. - If the basis curve C is periodic, there is an ambiguity because two parts are available. In this case, the trimmed curve has the same orientation as the basis curve if Sense is true (default value) or the opposite orientation if Sense is false. - If the curve is closed but not periodic, it is not possible to keep the part of the curve which includes the junction point (except if the junction point is at the beginning or at the end of the trimmed curve). If you tried to do this, you could alter the fundamental characteristics of the basis curve, which are used, for example, to compute the derivatives of the trimmed curve. The rules for a closed curve are therefore the same as those for an open curve. Warning: The trimmed curve is built from a copy of curve C. Therefore, when C is modified, the trimmed curve is not modified. - If the basis curve is periodic and theAdjustPeriodic is True, the bounds of the trimmed curve may be different from U1 and U2 if the parametric origin of the basis curve is within the arc of the trimmed curve. In this case, the modified parameter will be equal to U1 or U2 plus or minus the period. When theAdjustPeriodic is False, parameters U1 and U2 will be the same, without adjustment into the first period. Exceptions Standard_ConstructionError if: - C is not periodic and U1 or U2 is outside the bounds of C, or - U1 is equal to U2."]
    pub fn new_handlecurve_real2_bool2(
        C: &ffi::HandleGeomCurve,
        U1: f64,
        U2: f64,
        Sense: bool,
        theAdjustPeriodic: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::TrimmedCurve_ctor_handlecurve_real2_bool2(C, U1, U2, Sense, theAdjustPeriodic)
    }
}
