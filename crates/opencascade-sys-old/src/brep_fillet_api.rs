// Module BRepFilletAPI
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    BRepFilletAPI_MakeChamfer_ctor as MakeChamfer_ctor,
    BRepFilletAPI_MakeFillet2d_add_chamfer as MakeFillet2d_add_chamfer,
    BRepFilletAPI_MakeFillet2d_add_chamfer_angle as MakeFillet2d_add_chamfer_angle,
    BRepFilletAPI_MakeFillet2d_add_fillet as MakeFillet2d_add_fillet,
    BRepFilletAPI_MakeFillet2d_ctor as MakeFillet2d_ctor,
    BRepFilletAPI_MakeFillet_ctor as MakeFillet_ctor,
};

// Type aliases
pub type MakeChamfer = crate::ffi::BRepFilletAPI_MakeChamfer;
pub type MakeFillet = crate::ffi::BRepFilletAPI_MakeFillet;
pub type MakeFillet2d = crate::ffi::BRepFilletAPI_MakeFillet2d;
