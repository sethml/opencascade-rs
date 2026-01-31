// Module Law
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    Law_Function_to_handle as Function_to_handle,
    Law_Interpol_ctor as Interpol_ctor,
    Law_Interpol_into_Law_Function as Interpol_into_Law_Function,
};

// Type aliases
pub use crate::ffi::HandleLawFunction;
pub type Function = crate::ffi::Law_Function;
pub type Interpol = crate::ffi::Law_Interpol;
