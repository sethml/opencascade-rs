use opencascade_sys::b_rep;
use opencascade_sys::b_rep_algo_api;
use opencascade_sys::b_rep_builder_api;
use opencascade_sys::b_rep_fillet_api;
use opencascade_sys::ch_fi3d;
use opencascade_sys::b_rep_lib;
use opencascade_sys::b_rep_mesh;
use opencascade_sys::b_rep_offset_api;
use opencascade_sys::b_rep_prim_api;
use opencascade_sys::gc;
use opencascade_sys::gce2d;
use opencascade_sys::geom;
use opencascade_sys::geom2d;
use opencascade_sys::gp;
use opencascade_sys::message;
use opencascade_sys::stl_api;
use opencascade_sys::top_abs;
use opencascade_sys::top_exp;
use opencascade_sys::top_tools;
use opencascade_sys::topo_ds;

// All dimensions are in millimeters.
pub fn main() {
    let height = 70.0_f64;
    let width = 50.0_f64;
    let thickness = 30.0_f64;

    // Profile : Define Support Points
    let point_1 = gp::Pnt::new_real3(-width / 2.0, 0.0, 0.0);
    let point_2 = gp::Pnt::new_real3(-width / 2.0, -thickness / 4.0, 0.0);
    let point_3 = gp::Pnt::new_real3(0.0, -thickness / 2.0, 0.0);
    let point_4 = gp::Pnt::new_real3(width / 2.0, -thickness / 4.0, 0.0);
    let point_5 = gp::Pnt::new_real3(width / 2.0, 0.0, 0.0);

    // Profile : Define the Geometry
    let arc_of_circle = gc::MakeArcOfCircle::new_pnt3(&point_2, &point_3, &point_4);
    let segment_1 = gc::MakeSegment::new_pnt2(&point_1, &point_2);
    let segment_2 = gc::MakeSegment::new_pnt2(&point_4, &point_5);

    // Profile : Define the Topology
    // GC_Make* types have a .value() CXX method returning &HandleGeomTrimmedCurve.
    // We upcast to HandleGeomCurve for BRepBuilderAPI_MakeEdge.
    let segment_1_curve = segment_1.value().to_handle_curve();
    let arc_curve = arc_of_circle.value().to_handle_curve();
    let segment_2_curve = segment_2.value().to_handle_curve();

    let mut edge_1 = b_rep_builder_api::MakeEdge::new_handlecurve(&segment_1_curve);
    let mut edge_2 = b_rep_builder_api::MakeEdge::new_handlecurve(&arc_curve);
    let mut edge_3 = b_rep_builder_api::MakeEdge::new_handlecurve(&segment_2_curve);

    let mut wire = b_rep_builder_api::MakeWire::new_edge3(
        edge_1.pin_mut().edge(),
        edge_2.pin_mut().edge(),
        edge_3.pin_mut().edge(),
    );

    // Complete Profile
    // TODO: Use gp::OX() once the generator supports standalone package functions
    // (see TRANSITION_PLAN.md #11)
    let x_axis = gp::Ax1::new_pnt_dir(
        &gp::Pnt::new(),
        &gp::Dir::new_real3(1.0, 0.0, 0.0),
    );

    let mut trsf = gp::Trsf::new();
    trsf.pin_mut().set_mirror_ax1(&x_axis);

    // TODO: Use shorter Transform::new_shape_trsf() once the generator emits
    // default-argument helper functions (see TRANSITION_PLAN.md #12)
    let mut brep_transform =
        b_rep_builder_api::Transform::new_shape_trsf_bool2(wire.pin_mut().shape(), &trsf, false, false);
    let mirrored_shape = brep_transform.pin_mut().shape();
    let mirrored_wire = topo_ds::wire(mirrored_shape);

    let mut make_wire = b_rep_builder_api::MakeWire::new();
    make_wire.pin_mut().add_wire(wire.pin_mut().wire());
    make_wire.pin_mut().add_wire(mirrored_wire);

    let wire_profile = make_wire.pin_mut().wire();

    // Body : Prism the Profile
    let face_profile = b_rep_builder_api::MakeFace::new_wire_bool(wire_profile, false);
    let prism_vec = gp::Vec::new_real3(0.0, 0.0, height);
    let mut body =
        b_rep_prim_api::MakePrism::new_shape_vec_bool2(face_profile.face().as_shape(), &prism_vec, false, true);

    // Body : Apply Fillets
    let mut make_fillet =
        b_rep_fillet_api::MakeFillet::new_shape_filletshape(
            body.pin_mut().shape(),
            ch_fi3d::FilletShape::Rational as i32,
        );
    let mut edge_explorer = top_exp::Explorer::new_shape_shapeenum2(
        body.pin_mut().shape(),
        top_abs::ShapeEnum::Edge as i32,
        top_abs::ShapeEnum::Shape as i32,
    );

    while edge_explorer.more() {
        let edge = topo_ds::edge(edge_explorer.current());
        make_fillet.pin_mut().add_real_edge(thickness / 12.0, edge);
        edge_explorer.pin_mut().next();
    }

    let body_shape = make_fillet.pin_mut().shape();

    // Body : Add the Neck
    let neck_location = gp::Pnt::new_real3(0.0, 0.0, height);
    let neck_axis = gp::Dir::new_real3(0.0, 0.0, 1.0); // gp::DZ()
    let neck_ax2 = gp::Ax2::new_pnt_dir(&neck_location, &neck_axis);

    let neck_radius = thickness / 4.0;
    let neck_height = height / 10.0;

    let mut cylinder =
        b_rep_prim_api::MakeCylinder::new_ax2_real2(&neck_ax2, neck_radius, neck_height);
    let cylinder_shape = cylinder.pin_mut().shape();

    let progress = message::ProgressRange::new();
    let mut fuse = b_rep_algo_api::Fuse::new_shape2_progressrange(body_shape, cylinder_shape, &progress);
    let body_shape = fuse.pin_mut().shape();

    // Body : Create a Hollowed Solid
    let mut face_explorer = top_exp::Explorer::new_shape_shapeenum2(
        body_shape,
        top_abs::ShapeEnum::Face as i32,
        top_abs::ShapeEnum::Shape as i32,
    );
    let mut z_max = -1.0_f64;
    let mut top_face: Option<cxx::UniquePtr<topo_ds::Face>> = None;

    while face_explorer.more() {
        let current = face_explorer.current();
        let face = topo_ds::face(current);
        let surface = b_rep::Tool::surface_face(face);

        // Check if this face is a Geom_Plane
        let surface_ref = surface.get();
        let dynamic_type = surface_ref.dynamic_type();
        let type_obj = dynamic_type.get();
        let name = type_obj.name();

        if name == "Geom_Plane" {
            // TODO: The binding generator should produce Handle downcasts
            // (e.g., HandleGeomSurface → HandleGeomPlane via Handle::DownCast).
            // Until then, we use an unsafe pointer cast after confirming the
            // dynamic type. This is safe because OCCT's DynamicType() confirms
            // the concrete type is Geom_Plane, and Geom_Plane inherits from
            // Geom_Surface with the same object layout.
            let plane: &geom::Plane =
                unsafe { &*(surface_ref as *const geom::Surface as *const geom::Plane) };
            let plane_location = plane.location();
            let plane_z = plane_location.z();
            if plane_z > z_max {
                z_max = plane_z;
                top_face = Some(face.to_owned());
            }
        }

        face_explorer.pin_mut().next();
    }

    let top_face = top_face.unwrap();

    let mut faces_to_remove = top_tools::ListOfShape::new();
    faces_to_remove.pin_mut().append(top_face.as_shape());

    let mut solid_maker = b_rep_offset_api::MakeThickSolid::new();
    let progress = message::ProgressRange::new();
    solid_maker.pin_mut().make_thick_solid_by_join(
        body_shape,
        &faces_to_remove,
        -thickness / 50.0,
        1.0e-3,
        0,     // BRepOffset_Skin
        false, // Intersection
        false, // SelfInter
        0,     // GeomAbs_Arc
        false, // RemoveIntEdges
        &progress,
    );

    let body_shape = solid_maker.pin_mut().shape();

    // Threading : Create Surfaces
    let neck_ax3 = gp::Ax3::new_ax2(&neck_ax2);
    let cylinder_surface_1 = geom::CylindricalSurface::new_ax3_real(&neck_ax3, neck_radius * 0.99);
    let handle_cyl_1 = geom::CylindricalSurface::to_handle(cylinder_surface_1);
    let handle_surface_1 = handle_cyl_1.to_handle_surface();
    let cylinder_surface_2 = geom::CylindricalSurface::new_ax3_real(&neck_ax3, neck_radius * 1.05);
    let handle_cyl_2 = geom::CylindricalSurface::to_handle(cylinder_surface_2);
    let handle_surface_2 = handle_cyl_2.to_handle_surface();

    // Threading : Define 2D Curves
    let a_pnt = gp::Pnt2d::new_real2(std::f64::consts::TAU, neck_height / 2.0);
    let a_dir = gp::Dir2d::new_real2(std::f64::consts::TAU, neck_height / 4.0);
    let an_ax2d = gp::Ax2d::new_pnt2d_dir2d(&a_pnt, &a_dir);

    let a_major = std::f64::consts::TAU;
    let a_minor = neck_height / 10.0;

    let ellipse_1 = geom2d::Ellipse::new_ax2d_real2_bool(&an_ax2d, a_major, a_minor, true);
    let handle_ellipse_1 = geom2d::Ellipse::to_handle(ellipse_1);
    let handle_curve_1 = handle_ellipse_1.to_handle_curve();
    let ellipse_2 = geom2d::Ellipse::new_ax2d_real2_bool(&an_ax2d, a_major, a_minor / 4.0, true);
    let handle_ellipse_2 = geom2d::Ellipse::to_handle(ellipse_2);
    let handle_curve_2 = handle_ellipse_2.to_handle_curve();

    let arc_1 = geom2d::TrimmedCurve::new_handlecurve_real2_bool2(
        &handle_curve_1, 0.0, std::f64::consts::PI, true, true,
    );
    let handle_arc_1 = geom2d::TrimmedCurve::to_handle(arc_1);
    let arc_1_handle = handle_arc_1.to_handle_curve();
    let arc_2 = geom2d::TrimmedCurve::new_handlecurve_real2_bool2(
        &handle_curve_2, 0.0, std::f64::consts::PI, true, true,
    );
    let handle_arc_2 = geom2d::TrimmedCurve::to_handle(arc_2);
    let arc_2_handle = handle_arc_2.to_handle_curve();

    // Get ellipse endpoints via handle dereference (ellipse consumed by to_handle above)
    let ellipse_pnt_1 = handle_ellipse_1.get().value(0.0);
    let ellipse_pnt_2 = handle_ellipse_1.get().value(std::f64::consts::PI);

    let segment = gce2d::MakeSegment::new_pnt2d2(&ellipse_pnt_1, &ellipse_pnt_2);
    let segment_handle = segment.value().to_handle_curve();

    // Threading : Build Edges and Wires
    let mut edge_1_on_surf_1 =
        b_rep_builder_api::MakeEdge::new_handlecurve_handlesurface(&arc_1_handle, &handle_surface_1);
    let mut edge_2_on_surf_1 =
        b_rep_builder_api::MakeEdge::new_handlecurve_handlesurface(&segment_handle, &handle_surface_1);
    let mut edge_1_on_surf_2 =
        b_rep_builder_api::MakeEdge::new_handlecurve_handlesurface(&arc_2_handle, &handle_surface_2);
    let mut edge_2_on_surf_2 =
        b_rep_builder_api::MakeEdge::new_handlecurve_handlesurface(&segment_handle, &handle_surface_2);

    let mut threading_wire_1 = b_rep_builder_api::MakeWire::new_edge2(
        edge_1_on_surf_1.pin_mut().edge(),
        edge_2_on_surf_1.pin_mut().edge(),
    );
    let mut threading_wire_2 = b_rep_builder_api::MakeWire::new_edge2(
        edge_1_on_surf_2.pin_mut().edge(),
        edge_2_on_surf_2.pin_mut().edge(),
    );

    b_rep_lib::BRepLib::build_curves3d_shape(threading_wire_1.pin_mut().shape());
    b_rep_lib::BRepLib::build_curves3d_shape(threading_wire_2.pin_mut().shape());

    // Create Threading
    let mut threading_loft = b_rep_offset_api::ThruSections::new_bool2_real(true, false, 1.0e-06);
    threading_loft
        .pin_mut()
        .add_wire(threading_wire_1.pin_mut().wire());
    threading_loft
        .pin_mut()
        .add_wire(threading_wire_2.pin_mut().wire());
    threading_loft.pin_mut().check_compatibility(false);

    let threading_shape = threading_loft.pin_mut().shape();

    // Building the Resulting Compound
    let mut compound = topo_ds::Compound::new();
    let builder = b_rep::Builder::new();
    builder.make_compound(compound.pin_mut());

    builder.add(compound.pin_mut().as_shape_mut(), body_shape);
    builder.add(compound.pin_mut().as_shape_mut(), threading_shape);

    // Export to an STL file
    let progress = message::ProgressRange::new();
    let _mesh = b_rep_mesh::IncrementalMesh::new_shape_real_bool_real_bool(
        compound.as_shape(),
        0.01,
        false,
        0.5,
        false,
    );
    let mut stl_writer = stl_api::Writer::new();
    let success = stl_writer
        .pin_mut()
        .write(compound.as_shape(), "bottle.stl", &progress);

    println!("Done! Success = {success}");
}
