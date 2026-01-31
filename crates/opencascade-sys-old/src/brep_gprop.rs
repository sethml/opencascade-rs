// Module BRepGProp
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    BRepGProp_Face_ctor as Face_ctor,
};

// Type aliases
pub type Face = crate::ffi::BRepGProp_Face;
