// Module STEPControl
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    STEPControl_Reader_ctor as Reader_ctor,
    STEPControl_Writer_ctor as Writer_ctor,
    read_step,
    transfer_shape,
    write_step,
};

// Type aliases
pub type Reader = crate::ffi::STEPControl_Reader;
pub type Writer = crate::ffi::STEPControl_Writer;
