// Module IGESControl
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    IGESControl_Reader_ctor as Reader_ctor,
    IGESControl_Writer_ctor as Writer_ctor,
    add_shape,
    compute_model,
    read_iges,
    write_iges,
};

// Type aliases
pub type Reader = crate::ffi::IGESControl_Reader;
pub type Writer = crate::ffi::IGESControl_Writer;
