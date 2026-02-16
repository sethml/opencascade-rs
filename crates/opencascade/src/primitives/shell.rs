use opencascade_sys::{b_rep_offset_api, topo_ds};

use crate::primitives::Wire;

pub struct Shell {
    pub(crate) inner: opencascade_sys::OwnedPtr<topo_ds::Shell>,
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

    pub fn loft<T: AsRef<Wire>>(wires: impl IntoIterator<Item = T>) -> Self {
        let is_solid = false;
        let mut make_loft = b_rep_offset_api::ThruSections::new_bool(is_solid);

        for wire in wires.into_iter() {
            make_loft.add_wire(&wire.as_ref().inner);
        }

        // Set CheckCompatibility to `true` to avoid twisted results.
        make_loft.check_compatibility(true);

        let shape = make_loft.shape();
        let shell = topo_ds::shell(shape);

        Self::from_shell(shell)
    }
}
