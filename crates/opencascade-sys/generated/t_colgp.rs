#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_t_colgp.hxx");
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
        #[cxx_name = "Standard_Type"]
        type Standard_Type;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TColgp_Array1OfPnt"]
        type TColgp_Array1OfPnt;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleStandardType"]
        type HandleStandardType;
        #[doc = " ======================== TColgp_HArray1OfPnt ========================"]
        #[cxx_name = "TColgp_HArray1OfPnt"]
        type HArray1OfPnt;
        #[cxx_name = "TColgp_HArray1OfPnt_ctor"]
        fn HArray1OfPnt_ctor() -> UniquePtr<HArray1OfPnt>;
        #[cxx_name = "TColgp_HArray1OfPnt_ctor_int2"]
        fn HArray1OfPnt_ctor_int2(theLower: i32, theUpper: i32) -> UniquePtr<HArray1OfPnt>;
        #[cxx_name = "TColgp_HArray1OfPnt_ctor_int2_pnt"]
        fn HArray1OfPnt_ctor_int2_pnt(
            theLower: i32,
            theUpper: i32,
            theValue: &gp_Pnt,
        ) -> UniquePtr<HArray1OfPnt>;
        #[cxx_name = "TColgp_HArray1OfPnt_ctor_array1ofpnt"]
        fn HArray1OfPnt_ctor_array1ofpnt(theOther: &TColgp_Array1OfPnt) -> UniquePtr<HArray1OfPnt>;
        #[cxx_name = "Array1"]
        fn array1(self: &HArray1OfPnt) -> &TColgp_Array1OfPnt;
        #[cxx_name = "ChangeArray1"]
        fn change_array1(self: Pin<&mut HArray1OfPnt>) -> Pin<&mut TColgp_Array1OfPnt>;
        #[cxx_name = "DynamicType"]
        fn dynamic_type(self: &HArray1OfPnt) -> &HandleStandardType;
        #[cxx_name = "TColgp_HArray1OfPnt_get_type_name"]
        fn HArray1OfPnt_get_type_name() -> String;
    }
    impl UniquePtr<HArray1OfPnt> {}
}
pub use ffi::HArray1OfPnt;
impl HArray1OfPnt {
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::HArray1OfPnt_ctor()
    }

    pub fn new_int2(theLower: i32, theUpper: i32) -> cxx::UniquePtr<Self> {
        ffi::HArray1OfPnt_ctor_int2(theLower, theUpper)
    }

    pub fn new_int2_pnt(
        theLower: i32,
        theUpper: i32,
        theValue: &ffi::gp_Pnt,
    ) -> cxx::UniquePtr<Self> {
        ffi::HArray1OfPnt_ctor_int2_pnt(theLower, theUpper, theValue)
    }

    pub fn new_array1ofpnt(theOther: &ffi::TColgp_Array1OfPnt) -> cxx::UniquePtr<Self> {
        ffi::HArray1OfPnt_ctor_array1ofpnt(theOther)
    }
}
