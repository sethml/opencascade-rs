// NOTE: This module is blocked because:
// - TColgp_Array1OfPnt2d constructor is not generated (template typedef)
// - TColgp_Array1OfPnt2d::SetValue is not available
// See TRANSITION_PLAN.md for details.

use cxx::UniquePtr;
use opencascade_sys::law;

use crate::primitives::make_point2d;

// Stub implementation - blocked due to missing TColgp_Array1OfPnt2d constructor
#[allow(unused)]
#[must_use]
pub(crate) fn law_function_from_graph(
    _pairs: impl IntoIterator<Item = (f64, f64)>,
) -> UniquePtr<law::Function> {
    unimplemented!(
        "law_function_from_graph is blocked pending TColgp_Array1OfPnt2d constructor support"
    );
}
