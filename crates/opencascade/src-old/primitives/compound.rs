use crate::primitives::Shape;
use cxx::UniquePtr;
use opencascade_sys::{b_rep, shape_upgrade, topo_ds};

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

    #[must_use]
    pub fn clean(&self) -> Shape {
        let mut unifier = shape_upgrade::UnifySameDomain::new_shape_bool3(
            self.inner.as_shape(),
            true,  // UnifyEdges
            true,  // UnifyFaces
            false, // ConcatBSplines
        );
        unifier.pin_mut().allow_internal_edges(false);
        unifier.pin_mut().build();
        let result = unifier.shape();
        Shape::from_shape(result)
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
