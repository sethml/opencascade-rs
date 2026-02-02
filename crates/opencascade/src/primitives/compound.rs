// NOTE: This module is blocked because:
// - BRep_Builder constructor not generated (implicit default ctor)
// - Need BRep_Builder::MakeCompound and TopoDS_Builder::Add
// See TRANSITION_PLAN.md for details.

use crate::primitives::Shape;
use cxx::UniquePtr;
use opencascade_sys::topo_ds;

pub struct Compound {
    pub(crate) inner: UniquePtr<topo_ds::Compound>,
}

impl AsRef<Compound> for Compound {
    fn as_ref(&self) -> &Compound {
        self
    }
}

impl Compound {
    pub(crate) fn from_compound(compound: &topo_ds::Compound) -> Self {
        let inner = compound.to_owned();

        Self { inner }
    }

    // Stub implementation - blocked due to missing BRep_Builder
    #[allow(unused)]
    #[must_use]
    pub fn clean(&self) -> Shape {
        unimplemented!(
            "Compound::clean is blocked pending BRep_Builder constructor support"
        );
    }

    // Stub implementation - blocked due to missing BRep_Builder
    #[allow(unused)]
    pub fn from_shapes<T: AsRef<Shape>>(_shapes: impl IntoIterator<Item = T>) -> Self {
        unimplemented!(
            "Compound::from_shapes is blocked pending BRep_Builder constructor support"
        );
    }
}
