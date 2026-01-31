// Module StlAPI
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    StlAPI_Writer_ctor as Writer_ctor,
    write_stl,
};

// Type aliases
pub type Writer = crate::ffi::StlAPI_Writer;
