// NOTE: This module is blocked because:
// - GProp_GProps::mass() is in FFI but not in module re-exports
// - BRepGProp_Face::normal() is in FFI but not in module re-exports
// - GeomAPI_ProjectPointOnSurf::lower_distance_parameters() is in FFI but not re-exported
// - Poly_Triangle accessors (nodes, triangles) need iterating over triangulation
// See TRANSITION_PLAN.md for details.

use crate::primitives::Shape;
use crate::Error;
use glam::{DVec2, DVec3};

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<DVec3>,
    pub uvs: Vec<DVec2>,
    pub normals: Vec<DVec3>,
    pub indices: Vec<usize>,
}

pub struct Mesher {
    // Stubbed - requires FFI helper functions not generated
    _private: (),
}

impl Mesher {
    #[allow(unused)]
    pub fn try_new(_shape: &Shape, _triangulation_tolerance: f64) -> Result<Self, Error> {
        unimplemented!(
            "Mesher::try_new is blocked pending BRep_Tool_Triangulation and related helper functions"
        );
    }

    #[allow(unused)]
    pub fn mesh(self) -> Result<Mesh, Error> {
        unimplemented!(
            "Mesher::mesh is blocked pending triangulation helper functions"
        );
    }
}
