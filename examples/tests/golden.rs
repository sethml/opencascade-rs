use std::path::{Path, PathBuf};
use std::process::Command;

fn golden_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("golden")
}

/// Filter out the FILE_NAME line (and its continuation lines) from a STEP file,
/// since it contains a timestamp that changes on every run.
fn normalize_step(content: &str) -> String {
    let mut result = String::new();
    let mut in_file_name = false;
    for line in content.lines() {
        if line.starts_with("FILE_NAME(") {
            in_file_name = true;
            continue;
        }
        if in_file_name {
            if line.contains(");") {
                in_file_name = false;
            }
            continue;
        }
        result.push_str(line);
        result.push('\n');
    }
    result
}

fn check_example(name: &str) {
    let golden_path = golden_dir().join(format!("{name}.step"));
    assert!(
        golden_path.exists(),
        "Golden file not found: {}",
        golden_path.display()
    );

    let tmpdir = tempfile::tempdir().expect("failed to create temp dir");
    let output_path = tmpdir.path().join(format!("{name}.step"));

    // Run the write_model binary in a subprocess so OCCT global state is fresh
    let status = Command::new(env!("CARGO_BIN_EXE_write_model"))
        .arg(name)
        .arg("-o")
        .arg(&output_path)
        .status()
        .unwrap_or_else(|e| panic!("Failed to run write_model for {name}: {e}"));
    assert!(status.success(), "write_model failed for example '{name}'");

    let golden = std::fs::read_to_string(&golden_path)
        .unwrap_or_else(|e| panic!("Failed to read golden file {}: {e}", golden_path.display()));
    let actual = std::fs::read_to_string(&output_path)
        .unwrap_or_else(|e| panic!("Failed to read output file {}: {e}", output_path.display()));

    let golden_normalized = normalize_step(&golden);
    let actual_normalized = normalize_step(&actual);

    assert_eq!(
        golden_normalized, actual_normalized,
        "Output for example '{name}' does not match golden file.\n\
         Golden: {}\n\
         Actual: {}",
        golden_path.display(),
        output_path.display()
    );
}

// Generate a test for each example
macro_rules! golden_test {
    ($test_name:ident, $example_name:expr) => {
        #[test]
        fn $test_name() {
            check_example($example_name);
        }
    };
    ($test_name:ident, $example_name:expr, ignore = $reason:expr) => {
        #[test]
        #[ignore = $reason]
        fn $test_name() {
            check_example($example_name);
        }
    };
}

golden_test!(airfoil, "airfoil");
golden_test!(bounding_box, "bounding-box");
golden_test!(box_shape, "box-shape");
golden_test!(cable_bracket, "cable-bracket");
golden_test!(chamfer, "chamfer");
golden_test!(flat_ethernet_bracket, "flat-ethernet-bracket");
golden_test!(gizmo, "gizmo");
golden_test!(heater_coil, "heater-coil");
// high-level-bottle uses fillet() on all edges, then union() and hollow().
// These operations iterate over edges/faces using OCCT's internal shape maps
// (TopTools_IndexedMapOfShape), which are keyed by memory addresses of
// TopoDS_Shape handles. Due to ASLR and allocator non-determinism, the
// iteration order varies between runs, producing topologically equivalent but
// structurally reordered B-Rep results — and thus different STEP entity
// numbering on each run.
golden_test!(high_level_bottle, "high-level-bottle", ignore = "non-deterministic STEP output");
golden_test!(keyboard_case, "keyboard-case");
golden_test!(keycap, "keycap");
golden_test!(letter_a, "letter-a");
golden_test!(offset2d, "offset2d");
golden_test!(pentafoil, "pentafoil");
golden_test!(rounded_chamfer, "rounded-chamfer");
golden_test!(section, "section");
golden_test!(swept_face, "swept-face");
golden_test!(swept_face_variable, "swept-face-variable");
golden_test!(swept_wire, "swept-wire");
golden_test!(swept_wire_variable, "swept-wire-variable");
golden_test!(turners_cube, "turners-cube");
golden_test!(variable_fillet, "variable-fillet");
golden_test!(zbox_case, "zbox-case");
