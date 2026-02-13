use crate::primitives::Shape;
use cxx::UniquePtr;
use opencascade_sys::{b_rep, topo_ds};

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

    // NOTE: clean is blocked because ShapeUpgrade_UnifySameDomain build/shape
    // methods are not in module re-exports
    #[allow(unused)]
    #[must_use]
    pub fn clean(&self) -> Shape {
        unimplemented!(
            "Compound::clean is blocked pending ShapeUpgrade_UnifySameDomain re-export of build()/shape()"
        );
    }

    pub fn from_shapes<T: AsRef<Shape>>(shapes: impl IntoIterator<Item = T>) -> Self {
        let mut compound = topo_ds::Compound::new();
        let builder = b_rep::Builder::new();
        builder.make_compound(compound.pin_mut());
        let mut compound_shape = compound.as_shape().to_owned();
        for shape in shapes.into_iter() {
            builder.add(compound_shape.pin_mut(), &shape.as_ref().inner);
        }
        let result_compound = topo_ds::compound(&compound_shape);
        Self::from_compound(result_compound)
    }
}
