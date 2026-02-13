// NOTE: This file is partially blocked because:
// - Many helper functions (cast_face_to_shape, map_shapes, outer_wire) not generated
// See TRANSITION_PLAN.md for details.

use crate::{
    angle::Angle,
    primitives::{make_axis_1, make_vec, Shape, Solid, Surface, Wire},
    workplane::Workplane,
};
use cxx::UniquePtr;
use glam::DVec3;
use opencascade_sys::{b_rep_builder_api, b_rep_feat, b_rep_fillet_api, b_rep_prim_api, message, top_abs, top_exp, topo_ds};

pub struct Face {
    pub(crate) inner: UniquePtr<topo_ds::Face>,
}

impl AsRef<Face> for Face {
    fn as_ref(&self) -> &Face {
        self
    }
}

impl Face {
    pub(crate) fn from_face(face: &topo_ds::Face) -> Self {
        let inner = face.to_owned();
        Self { inner }
    }

    fn from_make_face(mut make_face: UniquePtr<b_rep_builder_api::MakeFace>) -> Self {
        Self::from_face(make_face.pin_mut().face())
    }

    pub fn from_wire(wire: &Wire) -> Self {
        let only_plane = false;
        let make_face = b_rep_builder_api::MakeFace::new_wire_bool(&wire.inner, only_plane);
        Self::from_make_face(make_face)
    }

    pub fn from_surface(surface: &Surface) -> Self {
        let tol_degen = 1.0e-6;
        let make_face =
            b_rep_builder_api::MakeFace::new_handlesurface_real(&surface.inner, tol_degen);
        Self::from_make_face(make_face)
    }

    #[must_use]
    pub fn extrude(&self, dir: DVec3) -> Solid {
        let prism_vec = make_vec(dir);
        let copy = false;
        let canonize = true;

        let inner_shape = self.inner.as_shape();
        let mut make_solid = b_rep_prim_api::MakePrism::new_shape_vec_bool2(
            inner_shape, &prism_vec, copy, canonize
        );
        // Use last_shape() which returns the final shape of the prism
        let extruded_shape = make_solid.pin_mut().last_shape();
        let solid = topo_ds::solid(&extruded_shape);

        Solid::from_solid(solid)
    }

    #[must_use]
    pub fn extrude_to_face(&self, shape_with_face: &Shape, face: &Face) -> Shape {
        let profile_base = &self.inner;
        let sketch_base = topo_ds::Face::new();
        let angle = 0.0;
        let fuse = 1; // 1 = additive (Boolean fusion)
        let modify = false;

        let mut make_prism = b_rep_feat::MakeDPrism::new_shape_face2_real_int_bool(
            &shape_with_face.inner,
            profile_base,
            &sketch_base,
            angle,
            fuse,
            modify,
        );

        let until_shape = face.inner.as_shape();
        make_prism.pin_mut().perform_shape(until_shape);

        let make_shape = make_prism.pin_mut().as_b_rep_builder_api_make_shape_mut();
        Shape::from_shape(make_shape.shape())
    }

    #[must_use]
    pub fn subtractive_extrude(&self, shape_with_face: &Shape, height: f64) -> Shape {
        let profile_base = &self.inner;
        let sketch_base = topo_ds::Face::new();
        let angle = 0.0;
        let fuse = 0; // 0 = subtractive (Boolean cut)
        let modify = false;

        let mut make_prism = b_rep_feat::MakeDPrism::new_shape_face2_real_int_bool(
            &shape_with_face.inner,
            profile_base,
            &sketch_base,
            angle,
            fuse,
            modify,
        );

        make_prism.pin_mut().perform_real(height);

        let make_shape = make_prism.pin_mut().as_b_rep_builder_api_make_shape_mut();
        Shape::from_shape(make_shape.shape())
    }

    #[must_use]
    pub fn revolve(&self, origin: DVec3, axis: DVec3, angle: Option<Angle>) -> Solid {
        let revol_axis = make_axis_1(origin, axis);
        let angle = angle.map(Angle::radians).unwrap_or(std::f64::consts::PI * 2.0);
        let copy = false;

        let inner_shape = self.inner.as_shape();
        let mut make_solid =
            b_rep_prim_api::MakeRevol::new_shape_ax1_real_bool(inner_shape, &revol_axis, angle, copy);
        let make_shape = make_solid.pin_mut().as_b_rep_builder_api_make_shape_mut();
        let revolved_shape = make_shape.shape();
        let solid = topo_ds::solid(revolved_shape);

        Solid::from_solid(solid)
    }

    #[must_use]
    pub fn fillet(&self, radius: f64) -> Self {
        let mut make_fillet = b_rep_fillet_api::MakeFillet2d::new_face(&self.inner);
        let explorer = top_exp::Explorer::new_shape_shapeenum2(
            self.inner.as_shape(),
            top_abs::ShapeEnum::Vertex.into(),
            top_abs::ShapeEnum::Shape.into(),
        );
        // Iterate all vertices and add fillets
        let mut explorer = explorer;
        while explorer.more() {
            let vertex = topo_ds::vertex(explorer.current());
            make_fillet.pin_mut().add_fillet(vertex, radius);
            explorer.pin_mut().next();
        }
        let progress = message::ProgressRange::new();
        make_fillet.pin_mut().build(&progress);
        let shape = make_fillet.pin_mut().shape();
        let face = topo_ds::face(shape);
        Face::from_face(face)
    }

    #[must_use]
    pub fn chamfer(&self, distance: f64) -> Self {
        let mut make_fillet = b_rep_fillet_api::MakeFillet2d::new_face(&self.inner);
        let explorer = top_exp::Explorer::new_shape_shapeenum2(
            self.inner.as_shape(),
            top_abs::ShapeEnum::Vertex.into(),
            top_abs::ShapeEnum::Shape.into(),
        );
        // Iterate all vertices — for each vertex, we need two adjacent edges
        // MakeFillet2d::AddChamfer needs the vertex and adjacent edge info
        // Use the simpler approach: iterate edges and chamfer between adjacent pairs
        // Actually, the AddChamfer(E, V, D, Ang) form is simpler
        let mut explorer = explorer;
        while explorer.more() {
            let shape = explorer.current();
            let vertex = topo_ds::vertex(shape);
            // Get an adjacent edge for the chamfer
            let edge_explorer = top_exp::Explorer::new_shape_shapeenum2(
                self.inner.as_shape(),
                top_abs::ShapeEnum::Edge.into(),
                top_abs::ShapeEnum::Shape.into(),
            );
            if edge_explorer.more() {
                let edge = topo_ds::edge(edge_explorer.current());
                make_fillet.pin_mut().add_chamfer_edge_vertex_real2(
                    edge,
                    vertex,
                    distance,
                    std::f64::consts::FRAC_PI_4, // 45 degree chamfer
                );
            }
            explorer.pin_mut().next();
        }
        let progress = message::ProgressRange::new();
        make_fillet.pin_mut().build(&progress);
        let shape = make_fillet.pin_mut().shape();
        let face = topo_ds::face(shape);
        Face::from_face(face)
    }

    #[must_use]
    pub fn offset(&self, distance: f64, join_type: super::JoinType) -> Self {
        use opencascade_sys::{b_rep_offset_api, geom_abs};
        let join: geom_abs::JoinType = join_type.into();
        let mut make_offset = b_rep_offset_api::MakeOffset::new_face_jointype_bool(
            &self.inner,
            join.into(),
            false, // IsOpenResult
        );
        make_offset.pin_mut().perform(distance, 0.0);
        let shape = make_offset.pin_mut().shape();
        let face = topo_ds::face(shape);
        Face::from_face(face)
    }

    // NOTE: sweep_along is blocked because BRepOffsetAPI_MakePipe is not generated
    #[allow(unused)]
    #[must_use]
    pub fn sweep_along(&self, _path: &Wire) -> Solid {
        unimplemented!(
            "Face::sweep_along is blocked pending BRepOffsetAPI_MakePipe support"
        );
    }

    // NOTE: sweep_along_with_radius_values is blocked because law_function is blocked
    #[allow(unused)]
    #[must_use]
    pub fn sweep_along_with_radius_values(
        &self,
        _path: &Wire,
        _radius_values: impl IntoIterator<Item = (f64, f64)>,
    ) -> Solid {
        unimplemented!(
            "Face::sweep_along_with_radius_values is blocked pending law_function support"
        );
    }

    pub fn edges(&self) -> super::EdgeIterator {
        let explorer = top_exp::Explorer::new_shape_shapeenum2(
            self.inner.as_shape(),
            top_abs::ShapeEnum::Edge.into(),
            top_abs::ShapeEnum::Shape.into(),
        );
        super::EdgeIterator { explorer }
    }

    // NOTE: center_of_mass is blocked because BRepGProp_SurfaceProperties is not generated
    #[allow(unused)]
    pub fn center_of_mass(&self) -> DVec3 {
        unimplemented!(
            "Face::center_of_mass is blocked pending BRepGProp_SurfaceProperties support"
        );
    }

    // NOTE: normal_at is blocked because BRep_Tool_Surface and related are not generated
    #[allow(unused)]
    pub fn normal_at(&self, _pos: DVec3) -> DVec3 {
        unimplemented!(
            "Face::normal_at is blocked pending BRep_Tool_Surface support"
        );
    }

    // NOTE: normal_at_center is blocked because it depends on center_of_mass and normal_at
    #[allow(unused)]
    pub fn normal_at_center(&self) -> DVec3 {
        unimplemented!(
            "Face::normal_at_center is blocked pending center_of_mass and normal_at support"
        );
    }

    // NOTE: workplane is blocked because it depends on center_of_mass and normal_at
    #[allow(unused)]
    pub fn workplane(&self) -> Workplane {
        unimplemented!(
            "Face::workplane is blocked pending center_of_mass and normal_at support"
        );
    }

    // NOTE: union is blocked because BRepAlgoAPI_Fuse needs as_shape upcast
    #[allow(unused)]
    pub fn union(&self, _other: &Face) -> CompoundFace {
        unimplemented!(
            "Face::union is blocked pending boolean operation support"
        );
    }

    // NOTE: intersect is blocked because BRepAlgoAPI_Common needs as_shape upcast
    #[allow(unused)]
    #[must_use]
    pub fn intersect(&self, _other: &Face) -> CompoundFace {
        unimplemented!(
            "Face::intersect is blocked pending boolean operation support"
        );
    }

    // NOTE: subtract is blocked because BRepAlgoAPI_Cut needs as_shape upcast
    #[allow(unused)]
    pub fn subtract(&self, _other: &Face) -> CompoundFace {
        unimplemented!(
            "Face::subtract is blocked pending boolean operation support"
        );
    }

    // NOTE: surface_area is blocked because BRepGProp_SurfaceProperties is not generated
    #[allow(unused)]
    pub fn surface_area(&self) -> f64 {
        unimplemented!(
            "Face::surface_area is blocked pending BRepGProp_SurfaceProperties support"
        );
    }

    pub fn orientation(&self) -> FaceOrientation {
        let raw = self.inner.as_shape().orientation();
        let orient = top_abs::Orientation::try_from(raw)
            .expect("Invalid Orientation value from OCCT");
        orient.into()
    }

    // NOTE: outer_wire is blocked because the outer_wire helper function is not generated
    #[allow(unused)]
    #[must_use]
    pub fn outer_wire(&self) -> Wire {
        unimplemented!(
            "Face::outer_wire is blocked pending outer_wire helper function support"
        );
    }
}

pub struct CompoundFace {
    inner: UniquePtr<topo_ds::Compound>,
}

impl AsRef<CompoundFace> for CompoundFace {
    fn as_ref(&self) -> &CompoundFace {
        self
    }
}

impl From<Face> for CompoundFace {
    fn from(_face: Face) -> Self {
        unimplemented!(
            "CompoundFace::from(Face) is blocked pending BRep_Builder support"
        );
    }
}

impl CompoundFace {
    pub(crate) fn from_compound(compound: &topo_ds::Compound) -> Self {
        let inner = compound.to_owned();
        Self { inner }
    }

    // NOTE: clean is blocked because it depends on Shape::clean
    #[allow(unused)]
    #[must_use]
    pub fn clean(&self) -> Self {
        unimplemented!(
            "CompoundFace::clean is blocked pending Shape::clean support"
        );
    }

    // NOTE: extrude is blocked because cast_compound_to_shape is not generated
    #[allow(unused)]
    #[must_use]
    pub fn extrude(&self, _dir: DVec3) -> Shape {
        unimplemented!(
            "CompoundFace::extrude is blocked pending compound upcast support"
        );
    }

    // NOTE: revolve is blocked because cast_compound_to_shape is not generated
    #[allow(unused)]
    #[must_use]
    pub fn revolve(&self, _origin: DVec3, _axis: DVec3, _angle: Option<Angle>) -> Shape {
        unimplemented!(
            "CompoundFace::revolve is blocked pending compound upcast support"
        );
    }

    // NOTE: union is blocked because BRepAlgoAPI_Fuse is not fully accessible
    #[allow(unused)]
    #[must_use]
    pub fn union(&self, _other: &CompoundFace) -> CompoundFace {
        unimplemented!(
            "CompoundFace::union is blocked pending boolean operation support"
        );
    }

    // NOTE: intersect is blocked because BRepAlgoAPI_Common is not fully accessible
    #[allow(unused)]
    #[must_use]
    pub fn intersect(&self, _other: &CompoundFace) -> CompoundFace {
        unimplemented!(
            "CompoundFace::intersect is blocked pending boolean operation support"
        );
    }

    // NOTE: subtract is blocked because BRepAlgoAPI_Cut is not fully accessible
    #[allow(unused)]
    #[must_use]
    pub fn subtract(&self, _other: &CompoundFace) -> CompoundFace {
        unimplemented!(
            "CompoundFace::subtract is blocked pending boolean operation support"
        );
    }

    // NOTE: set_global_translation is blocked because cast_compound_to_shape is not generated
    #[allow(unused)]
    pub fn set_global_translation(&mut self, _translation: DVec3) {
        unimplemented!(
            "CompoundFace::set_global_translation is blocked pending compound upcast support"
        );
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FaceOrientation {
    Forward,
    Reversed,
    Internal,
    External,
}

impl From<top_abs::Orientation> for FaceOrientation {
    fn from(value: top_abs::Orientation) -> Self {
        match value {
            top_abs::Orientation::Forward => FaceOrientation::Forward,
            top_abs::Orientation::Reversed => FaceOrientation::Reversed,
            top_abs::Orientation::Internal => FaceOrientation::Internal,
            top_abs::Orientation::External => FaceOrientation::External,
        }
    }
}

impl From<FaceOrientation> for top_abs::Orientation {
    fn from(value: FaceOrientation) -> Self {
        match value {
            FaceOrientation::Forward => top_abs::Orientation::Forward,
            FaceOrientation::Reversed => top_abs::Orientation::Reversed,
            FaceOrientation::Internal => top_abs::Orientation::Internal,
            FaceOrientation::External => top_abs::Orientation::External,
        }
    }
}
