// NOTE: arc() is now unblocked - HandleGeomTrimmedCurve is unified in ffi.rs
// and has to_handle_curve() upcast available.

use crate::primitives::{make_point, make_axis_2, make_vec};
use cxx::UniquePtr;
use glam::{dvec3, DVec3};
use opencascade_sys::{b_rep_adaptor, b_rep_builder_api, gc, gc_pnts, geom, geom_abs, geom_api, gp, t_colgp, topo_ds};

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

impl From<geom_abs::CurveType> for EdgeType {
    fn from(value: geom_abs::CurveType) -> Self {
        match value {
            geom_abs::CurveType::Line => EdgeType::Line,
            geom_abs::CurveType::Circle => EdgeType::Circle,
            geom_abs::CurveType::Ellipse => EdgeType::Ellipse,
            geom_abs::CurveType::Hyperbola => EdgeType::Hyperbola,
            geom_abs::CurveType::Parabola => EdgeType::Parabola,
            geom_abs::CurveType::Beziercurve => EdgeType::BezierCurve,
            geom_abs::CurveType::Bsplinecurve => EdgeType::BSplineCurve,
            geom_abs::CurveType::Offsetcurve => EdgeType::OffsetCurve,
            geom_abs::CurveType::Othercurve => EdgeType::OtherCurve,
        }
    }
}

impl From<EdgeType> for geom_abs::CurveType {
    fn from(value: EdgeType) -> Self {
        match value {
            EdgeType::Line => geom_abs::CurveType::Line,
            EdgeType::Circle => geom_abs::CurveType::Circle,
            EdgeType::Ellipse => geom_abs::CurveType::Ellipse,
            EdgeType::Hyperbola => geom_abs::CurveType::Hyperbola,
            EdgeType::Parabola => geom_abs::CurveType::Parabola,
            EdgeType::BezierCurve => geom_abs::CurveType::Beziercurve,
            EdgeType::BSplineCurve => geom_abs::CurveType::Bsplinecurve,
            EdgeType::OffsetCurve => geom_abs::CurveType::Offsetcurve,
            EdgeType::OtherCurve => geom_abs::CurveType::Othercurve,
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
        let make_edge = b_rep_builder_api::MakeEdge::new_pnt2(&make_point(p1), &make_point(p2));
        Self::from_make_edge(make_edge)
    }

    pub fn bezier(points: impl IntoIterator<Item = DVec3>) -> Self {
        let points: Vec<DVec3> = points.into_iter().collect();
        let n = points.len() as i32;
        let mut poles = t_colgp::Array1OfPnt::new_int2(1, n);
        for (i, p) in points.iter().enumerate() {
            let pnt = make_point(*p);
            poles.pin_mut().set_value(i as i32 + 1, &pnt);
        }
        let curve = geom::BezierCurve::new_array1ofpnt(&poles);
        let handle = geom::BezierCurve::to_handle(curve);
        let handle_curve = handle.to_handle_curve();
        let make_edge = b_rep_builder_api::MakeEdge::new_handlecurve(&handle_curve);
        Self::from_make_edge(make_edge)
    }

    pub fn circle(center: DVec3, normal: DVec3, radius: f64) -> Self {
        let axis = make_axis_2(center, normal);
        let circ = gp::Circ::new_ax2_real(&axis, radius);
        let make_edge = b_rep_builder_api::MakeEdge::new_circ(&circ);
        Self::from_make_edge(make_edge)
    }

    pub fn ellipse() {}

    pub fn spline_from_points(
        points: impl IntoIterator<Item = DVec3>,
        tangents: Option<(DVec3, DVec3)>,
    ) -> Self {
        let points: Vec<DVec3> = points.into_iter().collect();
        let n = points.len() as i32;

        // Build Array1OfPnt, then wrap in HArray1OfPnt for the interpolator
        let mut poles = t_colgp::Array1OfPnt::new_int2(1, n);
        for (i, p) in points.iter().enumerate() {
            let pnt = make_point(*p);
            poles.pin_mut().set_value(i as i32 + 1, &pnt);
        }
        let harray = t_colgp::HArray1OfPnt::new_array1ofpnt(&poles);
        let harray_handle = t_colgp::HArray1OfPnt::to_handle(harray);

        let mut interpolator = geom_api::Interpolate::new_handleharray1ofpnt_bool_real(
            &harray_handle,
            false,
            1.0e-6,
        );

        if let Some((start_tangent, end_tangent)) = tangents {
            let start_vec = make_vec(start_tangent);
            let end_vec = make_vec(end_tangent);
            interpolator.pin_mut().load_vec2_bool(&start_vec, &end_vec, false);
        }

        interpolator.pin_mut().perform();
        let bspline_handle = interpolator.curve();
        let handle_curve = bspline_handle.to_handle_curve();
        let make_edge = b_rep_builder_api::MakeEdge::new_handlecurve(&handle_curve);
        Self::from_make_edge(make_edge)
    }

    /// Create an arc passing through three points.
    pub fn arc(p1: DVec3, p2: DVec3, p3: DVec3) -> Self {
        let gp_p1 = make_point(p1);
        let gp_p2 = make_point(p2);
        let gp_p3 = make_point(p3);
        let arc = gc::MakeArcOfCircle::new_pnt3(&gp_p1, &gp_p2, &gp_p3);
        let handle_trimmed = arc.value();
        let handle_curve = handle_trimmed.to_handle_curve();
        let make_edge = b_rep_builder_api::MakeEdge::new_handlecurve(&handle_curve);
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
            0.1,   // angular deflection
            0.1,   // curvature deflection
            2,     // minimum points
            1.0e-9, // UTol
            0.0,   // MinLen
        );

        ApproximationSegmentIterator { count: 1, approximator }
    }

    pub fn tangent_arc(_p1: DVec3, _tangent: DVec3, _p3: DVec3) {}

    pub fn edge_type(&self) -> EdgeType {
        let curve = b_rep_adaptor::Curve::new_edge(&self.inner);
        let raw = curve.get_type();
        let curve_type = geom_abs::CurveType::try_from(raw)
            .expect("Invalid CurveType value from OCCT");
        curve_type.into()
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
