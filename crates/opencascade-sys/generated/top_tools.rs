#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    #[doc = "Defined TopTools format version"]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum TopTools_FormatVersion {
        #[doc = "< Does not write CurveOnSurface UV Points into the file."]
        TopTools_FormatVersion_VERSION_1,
        #[doc = "< Stores CurveOnSurface UV Points."]
        TopTools_FormatVersion_VERSION_2,
        #[doc = "On reading format is recognized from Version string."]
        TopTools_FormatVersion_VERSION_3,
        #[doc = "< Current version"]
        TopTools_FormatVersion_CURRENT,
    }
    unsafe extern "C++" {
        include!("wrapper_top_tools.hxx");
    }
}
