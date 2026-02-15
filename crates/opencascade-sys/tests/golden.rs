use std::path::Path;
use std::process::Command;

fn golden_dir() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/golden"))
}

/// Normalize an ASCII STL file for comparison by sorting facet blocks.
/// The triangulation order is not deterministic, but the set of triangles
/// should be identical.
fn normalize_stl(content: &[u8]) -> Vec<u8> {
    let text = String::from_utf8_lossy(content);
    let lines: Vec<&str> = text.lines().collect();

    // First and last lines are "solid ..." and "endsolid ..."
    if lines.len() < 2 {
        return content.to_vec();
    }

    let header = lines[0];
    let footer = lines[lines.len() - 1];

    // Each facet block is 7 lines: facet normal, outer loop, 3 vertices, endloop, endfacet
    let facet_lines = &lines[1..lines.len() - 1];
    let mut facets: Vec<String> = Vec::new();
    for chunk in facet_lines.chunks(7) {
        facets.push(chunk.join("\n"));
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

    assert_eq!(
        golden_normalized,
        actual_normalized,
        "bottle.stl output does not match golden file (after sorting facets)"
    );
}
