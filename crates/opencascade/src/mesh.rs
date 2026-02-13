use crate::{
    primitives::{FaceOrientation, Shape},
    Error,
};
use cxx::UniquePtr;
use glam::{dvec2, dvec3, DVec2, DVec3};
use opencascade_sys::{b_rep, b_rep_mesh, poly, top_loc};

#[derive(Debug)]
pub struct Mesh {
    pub vertices: Vec<DVec3>,
    pub uvs: Vec<DVec2>,
    pub normals: Vec<DVec3>,
    pub indices: Vec<usize>,
}

pub struct Mesher {
    pub(crate) inner: UniquePtr<b_rep_mesh::IncrementalMesh>,
}

impl Mesher {
    pub fn try_new(shape: &Shape, triangulation_tolerance: f64) -> Result<Self, Error> {
        let inner = b_rep_mesh::IncrementalMesh::new_shape_real_bool_real_bool(
            &shape.inner,
            triangulation_tolerance,
            false, // isRelative
            0.5,   // theAngDeflection (default)
            false, // isInParallel
        );
        if inner.is_done() {
            Ok(Self { inner })
        } else {
            Err(Error::TriangulationFailed)
        }
    }

    pub fn mesh(self) -> Result<Mesh, Error> {
        let mut vertices = vec![];
        let mut uvs = vec![];
        let mut normals = vec![];
        let mut indices = vec![];

        let triangulated_shape = Shape::from_shape(self.inner.shape());

        for face in triangulated_shape.faces() {
            let mut location = top_loc::Location::new();

            let triangulation_handle =
                b_rep::Tool::triangulation(&face.inner, location.pin_mut(), 0);

            let mut triangulation =
                poly::Triangulation::new_handletriangulation(triangulation_handle);

            if triangulation.is_null() {
                return Err(Error::UntriangulatedFace);
            }

            let trsf = location.transformation();
            let index_offset = vertices.len();
            let face_point_count = triangulation.nb_nodes();

            for i in 1..=face_point_count {
                let point = triangulation.node(i);
                let transformed = point.transformed(trsf);
                vertices.push(dvec3(transformed.x(), transformed.y(), transformed.z()));
            }

            let mut u_min = f64::INFINITY;
            let mut v_min = f64::INFINITY;
            let mut u_max = f64::NEG_INFINITY;
            let mut v_max = f64::NEG_INFINITY;

            for i in 1..=face_point_count {
                let uv = triangulation.uv_node(i);
                let (u, v) = (uv.x(), uv.y());

                u_min = u_min.min(u);
                v_min = v_min.min(v);
                u_max = u_max.max(u);
                v_max = v_max.max(v);

                uvs.push(dvec2(u, v));
            }

            // Normalize the newly added UV coordinates.
            for uv in &mut uvs[index_offset..(index_offset + face_point_count as usize)] {
                uv.x = (uv.x - u_min) / (u_max - u_min);
                uv.y = (uv.y - v_min) / (v_max - v_min);

                if face.orientation() != FaceOrientation::Forward {
                    uv.x = 1.0 - uv.x;
                }
            }

            // Compute normals on the triangulation.
            triangulation.pin_mut().compute_normals();

            for i in 1..=face_point_count {
                let normal = triangulation.normal(i);
                normals.push(dvec3(normal.x(), normal.y(), normal.z()));
            }

            for i in 1..=triangulation.nb_triangles() {
                let triangle = triangulation.triangle(i);

                if face.orientation() == FaceOrientation::Forward {
                    indices.push(index_offset + triangle.value(1) as usize - 1);
                    indices.push(index_offset + triangle.value(2) as usize - 1);
                    indices.push(index_offset + triangle.value(3) as usize - 1);
                } else {
                    indices.push(index_offset + triangle.value(3) as usize - 1);
                    indices.push(index_offset + triangle.value(2) as usize - 1);
                    indices.push(index_offset + triangle.value(1) as usize - 1);
                }
            }
        }

        Ok(Mesh { vertices, uvs, normals, indices })
    }
}
