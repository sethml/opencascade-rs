use crate::{
    angle::Angle,
    make_pipe_shell::make_pipe_shell_with_law_function,
    primitives::{make_axis_1, make_point, make_vec, Shape, Solid, Surface, Wire},
    workplane::Workplane,
};
use cxx::UniquePtr;
use glam::{dvec3, DVec3};
use opencascade_sys::{b_rep, b_rep_algo_api, b_rep_builder_api, b_rep_feat, b_rep_fillet_api, b_rep_g_prop, b_rep_offset_api, b_rep_prim_api, b_rep_tools, g_prop, geom_api, gp, message, shape_upgrade, top_abs, top_exp, top_loc, topo_ds};

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
            b_rep_builder_api::MakeFace::new_handlegeomsurface_real(&surface.inner, tol_degen);
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

    #[must_use]
    pub fn sweep_along(&self, path: &Wire) -> Solid {
        let profile_shape = self.inner.as_shape();
        let mut make_pipe = b_rep_offset_api::MakePipe::new_wire_shape(&path.inner, profile_shape);
        let make_shape = make_pipe.pin_mut().as_b_rep_builder_api_make_shape_mut();
        let pipe_shape = make_shape.shape();
        let solid = topo_ds::solid(pipe_shape);
        Solid::from_solid(solid)
    }

    #[must_use]
    pub fn sweep_along_with_radius_values(
        &self,
        path: &Wire,
        radius_values: impl IntoIterator<Item = (f64, f64)>,
    ) -> Solid {
        let profile = self.outer_wire();
        make_pipe_shell_with_law_function(&path, &profile, radius_values)
    }

    pub fn edges(&self) -> super::EdgeIterator {
        let explorer = top_exp::Explorer::new_shape_shapeenum2(
            self.inner.as_shape(),
            top_abs::ShapeEnum::Edge.into(),
            top_abs::ShapeEnum::Shape.into(),
        );
        super::EdgeIterator { explorer }
    }

    pub fn center_of_mass(&self) -> DVec3 {
        let mut props = g_prop::GProps::new();
        let inner_shape = self.inner.as_shape();
        b_rep_g_prop::surface_properties(
            inner_shape,
            props.pin_mut(),
            false, // SkipShared
            false, // UseTriangulation
        );
        let center = props.centre_of_mass();
        dvec3(center.x(), center.y(), center.z())
    }

    pub fn normal_at(&self, pos: DVec3) -> DVec3 {
        let surface_handle = b_rep::Tool::surface_face(&self.inner);
        let gp_point = make_point(pos);
        let projector = geom_api::ProjectPointOnSurf::new_pnt_handlegeomsurface_extalgo(
            &gp_point,
            &surface_handle,
            0, // Extrema_ExtAlgo_Grad
        );
        let mut u = 0.0;
        let mut v = 0.0;
        projector.lower_distance_parameters(&mut u, &mut v);

        let gprop_face = b_rep_g_prop::Face::new_face_bool(&self.inner, false);
        let mut pnt = gp::Pnt::new();
        let mut normal_vec = gp::Vec::new();
        gprop_face.normal(u, v, pnt.pin_mut(), normal_vec.pin_mut());

        let norm = dvec3(normal_vec.x(), normal_vec.y(), normal_vec.z());
        norm.normalize()
    }

    pub fn normal_at_center(&self) -> DVec3 {
        let center = self.center_of_mass();
        self.normal_at(center)
    }

    pub fn workplane(&self) -> Workplane {
        const NORMAL_DIFF_TOLERANCE: f64 = 0.0001;

        let origin = self.center_of_mass();
        let normal = self.normal_at(origin);
        let mut x_dir = dvec3(0.0, 0.0, 1.0).cross(normal);

        if x_dir.length() < NORMAL_DIFF_TOLERANCE {
            // The normal of this face is too close to the same direction
            // as the global Z axis. Use the global X axis for X instead.
            x_dir = dvec3(1.0, 0.0, 0.0);
        }

        let mut workplane = Workplane::new(x_dir, normal);
        workplane.set_translation(origin);
        workplane
    }

    pub fn union(&self, other: &Face) -> CompoundFace {
        let progress = message::ProgressRange::new();
        let inner_shape = self.inner.as_shape();
        let other_inner_shape = other.inner.as_shape();
        let mut fuse = b_rep_algo_api::Fuse::new_shape2_progressrange(
            inner_shape,
            other_inner_shape,
            &progress,
        );
        let make_shape = fuse.pin_mut().as_b_rep_builder_api_make_shape_mut();
        let fuse_shape = make_shape.shape();
        let compound = topo_ds::compound(fuse_shape);
        CompoundFace::from_compound(compound)
    }

    #[must_use]
    pub fn intersect(&self, other: &Face) -> CompoundFace {
        let progress = message::ProgressRange::new();
        let inner_shape = self.inner.as_shape();
        let other_inner_shape = other.inner.as_shape();
        let mut common = b_rep_algo_api::Common::new_shape2_progressrange(
            inner_shape,
            other_inner_shape,
            &progress,
        );
        let make_shape = common.pin_mut().as_b_rep_builder_api_make_shape_mut();
        let common_shape = make_shape.shape();
        let compound = topo_ds::compound(common_shape);
        CompoundFace::from_compound(compound)
    }

    pub fn subtract(&self, other: &Face) -> CompoundFace {
        let progress = message::ProgressRange::new();
        let inner_shape = self.inner.as_shape();
        let other_inner_shape = other.inner.as_shape();
        let mut cut = b_rep_algo_api::Cut::new_shape2_progressrange(
            inner_shape,
            other_inner_shape,
            &progress,
        );
        let make_shape = cut.pin_mut().as_b_rep_builder_api_make_shape_mut();
        let cut_shape = make_shape.shape();
        let compound = topo_ds::compound(cut_shape);
        CompoundFace::from_compound(compound)
    }

    pub fn surface_area(&self) -> f64 {
        let mut props = g_prop::GProps::new();
        let inner_shape = self.inner.as_shape();
        b_rep_g_prop::surface_properties(
            inner_shape,
            props.pin_mut(),
            false, // SkipShared
            false, // UseTriangulation
        );
        props.mass()
    }

    pub fn orientation(&self) -> FaceOrientation {
        let raw = self.inner.as_shape().orientation();
        let orient = top_abs::Orientation::try_from(raw)
            .expect("Invalid Orientation value from OCCT");
        orient.into()
    }

    #[must_use]
    pub fn outer_wire(&self) -> Wire {
        let inner = b_rep_tools::outer_wire(&self.inner);
        Wire { inner }
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
    fn from(face: Face) -> Self {
        let mut compound = topo_ds::Compound::new();
        let builder = b_rep::Builder::new();
        builder.make_compound(compound.pin_mut());
        let mut compound_shape = compound.as_shape().to_owned();
        builder.add(compound_shape.pin_mut(), face.inner.as_shape());
        let result_compound = topo_ds::compound(&compound_shape);
        CompoundFace::from_compound(result_compound)
    }
}

impl CompoundFace {
    pub(crate) fn from_compound(compound: &topo_ds::Compound) -> Self {
        let inner = compound.to_owned();
        Self { inner }
    }

    #[must_use]
    pub fn clean(&self) -> Self {
        let mut unifier = shape_upgrade::UnifySameDomain::new_shape_bool3(
            self.inner.as_shape(),
            true,  // UnifyEdges
            true,  // UnifyFaces
            false, // ConcatBSplines
        );
        unifier.pin_mut().allow_internal_edges(false);
        unifier.pin_mut().build();
        let result = unifier.shape();
        let compound = topo_ds::compound(result);
        CompoundFace::from_compound(compound)
    }

    #[must_use]
    pub fn extrude(&self, dir: DVec3) -> Shape {
        let prism_vec = make_vec(dir);
        let copy = false;
        let canonize = true;

        let inner_shape = self.inner.as_shape();
        let mut make_solid = b_rep_prim_api::MakePrism::new_shape_vec_bool2(
            inner_shape, &prism_vec, copy, canonize
        );
        let make_shape = make_solid.pin_mut().as_b_rep_builder_api_make_shape_mut();
        Shape::from_shape(make_shape.shape())
    }

    #[must_use]
    pub fn revolve(&self, origin: DVec3, axis: DVec3, angle: Option<Angle>) -> Shape {
        use crate::primitives::make_axis_1;
        let revol_axis = make_axis_1(origin, axis);
        let angle = angle.map(Angle::radians).unwrap_or(std::f64::consts::PI * 2.0);
        let copy = false;

        let inner_shape = self.inner.as_shape();
        let mut make_revol =
            b_rep_prim_api::MakeRevol::new_shape_ax1_real_bool(inner_shape, &revol_axis, angle, copy);
        let make_shape = make_revol.pin_mut().as_b_rep_builder_api_make_shape_mut();
        Shape::from_shape(make_shape.shape())
    }

    #[must_use]
    pub fn union(&self, other: &CompoundFace) -> CompoundFace {
        let progress = message::ProgressRange::new();
        let inner_shape = self.inner.as_shape();
        let other_inner_shape = other.inner.as_shape();
        let mut fuse = b_rep_algo_api::Fuse::new_shape2_progressrange(
            inner_shape,
            other_inner_shape,
            &progress,
        );
        let make_shape = fuse.pin_mut().as_b_rep_builder_api_make_shape_mut();
        let fuse_shape = make_shape.shape();
        let compound = topo_ds::compound(fuse_shape);
        CompoundFace::from_compound(compound)
    }

    #[must_use]
    pub fn intersect(&self, other: &CompoundFace) -> CompoundFace {
        let progress = message::ProgressRange::new();
        let inner_shape = self.inner.as_shape();
        let other_inner_shape = other.inner.as_shape();
        let mut common = b_rep_algo_api::Common::new_shape2_progressrange(
            inner_shape,
            other_inner_shape,
            &progress,
        );
        let make_shape = common.pin_mut().as_b_rep_builder_api_make_shape_mut();
        let common_shape = make_shape.shape();
        let compound = topo_ds::compound(common_shape);
        CompoundFace::from_compound(compound)
    }

    #[must_use]
    pub fn subtract(&self, other: &CompoundFace) -> CompoundFace {
        let progress = message::ProgressRange::new();
        let inner_shape = self.inner.as_shape();
        let other_inner_shape = other.inner.as_shape();
        let mut cut = b_rep_algo_api::Cut::new_shape2_progressrange(
            inner_shape,
            other_inner_shape,
            &progress,
        );
        let make_shape = cut.pin_mut().as_b_rep_builder_api_make_shape_mut();
        let cut_shape = make_shape.shape();
        let compound = topo_ds::compound(cut_shape);
        CompoundFace::from_compound(compound)
    }

    pub fn set_global_translation(&mut self, translation: DVec3) {
        let mut transform = gp::Trsf::new();
        let translation_vec = make_vec(translation);
        transform.pin_mut().set_translation_vec(&translation_vec);
        let location = top_loc::Location::new_trsf(&transform);
        let raise_exception = false;
        let mut compound_shape = self.inner.as_shape().to_owned();
        compound_shape.pin_mut().move_(&location, raise_exception);
        let updated = topo_ds::compound(&compound_shape);
        self.inner = updated.to_owned();
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
