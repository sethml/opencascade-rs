// Module TopExp
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    ExplorerCurrentShape,
    TopExp_CommonVertex as CommonVertex,
    TopExp_EdgeVertices as EdgeVertices,
    TopExp_Explorer_ctor as Explorer_ctor,
    TopExp_FirstVertex as FirstVertex,
    TopExp_LastVertex as LastVertex,
    TopExp_WireVertices as WireVertices,
};

// Type aliases
pub type Explorer = crate::ffi::TopExp_Explorer;
