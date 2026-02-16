use crate::{
    primitives::{BooleanShape, Compound, Edge, Face, Shape, Wire},
    Error,
};
use glam::{dvec3, DVec3};
use opencascade_sys::{
    b_rep_algo_api, b_rep_fillet_api, b_rep_offset_api, ch_fi3d, message, topo_ds,
};

pub struct Solid {
    pub(crate) inner: opencascade_sys::OwnedPtr<topo_ds::Solid>,
}

impl AsRef<Solid> for Solid {
    fn as_ref(&self) -> &Solid {
        self
    }
}

impl Solid {
    pub(crate) fn from_solid(solid: &topo_ds::Solid) -> Self {
        let inner = solid.to_owned();
        Self { inner }
    }

    // TODO(bschwind) - Do some cool stuff from this link:
    // https://neweopencascade.wordpress.com/2018/10/17/lets-talk-about-fillets/
    // Key takeaway: Use the `SectionEdges` function to retrieve edges that were
    // the result of combining two shapes.
    #[must_use]
    pub fn fillet_edge(&self, radius: f64, edge: &Edge) -> Compound {
        let inner_shape = self.inner.as_shape();

        let mut make_fillet = b_rep_fillet_api::MakeFillet::new_shape_filletshape(inner_shape, ch_fi3d::FilletShape::Rational);
        make_fillet.add_real_edge(radius, &edge.inner);

        let filleted_shape = make_fillet.shape();
        let compound = unsafe { &*topo_ds::compound(filleted_shape) };

        Compound::from_compound(compound)
    }

    pub fn loft<T: AsRef<Wire>>(wires: impl IntoIterator<Item = T>) -> Self {
        let is_solid = true;
        let mut make_loft = b_rep_offset_api::ThruSections::new_bool(is_solid);

        for wire in wires.into_iter() {
            make_loft.add_wire(&wire.as_ref().inner);
        }

        // Set to CheckCompatibility to `true` to avoid twisted results.
        make_loft.check_compatibility(true);

        let shape = make_loft.shape();
        let solid = unsafe { &*topo_ds::solid(shape) };

        Self::from_solid(solid)
    }

    #[must_use]
    pub fn subtract(&self, other: &Solid) -> BooleanShape {
        let inner_shape = self.inner.as_shape();
        let other_inner_shape = other.inner.as_shape();

        let progress = message::ProgressRange::new();
        let mut cut_operation =
            b_rep_algo_api::Cut::new_shape2_progressrange(inner_shape, other_inner_shape, &progress);

        let new_edges = collect_section_edges(cut_operation.section_edges());
        let shape = Shape::from_shape(cut_operation.shape());

        BooleanShape { shape, new_edges }
    }

    #[must_use]
    pub fn union(&self, other: &Solid) -> BooleanShape {
        let inner_shape = self.inner.as_shape();
        let other_inner_shape = other.inner.as_shape();

        let progress = message::ProgressRange::new();
        let mut fuse_operation =
            b_rep_algo_api::Fuse::new_shape2_progressrange(inner_shape, other_inner_shape, &progress);

        let new_edges = collect_section_edges(fuse_operation.section_edges());
        let shape = Shape::from_shape(fuse_operation.shape());

        BooleanShape { shape, new_edges }
    }

    #[must_use]
    pub fn intersect(&self, other: &Solid) -> BooleanShape {
        let inner_shape = self.inner.as_shape();
        let other_inner_shape = other.inner.as_shape();

        let progress = message::ProgressRange::new();
        let mut common_operation =
            b_rep_algo_api::Common::new_shape2_progressrange(inner_shape, other_inner_shape, &progress);

        let new_edges = collect_section_edges(common_operation.section_edges());
        let shape = Shape::from_shape(common_operation.shape());

        BooleanShape { shape, new_edges }
    }

    /// Purposefully underpowered for now, this simply takes a list of points,
    /// creates a face out of them, and then extrudes it by h in the positive Z
    /// direction.
    pub fn extrude_polygon(
        points: impl IntoIterator<Item = DVec3>,
        h: f64,
    ) -> Result<Solid, Error> {
        let wire = Wire::from_ordered_points(points)?;
        Ok(Face::from_wire(&wire).extrude(dvec3(0.0, 0.0, h)))
    }
}

fn collect_section_edges(edge_list: &opencascade_sys::top_tools::ListOfShape) -> Vec<Edge> {
    let mut new_edges = vec![];
    let mut iter = edge_list.iter();
    while let Some(shape) = iter.next() {
        let edge = unsafe { &*topo_ds::edge(shape.as_ptr()) };
        new_edges.push(Edge::from_edge(edge));
    }
    new_edges
}
