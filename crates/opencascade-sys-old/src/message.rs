// Module Message
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    Message_ProgressRange_ctor as ProgressRange_ctor,
};

// Type aliases
pub type ProgressRange = crate::ffi::Message_ProgressRange;
