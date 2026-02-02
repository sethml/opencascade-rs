// NOTE: This module is blocked because:
// - TColgp_Array2OfPnt constructor is not generated (template typedef)
// - geom::ffi::HandleGeomSurface is private (not re-exported)
// See TRANSITION_PLAN.md for details.

use glam::DVec3;

pub struct Surface {
    // Stubbed - the actual inner type (HandleGeomSurface) is private in geom module
    _private: (),
}

impl Surface {
    // Stub implementation - blocked due to missing TColgp_Array2OfPnt constructor
    #[allow(unused)]
    pub fn bezier(_poles: impl IntoIterator<Item = impl IntoIterator<Item = DVec3>>) -> Self {
        unimplemented!(
            "Surface::bezier is blocked pending TColgp_Array2OfPnt constructor support"
        );
    }
}
