// Module Geom
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    Geom_BezierCurve_ctor_points as BezierCurve_ctor_points,
    Geom_BezierCurve_to_handle as BezierCurve_to_handle,
    Geom_BezierSurface_ctor as BezierSurface_ctor,
    Geom_CylindricalSurface_ctor as CylindricalSurface_ctor,
    HandleGeomCurve_Value,
    bezier_to_surface,
    cylinder_to_surface,
    handle_geom_plane_location,
    new_HandleGeomCurve_from_HandleGeom_BSplineCurve,
    new_HandleGeomCurve_from_HandleGeom_BezierCurve,
    new_HandleGeomCurve_from_HandleGeom_TrimmedCurve,
    new_HandleGeomPlane_from_HandleGeomSurface,
};

// Type aliases
pub type BezierCurve = crate::ffi::Geom_BezierCurve;
pub type BezierSurface = crate::ffi::Geom_BezierSurface;
pub type CylindricalSurface = crate::ffi::Geom_CylindricalSurface;
pub type TrimmedCurve = crate::ffi::Geom_TrimmedCurve;
pub use crate::ffi::HandleGeomBSplineCurve;
pub use crate::ffi::HandleGeomBezierCurve;
pub use crate::ffi::HandleGeomBezierSurface;
pub use crate::ffi::HandleGeomCurve;
pub use crate::ffi::HandleGeomPlane;
pub use crate::ffi::HandleGeomSurface;
pub use crate::ffi::HandleGeomTrimmedCurve;
pub use crate::ffi::HandleGeom_CylindricalSurface;
