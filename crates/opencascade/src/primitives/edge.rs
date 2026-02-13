// NOTE: This file is partially blocked because:
// - EdgeType enum conversion from GeomAbs_CurveType is blocked (enums not generated)
// - arc() is blocked because gc::ffi::HandleGeomTrimmedCurve doesn't have upcast methods
//   (the Handle type is declared locally in gc.rs instead of imported from geom.rs)
// See TRANSITION_PLAN.md for details.

use crate::primitives::{make_point, make_axis_2, make_vec};
use cxx::UniquePtr;
use glam::{dvec3, DVec3};
use opencascade_sys::{b_rep_adaptor, b_rep_builder_api, gc_pnts, geom, geom_api, gp, t_colgp, topo_ds};

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

// NOTE: From<GeomAbs_CurveType> is blocked because enums aren't generated
// When enums are supported, uncomment this impl

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
        let mut poles = t_colgp::Array1OfPnt::new_with_bounds(1, n);
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
        let mut poles = t_colgp::Array1OfPnt::new_with_bounds(1, n);
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

    // NOTE: arc is blocked because gc::ffi::HandleGeomTrimmedCurve doesn't have upcast
    // methods to convert to HandleGeomCurve. The Handle type is declared locally in gc.rs
    // instead of being imported from geom.rs, so the impl block in geom.rs doesn't apply.
    #[allow(unused)]
    pub fn arc(_p1: DVec3, _p2: DVec3, _p3: DVec3) -> Self {
        unimplemented!(
            "Edge::arc is blocked pending Handle upcast support across modules"
        );
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

    // NOTE: edge_type is blocked because GeomAbs_CurveType enum is not generated
    #[allow(unused)]
    pub fn edge_type(&self) -> EdgeType {
        unimplemented!(
            "Edge::edge_type is blocked pending GeomAbs_CurveType enum support"
        );
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
