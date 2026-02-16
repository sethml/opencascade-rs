use crate::{
    mesh::{Mesh, Mesher},
    primitives::{
        make_axis_1, make_axis_2, make_point, make_point2d, make_vec, BooleanShape,
        Compound, Edge, Face, Shell, Solid, Vertex, Wire,
    },
    Error,
};
use glam::{dvec3, DVec2, DVec3};
use opencascade_sys::{
    b_rep, b_rep_algo_api, b_rep_feat, b_rep_fillet_api, b_rep_int_curve_surface, b_rep_mesh,
    b_rep_offset, b_rep_offset_api, b_rep_prim_api, ch_fi3d, geom_abs, gp, iges_control,
    if_select, message, shape_upgrade, step_control, stl_api, t_colgp, top_abs, top_exp,
    top_loc, top_tools, topo_ds,
};
use std::path::Path;

pub struct Shape {
    pub(crate) inner: opencascade_sys::OwnedPtr<topo_ds::Shape>,
}

impl AsRef<Shape> for Shape {
    fn as_ref(&self) -> &Shape {
        self
    }
}

impl From<Vertex> for Shape {
    fn from(vertex: Vertex) -> Self {
        Self::from_shape(vertex.inner.as_shape())
    }
}

impl From<&Vertex> for Shape {
    fn from(vertex: &Vertex) -> Self {
        Self::from_shape(vertex.inner.as_shape())
    }
}

impl From<Edge> for Shape {
    fn from(edge: Edge) -> Self {
        Self::from_shape(edge.inner.as_shape())
    }
}

impl From<&Edge> for Shape {
    fn from(edge: &Edge) -> Self {
        Self::from_shape(edge.inner.as_shape())
    }
}

impl From<Wire> for Shape {
    fn from(wire: Wire) -> Self {
        Self::from_shape(wire.inner.as_shape())
    }
}

impl From<&Wire> for Shape {
    fn from(wire: &Wire) -> Self {
        Self::from_shape(wire.inner.as_shape())
    }
}

impl From<Face> for Shape {
    fn from(face: Face) -> Self {
        Self::from_shape(face.inner.as_shape())
    }
}

impl From<&Face> for Shape {
    fn from(face: &Face) -> Self {
        Self::from_shape(face.inner.as_shape())
    }
}

impl From<Shell> for Shape {
    fn from(shell: Shell) -> Self {
        Self::from_shape(shell.inner.as_shape())
    }
}

impl From<&Shell> for Shape {
    fn from(shell: &Shell) -> Self {
        Self::from_shape(shell.inner.as_shape())
    }
}

impl From<Solid> for Shape {
    fn from(solid: Solid) -> Self {
        Self::from_shape(solid.inner.as_shape())
    }
}

impl From<&Solid> for Shape {
    fn from(solid: &Solid) -> Self {
        Self::from_shape(solid.inner.as_shape())
    }
}

impl From<Compound> for Shape {
    fn from(compound: Compound) -> Self {
        Self::from_shape(compound.inner.as_shape())
    }
}

impl From<&Compound> for Shape {
    fn from(compound: &Compound) -> Self {
        Self::from_shape(compound.inner.as_shape())
    }
}

impl From<BooleanShape> for Shape {
    fn from(boolean_shape: BooleanShape) -> Self {
        boolean_shape.shape
    }
}

pub struct SphereBuilder {
    center: DVec3,
    radius: f64,
    z_angle: f64,
}

impl SphereBuilder {
    pub fn build(self) -> Shape {
        let axis = make_axis_2(self.center, DVec3::Z);
        let mut make_sphere =
            b_rep_prim_api::MakeSphere::new_ax2_real2(&axis, self.radius, self.z_angle);
        let make_shape = make_sphere.as_b_rep_builder_api_make_shape_mut();
        Shape::from_shape(make_shape.shape())
    }

    pub fn at(mut self, center: DVec3) -> Self {
        self.center = center;
        self
    }

    pub fn z_angle(mut self, z_angle: f64) -> Self {
        self.z_angle = z_angle;
        self
    }
}

pub struct ConeBuilder {
    pos: DVec3,
    height: f64,
    bottom_radius: f64,
    top_radius: f64,
    z_angle: f64,
}

impl ConeBuilder {
    pub fn build(self) -> Shape {
        let axis = make_axis_2(self.pos, DVec3::Z);
        let mut make_cone = b_rep_prim_api::MakeCone::new_ax2_real4(
            &axis,
            self.bottom_radius,
            self.top_radius,
            self.height,
            self.z_angle,
        );
        let make_shape = make_cone.as_b_rep_builder_api_make_shape_mut();
        Shape::from_shape(make_shape.shape())
    }

    pub fn at(mut self, pos: DVec3) -> Self {
        self.pos = pos;
        self
    }

    pub fn bottom_radius(mut self, bottom_radius: f64) -> Self {
        self.bottom_radius = bottom_radius;
        self
    }

    pub fn top_radius(mut self, top_radius: f64) -> Self {
        self.top_radius = top_radius;
        self
    }

    pub fn height(mut self, height: f64) -> Self {
        self.height = height;
        self
    }

    pub fn z_angle(mut self, z_angle: f64) -> Self {
        self.z_angle = z_angle;
        self
    }
}

pub struct TorusBuilder {
    pos: DVec3,
    z_axis: DVec3,
    radius_1: f64,
    radius_2: f64,
    angle_1: f64,
    angle_2: f64,
    z_angle: f64,
}

impl TorusBuilder {
    pub fn build(self) -> Shape {
        let axis = make_axis_2(self.pos, self.z_axis);
        let mut make_torus = b_rep_prim_api::MakeTorus::new_ax2_real5(
            &axis,
            self.radius_1,
            self.radius_2,
            self.angle_1,
            self.angle_2,
            self.z_angle,
        );
        let make_shape = make_torus.as_b_rep_builder_api_make_shape_mut();
        Shape::from_shape(make_shape.shape())
    }

    pub fn at(mut self, pos: DVec3) -> Self {
        self.pos = pos;
        self
    }

    pub fn z_axis(mut self, z_axis: DVec3) -> Self {
        self.z_axis = z_axis;
        self
    }

    pub fn radius_1(mut self, radius_1: f64) -> Self {
        self.radius_1 = radius_1;
        self
    }

    pub fn radius_2(mut self, radius_2: f64) -> Self {
        self.radius_2 = radius_2;
        self
    }

    pub fn angle_1(mut self, angle_1: f64) -> Self {
        self.angle_1 = angle_1;
        self
    }

    pub fn angle_2(mut self, angle_2: f64) -> Self {
        self.angle_2 = angle_2;
        self
    }

    pub fn z_angle(mut self, z_angle: f64) -> Self {
        self.z_angle = z_angle;
        self
    }
}

impl Shape {
    pub(crate) fn from_shape(shape: &topo_ds::Shape) -> Self {
        let inner = shape.to_owned();
        Self { inner }
    }

    /// Make a shape that models empty space.
    pub fn empty() -> Self {
        // NOTE: It may seem like using `TopoDS_Shape()` directly should work,
        //       but shape operations such as union fail on actual "null shapes".

        // Construct an empty compound
        let mut compound = topo_ds::Compound::new();
        let builder = b_rep::Builder::new();
        builder.make_compound(&mut compound);
        let inner = compound.as_shape().to_owned();
        Self { inner }
    }

    /// Make a box with one corner at corner_1, and the opposite corner
    /// at corner_2.
    pub fn box_from_corners(corner_1: DVec3, corner_2: DVec3) -> Self {
        let min_corner = corner_1.min(corner_2);
        let max_corner = corner_1.max(corner_2);

        let point = gp::Pnt::new_real3(min_corner.x, min_corner.y, min_corner.z);
        let diff = max_corner - min_corner;
        let mut make_box =
            b_rep_prim_api::MakeBox::new_pnt_real3(&point, diff.x, diff.y, diff.z);
        let make_shape = make_box.as_b_rep_builder_api_make_shape_mut();
        Self::from_shape(make_shape.shape())
    }

    /// Make a box with `width` (x), `depth` (y), and `height` (z)
    /// centered around the origin.
    pub fn box_centered(width: f64, depth: f64, height: f64) -> Self {
        let half_width = width / 2.0;
        let half_depth = depth / 2.0;
        let half_height = height / 2.0;

        let corner_1 = dvec3(-half_width, -half_depth, -half_height);
        let corner_2 = dvec3(half_width, half_depth, half_height);
        Self::box_from_corners(corner_1, corner_2)
    }

    /// Make a box with `width` (x), `depth` (y), and `height` (z)
    /// extending into the positive axes
    pub fn box_with_dimensions(width: f64, depth: f64, height: f64) -> Self {
        let corner_1 = DVec3::ZERO;
        let corner_2 = dvec3(width, depth, height);
        Self::box_from_corners(corner_1, corner_2)
    }

    /// Make a cube with side length of `size`
    /// extending into the positive axes
    pub fn cube(size: f64) -> Self {
        Self::box_with_dimensions(size, size, size)
    }

    /// Make a centered cube with side length of `size`
    pub fn cube_centered(size: f64) -> Self {
        Self::box_centered(size, size, size)
    }

    /// Make a cylinder with base at point `p`, radius `r`, and height `h`.
    /// Extends from `p` along axis `dir`.
    pub fn cylinder(p: DVec3, r: f64, dir: DVec3, h: f64) -> Self {
        let axis = make_axis_2(p, dir);
        let mut make_cylinder = b_rep_prim_api::MakeCylinder::new_ax2_real2(&axis, r, h);
        let make_shape = make_cylinder.as_b_rep_builder_api_make_shape_mut();
        Self::from_shape(make_shape.shape())
    }

    /// Make a "default" cylinder with radius `r` and height `h`.
    /// The base is at the coordinate origin, and extends along the Z axis.
    pub fn cylinder_radius_height(r: f64, h: f64) -> Self {
        Self::cylinder(DVec3::ZERO, r, DVec3::Z, h)
    }

    /// Make a cylinder from start point `p1` and end point `p2`,
    /// with radius `r`.
    pub fn cylinder_from_points(p1: DVec3, p2: DVec3, r: f64) -> Self {
        let dir = p2 - p1;
        Self::cylinder(p1, r, dir, dir.length())
    }

    /// Make a cylinder centered at point `p`, with radius `r`, and height `h`.
    /// Extends along axis `dir`.
    pub fn cylinder_centered(p: DVec3, r: f64, dir: DVec3, h: f64) -> Self {
        let p = p - (dir.normalize() * (h / 2.0));
        Self::cylinder(p, r, dir, h)
    }

    pub fn sphere(radius: f64) -> SphereBuilder {
        SphereBuilder { center: DVec3::ZERO, radius, z_angle: std::f64::consts::TAU }
    }

    pub fn cone() -> ConeBuilder {
        ConeBuilder {
            pos: DVec3::ZERO,
            height: 1.0,
            bottom_radius: 1.0,
            top_radius: 0.0,
            z_angle: std::f64::consts::TAU,
        }
    }

    pub fn torus() -> TorusBuilder {
        TorusBuilder {
            pos: DVec3::ZERO,
            z_axis: DVec3::Z,
            radius_1: 20.0,
            radius_2: 10.0,
            angle_1: -std::f64::consts::PI,
            angle_2: std::f64::consts::PI,
            z_angle: std::f64::consts::TAU,
        }
    }

    pub fn shape_type(&self) -> super::ShapeType {
        let raw = self.inner.shape_type();
        let shape_enum =
            top_abs::ShapeEnum::try_from(raw).expect("Invalid ShapeEnum value from OCCT");
        shape_enum.into()
    }

    #[must_use]
    pub fn fillet_edge(&self, radius: f64, edge: &Edge) -> Self {
        self.fillet_edges(radius, [edge])
    }

    #[must_use]
    pub fn variable_fillet_edge(
        &self,
        radius_values: impl IntoIterator<Item = (f64, f64)>,
        edge: &Edge,
    ) -> Self {
        self.variable_fillet_edges(radius_values, [edge])
    }

    #[must_use]
    pub fn chamfer_edge(&self, distance: f64, edge: &Edge) -> Self {
        self.chamfer_edges(distance, [edge])
    }

    #[must_use]
    pub fn fillet_edges<T: AsRef<Edge>>(
        &self,
        radius: f64,
        edges: impl IntoIterator<Item = T>,
    ) -> Self {
        let progress = message::ProgressRange::new();
        let mut make_fillet = b_rep_fillet_api::MakeFillet::new_shape_filletshape(&self.inner, ch_fi3d::FilletShape::Rational);
        for edge in edges {
            make_fillet.add_real_edge(radius, &edge.as_ref().inner);
        }
        make_fillet.build(&progress);
        let shape = make_fillet.shape();
        Self::from_shape(shape)
    }

    #[must_use]
    pub fn variable_fillet_edges<T: AsRef<Edge>>(
        &self,
        radius_values: impl IntoIterator<Item = (f64, f64)>,
        edges: impl IntoIterator<Item = T>,
    ) -> Self {
        let progress = message::ProgressRange::new();
        let mut make_fillet = b_rep_fillet_api::MakeFillet::new_shape_filletshape(&self.inner, ch_fi3d::FilletShape::Rational);

        let pairs: Vec<(f64, f64)> = radius_values.into_iter().collect();
        let n = pairs.len() as i32;
        let mut array = t_colgp::Array1OfPnt2d::new_int2(1, n);
        for (i, &(param, radius)) in pairs.iter().enumerate() {
            let pnt2d = make_point2d(DVec2::new(param, radius));
            array.set_value(i as i32 + 1, &pnt2d);
        }

        for edge in edges {
            make_fillet
                .add_array1ofpnt2d_edge(&array, &edge.as_ref().inner);
        }

        make_fillet.build(&progress);
        let shape = make_fillet.shape();
        Self::from_shape(shape)
    }

    #[must_use]
    pub fn chamfer_edges<T: AsRef<Edge>>(
        &self,
        distance: f64,
        edges: impl IntoIterator<Item = T>,
    ) -> Self {
        let progress = message::ProgressRange::new();
        let mut make_chamfer = b_rep_fillet_api::MakeChamfer::new_shape(&self.inner);
        for edge in edges {
            make_chamfer.add_real_edge(distance, &edge.as_ref().inner);
        }
        make_chamfer.build(&progress);
        let shape = make_chamfer.shape();
        Self::from_shape(shape)
    }

    /// Performs fillet of `radius` on all edges of the shape
    #[must_use]
    pub fn fillet(&self, radius: f64) -> Self {
        let edges: Vec<Edge> = self.edges().collect();
        self.fillet_edges(radius, &edges)
    }

    /// Performs chamfer of `distance` on all edges of the shape
    #[must_use]
    pub fn chamfer(&self, distance: f64) -> Self {
        let edges: Vec<Edge> = self.edges().collect();
        self.chamfer_edges(distance, &edges)
    }

    /// Boolean subtraction: returns a new shape with `other` removed from `self`.
    #[must_use]
    pub fn subtract(&self, other: &Shape) -> BooleanShape {
        let progress = message::ProgressRange::new();
        let mut cut = b_rep_algo_api::Cut::new_shape2_progressrange(
            &self.inner,
            &other.inner,
            &progress,
        );

        let make_shape = cut.as_b_rep_builder_api_make_shape_mut();
        let result_shape = make_shape.shape();
        let shape = Shape::from_shape(result_shape);

        let new_edges = list_of_shape_to_edges(cut.section_edges());

        BooleanShape { shape, new_edges }
    }

    pub fn read_step(path: impl AsRef<Path>) -> Result<Self, Error> {
        let mut reader = step_control::Reader::new();
        let path_str = path.as_ref().to_string_lossy();
        let status = reader.read_file_charptr(&path_str);
        if status != if_select::ReturnStatus::Retvoid {
            return Err(Error::StepReadFailed);
        }
        let progress = message::ProgressRange::new();
        reader.transfer_roots(&progress);
        let inner = reader.one_shape();
        Ok(Self { inner })
    }

    pub fn write_step(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let mut writer = step_control::Writer::new();
        let progress = message::ProgressRange::new();
        // STEPControl_AsIs = 0
        let status = writer.transfer_shape_stepmodeltype_bool_progressrange(
            &self.inner,
            step_control::StepModelType::Asis,
            true, // compgraph
            &progress,
        );
        if status != if_select::ReturnStatus::Retdone {
            return Err(Error::StepWriteFailed);
        }
        let path_str = path.as_ref().to_string_lossy();
        let status = writer.write(&path_str);
        if status != if_select::ReturnStatus::Retdone {
            return Err(Error::StepWriteFailed);
        }
        Ok(())
    }

    pub fn read_iges(path: impl AsRef<Path>) -> Result<Self, Error> {
        let mut reader = iges_control::Reader::new();
        let path_str = path.as_ref().to_string_lossy();
        let status = reader.as_xs_control_reader_mut().read_file(&path_str);
        if status != if_select::ReturnStatus::Retvoid {
            return Err(Error::IgesReadFailed);
        }
        let progress = message::ProgressRange::new();
        reader.transfer_roots(&progress);
        let inner = reader.one_shape();
        Ok(Self { inner })
    }

    pub fn write_iges(&self, path: impl AsRef<Path>) -> Result<(), Error> {
        let mut writer = iges_control::Writer::new();
        let progress = message::ProgressRange::new();
        let success = writer.add_shape(&self.inner, &progress);
        if !success {
            return Err(Error::IgesWriteFailed);
        }
        writer.compute_model();
        let path_str = path.as_ref().to_string_lossy();
        let fnes = true;
        let success = writer.write(&path_str, fnes);
        if success {
            Ok(())
        } else {
            Err(Error::IgesWriteFailed)
        }
    }

    /// Boolean union: returns a new shape combining `self` and `other`.
    #[must_use]
    pub fn union(&self, other: &Shape) -> BooleanShape {
        let progress = message::ProgressRange::new();
        let mut fuse = b_rep_algo_api::Fuse::new_shape2_progressrange(
            &self.inner,
            &other.inner,
            &progress,
        );

        let make_shape = fuse.as_b_rep_builder_api_make_shape_mut();
        let result_shape = make_shape.shape();
        let shape = Shape::from_shape(result_shape);

        let new_edges = list_of_shape_to_edges(fuse.section_edges());

        BooleanShape { shape, new_edges }
    }

    /// Boolean intersection: returns a new shape containing only the volume
    /// common to both `self` and `other`.
    #[must_use]
    pub fn intersect(&self, other: &Shape) -> BooleanShape {
        let progress = message::ProgressRange::new();
        let mut common = b_rep_algo_api::Common::new_shape2_progressrange(
            &self.inner,
            &other.inner,
            &progress,
        );

        let make_shape = common.as_b_rep_builder_api_make_shape_mut();
        let result_shape = make_shape.shape();
        let shape = Shape::from_shape(result_shape);

        let new_edges = list_of_shape_to_edges(common.section_edges());

        BooleanShape { shape, new_edges }
    }

    pub fn write_stl<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        self.write_stl_with_tolerance(path, 0.001)
    }

    pub fn write_stl_with_tolerance<P: AsRef<Path>>(
        &self,
        path: P,
        triangulation_tolerance: f64,
    ) -> Result<(), Error> {
        let progress = message::ProgressRange::new();
        let is_relative = false;
        let angle_deflection = 0.5;
        let in_parallel = true;

        let mut mesher = b_rep_mesh::IncrementalMesh::new_shape_real_bool_real_bool(
            &self.inner,
            triangulation_tolerance,
            is_relative,
            angle_deflection,
            in_parallel,
        );
        mesher.perform(&progress);

        let mut writer = stl_api::Writer::new();
        let path_str = path.as_ref().to_string_lossy();
        let success = writer.write(&self.inner, &path_str, &progress);

        if success {
            Ok(())
        } else {
            Err(Error::StlWriteFailed)
        }
    }

    #[must_use]
    pub fn clean(&self) -> Self {
        let mut unifier = shape_upgrade::UnifySameDomain::new_shape_bool3(
            &self.inner,
            true,  // UnifyEdges
            true,  // UnifyFaces
            false, // ConcatBSplines
        );
        unifier.allow_internal_edges(false);
        unifier.build();
        let result = unifier.shape();
        Self::from_shape(result)
    }

    pub fn set_global_translation(&mut self, translation: DVec3) {
        let mut transform = gp::Trsf::new();
        let translation_vec = make_vec(translation);
        transform.set_translation_vec(&translation_vec);
        let location = top_loc::Location::new_trsf(&transform);
        let raise_exception = false;
        self.inner.move_(&location, raise_exception);
    }

    pub fn mesh(&self) -> Result<Mesh, Error> {
        self.mesh_with_tolerance(0.01)
    }

    pub fn mesh_with_tolerance(&self, triangulation_tolerance: f64) -> Result<Mesh, Error> {
        let mesher = Mesher::try_new(self, triangulation_tolerance)?;
        mesher.mesh()
    }

    pub fn edges(&self) -> super::EdgeIterator {
        let explorer = top_exp::Explorer::new_shape_shapeenum2(
            &self.inner,
            top_abs::ShapeEnum::Edge.into(),
            top_abs::ShapeEnum::Shape.into(),
        );
        super::EdgeIterator { explorer }
    }

    pub fn faces(&self) -> super::FaceIterator {
        let explorer = top_exp::Explorer::new_shape_shapeenum2(
            &self.inner,
            top_abs::ShapeEnum::Face.into(),
            top_abs::ShapeEnum::Shape.into(),
        );
        super::FaceIterator { explorer }
    }

    // TODO(bschwind) - Convert the return type to an iterator.
    pub fn faces_along_line(&self, line_origin: DVec3, line_dir: DVec3) -> Vec<LineFaceHitPoint> {
        let origin = make_point(line_origin);
        let dir = gp::Dir::new_real3(line_dir.x, line_dir.y, line_dir.z);
        let line = gp::Lin::new_pnt_dir(&origin, &dir);

        let mut intersector = b_rep_int_curve_surface::Inter::new();
        let tolerance = 0.001;
        intersector
            .init_shape_lin_real(&self.inner, &line, tolerance);

        let mut results = Vec::new();
        while intersector.more() {
            let face = intersector.face();
            let pnt = intersector.pnt();
            let t = intersector.w();
            let u = intersector.u();
            let v = intersector.v();

            results.push(LineFaceHitPoint {
                face: Face::from_face(face),
                t,
                u,
                v,
                point: dvec3(pnt.x(), pnt.y(), pnt.z()),
            });

            intersector.next();
        }

        results
    }

    #[must_use]
    pub fn hollow<T: AsRef<Face>>(
        &self,
        offset: f64,
        faces_to_remove: impl IntoIterator<Item = T>,
    ) -> Self {
        let mut faces_list = top_tools::ListOfShape::new();
        for face in faces_to_remove.into_iter() {
            faces_list.append(face.as_ref().inner.as_shape());
        }
        let progress = message::ProgressRange::new();
        let mut solid_maker = b_rep_offset_api::MakeThickSolid::new();
        // BRepOffset_Skin = 0, GeomAbs_Arc = 0
        solid_maker.make_thick_solid_by_join(
            &self.inner,
            &faces_list,
            offset,
            0.001, // tolerance
            b_rep_offset::Mode::Skin,
            false, // Intersection
            false, // SelfInter
            geom_abs::JoinType::Arc,
            false, // RemoveIntEdges
            &progress,
        );
        let make_shape = solid_maker
            .as_b_rep_builder_api_make_shape_mut();
        Self::from_shape(make_shape.shape())
    }

    #[must_use]
    pub fn offset_surface(&self, offset: f64) -> Self {
        let faces_to_remove: [Face; 0] = [];
        self.hollow(offset, faces_to_remove)
    }

    /// Drill a cylindrical hole along the line defined by point `p`
    /// and direction `dir`, with `radius`.
    #[must_use]
    pub fn drill_hole(&self, p: DVec3, dir: DVec3, radius: f64) -> Self {
        let axis = make_axis_1(p, dir);
        let mut hole_maker = b_rep_feat::MakeCylindricalHole::new();
        hole_maker.init_shape_ax1(&self.inner, &axis);
        hole_maker.perform_real(radius);
        let result = hole_maker.shape();
        Self::from_shape(result)
    }
}

/// Helper to convert a TopTools_ListOfShape reference to a Vec<Edge>.
pub(crate) fn list_of_shape_to_edges(list: &top_tools::ListOfShape) -> Vec<Edge> {
    let mut iter = list.iter();
    let mut edges = Vec::new();
    while let Some(shape) = iter.next() {
        let edge = topo_ds::edge(&shape);
        edges.push(Edge::from_edge(edge));
    }
    edges
}

/// Information about a point where a line hits (i.e. intersects) a face
pub struct LineFaceHitPoint {
    /// The face that is hit
    pub face: Face,
    /// The T parameter along the line
    pub t: f64,
    /// The U parameter on the face
    pub u: f64,
    /// The V parameter on the face
    pub v: f64,
    /// The intersection point
    pub point: DVec3,
}

pub struct ChamferMaker {
    inner: opencascade_sys::OwnedPtr<b_rep_fillet_api::MakeChamfer>,
}

impl ChamferMaker {
    pub fn new(shape: &Shape) -> Self {
        let inner = b_rep_fillet_api::MakeChamfer::new_shape(&shape.inner);
        Self { inner }
    }

    pub fn add_edge(&mut self, distance: f64, edge: &Edge) {
        self.inner.add_real_edge(distance, &edge.inner);
    }

    pub fn build(mut self) -> Shape {
        let progress = message::ProgressRange::new();
        self.inner.build(&progress);
        let shape = self.inner.shape();
        Shape::from_shape(shape)
    }
}
