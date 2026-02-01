#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_law.hxx");
        #[doc = "HArray1OfPnt from t_colgp module"]
        type TColgp_HArray1OfPnt = crate::t_colgp::ffi::HArray1OfPnt;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "GeomAbs_Shape"]
        type GeomAbs_Shape;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TColgp_Array1OfPnt2d"]
        type TColgp_Array1OfPnt2d;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TColStd_Array1OfReal"]
        type TColStd_Array1OfReal;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Standard_Type"]
        type Standard_Type;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleLawFunction"]
        type HandleLawFunction;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleStandardType"]
        type HandleStandardType;
        #[doc = " ======================== Law_Function ========================"]
        #[doc = "Root class for evolution laws."]
        #[cxx_name = "Law_Function"]
        type Function;
        #[doc = "Returns the value of the function at the point of parameter X."]
        #[cxx_name = "Value"]
        fn value(self: Pin<&mut Function>, X: f64) -> f64;
        #[doc = "Returns the value F and the first derivative D of the function at the point of parameter X."]
        #[cxx_name = "D1"]
        fn d1(self: Pin<&mut Function>, X: f64, F: &mut f64, D: &mut f64);
        #[doc = "Returns the value, first and seconde derivatives at parameter X."]
        #[cxx_name = "D2"]
        fn d2(self: Pin<&mut Function>, X: f64, F: &mut f64, D: &mut f64, D2: &mut f64);
        #[doc = "Returns the parametric bounds of the function."]
        #[cxx_name = "Bounds"]
        fn bounds(self: Pin<&mut Function>, PFirst: &mut f64, PLast: &mut f64);
        #[cxx_name = "DynamicType"]
        fn dynamic_type(self: &Function) -> &HandleStandardType;
        #[cxx_name = "Law_Function_Continuity"]
        fn Function_continuity(self_: &Function) -> UniquePtr<GeomAbs_Shape>;
        #[doc = "Returns a  law equivalent of  <me>  between parameters <First>  and <Last>. <Tol>  is used  to test for 3d points confusion. It is usfule to determines the derivatives in these values <First> and <Last> if the Law is not Cn."]
        #[cxx_name = "Law_Function_Trim"]
        fn Function_trim(
            self_: &Function,
            PFirst: f64,
            PLast: f64,
            Tol: f64,
        ) -> UniquePtr<HandleLawFunction>;
        #[cxx_name = "Law_Function_get_type_name"]
        fn Function_get_type_name() -> String;
        #[doc = " ======================== Law_Interpol ========================"]
        #[doc = "Provides an evolution law that interpolates a set of parameter and value pairs (wi, radi)"]
        #[cxx_name = "Law_Interpol"]
        type Interpol;
        #[doc = "Constructs an empty interpolative evolution law. The function Set is used to define the law."]
        #[cxx_name = "Law_Interpol_ctor"]
        fn Interpol_ctor() -> UniquePtr<Interpol>;
        #[doc = "Defines this evolution law by interpolating the set of 2D points ParAndRad. The Y coordinate of a point of ParAndRad is the value of the function at the parameter point given by its X coordinate. If Periodic is true, this function is assumed to be periodic. Warning -   The X coordinates of points in the table ParAndRad must be given in ascendant order. -   If Periodic is true, the first and last Y coordinates of points in the table ParAndRad are assumed to be equal. In addition, with the second syntax, Dd and Df are also assumed to be equal. If this is not the case, Set uses the first value(s) as last value(s)."]
        #[cxx_name = "Set"]
        fn setarray1ofpnt2d(
            self: Pin<&mut Interpol>,
            ParAndRad: &TColgp_Array1OfPnt2d,
            Periodic: bool,
        );
        #[cxx_name = "SetInRelative"]
        fn set_in_relativearray1ofpnt2d(
            self: Pin<&mut Interpol>,
            ParAndRad: &TColgp_Array1OfPnt2d,
            Ud: f64,
            Uf: f64,
            Periodic: bool,
        );
        #[doc = "Defines this evolution law by interpolating the set of 2D points ParAndRad. The Y coordinate of a point of ParAndRad is the value of the function at the parameter point given by its X coordinate. If Periodic is true, this function is assumed to be periodic. In the second syntax, Dd and Df define the values of the first derivative of the function at its first and last points. Warning -   The X coordinates of points in the table ParAndRad must be given in ascendant order. -   If Periodic is true, the first and last Y coordinates of points in the table ParAndRad are assumed to be equal. In addition, with the second syntax, Dd and Df are also assumed to be equal. If this is not the case, Set uses the first value(s) as last value(s)."]
        #[cxx_name = "Set"]
        fn setarray1ofpnt2d_2(
            self: Pin<&mut Interpol>,
            ParAndRad: &TColgp_Array1OfPnt2d,
            Dd: f64,
            Df: f64,
            Periodic: bool,
        );
        #[cxx_name = "SetInRelative"]
        fn set_in_relativearray1ofpnt2d_2(
            self: Pin<&mut Interpol>,
            ParAndRad: &TColgp_Array1OfPnt2d,
            Ud: f64,
            Uf: f64,
            Dd: f64,
            Df: f64,
            Periodic: bool,
        );
        #[cxx_name = "DynamicType"]
        fn dynamic_type(self: &Interpol) -> &HandleStandardType;
        #[cxx_name = "Law_Interpol_get_type_name"]
        fn Interpol_get_type_name() -> String;
    }
    impl UniquePtr<Function> {}
    impl UniquePtr<Interpol> {}
}
pub use ffi::{Function, Interpol};
impl Interpol {
    #[doc = "Constructs an empty interpolative evolution law. The function Set is used to define the law."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Interpol_ctor()
    }
}
