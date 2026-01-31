// Module TColgp
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    TColgp_Array1OfDir_Value as Array1OfDir_Value,
    TColgp_Array1OfDir_ctor as Array1OfDir_ctor,
    TColgp_Array1OfPnt2d_Value as Array1OfPnt2d_Value,
    TColgp_Array1OfPnt2d_ctor as Array1OfPnt2d_ctor,
    TColgp_Array2OfPnt_ctor as Array2OfPnt_ctor,
    TColgp_HArray1OfPnt_Value as HArray1OfPnt_Value,
    TColgp_HArray1OfPnt_ctor as HArray1OfPnt_ctor,
    new_HandleTColgpHArray1OfPnt_from_TColgpHArray1OfPnt,
};

// Type aliases
pub use crate::ffi::Handle_TColgpHArray1OfPnt;
pub type Array1OfDir = crate::ffi::TColgp_Array1OfDir;
pub type Array1OfPnt2d = crate::ffi::TColgp_Array1OfPnt2d;
pub type Array2OfPnt = crate::ffi::TColgp_Array2OfPnt;
pub type HArray1OfPnt = crate::ffi::TColgp_HArray1OfPnt;
