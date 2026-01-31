// Module BRepMesh
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    BRepMesh_IncrementalMesh_ctor as IncrementalMesh_ctor,
};

// Type aliases
pub type IncrementalMesh = crate::ffi::BRepMesh_IncrementalMesh;
