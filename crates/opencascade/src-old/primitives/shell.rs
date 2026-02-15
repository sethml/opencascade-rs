use cxx::UniquePtr;
use opencascade_sys::{b_rep_offset_api, topo_ds};

use crate::primitives::Wire;

pub struct Shell {
    pub(crate) inner: UniquePtr<topo_ds::Shell>,
}

impl AsRef<Shell> for Shell {
    fn as_ref(&self) -> &Shell {
        self
    }
}

impl Shell {
    pub(crate) fn from_shell(shell: &topo_ds::Shell) -> Self {
        let inner = shell.to_owned();

        Self { inner }
    }

    pub fn loft<T: AsRef<Wire>>(_wires: impl IntoIterator<Item = T>) -> Self {
        let is_solid = false;
        let ruled = false;
        let precision = 1.0e-6;
        let mut make_loft = b_rep_offset_api::ThruSections::new_bool2_real(is_solid, ruled, precision);

        for wire in _wires.into_iter() {
            make_loft.pin_mut().add_wire(&wire.as_ref().inner);
        }

        make_loft.pin_mut().check_compatibility(true);

        let make_shape = make_loft.pin_mut().as_b_rep_builder_api_make_shape_mut();
        let shell_shape = make_shape.shape();
        let shell = topo_ds::shell(shell_shape);

        Self::from_shell(shell)
    }
}
