use crate::{
    primitives::{BooleanShape, Compound, Edge, Face, Shape, Wire},
    Error,
};
use cxx::UniquePtr;
use glam::{dvec3, DVec3};
use opencascade_sys::{b_rep_algo_api, b_rep_fillet_api, b_rep_offset_api, message, topo_ds};

pub struct Solid {
    pub(crate) inner: UniquePtr<topo_ds::Solid>,
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

    #[must_use]
    pub fn fillet_edge(&self, radius: f64, edge: &Edge) -> Compound {
        let progress = message::ProgressRange::new();
        // ChFi3d_Rational = 0
        let mut make_fillet = b_rep_fillet_api::MakeFillet::new_shape_filletshape(
            self.inner.as_shape(),
            0,
        );
        make_fillet.pin_mut().add_real_edge(radius, &edge.inner);
        make_fillet.pin_mut().build(&progress);
        let shape = make_fillet.pin_mut().shape();
        let compound = topo_ds::compound(shape);
        Compound::from_compound(compound)
    }

    pub fn loft<T: AsRef<Wire>>(wires: impl IntoIterator<Item = T>) -> Self {
        let is_solid = true;
        let ruled = false;
        let precision = 1.0e-6;
        let mut make_loft = b_rep_offset_api::ThruSections::new_bool2_real(is_solid, ruled, precision);

        for wire in wires.into_iter() {
            make_loft.pin_mut().add_wire(&wire.as_ref().inner);
        }

        make_loft.pin_mut().check_compatibility(true);

        let make_shape = make_loft.pin_mut().as_b_rep_builder_api_make_shape_mut();
        let solid_shape = make_shape.shape();
        let solid = topo_ds::solid(solid_shape);

        Solid::from_solid(solid)
    }

    /// Boolean subtraction: returns a new shape with `other` removed from `self`.
    #[must_use]
    pub fn subtract(&self, other: &Solid) -> BooleanShape {
        let progress = message::ProgressRange::new();
        let mut cut = b_rep_algo_api::Cut::new_shape2_progressrange(
            self.inner.as_shape(),
            other.inner.as_shape(),
            &progress,
        );

        // Get the resulting shape
        let make_shape = cut.pin_mut().as_b_rep_builder_api_make_shape_mut();
        let result_shape = make_shape.shape();
        let shape = Shape::from_shape(result_shape);

        // TODO: section_edges returns a TopTools_ListOfShape that is locally declared
        // in b_rep_algo_api::ffi rather than imported from top_tools, so we can't
        // iterate it. This needs a binding generator fix to properly import cross-module types.
        let new_edges: Vec<Edge> = Vec::new();

        BooleanShape { shape, new_edges }
    }

    /// Boolean union: returns a new shape combining `self` and `other`.
    #[must_use]
    pub fn union(&self, other: &Solid) -> BooleanShape {
        let progress = message::ProgressRange::new();
        let mut fuse = b_rep_algo_api::Fuse::new_shape2_progressrange(
            self.inner.as_shape(),
            other.inner.as_shape(),
            &progress,
        );

        // Get the resulting shape
        let make_shape = fuse.pin_mut().as_b_rep_builder_api_make_shape_mut();
        let result_shape = make_shape.shape();
        let shape = Shape::from_shape(result_shape);

        // TODO: section_edges returns a TopTools_ListOfShape that is locally declared
        // in b_rep_algo_api::ffi rather than imported from top_tools, so we can't
        // iterate it. This needs a binding generator fix to properly import cross-module types.
        let new_edges: Vec<Edge> = Vec::new();

        BooleanShape { shape, new_edges }
    }

    /// Boolean intersection: returns a new shape containing only the volume 
    /// common to both `self` and `other`.
    #[must_use]
    pub fn intersect(&self, other: &Solid) -> BooleanShape {
        let progress = message::ProgressRange::new();
        let mut common = b_rep_algo_api::Common::new_shape2_progressrange(
            self.inner.as_shape(),
            other.inner.as_shape(),
            &progress,
        );

        // Get the resulting shape
        let make_shape = common.pin_mut().as_b_rep_builder_api_make_shape_mut();
        let result_shape = make_shape.shape();
        let shape = Shape::from_shape(result_shape);

        // TODO: section_edges returns a TopTools_ListOfShape that is locally declared
        // in b_rep_algo_api::ffi rather than imported from top_tools, so we can't
        // iterate it. This needs a binding generator fix to properly import cross-module types.
        let new_edges: Vec<Edge> = Vec::new();

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
