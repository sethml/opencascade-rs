use std::iter::once;

use crate::{
    angle::{Angle, ToAngle},
    primitives::{make_dir, make_point, make_vec, Edge, Face, JoinType, Shape, Shell},
    Error,
};
use cxx::UniquePtr;
use glam::{dvec3, DVec3};
use opencascade_sys::{
    b_rep_builder_api, b_rep_offset_api, gp, shape_analysis, top_loc, top_tools,
    topo_ds,
};

pub struct Wire {
    pub(crate) inner: UniquePtr<topo_ds::Wire>,
}

impl AsRef<Wire> for Wire {
    fn as_ref(&self) -> &Wire {
        self
    }
}

/// Provides control over how an edge is considered "connected" to another edge.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EdgeConnection {
    /// The edges must share the same exact vertices to be considered connected.
    Exact,

    /// The endpoints of two edges must be with `tolerance` distance to be considered connected.
    Fuzzy { tolerance: f64 },
}

impl Default for EdgeConnection {
    fn default() -> Self {
        Self::Fuzzy { tolerance: 0.001 }
    }
}

impl Wire {
    pub(crate) fn from_wire(wire: &topo_ds::Wire) -> Self {
        let inner = wire.to_owned();
        Self { inner }
    }

    fn from_make_wire(mut make_wire: UniquePtr<b_rep_builder_api::MakeWire>) -> Self {
        Self::from_wire(make_wire.pin_mut().wire())
    }

    pub fn from_ordered_points(points: impl IntoIterator<Item = DVec3>) -> Result<Self, Error> {
        let points: Vec<_> = points.into_iter().collect();
        if points.len() < 2 {
            return Err(Error::NotEnoughPoints);
        }

        let (first, last) = (points.first().unwrap(), points.last().unwrap());
        let mut make_wire = b_rep_builder_api::MakeWire::new();

        if points.len() == 2 {
            make_wire.pin_mut().add_edge(&Edge::segment(*first, *last).inner);
        } else {
            for window in points.windows(2).chain(once([*last, *first].as_slice())) {
                let edge = Edge::segment(window[0], window[1]);
                make_wire.pin_mut().add_edge(&edge.inner);
            }
        }

        Ok(Self::from_make_wire(make_wire))
    }

    pub fn from_edges<'a>(edges: impl IntoIterator<Item = &'a Edge>) -> Self {
        let mut make_wire = b_rep_builder_api::MakeWire::new();

        for edge in edges.into_iter() {
            make_wire.pin_mut().add_edge(&edge.inner);
        }

        Self::from_make_wire(make_wire)
    }

    pub fn from_unordered_edges<T: AsRef<Edge>>(
        unordered_edges: impl IntoIterator<Item = T>,
        edge_connection: EdgeConnection,
    ) -> Self {
        let mut edges_seq = top_tools::HSequenceOfShape::new();

        for edge in unordered_edges {
            edges_seq.pin_mut().append_shape(edge.as_ref().inner.as_shape());
        }

        let mut edges_handle = top_tools::HSequenceOfShape::to_handle(edges_seq);
        let wires_seq = top_tools::HSequenceOfShape::new();
        let mut wires_handle = top_tools::HSequenceOfShape::to_handle(wires_seq);

        let (tolerance, shared) = match edge_connection {
            EdgeConnection::Exact => (0.0, true),
            EdgeConnection::Fuzzy { tolerance } => (tolerance, false),
        };

        shape_analysis::FreeBounds::connect_edges_to_wires(
            edges_handle.pin_mut(), tolerance, shared, wires_handle.pin_mut(),
        );

        let mut make_wire = b_rep_builder_api::MakeWire::new();

        let wires_obj = wires_handle.get();
        let wire_seq = wires_obj.as_sequence_of_shape();
        let wire_len = wire_seq.size();

        for index in 1..=wire_len {
            let wire_shape = wire_seq.value(index);
            let wire = topo_ds::wire(wire_shape);

            make_wire.pin_mut().add_wire(wire);
        }

        Self::from_make_wire(make_wire)
    }

    pub fn from_wires<'a>(wires: impl IntoIterator<Item = &'a Wire>) -> Self {
        let mut make_wire = b_rep_builder_api::MakeWire::new();

        for wire in wires.into_iter() {
            make_wire.pin_mut().add_wire(&wire.inner);
        }

        Self::from_make_wire(make_wire)
    }

    #[must_use]
    pub fn mirror_along_axis(&self, axis_origin: DVec3, axis_dir: DVec3) -> Self {
        let axis_dir = make_dir(axis_dir);
        let axis = gp::Ax1::new_pnt_dir(&make_point(axis_origin), &axis_dir);

        let mut transform = gp::Trsf::new();

        transform.pin_mut().set_mirror_ax1(&axis);

        let wire_shape = self.inner.as_shape();

        let mut brep_transform =
            b_rep_builder_api::Transform::new_shape_trsf_bool2(wire_shape, &transform, false, false);

        let mirrored_shape = brep_transform.pin_mut().shape();
        let mirrored_wire = topo_ds::wire(mirrored_shape);

        Self::from_wire(mirrored_wire)
    }

    pub fn rect(width: f64, height: f64) -> Self {
        let half_width = width / 2.0;
        let half_height = height / 2.0;

        let p1 = dvec3(-half_width, half_height, 0.0);
        let p2 = dvec3(half_width, half_height, 0.0);
        let p3 = dvec3(half_width, -half_height, 0.0);
        let p4 = dvec3(-half_width, -half_height, 0.0);

        let top = Edge::segment(p1, p2);
        let right = Edge::segment(p2, p3);
        let bottom = Edge::segment(p3, p4);
        let left = Edge::segment(p4, p1);

        Self::from_edges([&top, &right, &bottom, &left])
    }

    #[must_use]
    pub fn fillet(&self, radius: f64) -> Wire {
        // Create a face from this wire
        let face = Face::from_wire(self).fillet(radius);
        let inner = opencascade_sys::b_rep_tools::outer_wire(&face.inner);

        Self { inner }
    }

    /// Chamfer the wire edges at each vertex by a given distance.
    #[must_use]
    pub fn chamfer(&self, distance_1: f64) -> Wire {
        let face = Face::from_wire(self).chamfer(distance_1);
        let inner = opencascade_sys::b_rep_tools::outer_wire(&face.inner);

        Self { inner }
    }

    /// Offset the wire by a given distance and join settings
    #[must_use]
    pub fn offset(&self, distance: f64, join_type: JoinType) -> Self {
        let mut make_offset =
            b_rep_offset_api::MakeOffset::new_wire_jointype(&self.inner, join_type.to_geom_abs());
        make_offset.pin_mut().perform(distance, 0.0);

        let offset_shape = make_offset.pin_mut().shape();
        let result_wire = topo_ds::wire(offset_shape);

        Self::from_wire(result_wire)
    }

    /// Sweep the wire along a path to produce a shell
    #[must_use]
    pub fn sweep_along(&self, path: &Wire) -> Shell {
        let profile_shape = self.inner.as_shape();
        let mut make_pipe = b_rep_offset_api::MakePipe::new_wire_shape(&path.inner, profile_shape);

        let pipe_shape = make_pipe.pin_mut().shape();
        let result_shell = topo_ds::shell(pipe_shape);

        Shell::from_shell(result_shell)
    }

    /// Sweep the wire along a path, modulated by a function, to produce a shell
    #[must_use]
    pub fn sweep_along_with_radius_values(
        &self,
        path: &Wire,
        radius_values: impl IntoIterator<Item = (f64, f64)>,
    ) -> Shell {
        crate::make_pipe_shell::make_pipe_shell_with_law_function_shell(path, self, radius_values)
    }

    #[must_use]
    pub fn translate(&self, offset: DVec3) -> Self {
        self.transform(offset, dvec3(1.0, 0.0, 0.0), 0.degrees())
    }

    #[must_use]
    pub fn transform(&self, translation: DVec3, rotation_axis: DVec3, angle: Angle) -> Self {
        let mut transform = gp::Trsf::new();
        let rotation_axis_vec =
            gp::Ax1::new_pnt_dir(&make_point(DVec3::ZERO), &make_dir(rotation_axis));
        let translation_vec = make_vec(translation);

        transform.pin_mut().set_rotation_ax1_real(&rotation_axis_vec, angle.radians());
        transform.pin_mut().set_translation_vec(&translation_vec);
        let location = top_loc::Location::new_trsf(&transform);

        let wire_shape = self.inner.as_shape();
        let mut wire_shape = Shape::from_shape(wire_shape).inner;

        let raise_exception = false;
        wire_shape.pin_mut().move_(&location, raise_exception);

        let translated_wire = topo_ds::wire(&wire_shape);

        Self::from_wire(translated_wire)
    }

    pub fn to_face(self) -> Face {
        let only_plane = false;
        let make_face = b_rep_builder_api::MakeFace::new_wire_bool(&self.inner, only_plane);

        Face::from_face(make_face.face())
    }

    // Create a closure-based API
    pub fn freeform() {}
}

pub struct WireBuilder {
    inner: UniquePtr<b_rep_builder_api::MakeWire>,
}

impl Default for WireBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl WireBuilder {
    pub fn new() -> Self {
        let make_wire = b_rep_builder_api::MakeWire::new();

        Self { inner: make_wire }
    }

    pub fn add_edge(&mut self, edge: &Edge) {
        self.inner.pin_mut().add_edge(&edge.inner);
    }

    pub fn build(self) -> Wire {
        Wire::from_make_wire(self.inner)
    }
}
