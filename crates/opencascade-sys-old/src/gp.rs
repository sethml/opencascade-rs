// Module providing access to geometric primitives from OpenCASCADE gp package
// This module provides type aliases and re-exports for easier access to gp types.

use cxx::UniquePtr;

// Type aliases for cleaner usage within the gp module context
pub type Pnt = crate::ffi::gp_Pnt;
pub type Pnt2d = crate::ffi::gp_Pnt2d;
pub type Vec = crate::ffi::gp_Vec;
pub type Dir = crate::ffi::gp_Dir;
pub type Dir2d = crate::ffi::gp_Dir2d;
pub type Ax1 = crate::ffi::gp_Ax1;
pub type Ax2 = crate::ffi::gp_Ax2;
pub type Ax3 = crate::ffi::gp_Ax3;
pub type Ax2d = crate::ffi::gp_Ax2d;
pub type Lin = crate::ffi::gp_Lin;
pub type Circ = crate::ffi::gp_Circ;
pub type Trsf = crate::ffi::gp_Trsf;
pub type GTrsf = crate::ffi::gp_GTrsf;

// Re-export all gp functions from the main FFI module for convenience
pub use crate::ffi::{
    new_point, new_point_2d, new_vec, gp_DZ, gp_Dir_ctor, gp_Dir2d_ctor,
    gp_OX, gp_OY, gp_OZ, gp_Ax1_ctor, gp_Ax2_ctor, gp_Ax3_from_gp_Ax2, gp_Ax2d_ctor,
    gp_Lin_ctor, gp_Circ_ctor, new_transform, new_gp_GTrsf
};

// Utility functions for easier construction
pub fn point(x: f64, y: f64, z: f64) -> UniquePtr<Pnt> {
    new_point(x, y, z)
}

pub fn point_2d(x: f64, y: f64) -> UniquePtr<Pnt2d> {
    new_point_2d(x, y)
}

pub fn vec(x: f64, y: f64, z: f64) -> UniquePtr<Vec> {
    new_vec(x, y, z)
}

pub fn dir(x: f64, y: f64, z: f64) -> UniquePtr<Dir> {
    gp_Dir_ctor(x, y, z)
}

pub fn dir_2d(x: f64, y: f64) -> UniquePtr<Dir2d> {
    gp_Dir2d_ctor(x, y)
}

pub fn transform() -> UniquePtr<Trsf> {
    new_transform()
}

pub fn gtransform() -> UniquePtr<GTrsf> {
    new_gp_GTrsf()
}