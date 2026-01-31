// Module GC
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    GC_MakeArcOfCircle_Value as MakeArcOfCircle_Value,
    GC_MakeArcOfCircle_point_point_point as MakeArcOfCircle_point_point_point,
    GC_MakeSegment_Value as MakeSegment_Value,
    GC_MakeSegment_point_point as MakeSegment_point_point,
};

// Type aliases
pub type MakeArcOfCircle = crate::ffi::GC_MakeArcOfCircle;
pub type MakeSegment = crate::ffi::GC_MakeSegment;
