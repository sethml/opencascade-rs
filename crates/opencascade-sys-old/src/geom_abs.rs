// Module GeomAbs
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
};

// Type aliases
pub type CurveType = crate::ffi::GeomAbs_CurveType;
pub type JoinType = crate::ffi::GeomAbs_JoinType;
