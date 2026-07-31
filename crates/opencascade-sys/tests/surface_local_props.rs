//! Tests for `BRepLProp_SLProps` — the surface differential-geometry API.
//!
//! `GeomLProp_SLProps` / `BRepLProp_SLProps` are aliases for instantiations of the
//! `GeomLProp_SLPropsBase` template, so before template-instantiation support they
//! were bound as opaque types with no methods. These tests check the real numbers,
//! not just that the calls link: a sphere of radius R has Gaussian curvature
//! 1/R^2 and principal curvatures of magnitude 1/R everywhere, and a planar face
//! has zero curvature, so both are exact oracles.

use opencascade_sys::b_rep_adaptor;
use opencascade_sys::b_rep_l_prop::SLProps;
use opencascade_sys::b_rep_prim_api::{MakeBox, MakeSphere};
use opencascade_sys::top_abs;
use opencascade_sys::top_exp;
use opencascade_sys::topo_ds;

/// Second-order properties, with a tight resolution for the degeneracy tests.
const DEGREE: i32 = 2;
const RESOLUTION: f64 = 1e-9;

/// Build a surface adaptor over the first face of a shape. The explorer only has
/// to outlive the borrowed face, not the adaptor, which owns its copy.
fn first_face_adaptor(
    shape: &topo_ds::Shape,
) -> opencascade_sys::OwnedPtr<b_rep_adaptor::Surface> {
    let explorer = top_exp::Explorer::new_shape_shapeenum2(
        shape,
        top_abs::ShapeEnum::Face,
        top_abs::ShapeEnum::Shape,
    );
    assert!(explorer.more(), "shape should have at least one face");
    b_rep_adaptor::Surface::new_face(topo_ds::face_shape(explorer.current()))
}

#[test]
fn sphere_curvature_matches_closed_form() {
    const R: f64 = 5.0;
    let mut sphere = MakeSphere::new_real(R);
    let shape = sphere.as_b_rep_builder_api_make_shape_mut().shape();
    let adaptor = first_face_adaptor(shape);
    let mut props = SLProps::new_surface_int_real(&adaptor, DEGREE, RESOLUTION);

    // Sample a few (u,v) away from the poles/seam.
    for (u, v) in [(0.5, 0.0), (1.0, 0.4), (2.5, -0.3)] {
        props.set_parameters(u, v);

        assert!(
            props.is_curvature_defined(),
            "curvature should be defined at ({u}, {v})"
        );

        // Gaussian curvature of a sphere is 1/R^2 regardless of orientation.
        let gaussian = props.gaussian_curvature();
        assert!(
            (gaussian - 1.0 / (R * R)).abs() < 1e-9,
            "gaussian curvature at ({u}, {v}) was {gaussian}, expected {}",
            1.0 / (R * R)
        );

        // Principal curvatures both have magnitude 1/R (sign follows the normal).
        for k in [props.max_curvature(), props.min_curvature()] {
            assert!(
                (k.abs() - 1.0 / R).abs() < 1e-9,
                "principal curvature {k} should have magnitude {}",
                1.0 / R
            );
        }

        // |mean| = 1/R on a sphere.
        let mean = props.mean_curvature().abs();
        assert!(
            (mean - 1.0 / R).abs() < 1e-9,
            "mean curvature magnitude at ({u}, {v}) was {mean}, expected {}",
            1.0 / R
        );
    }
}

/// On a sphere centred at the origin the unit normal at a point is parallel to
/// the position vector, which also confirms Value() and Normal() agree.
#[test]
fn sphere_normal_is_radial() {
    const R: f64 = 3.0;
    let mut sphere = MakeSphere::new_real(R);
    let shape = sphere.as_b_rep_builder_api_make_shape_mut().shape();
    let adaptor = first_face_adaptor(shape);
    let mut props = SLProps::new_surface_int_real(&adaptor, DEGREE, RESOLUTION);
    props.set_parameters(1.0, 0.3);

    assert!(props.is_normal_defined(), "normal should be defined");

    let p = props.value();
    let (px, py, pz) = (p.x(), p.y(), p.z());
    let len = (px * px + py * py + pz * pz).sqrt();
    assert!(
        (len - R).abs() < 1e-9,
        "point should lie on the sphere, got radius {len}"
    );

    let n = props.normal();
    // |dot(n, p/|p|)| == 1 when the normal is radial (sign depends on face sense).
    let dot = (n.x() * px + n.y() * py + n.z() * pz) / len;
    assert!(
        (dot.abs() - 1.0).abs() < 1e-9,
        "normal should be radial; |cos| was {}",
        dot.abs()
    );
}

/// A planar face is flat: both principal curvatures, and hence Gaussian and mean
/// curvature, are zero, and it is umbilic.
#[test]
fn planar_face_is_flat() {
    let mut boxy = MakeBox::new_real3(10.0, 10.0, 10.0);
    let shape = boxy.as_b_rep_builder_api_make_shape_mut().shape();
    let adaptor = first_face_adaptor(shape);
    let mut props = SLProps::new_surface_int_real(&adaptor, DEGREE, RESOLUTION);
    props.set_parameters(5.0, 5.0);

    assert!(props.is_curvature_defined(), "curvature defined on a plane");
    assert!(
        props.gaussian_curvature().abs() < 1e-12,
        "plane should have zero gaussian curvature, got {}",
        props.gaussian_curvature()
    );
    assert!(
        props.mean_curvature().abs() < 1e-12,
        "plane should have zero mean curvature, got {}",
        props.mean_curvature()
    );
    // Every direction is principal on a plane, so OCCT reports it as umbilic.
    assert!(props.is_umbilic(), "a plane is umbilic");
}
