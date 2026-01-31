// Module BRepPrimAPI
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    BRepPrimAPI_MakeBox_ctor as MakeBox_ctor,
    BRepPrimAPI_MakeCone_ctor as MakeCone_ctor,
    BRepPrimAPI_MakeCylinder_ctor as MakeCylinder_ctor,
    BRepPrimAPI_MakePrism_ctor as MakePrism_ctor,
    BRepPrimAPI_MakeRevol_ctor as MakeRevol_ctor,
    BRepPrimAPI_MakeSphere_ctor as MakeSphere_ctor,
    BRepPrimAPI_MakeTorus_ctor as MakeTorus_ctor,
};

// Type aliases
pub type MakeBox = crate::ffi::BRepPrimAPI_MakeBox;
pub type MakeCone = crate::ffi::BRepPrimAPI_MakeCone;
pub type MakeCylinder = crate::ffi::BRepPrimAPI_MakeCylinder;
pub type MakePrism = crate::ffi::BRepPrimAPI_MakePrism;
pub type MakeRevol = crate::ffi::BRepPrimAPI_MakeRevol;
pub type MakeSphere = crate::ffi::BRepPrimAPI_MakeSphere;
pub type MakeTorus = crate::ffi::BRepPrimAPI_MakeTorus;
