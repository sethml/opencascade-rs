// Module BRepAdaptor
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    BRepAdaptor_Curve_ctor as Curve_ctor,
    BRepAdaptor_Curve_value as Curve_value,
};

// Type aliases
pub type Curve = crate::ffi::BRepAdaptor_Curve;
