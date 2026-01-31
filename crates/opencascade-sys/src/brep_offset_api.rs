// Module BRepOffsetAPI
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    BRepOffsetAPI_MakeOffset_face_ctor as MakeOffset_face_ctor,
    BRepOffsetAPI_MakeOffset_wire_ctor as MakeOffset_wire_ctor,
    BRepOffsetAPI_MakePipeShell_ctor as MakePipeShell_ctor,
    BRepOffsetAPI_MakePipe_ctor as MakePipe_ctor,
    BRepOffsetAPI_MakeThickSolid_ctor as MakeThickSolid_ctor,
    BRepOffsetAPI_ThruSections_ctor as ThruSections_ctor,
    MakeThickSolidByJoin,
};

// Type aliases
pub type MakeOffset = crate::ffi::BRepOffsetAPI_MakeOffset;
pub type MakePipe = crate::ffi::BRepOffsetAPI_MakePipe;
pub type MakePipeShell = crate::ffi::BRepOffsetAPI_MakePipeShell;
pub type MakeThickSolid = crate::ffi::BRepOffsetAPI_MakeThickSolid;
pub type ThruSections = crate::ffi::BRepOffsetAPI_ThruSections;
