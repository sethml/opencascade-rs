use glam::DVec2;
use opencascade_sys::{law, t_colgp};

use crate::primitives::make_point2d;

/// Creates a Law_Function handle from a set of (parameter, radius) pairs.
/// Used for variable-radius pipe shell sweeps and variable fillets.
#[must_use]
pub(crate) fn law_function_from_graph(
    pairs: impl IntoIterator<Item = (f64, f64)>,
) -> opencascade_sys::OwnedPtr<law::HandleLawFunction> {
    let pairs: Vec<(f64, f64)> = pairs.into_iter().collect();
    let n = pairs.len() as i32;

    let mut array = t_colgp::Array1OfPnt2d::new_int2(1, n);
    for (i, &(param, radius)) in pairs.iter().enumerate() {
        let pnt2d = make_point2d(DVec2::new(param, radius));
        array.set_value(i as i32 + 1, &pnt2d);
    }

    let mut interpol = law::Interpol::new();
    interpol.set_array1ofpnt2d_bool(&array, false);

    let handle = law::Interpol::to_handle(interpol);
    handle.to_handle_function()
}
