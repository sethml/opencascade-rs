#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_gc.hxx");
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
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleGeomTrimmedCurve"]
        type HandleGeomTrimmedCurve;
        #[doc = " ======================== GC_MakeArcOfCircle ========================"]
        #[doc = "Implements construction algorithms for an arc of circle in 3D space. The result is a Geom_TrimmedCurve curve. A MakeArcOfCircle object provides a framework for: -   defining the construction of the arc of circle, -   implementing the construction algorithm, and -   consulting the results. In particular, the Value function returns the constructed arc of circle."]
        #[cxx_name = "GC_MakeArcOfCircle"]
        type MakeArcOfCircle;
        #[doc = "Make an arc of circle (TrimmedCurve from Geom) from a circle between two angles Alpha1 and Alpha2 given in radiians."]
        #[cxx_name = "GC_MakeArcOfCircle_ctor_circ_real2_bool"]
        fn MakeArcOfCircle_ctor_circ_real2_bool(
            Circ: &gp_Circ,
            Alpha1: f64,
            Alpha2: f64,
            Sense: bool,
        ) -> UniquePtr<MakeArcOfCircle>;
        #[doc = "Make an arc of circle (TrimmedCurve from Geom) from a circle between point <P> and the angle Alpha given in radians."]
        #[cxx_name = "GC_MakeArcOfCircle_ctor_circ_pnt_real_bool"]
        fn MakeArcOfCircle_ctor_circ_pnt_real_bool(
            Circ: &gp_Circ,
            P: &gp_Pnt,
            Alpha: f64,
            Sense: bool,
        ) -> UniquePtr<MakeArcOfCircle>;
        #[doc = "Make an arc of circle (TrimmedCurve from Geom) from a circle between two points P1 and P2."]
        #[cxx_name = "GC_MakeArcOfCircle_ctor_circ_pnt2_bool"]
        fn MakeArcOfCircle_ctor_circ_pnt2_bool(
            Circ: &gp_Circ,
            P1: &gp_Pnt,
            P2: &gp_Pnt,
            Sense: bool,
        ) -> UniquePtr<MakeArcOfCircle>;
        #[doc = "Make an arc of circle (TrimmedCurve from Geom) from three points P1,P2,P3 between two points P1 and P2."]
        #[cxx_name = "GC_MakeArcOfCircle_ctor_pnt3"]
        fn MakeArcOfCircle_ctor_pnt3(
            P1: &gp_Pnt,
            P2: &gp_Pnt,
            P3: &gp_Pnt,
        ) -> UniquePtr<MakeArcOfCircle>;
        #[doc = "Make an arc of circle (TrimmedCurve from Geom) from two points P1,P2 and the tangente to the solution at the point P1. The orientation of the arc is: -   the sense determined by the order of the points P1, P3 and P2; -   the sense defined by the vector V; or -   for other syntaxes: -   the sense of Circ if Sense is true, or -   the opposite sense if Sense is false. Note: Alpha1, Alpha2 and Alpha are angle values, given in radians. Warning If an error occurs (that is, when IsDone returns false), the Status function returns: -   gce_ConfusedPoints if: -   any 2 of the 3 points P1, P2 and P3 are coincident, or -   P1 and P2 are coincident; or -   gce_IntersectionError if: -   P1, P2 and P3 are collinear and not coincident, or -   the vector defined by the points P1 and P2 is collinear with the vector V."]
        #[cxx_name = "GC_MakeArcOfCircle_ctor_pnt_vec_pnt"]
        fn MakeArcOfCircle_ctor_pnt_vec_pnt(
            P1: &gp_Pnt,
            V: &gp_Vec,
            P2: &gp_Pnt,
        ) -> UniquePtr<MakeArcOfCircle>;
        #[doc = "Returns the constructed arc of circle. Exceptions StdFail_NotDone if no arc of circle is constructed."]
        #[cxx_name = "Value"]
        fn value(self: &MakeArcOfCircle) -> &HandleGeomTrimmedCurve;
        #[doc = " ======================== GC_MakeSegment ========================"]
        #[doc = "Implements construction algorithms for a line segment in 3D space. Makes a segment of Line from the 2 points <P1> and <P2>. The result is a Geom_TrimmedCurve curve. A MakeSegment object provides a framework for: -   defining the construction of the line segment, -   implementing the construction algorithm, and -   consulting the results. In particular, the Value function returns the constructed line segment."]
        #[cxx_name = "GC_MakeSegment"]
        type MakeSegment;
        #[doc = "Make a segment of Line from the 2 points <P1> and <P2>. It returns NullObject if <P1> and <P2> are confused."]
        #[cxx_name = "GC_MakeSegment_ctor_pnt2"]
        fn MakeSegment_ctor_pnt2(P1: &gp_Pnt, P2: &gp_Pnt) -> UniquePtr<MakeSegment>;
        #[doc = "Make a segment of Line from the line <Line1> between the two parameters U1 and U2. It returns NullObject if <U1> is equal <U2>."]
        #[cxx_name = "GC_MakeSegment_ctor_lin_real2"]
        fn MakeSegment_ctor_lin_real2(Line: &gp_Lin, U1: f64, U2: f64) -> UniquePtr<MakeSegment>;
        #[doc = "Make a segment of Line from the line <Line1> between the point <Point> and the parameter Ulast. It returns NullObject if <U1> is equal <U2>."]
        #[cxx_name = "GC_MakeSegment_ctor_lin_pnt_real"]
        fn MakeSegment_ctor_lin_pnt_real(
            Line: &gp_Lin,
            Point: &gp_Pnt,
            Ulast: f64,
        ) -> UniquePtr<MakeSegment>;
        #[doc = "Make a segment of Line from the line <Line1> between the two points <P1> and <P2>. It returns NullObject if <U1> is equal <U2>."]
        #[cxx_name = "GC_MakeSegment_ctor_lin_pnt2"]
        fn MakeSegment_ctor_lin_pnt2(
            Line: &gp_Lin,
            P1: &gp_Pnt,
            P2: &gp_Pnt,
        ) -> UniquePtr<MakeSegment>;
        #[doc = "Returns the constructed line segment."]
        #[cxx_name = "Value"]
        fn value(self: &MakeSegment) -> &HandleGeomTrimmedCurve;
    }
    impl UniquePtr<MakeArcOfCircle> {}
    impl UniquePtr<MakeSegment> {}
}
pub use ffi::MakeArcOfCircle;
impl MakeArcOfCircle {
    #[doc = "Make an arc of circle (TrimmedCurve from Geom) from a circle between two angles Alpha1 and Alpha2 given in radiians."]
    pub fn new_circ_real2_bool(
        Circ: &ffi::gp_Circ,
        Alpha1: f64,
        Alpha2: f64,
        Sense: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakeArcOfCircle_ctor_circ_real2_bool(Circ, Alpha1, Alpha2, Sense)
    }

    #[doc = "Make an arc of circle (TrimmedCurve from Geom) from a circle between point <P> and the angle Alpha given in radians."]
    pub fn new_circ_pnt_real_bool(
        Circ: &ffi::gp_Circ,
        P: &ffi::gp_Pnt,
        Alpha: f64,
        Sense: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakeArcOfCircle_ctor_circ_pnt_real_bool(Circ, P, Alpha, Sense)
    }

    #[doc = "Make an arc of circle (TrimmedCurve from Geom) from a circle between two points P1 and P2."]
    pub fn new_circ_pnt2_bool(
        Circ: &ffi::gp_Circ,
        P1: &ffi::gp_Pnt,
        P2: &ffi::gp_Pnt,
        Sense: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakeArcOfCircle_ctor_circ_pnt2_bool(Circ, P1, P2, Sense)
    }

    #[doc = "Make an arc of circle (TrimmedCurve from Geom) from three points P1,P2,P3 between two points P1 and P2."]
    pub fn new_pnt3(P1: &ffi::gp_Pnt, P2: &ffi::gp_Pnt, P3: &ffi::gp_Pnt) -> cxx::UniquePtr<Self> {
        ffi::MakeArcOfCircle_ctor_pnt3(P1, P2, P3)
    }

    #[doc = "Make an arc of circle (TrimmedCurve from Geom) from two points P1,P2 and the tangente to the solution at the point P1. The orientation of the arc is: -   the sense determined by the order of the points P1, P3 and P2; -   the sense defined by the vector V; or -   for other syntaxes: -   the sense of Circ if Sense is true, or -   the opposite sense if Sense is false. Note: Alpha1, Alpha2 and Alpha are angle values, given in radians. Warning If an error occurs (that is, when IsDone returns false), the Status function returns: -   gce_ConfusedPoints if: -   any 2 of the 3 points P1, P2 and P3 are coincident, or -   P1 and P2 are coincident; or -   gce_IntersectionError if: -   P1, P2 and P3 are collinear and not coincident, or -   the vector defined by the points P1 and P2 is collinear with the vector V."]
    pub fn new_pnt_vec_pnt(
        P1: &ffi::gp_Pnt,
        V: &ffi::gp_Vec,
        P2: &ffi::gp_Pnt,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakeArcOfCircle_ctor_pnt_vec_pnt(P1, V, P2)
    }
}
pub use ffi::MakeSegment;
impl MakeSegment {
    #[doc = "Make a segment of Line from the 2 points <P1> and <P2>. It returns NullObject if <P1> and <P2> are confused."]
    pub fn new_pnt2(P1: &ffi::gp_Pnt, P2: &ffi::gp_Pnt) -> cxx::UniquePtr<Self> {
        ffi::MakeSegment_ctor_pnt2(P1, P2)
    }

    #[doc = "Make a segment of Line from the line <Line1> between the two parameters U1 and U2. It returns NullObject if <U1> is equal <U2>."]
    pub fn new_lin_real2(Line: &ffi::gp_Lin, U1: f64, U2: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeSegment_ctor_lin_real2(Line, U1, U2)
    }

    #[doc = "Make a segment of Line from the line <Line1> between the point <Point> and the parameter Ulast. It returns NullObject if <U1> is equal <U2>."]
    pub fn new_lin_pnt_real(
        Line: &ffi::gp_Lin,
        Point: &ffi::gp_Pnt,
        Ulast: f64,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakeSegment_ctor_lin_pnt_real(Line, Point, Ulast)
    }

    #[doc = "Make a segment of Line from the line <Line1> between the two points <P1> and <P2>. It returns NullObject if <U1> is equal <U2>."]
    pub fn new_lin_pnt2(
        Line: &ffi::gp_Lin,
        P1: &ffi::gp_Pnt,
        P2: &ffi::gp_Pnt,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakeSegment_ctor_lin_pnt2(Line, P1, P2)
    }
}
