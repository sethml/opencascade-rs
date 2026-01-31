// Module BRepIntCurveSurface
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    BRepIntCurveSurface_Inter_ctor as Inter_ctor,
    BRepIntCurveSurface_Inter_face as Inter_face,
    BRepIntCurveSurface_Inter_point as Inter_point,
};

// Type aliases
pub type Inter = crate::ffi::BRepIntCurveSurface_Inter;
