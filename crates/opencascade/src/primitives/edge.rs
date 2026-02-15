use crate::primitives::{make_axis_2, make_point};
use cxx::UniquePtr;
use glam::{dvec3, DVec3};
use opencascade_sys::{b_rep_adaptor, b_rep_builder_api, gc, gc_pnts, geom, geom_abs, geom_api, gp, t_colgp, topo_ds};

use super::make_vec;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EdgeType {
    Line,
    Circle,
    Ellipse,
    Hyperbola,
    Parabola,
    BezierCurve,
    BSplineCurve,
    OffsetCurve,
    OtherCurve,
}

impl EdgeType {
    pub fn from_curve_type(value: geom_abs::CurveType) -> Self {
        match value {
            geom_abs::CurveType::Line => Self::Line,
            geom_abs::CurveType::Circle => Self::Circle,
            geom_abs::CurveType::Ellipse => Self::Ellipse,
            geom_abs::CurveType::Hyperbola => Self::Hyperbola,
            geom_abs::CurveType::Parabola => Self::Parabola,
            geom_abs::CurveType::Beziercurve => Self::BezierCurve,
            geom_abs::CurveType::Bsplinecurve => Self::BSplineCurve,
            geom_abs::CurveType::Offsetcurve => Self::OffsetCurve,
            geom_abs::CurveType::Othercurve => Self::OtherCurve,
        }
    }
}

pub struct Edge {
    pub(crate) inner: UniquePtr<topo_ds::Edge>,
}

impl AsRef<Edge> for Edge {
    fn as_ref(&self) -> &Edge {
        self
    }
}

impl Edge {
    pub(crate) fn from_edge(edge: &topo_ds::Edge) -> Self {
        let inner = edge.to_owned();
        Self { inner }
    }

    fn from_make_edge(mut make_edge: UniquePtr<b_rep_builder_api::MakeEdge>) -> Self {
        Self::from_edge(make_edge.pin_mut().edge())
    }

    pub fn segment(p1: DVec3, p2: DVec3) -> Self {
        let make_edge =
            b_rep_builder_api::MakeEdge::new_pnt2(&make_point(p1), &make_point(p2));

        Self::from_make_edge(make_edge)
    }

    pub fn bezier(points: impl IntoIterator<Item = DVec3>) -> Self {
        let points: Vec<_> = points.into_iter().collect();
        let mut array = t_colgp::Array1OfPnt::new_int2(1, points.len() as i32);
        for (index, point) in points.into_iter().enumerate() {
            array.pin_mut().set_value(index as i32 + 1, &make_point(point));
        }

        let bezier = geom::BezierCurve::new_array1ofpnt(&array);
        let bezier_handle = geom::BezierCurve::to_handle(bezier);
        let curve_handle = bezier_handle.to_handle_curve();

        let make_edge = b_rep_builder_api::MakeEdge::new_handlegeomcurve(&curve_handle);
        Self::from_make_edge(make_edge)
    }

    pub fn circle(center: DVec3, normal: DVec3, radius: f64) -> Self {
        let axis = make_axis_2(center, normal);

        let make_circle = gp::Circ::new_ax2_real(&axis, radius);
        let make_edge = b_rep_builder_api::MakeEdge::new_circ(&make_circle);

        Self::from_make_edge(make_edge)
    }

    pub fn ellipse() {}

    pub fn spline_from_points(
        points: impl IntoIterator<Item = DVec3>,
        tangents: Option<(DVec3, DVec3)>,
    ) -> Self {
        let points: Vec<_> = points.into_iter().collect();
        let mut array = t_colgp::HArray1OfPnt::new_int2(1, points.len() as i32);
        for (index, point) in points.into_iter().enumerate() {
            array.pin_mut().as_array1_of_pnt_mut().set_value(index as i32 + 1, &make_point(point));
        }
        let array_handle = t_colgp::HArray1OfPnt::to_handle(array);

        let periodic = false;
        let tolerance = 1.0e-7;
        let mut interpolate = geom_api::Interpolate::new_handletcolgpharray1ofpnt_bool_real(
            &array_handle, periodic, tolerance,
        );
        if let Some((t_start, t_end)) = tangents {
            interpolate.pin_mut().load_vec2_bool(&make_vec(t_start), &make_vec(t_end), true);
        }

        interpolate.pin_mut().perform();
        let bspline_handle = interpolate.curve();
        let curve_handle = bspline_handle.to_handle_curve();

        let make_edge = b_rep_builder_api::MakeEdge::new_handlegeomcurve(&curve_handle);
        Self::from_make_edge(make_edge)
    }

    pub fn arc(p1: DVec3, p2: DVec3, p3: DVec3) -> Self {
        let make_arc = gc::MakeArcOfCircle::new_pnt3(
            &make_point(p1),
            &make_point(p2),
            &make_point(p3),
        );

        let trimmed_handle = make_arc.value();
        let curve_handle = trimmed_handle.to_handle_curve();
        let make_edge = b_rep_builder_api::MakeEdge::new_handlegeomcurve(&curve_handle);

        Self::from_make_edge(make_edge)
    }

    pub fn start_point(&self) -> DVec3 {
        let curve = b_rep_adaptor::Curve::new_edge(&self.inner);
        let start_param = curve.first_parameter();
        let point = curve.value(start_param);

        dvec3(point.x(), point.y(), point.z())
    }

    pub fn end_point(&self) -> DVec3 {
        let curve = b_rep_adaptor::Curve::new_edge(&self.inner);
        let last_param = curve.last_parameter();
        let point = curve.value(last_param);

        dvec3(point.x(), point.y(), point.z())
    }

    pub fn approximation_segments(&self) -> ApproximationSegmentIterator {
        let adaptor_curve = b_rep_adaptor::Curve::new_edge(&self.inner);
        let approximator = gc_pnts::TangentialDeflection::new_curve_real2_int_real2(
            adaptor_curve.as_adaptor3d_curve(),
            0.1, 0.1, 2, 1.0e-9, 1.0e-7,
        );

        ApproximationSegmentIterator { count: 1, approximator }
    }

    pub fn tangent_arc(_p1: DVec3, _tangent: DVec3, _p3: DVec3) {}

    pub fn edge_type(&self) -> EdgeType {
        let curve = b_rep_adaptor::Curve::new_edge(&self.inner);

        EdgeType::from_curve_type(curve.get_type())
    }
}

pub struct ApproximationSegmentIterator {
    count: usize,
    approximator: UniquePtr<gc_pnts::TangentialDeflection>,
}

impl Iterator for ApproximationSegmentIterator {
    type Item = DVec3;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count <= self.approximator.nb_points() as usize {
            let point = self.approximator.value(self.count as i32);

            self.count += 1;
            Some(dvec3(point.x(), point.y(), point.z()))
        } else {
            None
        }
    }
}
