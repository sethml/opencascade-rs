use std::collections::HashMap;

use crate::{
    angle::Angle,
    make_pipe_shell::make_pipe_shell_with_law_function,
    primitives::{
        make_axis_1, make_point, make_vec, EdgeIterator, JoinType, Shape, Solid, Surface, Wire,
    },
    workplane::Workplane,
};
use cxx::UniquePtr;
use glam::{dvec3, DVec3};
use opencascade_sys::{
    b_rep, b_rep_algo_api, b_rep_builder_api, b_rep_feat, b_rep_fillet_api, b_rep_g_prop,
    b_rep_offset_api, b_rep_prim_api, b_rep_tools, g_prop, geom_api, gp, message, top_abs,
    top_exp, topo_ds,
};

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

    fn from_make_face(make_face: UniquePtr<b_rep_builder_api::MakeFace>) -> Self {
        Self::from_face(make_face.face())
    }

    pub fn from_wire(wire: &Wire) -> Self {
        let only_plane = false;
        let make_face = b_rep_builder_api::MakeFace::new_wire_bool(&wire.inner, only_plane);

        Self::from_make_face(make_face)
    }

    pub fn from_surface(surface: &Surface) -> Self {
        const EDGE_TOLERANCE: f64 = 0.0001;

        let make_face =
            b_rep_builder_api::MakeFace::new_handlegeomsurface_real(&surface.inner, EDGE_TOLERANCE);

        Self::from_make_face(make_face)
    }

    #[must_use]
    pub fn extrude(&self, dir: DVec3) -> Solid {
        let prism_vec = make_vec(dir);

        let copy = false;
        let canonize = true;

        let inner_shape = self.inner.as_shape();
        let mut make_solid =
            b_rep_prim_api::MakePrism::new_shape_vec_bool2(inner_shape, &prism_vec, copy, canonize);
        let extruded_shape = make_solid.pin_mut().shape();
        let solid = topo_ds::solid(extruded_shape);

        Solid::from_solid(solid)
    }

    #[must_use]
    pub fn extrude_to_face(&self, shape_with_face: &Shape, face: &Face) -> Shape {
        let profile_base = &self.inner;
        let sketch_base = topo_ds::Face::new();
        let angle = 0.0;
        let fuse = 1; // 0 = subtractive, 1 = additive
        let modify = false;

        let mut make_prism = b_rep_feat::MakeDPrism::new_shape_face2_real_int_bool(
            &shape_with_face.inner,
            profile_base,
            &sketch_base,
            angle,
            fuse,
            modify,
        );

        let until_face = face.inner.as_shape();
        make_prism.pin_mut().perform_shape(until_face);

        Shape::from_shape(make_prism.pin_mut().shape())
    }

    #[must_use]
    pub fn subtractive_extrude(&self, shape_with_face: &Shape, height: f64) -> Shape {
        let profile_base = &self.inner;
        let sketch_base = topo_ds::Face::new();
        let angle = 0.0;
        let fuse = 0; // 0 = subtractive, 1 = additive
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

        Shape::from_shape(make_prism.pin_mut().shape())
    }

    #[must_use]
    pub fn revolve(&self, origin: DVec3, axis: DVec3, angle: Option<Angle>) -> Solid {
        let revol_vec = make_axis_1(origin, axis);

        let angle = angle.map(Angle::radians).unwrap_or(std::f64::consts::PI * 2.0);
        let copy = false;

        let inner_shape = self.inner.as_shape();
        let mut make_solid =
            b_rep_prim_api::MakeRevol::new_shape_ax1_real_bool(inner_shape, &revol_vec, angle, copy);
        let revolved_shape = make_solid.pin_mut().shape();
        let solid = topo_ds::solid(revolved_shape);

        Solid::from_solid(solid)
    }

    /// Fillets the face edges by a given radius at each vertex
    #[must_use]
    pub fn fillet(&self, radius: f64) -> Self {
        let mut make_fillet = b_rep_fillet_api::MakeFillet2d::new_face(&self.inner);

        let face_shape = self.inner.as_shape();

        let mut explorer = top_exp::Explorer::new_shape_shapeenum2(
            face_shape,
            top_abs::ShapeEnum::Vertex.into(),
            top_abs::ShapeEnum::Shape.into(),
        );
        while explorer.more() {
            let vertex = topo_ds::vertex(explorer.current());
            make_fillet.pin_mut().add_fillet(&vertex, radius);
            explorer.pin_mut().next();
        }

        let progress = message::ProgressRange::new();
        make_fillet.pin_mut().build(&progress);

        let result_shape = make_fillet.pin_mut().shape();
        let result_face = topo_ds::face(result_shape);

        Self::from_face(result_face)
    }

    /// Chamfer the wire edges at each vertex by a given distance
    #[must_use]
    pub fn chamfer(&self, distance_1: f64) -> Self {
        // TODO - Support asymmetric chamfers.
        let distance_2 = distance_1;

        let face_shape = self.inner.as_shape();
        let mut make_fillet = b_rep_fillet_api::MakeFillet2d::new_face(&self.inner);

        // Build vertex→edges mapping manually since map_shapes_and_ancestors
        // is not available in the current bindings.
        // Collect all edges and their endpoint vertex positions.
        let mut edges: Vec<UniquePtr<topo_ds::Edge>> = Vec::new();
        let mut edge_explorer = top_exp::Explorer::new_shape_shapeenum2(
            face_shape,
            top_abs::ShapeEnum::Edge.into(),
            top_abs::ShapeEnum::Shape.into(),
        );
        while edge_explorer.more() {
            let edge = topo_ds::edge(edge_explorer.current());
            edges.push(edge.to_owned());
            edge_explorer.pin_mut().next();
        }

        // Map vertex positions to edge indices
        let mut vertex_edges: HashMap<[i64; 3], Vec<usize>> = HashMap::new();
        for (i, edge) in edges.iter().enumerate() {
            let first = top_exp::first_vertex(edge, false);
            let last = top_exp::last_vertex(edge, false);

            let first_pnt = b_rep::Tool::pnt(&first);
            let last_pnt = b_rep::Tool::pnt(&last);

            let first_key = pos_key(&first_pnt);
            let last_key = pos_key(&last_pnt);

            vertex_edges.entry(first_key).or_default().push(i);
            vertex_edges.entry(last_key).or_default().push(i);
        }

        // Chamfer at each vertex that connects exactly two edges.
        for edge_indices in vertex_edges.values() {
            if edge_indices.len() >= 2 {
                make_fillet.pin_mut().add_chamfer_edge2_real2(
                    &edges[edge_indices[0]],
                    &edges[edge_indices[1]],
                    distance_1,
                    distance_2,
                );
            }
        }

        let progress = message::ProgressRange::new();
        make_fillet.pin_mut().build(&progress);

        let filleted_shape = make_fillet.pin_mut().shape();
        let result_face = topo_ds::face(filleted_shape);

        Self::from_face(result_face)
    }

    /// Offset the face by a given distance and join settings
    #[must_use]
    pub fn offset(&self, distance: f64, join_type: JoinType) -> Self {
        let mut make_offset =
            b_rep_offset_api::MakeOffset::new_face_jointype(&self.inner, join_type.to_i32());
        make_offset.pin_mut().perform(distance, 0.0);

        let offset_shape = make_offset.pin_mut().shape();
        let result_wire = topo_ds::wire(offset_shape);
        let wire = Wire::from_wire(result_wire);

        wire.to_face()
    }

    /// Sweep the face along a path to produce a solid
    #[must_use]
    pub fn sweep_along(&self, path: &Wire) -> Solid {
        let profile_shape = self.inner.as_shape();
        let mut make_pipe = b_rep_offset_api::MakePipe::new_wire_shape(&path.inner, profile_shape);

        let pipe_shape = make_pipe.pin_mut().shape();
        let result_solid = topo_ds::solid(pipe_shape);

        Solid::from_solid(result_solid)
    }

    /// Sweep the face along a path, modulated by a function, to produce a solid
    #[must_use]
    pub fn sweep_along_with_radius_values(
        &self,
        path: &Wire,
        radius_values: impl IntoIterator<Item = (f64, f64)>,
    ) -> Solid {
        let profile = self.outer_wire();
        make_pipe_shell_with_law_function(path, &profile, radius_values)
    }

    pub fn edges(&self) -> EdgeIterator {
        let explorer = top_exp::Explorer::new_shape_shapeenum2(
            self.inner.as_shape(),
            top_abs::ShapeEnum::Edge.into(),
            top_abs::ShapeEnum::Shape.into(),
        );

        EdgeIterator { explorer }
    }

    pub fn center_of_mass(&self) -> DVec3 {
        let mut props = g_prop::GProps::new();

        let inner_shape = self.inner.as_shape();
        b_rep_g_prop::surface_properties(inner_shape, props.pin_mut(), false, false);

        let center = props.centre_of_mass();

        dvec3(center.x(), center.y(), center.z())
    }

    pub fn normal_at(&self, pos: DVec3) -> DVec3 {
        let surface = b_rep::Tool::surface_face(&self.inner);
        let projector =
            geom_api::ProjectPointOnSurf::new_pnt_handlegeomsurface_extalgo(&make_point(pos), &surface, 0);
        let mut u: f64 = 0.0;
        let mut v: f64 = 0.0;

        projector.lower_distance_parameters(&mut u, &mut v);

        let mut p = gp::Pnt::new();
        let mut normal = gp::Vec::new();

        let face = b_rep_g_prop::Face::new_face_bool(&self.inner, false);
        face.normal(u, v, p.pin_mut(), normal.pin_mut());

        dvec3(normal.x(), normal.y(), normal.z())
    }

    pub fn normal_at_center(&self) -> DVec3 {
        let center = self.center_of_mass();
        self.normal_at(center)
    }

    pub fn workplane(&self) -> Workplane {
        const NORMAL_DIFF_TOLERANCE: f64 = 0.0001;

        let center = self.center_of_mass();
        let normal = self.normal_at(center);
        let mut x_dir = dvec3(0.0, 0.0, 1.0).cross(normal);

        if x_dir.length() < NORMAL_DIFF_TOLERANCE {
            // The normal of this face is too close to the same direction
            // as the global Z axis. Use the global X axis for X instead.
            x_dir = dvec3(1.0, 0.0, 0.0);
        }

        let mut workplane = Workplane::new(x_dir, normal);
        workplane.set_translation(center);
        workplane
    }

    pub fn union(&self, other: &Face) -> CompoundFace {
        let inner_shape = self.inner.as_shape();
        let other_inner_shape = other.inner.as_shape();

        let progress = message::ProgressRange::new();
        let mut fuse_operation =
            b_rep_algo_api::Fuse::new_shape2_progressrange(inner_shape, other_inner_shape, &progress);

        let fuse_shape = fuse_operation.pin_mut().shape();
        let compound = topo_ds::compound(fuse_shape);

        CompoundFace::from_compound(compound)
    }

    #[must_use]
    pub fn intersect(&self, other: &Face) -> CompoundFace {
        let inner_shape = self.inner.as_shape();
        let other_inner_shape = other.inner.as_shape();

        let progress = message::ProgressRange::new();
        let mut common_operation =
            b_rep_algo_api::Common::new_shape2_progressrange(inner_shape, other_inner_shape, &progress);

        let common_shape = common_operation.pin_mut().shape();
        let compound = topo_ds::compound(common_shape);

        CompoundFace::from_compound(compound)
    }

    pub fn subtract(&self, other: &Face) -> CompoundFace {
        let inner_shape = self.inner.as_shape();
        let other_inner_shape = other.inner.as_shape();

        let progress = message::ProgressRange::new();
        let mut cut_operation =
            b_rep_algo_api::Cut::new_shape2_progressrange(inner_shape, other_inner_shape, &progress);

        let cut_shape = cut_operation.pin_mut().shape();
        let compound = topo_ds::compound(cut_shape);

        CompoundFace::from_compound(compound)
    }

    pub fn surface_area(&self) -> f64 {
        let mut props = g_prop::GProps::new();

        let inner_shape = self.inner.as_shape();
        b_rep_g_prop::surface_properties(inner_shape, props.pin_mut(), false, false);

        // Returns surface area, obviously.
        props.mass()
    }

    pub fn orientation(&self) -> FaceOrientation {
        FaceOrientation::from_i32(self.inner.as_shape().orientation())
    }

    #[must_use]
    pub fn outer_wire(&self) -> Wire {
        let inner = b_rep_tools::outer_wire(&self.inner);

        Wire { inner }
    }
}

/// Hash key for vertex position, used to group edges by shared vertices.
fn pos_key(pnt: &gp::Pnt) -> [i64; 3] {
    [
        (pnt.x() * 1e9) as i64,
        (pnt.y() * 1e9) as i64,
        (pnt.z() * 1e9) as i64,
    ]
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
    fn from(face: Face) -> Self {
        let mut compound = topo_ds::Compound::new();
        let builder = b_rep::Builder::new();
        builder.make_compound(compound.pin_mut());
        builder.add(compound.pin_mut().as_shape_mut(), face.inner.as_shape());
        Self::from_compound(&compound)
    }
}

impl CompoundFace {
    pub(crate) fn from_compound(compound: &topo_ds::Compound) -> Self {
        let inner = compound.to_owned();
        Self { inner }
    }

    #[must_use]
    pub fn clean(&self) -> Self {
        let shape = self.inner.as_shape();
        let shape = Shape::from_shape(shape).clean();

        let compound = topo_ds::compound(&shape.inner);

        Self::from_compound(compound)
    }

    #[must_use]
    pub fn extrude(&self, dir: DVec3) -> Shape {
        let prism_vec = make_vec(dir);

        let copy = false;
        let canonize = true;

        let inner_shape = self.inner.as_shape();

        let mut make_solid =
            b_rep_prim_api::MakePrism::new_shape_vec_bool2(inner_shape, &prism_vec, copy, canonize);
        let extruded_shape = make_solid.pin_mut().shape();

        Shape::from_shape(extruded_shape)
    }

    #[must_use]
    pub fn revolve(&self, origin: DVec3, axis: DVec3, angle: Option<Angle>) -> Shape {
        let revol_axis = make_axis_1(origin, axis);

        let angle = angle.map(Angle::radians).unwrap_or(std::f64::consts::PI * 2.0);
        let copy = false;

        let inner_shape = self.inner.as_shape();

        let mut make_solid =
            b_rep_prim_api::MakeRevol::new_shape_ax1_real_bool(inner_shape, &revol_axis, angle, copy);
        let revolved_shape = make_solid.pin_mut().shape();

        Shape::from_shape(revolved_shape)
    }

    #[must_use]
    pub fn union(&self, other: &CompoundFace) -> CompoundFace {
        let inner_shape = self.inner.as_shape();
        let other_inner_shape = other.inner.as_shape();

        let progress = message::ProgressRange::new();
        let mut fuse_operation =
            b_rep_algo_api::Fuse::new_shape2_progressrange(inner_shape, other_inner_shape, &progress);

        let fuse_shape = fuse_operation.pin_mut().shape();
        let compound = topo_ds::compound(fuse_shape);

        CompoundFace::from_compound(compound)
    }

    #[must_use]
    pub fn intersect(&self, other: &CompoundFace) -> CompoundFace {
        let inner_shape = self.inner.as_shape();
        let other_inner_shape = other.inner.as_shape();

        let progress = message::ProgressRange::new();
        let mut common_operation =
            b_rep_algo_api::Common::new_shape2_progressrange(inner_shape, other_inner_shape, &progress);

        let common_shape = common_operation.pin_mut().shape();
        let compound = topo_ds::compound(common_shape);

        CompoundFace::from_compound(compound)
    }

    #[must_use]
    pub fn subtract(&self, other: &CompoundFace) -> CompoundFace {
        let inner_shape = self.inner.as_shape();
        let other_inner_shape = other.inner.as_shape();

        let progress = message::ProgressRange::new();
        let mut cut_operation =
            b_rep_algo_api::Cut::new_shape2_progressrange(inner_shape, other_inner_shape, &progress);

        let cut_shape = cut_operation.pin_mut().shape();
        let compound = topo_ds::compound(cut_shape);

        CompoundFace::from_compound(compound)
    }

    pub fn set_global_translation(&mut self, translation: DVec3) {
        let shape = self.inner.as_shape();
        let mut shape = Shape::from_shape(shape);

        shape.set_global_translation(translation);

        let compound = topo_ds::compound(&shape.inner);
        *self = Self::from_compound(compound);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FaceOrientation {
    Forward,
    Reversed,
    Internal,
    External,
}

impl FaceOrientation {
    pub fn from_i32(value: i32) -> Self {
        match value {
            0 => Self::Forward,
            1 => Self::Reversed,
            2 => Self::Internal,
            3 => Self::External,
            _ => panic!("TopAbs_Orientation had an unrepresentable value: {value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let face = Workplane::xy().rect(7.0, 5.0).to_face();
        assert!(
            (face.surface_area() - 35.0).abs() <= 0.00001,
            "Expected surface_area() to be ~35.0, was actually {}",
            face.surface_area()
        );
    }
}
