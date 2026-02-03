// NOTE: This file is partially blocked because:
// - TopAbs_ShapeEnum not generated (enums blocked)
// - BRepFilletAPI needs ChFi3d_FilletShape enum (not generated)
// - STEP/IGES readers need custom helper functions
// - Mesher is blocked
// See TRANSITION_PLAN.md for details.

use crate::{
    mesh::Mesh,
    primitives::{
        make_axis_2, BooleanShape, Compound, Edge, Face, Shell, Solid,
        Vertex, Wire,
    },
    Error,
};
use cxx::UniquePtr;
use glam::{dvec3, DVec3};
use opencascade_sys::{b_rep_algo_api, b_rep_mesh, b_rep_prim_api, gp, message, stl_api, topo_ds};
use std::path::Path;

pub struct Shape {
    pub(crate) inner: UniquePtr<topo_ds::Shape>,
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
        let make_shape = make_sphere.pin_mut().as_b_rep_builder_api_make_shape_mut();
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
        let make_shape = make_cone.pin_mut().as_b_rep_builder_api_make_shape_mut();
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
        let make_shape = make_torus.pin_mut().as_b_rep_builder_api_make_shape_mut();
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

    // NOTE: empty() is blocked because BRep_Builder default constructor not generated
    #[allow(unused)]
    pub fn empty() -> Self {
        unimplemented!(
            "Shape::empty is blocked pending BRep_Builder default constructor support"
        );
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
        let make_shape = make_box.pin_mut().as_b_rep_builder_api_make_shape_mut();
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
        let make_shape = make_cylinder.pin_mut().as_b_rep_builder_api_make_shape_mut();
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

    // NOTE: shape_type is blocked because TopAbs_ShapeEnum not generated
    #[allow(unused)]
    pub fn shape_type(&self) -> super::ShapeType {
        unimplemented!(
            "Shape::shape_type is blocked pending TopAbs_ShapeEnum support"
        );
    }

    // NOTE: fillet_edge is blocked because BRepFilletAPI_MakeFillet needs edge casting
    #[allow(unused)]
    #[must_use]
    pub fn fillet_edge(&self, _radius: f64, _edge: &Edge) -> Self {
        unimplemented!(
            "Shape::fillet_edge is blocked pending BRepFilletAPI_MakeFillet support"
        );
    }

    #[allow(unused)]
    #[must_use]
    pub fn variable_fillet_edge(
        &self,
        _radius_values: impl IntoIterator<Item = (f64, f64)>,
        _edge: &Edge,
    ) -> Self {
        unimplemented!(
            "Shape::variable_fillet_edge is blocked pending BRepFilletAPI_MakeFillet support"
        );
    }

    #[allow(unused)]
    #[must_use]
    pub fn chamfer_edge(&self, _distance: f64, _edge: &Edge) -> Self {
        unimplemented!(
            "Shape::chamfer_edge is blocked pending BRepFilletAPI_MakeChamfer support"
        );
    }

    #[allow(unused)]
    #[must_use]
    pub fn fillet_edges<T: AsRef<Edge>>(
        &self,
        _radius: f64,
        _edges: impl IntoIterator<Item = T>,
    ) -> Self {
        unimplemented!(
            "Shape::fillet_edges is blocked pending BRepFilletAPI_MakeFillet support"
        );
    }

    #[allow(unused)]
    #[must_use]
    pub fn variable_fillet_edges<T: AsRef<Edge>>(
        &self,
        _radius_values: impl IntoIterator<Item = (f64, f64)>,
        _edges: impl IntoIterator<Item = T>,
    ) -> Self {
        unimplemented!(
            "Shape::variable_fillet_edges is blocked pending BRepFilletAPI_MakeFillet support"
        );
    }

    #[allow(unused)]
    #[must_use]
    pub fn chamfer_edges<T: AsRef<Edge>>(
        &self,
        _distance: f64,
        _edges: impl IntoIterator<Item = T>,
    ) -> Self {
        unimplemented!(
            "Shape::chamfer_edges is blocked pending BRepFilletAPI_MakeChamfer support"
        );
    }

    // NOTE: fillet (all edges) is blocked because edges() is blocked
    #[allow(unused)]
    #[must_use]
    pub fn fillet(&self, _radius: f64) -> Self {
        unimplemented!(
            "Shape::fillet is blocked pending TopAbs_ShapeEnum enum support"
        );
    }

    #[allow(unused)]
    #[must_use]
    pub fn chamfer(&self, _distance: f64) -> Self {
        unimplemented!(
            "Shape::chamfer is blocked pending TopAbs_ShapeEnum enum support"
        );
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

    // NOTE: read_step is blocked because STEPControl_Reader helpers not generated
    #[allow(unused)]
    pub fn read_step(_path: impl AsRef<Path>) -> Result<Self, Error> {
        unimplemented!(
            "Shape::read_step is blocked pending STEPControl_Reader helper functions"
        );
    }

    // NOTE: write_step is blocked because STEPControl_Writer helpers not generated
    #[allow(unused)]
    pub fn write_step(&self, _path: impl AsRef<Path>) -> Result<(), Error> {
        unimplemented!(
            "Shape::write_step is blocked pending STEPControl_Writer helper functions"
        );
    }

    // NOTE: read_iges is blocked because IGESControl_Reader helpers not generated
    #[allow(unused)]
    pub fn read_iges(_path: impl AsRef<Path>) -> Result<Self, Error> {
        unimplemented!(
            "Shape::read_iges is blocked pending IGESControl_Reader helper functions"
        );
    }

    // NOTE: write_iges is blocked because IGESControl_Writer helpers not generated
    #[allow(unused)]
    pub fn write_iges(&self, _path: impl AsRef<Path>) -> Result<(), Error> {
        unimplemented!(
            "Shape::write_iges is blocked pending IGESControl_Writer helper functions"
        );
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
    pub fn intersect(&self, other: &Shape) -> BooleanShape {
        let progress = message::ProgressRange::new();
        let mut common = b_rep_algo_api::Common::new_shape2_progressrange(
            &self.inner,
            &other.inner,
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

    /// Writes the shape to an STL file at the given path.
    /// 
    /// Note: This will automatically mesh the shape if it isn't already meshed.
    /// Uses a default linear deflection tolerance of 0.1.
    pub fn write_stl<P: AsRef<Path>>(&self, path: P) -> Result<(), Error> {
        self.write_stl_with_tolerance(path, 0.1)
    }

    /// Writes the shape to an STL file with custom meshing tolerance.
    /// 
    /// The `triangulation_tolerance` parameter controls the linear deflection 
    /// (maximum deviation between the mesh and the original surface).
    /// Smaller values create finer meshes but larger files.
    pub fn write_stl_with_tolerance<P: AsRef<Path>>(
        &self,
        path: P,
        triangulation_tolerance: f64,
    ) -> Result<(), Error> {
        // Mesh the shape first
        let progress = message::ProgressRange::new();
        let is_relative = false;
        let angle_deflection = 0.5; // radians, ~30 degrees
        let in_parallel = true;
        
        let mut mesher = b_rep_mesh::IncrementalMesh::new_shape_real_bool_real_bool(
            &self.inner,
            triangulation_tolerance,
            is_relative,
            angle_deflection,
            in_parallel,
        );
        mesher.pin_mut().perform_progressrange(&progress);

        // Write to STL
        let mut writer = stl_api::Writer::new();
        let path_str = path.as_ref().to_str()
            .ok_or_else(|| Error::PathContainsNonUtf8Characters)?;
        
        let success = writer.pin_mut().write(&self.inner, path_str, &progress);
        
        if success {
            Ok(())
        } else {
            Err(Error::StlWriteFailed)
        }
    }

    // NOTE: clean is blocked because ShapeUpgrade_UnifySameDomain not generated
    #[allow(unused)]
    #[must_use]
    pub fn clean(&self) -> Self {
        unimplemented!(
            "Shape::clean is blocked pending ShapeUpgrade_UnifySameDomain support"
        );
    }

    // NOTE: set_global_translation is blocked because TopLoc_Location helpers not generated
    #[allow(unused)]
    pub fn set_global_translation(&mut self, _translation: DVec3) {
        unimplemented!(
            "Shape::set_global_translation is blocked pending TopLoc_Location helpers"
        );
    }

    // NOTE: mesh is blocked because Mesher is blocked
    #[allow(unused)]
    pub fn mesh(&self) -> Result<Mesh, Error> {
        unimplemented!(
            "Shape::mesh is blocked pending Mesher support"
        );
    }

    #[allow(unused)]
    pub fn mesh_with_tolerance(&self, _triangulation_tolerance: f64) -> Result<Mesh, Error> {
        unimplemented!(
            "Shape::mesh_with_tolerance is blocked pending Mesher support"
        );
    }

    // NOTE: edges is blocked because TopExp_Explorer needs TopAbs_ShapeEnum
    #[allow(unused)]
    pub fn edges(&self) -> super::EdgeIterator {
        unimplemented!(
            "Shape::edges is blocked pending TopAbs_ShapeEnum enum support"
        );
    }

    // NOTE: faces is blocked because TopExp_Explorer needs TopAbs_ShapeEnum
    #[allow(unused)]
    pub fn faces(&self) -> super::FaceIterator {
        unimplemented!(
            "Shape::faces is blocked pending TopAbs_ShapeEnum enum support"
        );
    }

    // NOTE: faces_along_line is blocked because BRepIntCurveSurface_Inter not generated
    #[allow(unused)]
    pub fn faces_along_line(&self, _line_origin: DVec3, _line_dir: DVec3) -> Vec<LineFaceHitPoint> {
        unimplemented!(
            "Shape::faces_along_line is blocked pending BRepIntCurveSurface_Inter support"
        );
    }

    // NOTE: hollow is blocked because BRepOffsetAPI_MakeThickSolid helpers not generated
    #[allow(unused)]
    #[must_use]
    pub fn hollow<T: AsRef<Face>>(
        &self,
        _offset: f64,
        _faces_to_remove: impl IntoIterator<Item = T>,
    ) -> Self {
        unimplemented!(
            "Shape::hollow is blocked pending BRepOffsetAPI_MakeThickSolid support"
        );
    }

    #[allow(unused)]
    #[must_use]
    pub fn offset_surface(&self, _offset: f64) -> Self {
        unimplemented!(
            "Shape::offset_surface is blocked pending hollow support"
        );
    }

    // NOTE: drill_hole is blocked because BRepFeat_MakeCylindricalHole not generated
    #[allow(unused)]
    #[must_use]
    pub fn drill_hole(&self, _p: DVec3, _dir: DVec3, _radius: f64) -> Self {
        unimplemented!(
            "Shape::drill_hole is blocked pending BRepFeat_MakeCylindricalHole support"
        );
    }
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

// NOTE: ChamferMaker is blocked because BRepFilletAPI_MakeChamfer is not fully accessible
pub struct ChamferMaker {
    _private: (),
}

impl ChamferMaker {
    #[allow(unused)]
    pub fn new(_shape: &Shape) -> Self {
        unimplemented!(
            "ChamferMaker::new is blocked pending BRepFilletAPI_MakeChamfer support"
        );
    }

    #[allow(unused)]
    pub fn add_edge(&mut self, _distance: f64, _edge: &Edge) {
        unimplemented!(
            "ChamferMaker::add_edge is blocked pending BRepFilletAPI_MakeChamfer support"
        );
    }

    #[allow(unused)]
    pub fn build(self) -> Shape {
        unimplemented!(
            "ChamferMaker::build is blocked pending BRepFilletAPI_MakeChamfer support"
        );
    }
}
