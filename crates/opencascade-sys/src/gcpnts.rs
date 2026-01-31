// Module GCPnts
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    GCPnts_TangentialDeflection_Value as TangentialDeflection_Value,
    GCPnts_TangentialDeflection_ctor as TangentialDeflection_ctor,
};

// Type aliases
pub type TangentialDeflection = crate::ffi::GCPnts_TangentialDeflection;
