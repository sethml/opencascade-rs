// NOTE: This file is partially blocked because:
// - BRepFilletAPI_MakeFillet needs shape upcasts
// - BRepAlgoAPI_Cut/Fuse/Common need shape upcasts and shape_list_to_vector
// See TRANSITION_PLAN.md for details.

use crate::{
    primitives::{BooleanShape, Compound, Edge, Face, Wire},
    Error,
};
use cxx::UniquePtr;
use glam::{dvec3, DVec3};
use opencascade_sys::{b_rep_offset_api, topo_ds};

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

    // NOTE: fillet_edge is blocked because cast_solid_to_shape and BRepFilletAPI_MakeFillet
    // aren't fully accessible
    #[allow(unused)]
    #[must_use]
    pub fn fillet_edge(&self, _radius: f64, _edge: &Edge) -> Compound {
        unimplemented!(
            "Solid::fillet_edge is blocked pending BRepFilletAPI_MakeFillet support"
        );
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

    // NOTE: subtract is blocked because BRepAlgoAPI_Cut needs shape upcasts
    #[allow(unused)]
    #[must_use]
    pub fn subtract(&self, _other: &Solid) -> BooleanShape {
        unimplemented!(
            "Solid::subtract is blocked pending BRepAlgoAPI_Cut and shape list support"
        );
    }

    // NOTE: union is blocked because BRepAlgoAPI_Fuse needs shape upcasts
    #[allow(unused)]
    #[must_use]
    pub fn union(&self, _other: &Solid) -> BooleanShape {
        unimplemented!(
            "Solid::union is blocked pending BRepAlgoAPI_Fuse and shape list support"
        );
    }

    // NOTE: intersect is blocked because BRepAlgoAPI_Common needs shape upcasts
    #[allow(unused)]
    #[must_use]
    pub fn intersect(&self, _other: &Solid) -> BooleanShape {
        unimplemented!(
            "Solid::intersect is blocked pending BRepAlgoAPI_Common and shape list support"
        );
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
