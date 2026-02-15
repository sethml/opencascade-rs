use crate::primitives::Shape;
use opencascade_sys::{b_rep, topo_ds};

pub struct Compound {
    pub(crate) inner: opencascade_sys::OwnedPtr<topo_ds::Compound>,
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

    #[must_use]
    pub fn clean(&self) -> Shape {
        let shape = self.inner.as_shape();
        Shape::from_shape(shape).clean()
    }

    pub fn from_shapes<T: AsRef<Shape>>(shapes: impl IntoIterator<Item = T>) -> Self {
        let mut compound = topo_ds::Compound::new();
        let builder = b_rep::Builder::new();
        builder.make_compound(&mut compound);

        for shape in shapes.into_iter() {
            builder.add(compound.as_shape_mut(), &shape.as_ref().inner);
        }

        Self::from_compound(&compound)
    }
}
