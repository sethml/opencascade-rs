use cxx::UniquePtr;
use glam::{DVec2, DVec3};
use opencascade_sys::{geom_abs, gp, top_abs, top_exp, topo_ds};

mod boolean_shape;
mod compound;
mod edge;
mod face;
mod shape;
mod shell;
mod solid;
mod surface;
mod vertex;
mod wire;

pub use boolean_shape::*;
pub use compound::*;
pub use edge::*;
pub use face::*;
pub use shape::*;
pub use shell::*;
pub use solid::*;
pub use surface::*;
pub use vertex::*;
pub use wire::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ShapeType {
    /// Abstract topological data structure describes a basic entity.
    Shape,

    /// A zero-dimensional shape corresponding to a point in geometry.
    Vertex,

    /// A single dimensional shape correspondingto a curve, and bound
    /// by a vertex at each extremity.
    Edge,

    /// A sequence of edges connected by their vertices. It can be open
    /// or closed depending on whether the edges are linked or not.
    Wire,

    /// Part of a plane (in 2D geometry) or a surface(in 3D geometry)
    /// bounded by a closed wire. Its geometry is constrained (trimmed)
    /// by contours.
    Face,

    /// A set of faces connected by some of the
    /// edges of their wire boundaries. A shell can be open or closed.
    Shell,

    /// A part of 3D space bounded by shells.
    Solid,

    /// A set of solids connected by their faces. This expands
    /// the notions of Wire and Shell to solids.
    CompoundSolid,

    /// A group of any of the shapes below.
    Compound,
}

impl From<top_abs::ShapeEnum> for ShapeType {
    fn from(value: top_abs::ShapeEnum) -> Self {
        match value {
            top_abs::ShapeEnum::Compound => ShapeType::Compound,
            top_abs::ShapeEnum::Compsolid => ShapeType::CompoundSolid,
            top_abs::ShapeEnum::Solid => ShapeType::Solid,
            top_abs::ShapeEnum::Shell => ShapeType::Shell,
            top_abs::ShapeEnum::Face => ShapeType::Face,
            top_abs::ShapeEnum::Wire => ShapeType::Wire,
            top_abs::ShapeEnum::Edge => ShapeType::Edge,
            top_abs::ShapeEnum::Vertex => ShapeType::Vertex,
            top_abs::ShapeEnum::Shape => ShapeType::Shape,
        }
    }
}

impl From<ShapeType> for top_abs::ShapeEnum {
    fn from(value: ShapeType) -> Self {
        match value {
            ShapeType::Compound => top_abs::ShapeEnum::Compound,
            ShapeType::CompoundSolid => top_abs::ShapeEnum::Compsolid,
            ShapeType::Solid => top_abs::ShapeEnum::Solid,
            ShapeType::Shell => top_abs::ShapeEnum::Shell,
            ShapeType::Face => top_abs::ShapeEnum::Face,
            ShapeType::Wire => top_abs::ShapeEnum::Wire,
            ShapeType::Edge => top_abs::ShapeEnum::Edge,
            ShapeType::Vertex => top_abs::ShapeEnum::Vertex,
            ShapeType::Shape => top_abs::ShapeEnum::Shape,
        }
    }
}

pub trait IntoShape {
    fn into_shape(self) -> Shape;
}

impl<T: Into<Shape>> IntoShape for T {
    fn into_shape(self) -> Shape {
        self.into()
    }
}

pub fn make_point(p: DVec3) -> UniquePtr<gp::Pnt> {
    gp::Pnt::new_real3(p.x, p.y, p.z)
}

pub fn make_point2d(p: DVec2) -> UniquePtr<gp::Pnt2d> {
    gp::Pnt2d::new_real2(p.x, p.y)
}

fn make_dir(p: DVec3) -> UniquePtr<gp::Dir> {
    gp::Dir::new_real3(p.x, p.y, p.z)
}

pub(crate) fn make_vec(vec: DVec3) -> UniquePtr<gp::Vec> {
    gp::Vec::new_real3(vec.x, vec.y, vec.z)
}

fn make_axis_1(origin: DVec3, dir: DVec3) -> UniquePtr<gp::Ax1> {
    gp::Ax1::new_pnt_dir(&make_point(origin), &make_dir(dir))
}

pub fn make_axis_2(origin: DVec3, dir: DVec3) -> UniquePtr<gp::Ax2> {
    gp::Ax2::new_pnt_dir(&make_point(origin), &make_dir(dir))
}

pub struct EdgeIterator {
    explorer: UniquePtr<top_exp::Explorer>,
}

impl Iterator for EdgeIterator {
    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        if self.explorer.more() {
            let shape = self.explorer.current();
            let edge = topo_ds::edge(shape);
            let edge = Edge::from_edge(edge);

            self.explorer.pin_mut().next();

            Some(edge)
        } else {
            None
        }
    }
}

impl EdgeIterator {
    pub fn parallel_to(
        self,
        direction: Direction,
    ) -> impl Iterator<Item = <Self as Iterator>::Item> {
        let normalized_dir = direction.normalized_vec();

        self.filter(move |edge| {
            edge.edge_type() == EdgeType::Line
                && 1.0
                    - (edge.end_point() - edge.start_point()).normalize().dot(normalized_dir).abs()
                    < 0.0001
        })
    }
}

pub struct FaceIterator {
    explorer: UniquePtr<top_exp::Explorer>,
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
    Custom(DVec3),
}

impl Direction {
    pub fn normalized_vec(&self) -> DVec3 {
        match self {
            Self::PosX => DVec3::X,
            Self::NegX => DVec3::NEG_X,
            Self::PosY => DVec3::Y,
            Self::NegY => DVec3::NEG_Y,
            Self::PosZ => DVec3::Z,
            Self::NegZ => DVec3::NEG_Z,
            Self::Custom(dir) => dir.normalize(),
        }
    }
}

impl FaceIterator {
    pub fn farthest(self, direction: Direction) -> Face {
        self.try_farthest(direction).unwrap()
    }

    pub fn try_farthest(self, direction: Direction) -> Option<Face> {
        let normalized_dir = direction.normalized_vec();

        self.max_by(|face_1, face_2| {
            let dist_1 = face_1.center_of_mass().dot(normalized_dir);
            let dist_2 = face_2.center_of_mass().dot(normalized_dir);

            PartialOrd::partial_cmp(&dist_1, &dist_2)
                .expect("Face center of masses should contain no NaNs")
        })
    }
}

impl Iterator for FaceIterator {
    type Item = Face;

    fn next(&mut self) -> Option<Self::Item> {
        if self.explorer.more() {
            let shape = self.explorer.current();
            let face = topo_ds::face(shape);
            let face = Face::from_face(face);

            self.explorer.pin_mut().next();

            Some(face)
        } else {
            None
        }
    }
}

/// Given n and func, returns an iterator of (t, f(t)) values
/// where t is in the range [0, 1].
/// Note that n + 1 values are returned.
pub fn approximate_function<F: FnMut(f64) -> f64>(
    n: usize,
    mut func: F,
) -> impl Iterator<Item = (f64, f64)> {
    let mut count = 0;

    std::iter::from_fn(move || {
        if count > n {
            return None;
        }

        let t = count as f64 / n as f64;
        count += 1;

        let val = func(t);

        Some((t, val))
    })
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum JoinType {
    Arc,
    // TODO(mkovaxx): Figure out how to make tangent joints work without segfaults
    //Tangent,
    Intersection,
}

impl From<geom_abs::JoinType> for JoinType {
    fn from(value: geom_abs::JoinType) -> Self {
        match value {
            geom_abs::JoinType::Arc => JoinType::Arc,
            geom_abs::JoinType::Intersection => JoinType::Intersection,
            geom_abs::JoinType::Tangent => JoinType::Intersection, // Map unsupported Tangent to Intersection
        }
    }
}

impl From<JoinType> for geom_abs::JoinType {
    fn from(value: JoinType) -> Self {
        match value {
            JoinType::Arc => geom_abs::JoinType::Arc,
            JoinType::Intersection => geom_abs::JoinType::Intersection,
        }
    }
}
