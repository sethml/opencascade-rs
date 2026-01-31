// Module BRep
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    BRep_Builder,
    BRep_Builder_ctor,
    BRep_Builder_upcast_to_topods_builder,
    BRep_Tool_Curve,
    BRep_Tool_Pnt,
    BRep_Tool_Surface,
    BRep_Tool_Triangulation,
};

// Type aliases
pub type Builder = crate::ffi::BRep_Builder;

