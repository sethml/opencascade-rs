use crate::law_function::law_function_from_graph;
use crate::primitives::{Solid, Wire};
use opencascade_sys::{b_rep_offset_api, message, topo_ds};

/// Creates a solid by sweeping a wire profile along a spine wire,
/// with a law function controlling the scaling along the path.
#[must_use]
pub(crate) fn make_pipe_shell_with_law_function(
    spine: &Wire,
    profile: &Wire,
    radius_values: impl IntoIterator<Item = (f64, f64)>,
) -> Solid {
    let law_handle = law_function_from_graph(radius_values);

    let mut pipe = b_rep_offset_api::MakePipeShell::new_wire(&spine.inner);
    pipe.pin_mut().set_mode_bool(false);

    let profile_shape = profile.inner.as_shape();
    let with_contact = false;
    let with_correction = true;
    pipe.pin_mut()
        .set_law_shape_handlefunction_bool2(profile_shape, &law_handle, with_contact, with_correction);

    let progress = message::ProgressRange::new();
    pipe.pin_mut().build(&progress);
    pipe.pin_mut().make_solid();

    let result_shape = pipe.pin_mut().shape();
    let solid = topo_ds::solid(result_shape);
    Solid::from_solid(solid)
}

/// Creates a shell by sweeping a wire profile along a spine wire,
/// with a law function controlling the scaling along the path.
#[must_use]
pub(crate) fn make_pipe_shell_with_law_function_shell(
    spine: &Wire,
    profile: &Wire,
    radius_values: impl IntoIterator<Item = (f64, f64)>,
) -> crate::primitives::Shell {
    let law_handle = law_function_from_graph(radius_values);

    let mut pipe = b_rep_offset_api::MakePipeShell::new_wire(&spine.inner);
    pipe.pin_mut().set_mode_bool(false);

    let profile_shape = profile.inner.as_shape();
    let with_contact = false;
    let with_correction = true;
    pipe.pin_mut()
        .set_law_shape_handlefunction_bool2(profile_shape, &law_handle, with_contact, with_correction);

    let progress = message::ProgressRange::new();
    pipe.pin_mut().build(&progress);

    let result_shape = pipe.pin_mut().shape();
    let shell = topo_ds::shell(result_shape);
    crate::primitives::Shell::from_shell(shell)
}
