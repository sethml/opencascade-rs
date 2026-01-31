// Module GCE2d
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    GCE2d_MakeSegment_point_point as MakeSegment_point_point,
};

// Type aliases
pub type MakeSegment = crate::ffi::GCE2d_MakeSegment;
