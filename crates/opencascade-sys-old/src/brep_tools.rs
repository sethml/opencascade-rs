// Module BRepTools
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    connect_edges_to_wires,
    outer_wire,
};

// Type aliases
