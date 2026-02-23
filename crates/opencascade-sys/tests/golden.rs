use std::path::Path;
use std::process::Command;

const ABS_EPSILON: f64 = 1e-6;
const REL_EPSILON: f64 = 1e-9;
const MESH_REL_EPSILON: f64 = 1e-3;

fn is_close(a: f64, b: f64) -> bool {
    let diff = (a - b).abs();
    let scale = a.abs().max(b.abs());
    diff <= ABS_EPSILON.max(REL_EPSILON * scale)
}

#[derive(Clone, Copy)]
struct Triangle {
    a: [f64; 3],
    b: [f64; 3],
    c: [f64; 3],
}

fn subtract(left: [f64; 3], right: [f64; 3]) -> [f64; 3] {
    [left[0] - right[0], left[1] - right[1], left[2] - right[2]]
}

fn cross(left: [f64; 3], right: [f64; 3]) -> [f64; 3] {
    [
        left[1] * right[2] - left[2] * right[1],
        left[2] * right[0] - left[0] * right[2],
        left[0] * right[1] - left[1] * right[0],
    ]
}

fn dot(left: [f64; 3], right: [f64; 3]) -> f64 {
    left[0] * right[0] + left[1] * right[1] + left[2] * right[2]
}

fn norm(vector: [f64; 3]) -> f64 {
    dot(vector, vector).sqrt()
}

fn parse_vertex(line: &str) -> Option<[f64; 3]> {
    let mut parts = line.split_whitespace();
    if parts.next()? != "vertex" {
        return None;
    }

    let x = parts.next()?.parse::<f64>().ok()?;
    let y = parts.next()?.parse::<f64>().ok()?;
    let z = parts.next()?.parse::<f64>().ok()?;
    Some([x, y, z])
}

fn parse_stl_triangles(content: &[u8]) -> Vec<Triangle> {
    let text = String::from_utf8_lossy(content);
    let mut triangles = Vec::new();
    let mut current_vertices: Vec<[f64; 3]> = Vec::new();

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.starts_with("vertex") {
            if let Some(vertex) = parse_vertex(line) {
                current_vertices.push(vertex);
            }
        } else if line == "endfacet" {
            if current_vertices.len() >= 3 {
                let vertex_count = current_vertices.len();
                triangles.push(Triangle {
                    a: current_vertices[vertex_count - 3],
                    b: current_vertices[vertex_count - 2],
                    c: current_vertices[vertex_count - 1],
                });
            }
            current_vertices.clear();
        }
    }

    triangles
}

#[derive(Clone, Copy)]
struct MeshMetrics {
    triangle_count: usize,
    bbox_min: [f64; 3],
    bbox_max: [f64; 3],
    total_area: f64,
    signed_volume: f64,
}

fn mesh_metrics(triangles: &[Triangle]) -> MeshMetrics {
    let mut bbox_min = [f64::INFINITY; 3];
    let mut bbox_max = [f64::NEG_INFINITY; 3];
    let mut total_area = 0.0;
    let mut signed_volume = 0.0;

    for triangle in triangles {
        for vertex in [triangle.a, triangle.b, triangle.c] {
            for axis in 0..3 {
                bbox_min[axis] = bbox_min[axis].min(vertex[axis]);
                bbox_max[axis] = bbox_max[axis].max(vertex[axis]);
            }
        }

        let edge_ab = subtract(triangle.b, triangle.a);
        let edge_ac = subtract(triangle.c, triangle.a);
        let cross_product = cross(edge_ab, edge_ac);
        total_area += 0.5 * norm(cross_product);
        signed_volume += dot(triangle.a, cross(triangle.b, triangle.c)) / 6.0;
    }

    MeshMetrics {
        triangle_count: triangles.len(),
        bbox_min,
        bbox_max,
        total_area,
        signed_volume,
    }
}

fn assert_mesh_metrics_close(expected: MeshMetrics, actual: MeshMetrics, context: &str) {
    let triangle_delta = expected.triangle_count.abs_diff(actual.triangle_count);
    let triangle_rel = triangle_delta as f64 / expected.triangle_count.max(1) as f64;
    assert!(
        triangle_rel <= MESH_REL_EPSILON,
        "{context}: triangle count differs too much: expected {}, actual {}, rel diff {}",
        expected.triangle_count,
        actual.triangle_count,
        triangle_rel
    );

    for axis in 0..3 {
        assert!(
            is_close(expected.bbox_min[axis], actual.bbox_min[axis]),
            "{context}: bbox min mismatch on axis {axis}: expected {}, actual {}",
            expected.bbox_min[axis],
            actual.bbox_min[axis]
        );
        assert!(
            is_close(expected.bbox_max[axis], actual.bbox_max[axis]),
            "{context}: bbox max mismatch on axis {axis}: expected {}, actual {}",
            expected.bbox_max[axis],
            actual.bbox_max[axis]
        );
    }

    let area_diff = (expected.total_area - actual.total_area).abs();
    let area_scale = expected.total_area.abs().max(actual.total_area.abs()).max(1.0);
    assert!(
        area_diff <= MESH_REL_EPSILON * area_scale,
        "{context}: total area differs too much: expected {}, actual {}, diff {}",
        expected.total_area,
        actual.total_area,
        area_diff
    );

    let expected_volume = expected.signed_volume.abs();
    let actual_volume = actual.signed_volume.abs();
    let volume_diff = (expected_volume - actual_volume).abs();
    let volume_scale = expected_volume.max(actual_volume).max(1.0);
    assert!(
        volume_diff <= MESH_REL_EPSILON * volume_scale,
        "{context}: volume differs too much: expected {}, actual {}, diff {}",
        expected_volume,
        actual_volume,
        volume_diff
    );
}

fn golden_dir() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/golden"))
}

/// Normalize an ASCII STL file for comparison by sorting facet blocks.
/// The triangulation order is not deterministic, but the set of triangles
/// should be identical.
fn normalize_stl(content: &[u8]) -> Vec<u8> {
    let text = String::from_utf8_lossy(content);
    let lines: Vec<&str> = text.lines().map(str::trim).filter(|line| !line.is_empty()).collect();

    // First and last lines are "solid ..." and "endsolid ..."
    if lines.len() < 2 {
        return content.to_vec();
    }

    let header = lines[0];
    let footer = lines[lines.len() - 1];

    let facet_lines = &lines[1..lines.len() - 1];
    let mut facets: Vec<String> = Vec::new();
    let mut current_facet: Vec<&str> = Vec::new();
    let mut in_facet = false;

    for line in facet_lines {
        if line.starts_with("facet ") {
            in_facet = true;
            current_facet.clear();
            current_facet.push(line);
            continue;
        }

        if in_facet {
            current_facet.push(line);
            if *line == "endfacet" {
                facets.push(current_facet.join("\n"));
                current_facet.clear();
                in_facet = false;
            }
        }
    }

    if facets.is_empty() {
        return content.to_vec();
    }

    facets.sort();

    let mut result = String::new();
    result.push_str(header);
    result.push('\n');
    for facet in &facets {
        result.push_str(facet);
        result.push('\n');
    }
    result.push_str(footer);
    result.push('\n');
    result.into_bytes()
}

#[test]
fn bottle_stl_matches_golden() {
    // Build the bottle example first
    let build_status = Command::new("cargo")
        .args(["build", "--example", "bottle", "--manifest-path"])
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .status()
        .expect("failed to build bottle example");
    assert!(build_status.success(), "Failed to build bottle example");

    // Run it in a temp dir so bottle.stl is written there
    let tmpdir = tempfile::tempdir().expect("failed to create temp dir");

    let run_status = Command::new("cargo")
        .args(["run", "--example", "bottle", "--manifest-path"])
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/Cargo.toml"))
        .current_dir(tmpdir.path())
        .status()
        .expect("failed to run bottle example");
    assert!(run_status.success(), "Failed to run bottle example");

    let output_path = tmpdir.path().join("bottle.stl");
    assert!(output_path.exists(), "bottle.stl was not created");

    let golden_path = golden_dir().join("bottle.stl");
    assert!(
        golden_path.exists(),
        "Golden file not found: {}",
        golden_path.display()
    );

    let golden = std::fs::read(&golden_path)
        .unwrap_or_else(|e| panic!("Failed to read golden file: {e}"));
    let actual = std::fs::read(&output_path)
        .unwrap_or_else(|e| panic!("Failed to read output file: {e}"));

    let golden_normalized = normalize_stl(&golden);
    let actual_normalized = normalize_stl(&actual);

    let golden_triangles = parse_stl_triangles(&golden_normalized);
    let actual_triangles = parse_stl_triangles(&actual_normalized);
    assert!(
        !golden_triangles.is_empty() && !actual_triangles.is_empty(),
        "Failed to parse STL triangle data"
    );

    let golden_metrics = mesh_metrics(&golden_triangles);
    let actual_metrics = mesh_metrics(&actual_triangles);
    assert_mesh_metrics_close(
        golden_metrics,
        actual_metrics,
        "bottle.stl output does not match golden mesh metrics",
    );
}
