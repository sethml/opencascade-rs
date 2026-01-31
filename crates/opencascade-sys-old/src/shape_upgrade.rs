// Module ShapeUpgrade
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    ShapeUpgrade_UnifySameDomain_ctor as UnifySameDomain_ctor,
};

// Type aliases
pub type UnifySameDomain = crate::ffi::ShapeUpgrade_UnifySameDomain;
