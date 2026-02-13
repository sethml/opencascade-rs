// NOTE: This module is blocked because:
// - Law_Interpol::Set() is not generated (has Standard_Boolean Periodic = Standard_False
//   default parameter that is filtered as an enum value)
// - TColgp_Array1OfPnt2d constructors and set_value ARE now available via ffi impl methods
// See TRANSITION_PLAN.md for details.

use cxx::UniquePtr;
use opencascade_sys::law;

// use crate::primitives::make_point2d; // Not used in stubbed implementation

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
