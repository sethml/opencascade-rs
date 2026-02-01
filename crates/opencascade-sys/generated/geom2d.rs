#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_geom2d.hxx");
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
        #[cxx_name = "Geom2d_Geometry"]
        type Geom2d_Geometry;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "GeomAbs_Shape"]
        type GeomAbs_Shape;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "gp_Elips2d"]
        type gp_Elips2d;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "gp_Ax22d"]
        type gp_Ax22d;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Standard_Type"]
        type Standard_Type;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleGeom2dCurve"]
        type HandleGeom2dCurve;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleGeom2dGeometry"]
        type HandleGeom2dGeometry;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleStandardType"]
        type HandleStandardType;
        #[doc = " ======================== Geom2d_Curve ========================"]
        #[doc = "The abstract class Curve describes the common behavior of curves in 2D space. The Geom2d package provides numerous concrete classes of derived curves, including lines, circles, conics, Bezier or BSpline curves, etc. The main characteristic of these curves is that they are parameterized. The Geom2d_Curve class shows: - how to work with the parametric equation of a curve in order to calculate the point of parameter u, together with the vector tangent and the derivative vectors of order 2, 3,..., N at this point; - how to obtain general information about the curve (for example, level of continuity, closed characteristics, periodicity, bounds of the parameter field); - how the parameter changes when a geometric transformation is applied to the curve or when the orientation of the curve is inverted. All curves must have a geometric continuity: a curve is at least \"C0\". Generally, this property is checked at the time of construction or when the curve is edited. Where this is not the case, the documentation explicitly states so. Warning The Geom2d package does not prevent the construction of curves with null length or curves which self-intersect."]
        #[cxx_name = "Geom2d_Curve"]
        type Curve;
        #[doc = "Changes the direction of parametrization of <me>. The \"FirstParameter\" and the \"LastParameter\" are not changed but the orientation  of the curve is modified. If the curve is bounded the StartPoint of the initial curve becomes the EndPoint of the reversed curve  and the EndPoint of the initial curve becomes the StartPoint of the reversed curve."]
        #[cxx_name = "Reverse"]
        fn reverse(self: Pin<&mut Curve>);
        #[doc = "Computes the parameter on the reversed curve for the point of parameter U on this curve. Note: The point of parameter U on this curve is identical to the point of parameter ReversedParameter(U) on the reversed curve."]
        #[cxx_name = "ReversedParameter"]
        fn reversed_parameter(self: &Curve, U: f64) -> f64;
        #[doc = "Computes the parameter on the curve transformed by T for the point of parameter U on this curve. Note: this function generally returns U but it can be redefined (for example, on a line)."]
        #[cxx_name = "TransformedParameter"]
        fn transformed_parameter(self: &Curve, U: f64, T: &gp_Trsf2d) -> f64;
        #[doc = "Returns the coefficient required to compute the parametric transformation of this curve when transformation T is applied. This coefficient is the ratio between the parameter of a point on this curve and the parameter of the transformed point on the new curve transformed by T. Note: this function generally returns 1. but it can be redefined (for example, on a line)."]
        #[cxx_name = "ParametricTransformation"]
        fn parametric_transformation(self: &Curve, T: &gp_Trsf2d) -> f64;
        #[doc = "Returns the value of the first parameter. Warnings : It can be RealFirst or RealLast from package Standard if the curve is infinite"]
        #[cxx_name = "FirstParameter"]
        fn first_parameter(self: &Curve) -> f64;
        #[doc = "Value of the last parameter. Warnings : It can be RealFirst or RealLast from package Standard if the curve is infinite"]
        #[cxx_name = "LastParameter"]
        fn last_parameter(self: &Curve) -> f64;
        #[doc = "Returns true if the curve is closed. Examples : Some curves such as circle are always closed, others such as line are never closed (by definition). Some Curves such as OffsetCurve can be closed or not. These curves are considered as closed if the distance between the first point and the last point of the curve is lower or equal to the Resolution from package gp which is a fixed criterion independent of the application."]
        #[cxx_name = "IsClosed"]
        fn is_closed(self: &Curve) -> bool;
        #[doc = "Returns true if the parameter of the curve is periodic. It is possible only if the curve is closed and if the following relation is satisfied : for each parametric value U the distance between the point P(u) and the point P (u + T) is lower or equal to Resolution from package gp, T is the period and must be a constant. There are three possibilities : . the curve is never periodic by definition (SegmentLine) . the curve is always periodic by definition (Circle) . the curve can be defined as periodic (BSpline). In this case a function SetPeriodic allows you to give the shape of the curve.  The general rule for this case is : if a curve can be periodic or not the default periodicity set is non periodic and you have to turn (explicitly) the curve into a periodic curve  if you want the curve to be periodic."]
        #[cxx_name = "IsPeriodic"]
        fn is_periodic(self: &Curve) -> bool;
        #[doc = "Returns the period of this curve. raises if the curve is not periodic"]
        #[cxx_name = "Period"]
        fn period(self: &Curve) -> f64;
        #[doc = "Returns true if the degree of continuity of this curve is at least N. Exceptions Standard_RangeError if N is less than 0."]
        #[cxx_name = "IsCN"]
        fn is_cn(self: &Curve, N: i32) -> bool;
        #[doc = "Returns in P the point of parameter U. If the curve is periodic  then the returned point is P(U) with U = Ustart + (U - Uend)  where Ustart and Uend are the parametric bounds of the curve. Raised only for the \"OffsetCurve\" if it is not possible to compute the current point. For example when the first derivative on the basis curve and the offset direction are parallel."]
        #[cxx_name = "D0"]
        fn d0(self: &Curve, U: f64, P: Pin<&mut gp_Pnt2d>);
        #[doc = "Returns the point P of parameter U and the first derivative V1. Raised if the continuity of the curve is not C1."]
        #[cxx_name = "D1"]
        fn d1(self: &Curve, U: f64, P: Pin<&mut gp_Pnt2d>, V1: Pin<&mut gp_Vec2d>);
        #[doc = "Returns the point P of parameter U, the first and second derivatives V1 and V2. Raised if the continuity of the curve is not C2."]
        #[cxx_name = "D2"]
        fn d2(
            self: &Curve,
            U: f64,
            P: Pin<&mut gp_Pnt2d>,
            V1: Pin<&mut gp_Vec2d>,
            V2: Pin<&mut gp_Vec2d>,
        );
        #[doc = "Returns the point P of parameter U, the first, the second and the third derivative. Raised if the continuity of the curve is not C3."]
        #[cxx_name = "D3"]
        fn d3(
            self: &Curve,
            U: f64,
            P: Pin<&mut gp_Pnt2d>,
            V1: Pin<&mut gp_Vec2d>,
            V2: Pin<&mut gp_Vec2d>,
            V3: Pin<&mut gp_Vec2d>,
        );
        #[cxx_name = "DynamicType"]
        fn dynamic_type(self: &Curve) -> &HandleStandardType;
        #[doc = "Creates a reversed duplicate Changes the orientation of this curve. The first and last parameters are not changed, but the parametric direction of the curve is reversed. If the curve is bounded: - the start point of the initial curve becomes the end point of the reversed curve, and - the end point of the initial curve becomes the start point of the reversed curve. - Reversed creates a new curve."]
        #[cxx_name = "Geom2d_Curve_Reversed"]
        fn Curve_reversed(self_: &Curve) -> UniquePtr<HandleGeom2dCurve>;
        #[doc = "It is the global continuity of the curve : C0 : only geometric continuity, C1 : continuity of the first derivative all along the Curve, C2 : continuity of the second derivative all along the Curve, C3 : continuity of the third derivative all along the Curve, G1 : tangency continuity all along the Curve, G2 : curvature continuity all along the Curve, CN : the order of continuity is infinite."]
        #[cxx_name = "Geom2d_Curve_Continuity"]
        fn Curve_continuity(self_: &Curve) -> UniquePtr<GeomAbs_Shape>;
        #[doc = "For the point of parameter U of this curve, computes the vector corresponding to the Nth derivative. Exceptions StdFail_UndefinedDerivative if: - the continuity of the curve is not \"CN\", or - the derivative vector cannot be computed easily; this is the case with specific types of curve (for example, a rational BSpline curve where N is greater than 3). Standard_RangeError if N is less than 1."]
        #[cxx_name = "Geom2d_Curve_DN"]
        fn Curve_dn(self_: &Curve, U: f64, N: i32) -> UniquePtr<gp_Vec2d>;
        #[doc = "Computes the point of parameter U on <me>. If the curve is periodic  then the returned point is P(U) with U = Ustart + (U - Uend)  where Ustart and Uend are the parametric bounds of the curve. it is implemented with D0. Raised only for the \"OffsetCurve\" if it is not possible to compute the current point. For example when the first derivative on the basis curve and the offset direction are parallel."]
        #[cxx_name = "Geom2d_Curve_Value"]
        fn Curve_value(self_: &Curve, U: f64) -> UniquePtr<gp_Pnt2d>;
        #[cxx_name = "Geom2d_Curve_get_type_name"]
        fn Curve_get_type_name() -> String;
        #[doc = " ======================== Geom2d_Ellipse ========================"]
        #[doc = "Describes an ellipse in the plane (2D space). An ellipse is defined by its major and minor radii and, as with any conic curve, is positioned in the plane with a coordinate system (gp_Ax22d object) where: - the origin is the center of the ellipse, - the \"X Direction\" defines the major axis, and - the \"Y Direction\" defines the minor axis. This coordinate system is the local coordinate system of the ellipse. The orientation (direct or indirect) of the local coordinate system gives an explicit orientation to the ellipse, determining the direction in which the parameter increases along the ellipse. The Geom2d_Ellipse ellipse is parameterized by an angle: P(U) = O + MajorRad*Cos(U)*XDir + MinorRad*Sin(U)*YDir where: - P is the point of parameter U, - O, XDir and YDir are respectively the origin, \"X Direction\" and \"Y Direction\" of its local coordinate system, - MajorRad and MinorRad are the major and minor radii of the ellipse. The \"X Axis\" of the local coordinate system therefore defines the origin of the parameter of the ellipse. An ellipse is a closed and periodic curve. The period is 2.*Pi and the parameter range is [ 0,2.*Pi [. See Also GCE2d_MakeEllipse which provides functions for more complex ellipse constructions gp_Ax22d gp_Elips2d for an equivalent, non-parameterized data structure"]
        #[cxx_name = "Geom2d_Ellipse"]
        type Ellipse;
        #[doc = "Creates an ellipse by conversion of the gp_Elips2d ellipse E."]
        #[cxx_name = "Geom2d_Ellipse_ctor_elips2d"]
        fn Ellipse_ctor_elips2d(E: &gp_Elips2d) -> UniquePtr<Ellipse>;
        #[doc = "Creates an ellipse defined by its major and minor radii, MajorRadius and MinorRadius, and positioned in the plane by its major axis MajorAxis; the center of the ellipse is the origin of MajorAxis and the unit vector of MajorAxis is the \"X Direction\" of the local coordinate system of the ellipse; this coordinate system is direct if Sense is true (default value) or indirect if Sense is false. Warnings : It is not forbidden to create an ellipse with MajorRadius = MinorRadius. Exceptions Standard_ConstructionError if: - MajorRadius is less than MinorRadius, or - MinorRadius is less than 0."]
        #[cxx_name = "Geom2d_Ellipse_ctor_ax2d_real2_bool"]
        fn Ellipse_ctor_ax2d_real2_bool(
            MajorAxis: &gp_Ax2d,
            MajorRadius: f64,
            MinorRadius: f64,
            Sense: bool,
        ) -> UniquePtr<Ellipse>;
        #[doc = "Creates an ellipse defined by its major and minor radii, MajorRadius and MinorRadius, where the coordinate system Axis locates the ellipse and defines its orientation in the plane such that: - the center of the ellipse is the origin of Axis, - the \"X Direction\" of Axis defines the major axis of the ellipse, - the \"Y Direction\" of Axis defines the minor axis of the ellipse, - the orientation of Axis (direct or indirect) gives the orientation of the ellipse. Warnings : It is not forbidden to create an ellipse with MajorRadius = MinorRadius. Exceptions Standard_ConstructionError if: - MajorRadius is less than MinorRadius, or - MinorRadius is less than 0."]
        #[cxx_name = "Geom2d_Ellipse_ctor_ax22d_real2"]
        fn Ellipse_ctor_ax22d_real2(
            Axis: &gp_Ax22d,
            MajorRadius: f64,
            MinorRadius: f64,
        ) -> UniquePtr<Ellipse>;
        #[doc = "Converts the gp_Elips2d ellipse E into this ellipse."]
        #[cxx_name = "SetElips2d"]
        fn set_elips2d(self: Pin<&mut Ellipse>, E: &gp_Elips2d);
        #[doc = "Assigns a value to the major radius of this ellipse. Exceptions Standard_ConstructionError if: - the major radius of this ellipse becomes less than the minor radius, or - MinorRadius is less than 0."]
        #[cxx_name = "SetMajorRadius"]
        fn set_major_radius(self: Pin<&mut Ellipse>, MajorRadius: f64);
        #[doc = "Assigns a value to the minor radius of this ellipse. Exceptions Standard_ConstructionError if: - the major radius of this ellipse becomes less than the minor radius, or - MinorRadius is less than 0."]
        #[cxx_name = "SetMinorRadius"]
        fn set_minor_radius(self: Pin<&mut Ellipse>, MinorRadius: f64);
        #[doc = "Computes the parameter on the reversed ellipse for the point of parameter U on this ellipse. For an ellipse, the returned value is: 2.*Pi - U."]
        #[cxx_name = "ReversedParameter"]
        fn reversed_parameter(self: &Ellipse, U: f64) -> f64;
        #[doc = "Returns the eccentricity of the ellipse  between 0.0 and 1.0 If f is the distance between the center of the ellipse and the Focus1 then the eccentricity e = f / MajorRadius. Returns 0 if MajorRadius = 0"]
        #[cxx_name = "Eccentricity"]
        fn eccentricity(self: &Ellipse) -> f64;
        #[doc = "Computes the focal distance. The focal distance is the distance between the center and a focus of the ellipse."]
        #[cxx_name = "Focal"]
        fn focal(self: &Ellipse) -> f64;
        #[doc = "Returns the major radius of this ellipse."]
        #[cxx_name = "MajorRadius"]
        fn major_radius(self: &Ellipse) -> f64;
        #[doc = "Returns the minor radius of this ellipse."]
        #[cxx_name = "MinorRadius"]
        fn minor_radius(self: &Ellipse) -> f64;
        #[doc = "Computes the parameter of this ellipse. This value is given by the formula p = (1 - e * e) * MajorRadius where e is the eccentricity of the ellipse. Returns 0 if MajorRadius = 0"]
        #[cxx_name = "Parameter"]
        fn parameter(self: &Ellipse) -> f64;
        #[doc = "Returns the value of the first parameter of this ellipse. This is  0.0, which gives the start point of this ellipse. The start point and end point of an ellipse are coincident."]
        #[cxx_name = "FirstParameter"]
        fn first_parameter(self: &Ellipse) -> f64;
        #[doc = "Returns the value of the  last parameter of this ellipse. This is  2.*Pi, which gives the end point of this ellipse. The start point and end point of an ellipse are coincident."]
        #[cxx_name = "LastParameter"]
        fn last_parameter(self: &Ellipse) -> f64;
        #[doc = "return True."]
        #[cxx_name = "IsClosed"]
        fn is_closed(self: &Ellipse) -> bool;
        #[doc = "return True."]
        #[cxx_name = "IsPeriodic"]
        fn is_periodic(self: &Ellipse) -> bool;
        #[doc = "Returns in P the point of parameter U. P = C + MajorRadius * Cos (U) * XDir + MinorRadius * Sin (U) * YDir where C is the center of the ellipse , XDir the direction of the \"XAxis\" and \"YDir\" the \"YAxis\" of the ellipse."]
        #[cxx_name = "D0"]
        fn d0(self: &Ellipse, U: f64, P: Pin<&mut gp_Pnt2d>);
        #[cxx_name = "D1"]
        fn d1(self: &Ellipse, U: f64, P: Pin<&mut gp_Pnt2d>, V1: Pin<&mut gp_Vec2d>);
        #[doc = "Returns the point P of parameter U. The vectors V1 and V2 are the first and second derivatives at this point."]
        #[cxx_name = "D2"]
        fn d2(
            self: &Ellipse,
            U: f64,
            P: Pin<&mut gp_Pnt2d>,
            V1: Pin<&mut gp_Vec2d>,
            V2: Pin<&mut gp_Vec2d>,
        );
        #[doc = "Returns the point P of parameter U, the first second and third derivatives V1 V2 and V3."]
        #[cxx_name = "D3"]
        fn d3(
            self: &Ellipse,
            U: f64,
            P: Pin<&mut gp_Pnt2d>,
            V1: Pin<&mut gp_Vec2d>,
            V2: Pin<&mut gp_Vec2d>,
            V3: Pin<&mut gp_Vec2d>,
        );
        #[doc = "Applies the transformation T to this ellipse."]
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Ellipse>, T: &gp_Trsf2d);
        #[cxx_name = "DynamicType"]
        fn dynamic_type(self: &Ellipse) -> &HandleStandardType;
        #[doc = "Converts this ellipse into a gp_Elips2d ellipse."]
        #[cxx_name = "Geom2d_Ellipse_Elips2d"]
        fn Ellipse_elips2d(self_: &Ellipse) -> UniquePtr<gp_Elips2d>;
        #[doc = "Computes the directrices of this ellipse. This directrix is the line normal to the XAxis of the ellipse in the local plane (Z = 0) at a distance d = MajorRadius / e from the center of the ellipse, where e is the eccentricity of the ellipse. This line is parallel to the \"YAxis\". The intersection point between directrix1 and the \"XAxis\" is the \"Location\" point of the directrix1. This point is on the positive side of the \"XAxis\". Raises ConstructionError if Eccentricity = 0.0. (The ellipse degenerates into a circle)"]
        #[cxx_name = "Geom2d_Ellipse_Directrix1"]
        fn Ellipse_directrix1(self_: &Ellipse) -> UniquePtr<gp_Ax2d>;
        #[doc = "This line is obtained by the symmetrical transformation of \"Directrix1\" with respect to the \"YAxis\" of the ellipse. Raises ConstructionError if Eccentricity = 0.0. (The ellipse degenerates into a circle)."]
        #[cxx_name = "Geom2d_Ellipse_Directrix2"]
        fn Ellipse_directrix2(self_: &Ellipse) -> UniquePtr<gp_Ax2d>;
        #[doc = "Returns the first focus of the ellipse. This focus is on the positive side of the \"XAxis\" of the ellipse."]
        #[cxx_name = "Geom2d_Ellipse_Focus1"]
        fn Ellipse_focus1(self_: &Ellipse) -> UniquePtr<gp_Pnt2d>;
        #[doc = "Returns the second focus of the ellipse. This focus is on the negative side of the \"XAxis\" of the ellipse."]
        #[cxx_name = "Geom2d_Ellipse_Focus2"]
        fn Ellipse_focus2(self_: &Ellipse) -> UniquePtr<gp_Pnt2d>;
        #[doc = "For the point of parameter U of this ellipse, computes the vector corresponding to the Nth derivative. Exceptions Standard_RangeError if N is less than 1."]
        #[cxx_name = "Geom2d_Ellipse_DN"]
        fn Ellipse_dn(self_: &Ellipse, U: f64, N: i32) -> UniquePtr<gp_Vec2d>;
        #[doc = "Creates a new object which is a copy of this ellipse."]
        #[cxx_name = "Geom2d_Ellipse_Copy"]
        fn Ellipse_copy(self_: &Ellipse) -> UniquePtr<HandleGeom2dGeometry>;
        #[cxx_name = "Geom2d_Ellipse_get_type_name"]
        fn Ellipse_get_type_name() -> String;
        #[doc = " ======================== Geom2d_TrimmedCurve ========================"]
        #[doc = "Defines a portion of a curve limited by two values of parameters inside the parametric domain of the curve. The trimmed curve is defined by: - the basis curve, and - the two parameter values which limit it. The trimmed curve can either have the same orientation as the basis curve or the opposite orientation."]
        #[cxx_name = "Geom2d_TrimmedCurve"]
        type TrimmedCurve;
        #[doc = "Creates a trimmed curve from the basis curve C limited between U1 and U2. . U1 can be greater or lower than U2. . The returned curve is oriented from U1 to U2. . If the basis curve C is periodic there is an ambiguity because two parts are available. In this case by default the trimmed curve has the same orientation as the basis curve (Sense = True). If Sense = False then the orientation of the trimmed curve is opposite to the orientation of the basis curve C. If the curve is closed but not periodic it is not possible to keep the part of the curve including the junction point (except if the junction point is at the beginning or at the end of the trimmed curve) because you could lose the fundamental characteristics of the basis curve which are used for example to compute the derivatives of the trimmed curve. So for a closed curve the rules are the same as for a open curve. Warnings : In this package the entities are not shared. The TrimmedCurve is built with a copy of the curve C. So when C is modified the TrimmedCurve is not modified Warnings : If <C> is periodic and <theAdjustPeriodic> is True, parametrics bounds of the TrimmedCurve, can be different to [<U1>;<U2>}, if <U1> or <U2> are not in the principal period. Include : For more explanation see the scheme given with this class. Raises ConstructionError the C is not periodic and U1 or U2 are out of the bounds of C. Raised if U1 = U2."]
        #[cxx_name = "Geom2d_TrimmedCurve_ctor_handlecurve_real2_bool2"]
        fn TrimmedCurve_ctor_handlecurve_real2_bool2(
            C: &HandleGeom2dCurve,
            U1: f64,
            U2: f64,
            Sense: bool,
            theAdjustPeriodic: bool,
        ) -> UniquePtr<TrimmedCurve>;
        #[doc = "Changes the direction of parametrization of <me>. The first and the last parametric values are modified. The \"StartPoint\" of the initial curve becomes the \"EndPoint\" of the reversed curve and the \"EndPoint\" of the initial curve becomes the \"StartPoint\" of the reversed curve. Example  -   If the trimmed curve is defined by: - a basis curve whose parameter range is [ 0.,1. ], and - the two trim values U1 (first parameter) and U2 (last parameter), the reversed trimmed curve is defined by: - the reversed basis curve, whose parameter range is still [ 0.,1. ], and - the two trim values 1. - U2 (first parameter) and 1. - U1 (last parameter)."]
        #[cxx_name = "Reverse"]
        fn reverse(self: Pin<&mut TrimmedCurve>);
        #[doc = "Returns the  parameter on the  reversed  curve for the point of parameter U on <me>. returns UFirst + ULast - U"]
        #[cxx_name = "ReversedParameter"]
        fn reversed_parameter(self: &TrimmedCurve, U: f64) -> f64;
        #[doc = "Changes this trimmed curve, by redefining the parameter values U1 and U2, which limit its basis curve. Note: If the basis curve is periodic, the trimmed curve has the same orientation as the basis curve if Sense is true (default value) or the opposite orientation if Sense is false. Warning If the basis curve is periodic and theAdjustPeriodic is True, the bounds of the trimmed curve may be different from U1 and U2 if the parametric origin of the basis curve is within the arc of the trimmed curve. In this case, the modified parameter will be equal to U1 or U2 plus or minus the period. If theAdjustPeriodic is False, parameters U1 and U2 will stay unchanged. Exceptions Standard_ConstructionError if: - the basis curve is not periodic, and either U1 or U2 are outside the bounds of the basis curve, or - U1 is equal to U2."]
        #[cxx_name = "SetTrim"]
        fn set_trim(
            self: Pin<&mut TrimmedCurve>,
            U1: f64,
            U2: f64,
            Sense: bool,
            theAdjustPeriodic: bool,
        );
        #[doc = "--- Purpose Returns True if the order of continuity of the trimmed curve is N. A trimmed curve is at least \"C0\" continuous. Warnings : The continuity of the trimmed curve can be greater than the continuity of the basis curve because you consider only a part of the basis curve. Raised if N < 0."]
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
        #[doc = "If the basis curve is an OffsetCurve sometimes it is not possible to do the evaluation of the curve at the parameter U (see class OffsetCurve)."]
        #[cxx_name = "D0"]
        fn d0(self: &TrimmedCurve, U: f64, P: Pin<&mut gp_Pnt2d>);
        #[doc = "Raised if the continuity of the curve is not C1."]
        #[cxx_name = "D1"]
        fn d1(self: &TrimmedCurve, U: f64, P: Pin<&mut gp_Pnt2d>, V1: Pin<&mut gp_Vec2d>);
        #[doc = "Raised if the continuity of the curve is not C2."]
        #[cxx_name = "D2"]
        fn d2(
            self: &TrimmedCurve,
            U: f64,
            P: Pin<&mut gp_Pnt2d>,
            V1: Pin<&mut gp_Vec2d>,
            V2: Pin<&mut gp_Vec2d>,
        );
        #[doc = "Raised if the continuity of the curve is not C3."]
        #[cxx_name = "D3"]
        fn d3(
            self: &TrimmedCurve,
            U: f64,
            P: Pin<&mut gp_Pnt2d>,
            V1: Pin<&mut gp_Vec2d>,
            V2: Pin<&mut gp_Vec2d>,
            V3: Pin<&mut gp_Vec2d>,
        );
        #[doc = "Applies the transformation T to this trimmed curve. Warning The basis curve is also modified."]
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut TrimmedCurve>, T: &gp_Trsf2d);
        #[doc = "Returns the  parameter on the  transformed  curve for the transform of the point of parameter U on <me>. me->Transformed(T)->Value(me->TransformedParameter(U,T)) is the same point as me->Value(U).Transformed(T) This methods calls the basis curve method."]
        #[cxx_name = "TransformedParameter"]
        fn transformed_parameter(self: &TrimmedCurve, U: f64, T: &gp_Trsf2d) -> f64;
        #[doc = "Returns a  coefficient to compute the parameter on the transformed  curve  for  the transform  of the point on <me>. Transformed(T)->Value(U * ParametricTransformation(T)) is the same point as Value(U).Transformed(T) This methods calls the basis curve method."]
        #[cxx_name = "ParametricTransformation"]
        fn parametric_transformation(self: &TrimmedCurve, T: &gp_Trsf2d) -> f64;
        #[cxx_name = "DynamicType"]
        fn dynamic_type(self: &TrimmedCurve) -> &HandleStandardType;
        #[doc = "Returns the basis curve. Warning This function does not return a constant reference. Consequently, any modification of the returned value directly modifies the trimmed curve."]
        #[cxx_name = "Geom2d_TrimmedCurve_BasisCurve"]
        fn TrimmedCurve_basis_curve(self_: &TrimmedCurve) -> UniquePtr<HandleGeom2dCurve>;
        #[doc = "Returns the global continuity of the basis curve of this trimmed curve. C0 : only geometric continuity, C1 : continuity of the first derivative all along the Curve, C2 : continuity of the second derivative all along the Curve, C3 : continuity of the third derivative all along the Curve, CN : the order of continuity is infinite."]
        #[cxx_name = "Geom2d_TrimmedCurve_Continuity"]
        fn TrimmedCurve_continuity(self_: &TrimmedCurve) -> UniquePtr<GeomAbs_Shape>;
        #[doc = "Returns the end point of <me>. This point is the evaluation of the curve for the \"LastParameter\"."]
        #[cxx_name = "Geom2d_TrimmedCurve_EndPoint"]
        fn TrimmedCurve_end_point(self_: &TrimmedCurve) -> UniquePtr<gp_Pnt2d>;
        #[doc = "Returns the start point of <me>. This point is the evaluation of the curve from the \"FirstParameter\". value and derivatives Warnings : The returned derivatives have the same orientation as the derivatives of the basis curve."]
        #[cxx_name = "Geom2d_TrimmedCurve_StartPoint"]
        fn TrimmedCurve_start_point(self_: &TrimmedCurve) -> UniquePtr<gp_Pnt2d>;
        #[doc = "For the point of parameter U of this trimmed curve, computes the vector corresponding to the Nth derivative. Warning The returned derivative vector has the same orientation as the derivative vector of the basis curve, even if the trimmed curve does not have the same orientation as the basis curve. Exceptions Standard_RangeError if N is less than 1. geometric transformations"]
        #[cxx_name = "Geom2d_TrimmedCurve_DN"]
        fn TrimmedCurve_dn(self_: &TrimmedCurve, U: f64, N: i32) -> UniquePtr<gp_Vec2d>;
        #[doc = "Creates a new object, which is a copy of this trimmed curve."]
        #[cxx_name = "Geom2d_TrimmedCurve_Copy"]
        fn TrimmedCurve_copy(self_: &TrimmedCurve) -> UniquePtr<HandleGeom2dGeometry>;
        #[cxx_name = "Geom2d_TrimmedCurve_get_type_name"]
        fn TrimmedCurve_get_type_name() -> String;
    }
    impl UniquePtr<Curve> {}
    impl UniquePtr<Ellipse> {}
    impl UniquePtr<TrimmedCurve> {}
}
pub use ffi::{Curve, Ellipse};
impl Ellipse {
    #[doc = "Creates an ellipse by conversion of the gp_Elips2d ellipse E."]
    pub fn new_elips2d(E: &ffi::gp_Elips2d) -> cxx::UniquePtr<Self> {
        ffi::Ellipse_ctor_elips2d(E)
    }

    #[doc = "Creates an ellipse defined by its major and minor radii, MajorRadius and MinorRadius, and positioned in the plane by its major axis MajorAxis; the center of the ellipse is the origin of MajorAxis and the unit vector of MajorAxis is the \"X Direction\" of the local coordinate system of the ellipse; this coordinate system is direct if Sense is true (default value) or indirect if Sense is false. Warnings : It is not forbidden to create an ellipse with MajorRadius = MinorRadius. Exceptions Standard_ConstructionError if: - MajorRadius is less than MinorRadius, or - MinorRadius is less than 0."]
    pub fn new_ax2d_real2_bool(
        MajorAxis: &ffi::gp_Ax2d,
        MajorRadius: f64,
        MinorRadius: f64,
        Sense: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::Ellipse_ctor_ax2d_real2_bool(MajorAxis, MajorRadius, MinorRadius, Sense)
    }

    #[doc = "Creates an ellipse defined by its major and minor radii, MajorRadius and MinorRadius, where the coordinate system Axis locates the ellipse and defines its orientation in the plane such that: - the center of the ellipse is the origin of Axis, - the \"X Direction\" of Axis defines the major axis of the ellipse, - the \"Y Direction\" of Axis defines the minor axis of the ellipse, - the orientation of Axis (direct or indirect) gives the orientation of the ellipse. Warnings : It is not forbidden to create an ellipse with MajorRadius = MinorRadius. Exceptions Standard_ConstructionError if: - MajorRadius is less than MinorRadius, or - MinorRadius is less than 0."]
    pub fn new_ax22d_real2(
        Axis: &ffi::gp_Ax22d,
        MajorRadius: f64,
        MinorRadius: f64,
    ) -> cxx::UniquePtr<Self> {
        ffi::Ellipse_ctor_ax22d_real2(Axis, MajorRadius, MinorRadius)
    }
}
pub use ffi::TrimmedCurve;
impl TrimmedCurve {
    #[doc = "Creates a trimmed curve from the basis curve C limited between U1 and U2. . U1 can be greater or lower than U2. . The returned curve is oriented from U1 to U2. . If the basis curve C is periodic there is an ambiguity because two parts are available. In this case by default the trimmed curve has the same orientation as the basis curve (Sense = True). If Sense = False then the orientation of the trimmed curve is opposite to the orientation of the basis curve C. If the curve is closed but not periodic it is not possible to keep the part of the curve including the junction point (except if the junction point is at the beginning or at the end of the trimmed curve) because you could lose the fundamental characteristics of the basis curve which are used for example to compute the derivatives of the trimmed curve. So for a closed curve the rules are the same as for a open curve. Warnings : In this package the entities are not shared. The TrimmedCurve is built with a copy of the curve C. So when C is modified the TrimmedCurve is not modified Warnings : If <C> is periodic and <theAdjustPeriodic> is True, parametrics bounds of the TrimmedCurve, can be different to [<U1>;<U2>}, if <U1> or <U2> are not in the principal period. Include : For more explanation see the scheme given with this class. Raises ConstructionError the C is not periodic and U1 or U2 are out of the bounds of C. Raised if U1 = U2."]
    pub fn new_handlecurve_real2_bool2(
        C: &ffi::HandleGeom2dCurve,
        U1: f64,
        U2: f64,
        Sense: bool,
        theAdjustPeriodic: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::TrimmedCurve_ctor_handlecurve_real2_bool2(C, U1, U2, Sense, theAdjustPeriodic)
    }
}
