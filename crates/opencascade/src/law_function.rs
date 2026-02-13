// NOTE: This module is blocked because:
// - Law_Interpol::Set() method is in FFI but not in module re-exports
// - TColgp_Array1OfPnt2d has no impl block in re-exports (just a type alias)
// See TRANSITION_PLAN.md for details.

use cxx::UniquePtr;
use opencascade_sys::law;

// Stub implementation - blocked due to missing Law_Interpol::Set() re-export
// and TColgp_Array1OfPnt2d constructor re-export
#[allow(unused)]
#[must_use]
pub(crate) fn law_function_from_graph(
    _pairs: impl IntoIterator<Item = (f64, f64)>,
) -> UniquePtr<law::Function> {
    unimplemented!(
        "law_function_from_graph is blocked pending Law_Interpol::set() and TColgp_Array1OfPnt2d re-exports"
    );
}
