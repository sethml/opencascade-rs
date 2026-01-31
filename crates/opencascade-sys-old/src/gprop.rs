// Module GProp
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    BRepGProp_LinearProperties,
    BRepGProp_SurfaceProperties,
    BRepGProp_VolumeProperties,
    GProp_GProps_CentreOfMass as GProps_CentreOfMass,
    GProp_GProps_ctor as GProps_ctor,
};

// Type aliases
pub type GProps = crate::ffi::GProp_GProps;
