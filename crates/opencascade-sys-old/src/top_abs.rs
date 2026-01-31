// Module TopAbs
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
};

// Type aliases
pub type Orientation = crate::ffi::TopAbs_Orientation;
pub type ShapeEnum = crate::ffi::TopAbs_ShapeEnum;
