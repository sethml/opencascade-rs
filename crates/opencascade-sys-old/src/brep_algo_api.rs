// Module BRepAlgoAPI
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    BRepAlgoAPI_Common_ctor as Common_ctor,
    BRepAlgoAPI_Cut_ctor as Cut_ctor,
    BRepAlgoAPI_Fuse_ctor as Fuse_ctor,
    BRepAlgoAPI_Section_ctor as Section_ctor,
    cast_section_to_builderalgo,
};

// Type aliases
pub use crate::ffi::BOPAlgo_GlueEnum;
pub type Common = crate::ffi::BRepAlgoAPI_Common;
pub type Cut = crate::ffi::BRepAlgoAPI_Cut;
pub type Section = crate::ffi::BRepAlgoAPI_Section;
