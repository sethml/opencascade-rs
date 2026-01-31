// Module GeomAPI
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    GeomAPI_Interpolate_Curve as Interpolate_Curve,
    GeomAPI_Interpolate_ctor as Interpolate_ctor,
    GeomAPI_ProjectPointOnSurf_ctor as ProjectPointOnSurf_ctor,
};

// Type aliases
pub type Interpolate = crate::ffi::GeomAPI_Interpolate;
pub type ProjectPointOnSurf = crate::ffi::GeomAPI_ProjectPointOnSurf;
