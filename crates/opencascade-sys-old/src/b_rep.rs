// Module BRep
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    BRep_Builder_ctor as Builder_ctor,
    BRep_Builder_upcast_to_topods_builder as Builder_upcast_to_topods_builder,
    BRep_Tool_Curve as Tool_Curve,
    BRep_Tool_Pnt as Tool_Pnt,
    BRep_Tool_Surface as Tool_Surface,
    BRep_Tool_Triangulation as Tool_Triangulation,
};

// Type aliases
pub type Builder = crate::ffi::BRep_Builder;
