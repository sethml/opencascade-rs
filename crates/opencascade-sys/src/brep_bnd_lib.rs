// Module BRepBndLib
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    BRepBndLib_Add as Add,
};

// Type aliases
pub use crate::ffi::BRepBndLib;
