// Module Bnd
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    Bnd_Box_CornerMax as Box_CornerMax,
    Bnd_Box_CornerMin as Box_CornerMin,
    Bnd_Box_ctor as Box_ctor,
};

// Type aliases
pub type Box = crate::ffi::Bnd_Box;
