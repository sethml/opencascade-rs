// Module Geom2d
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    Geom2d_Ellipse_ctor as Ellipse_ctor,
    Geom2d_TrimmedCurve_ctor as TrimmedCurve_ctor,
    HandleGeom2d_TrimmedCurve_to_curve,
    ellipse_to_HandleGeom2d_Curve,
    ellipse_value,
};

// Type aliases
pub type Curve = crate::ffi::Geom2d_Curve;
pub type Ellipse = crate::ffi::Geom2d_Ellipse;
pub type TrimmedCurve = crate::ffi::Geom2d_TrimmedCurve;
pub use crate::ffi::HandleGeom2d_Curve;
pub use crate::ffi::HandleGeom2d_Ellipse;
pub use crate::ffi::HandleGeom2d_TrimmedCurve;
