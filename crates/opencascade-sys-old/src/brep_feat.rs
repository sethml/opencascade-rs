// Module BRepFeat
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    BRepFeat_MakeCylindricalHole_ctor as MakeCylindricalHole_ctor,
    BRepFeat_MakeDPrism_ctor as MakeDPrism_ctor,
};

// Type aliases
pub type MakeCylindricalHole = crate::ffi::BRepFeat_MakeCylindricalHole;
pub type MakeDPrism = crate::ffi::BRepFeat_MakeDPrism;
