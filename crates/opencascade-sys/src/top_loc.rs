// Module TopLoc
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    TopLoc_Location_Transformation as Location_Transformation,
    TopLoc_Location_ctor as Location_ctor,
    TopLoc_Location_from_transform as Location_from_transform,
};

// Type aliases
pub type Location = crate::ffi::TopLoc_Location;
