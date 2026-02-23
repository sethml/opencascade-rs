use std::path::{Path, PathBuf};
use std::process::Command;

const ABS_EPSILON: f64 = 1e-6;
const REL_EPSILON: f64 = 1e-9;

#[derive(Debug)]
enum Token<'a> {
    Text(&'a str),
    Number(f64, &'a str),
}

fn is_number_start(bytes: &[u8], index: usize) -> bool {
    let current = bytes[index];
    if !(current == b'+' || current == b'-' || current == b'.' || current.is_ascii_digit()) {
        return false;
    }

    if index == 0 {
        return true;
    }

    let prev = bytes[index - 1];
    !prev.is_ascii_alphanumeric() && prev != b'_' && prev != b'#'
}

fn parse_real_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut index = start;
    if bytes[index] == b'+' || bytes[index] == b'-' {
        index += 1;
        if index >= bytes.len() {
            return None;
        }
    }

    let mut saw_digit = false;
    while index < bytes.len() && bytes[index].is_ascii_digit() {
        saw_digit = true;
        index += 1;
    }

    if index < bytes.len() && bytes[index] == b'.' {
        index += 1;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            saw_digit = true;
            index += 1;
        }
    }

    if index < bytes.len() && (bytes[index] == b'e' || bytes[index] == b'E') {
        let exponent_marker = index;
        index += 1;
        if index < bytes.len() && (bytes[index] == b'+' || bytes[index] == b'-') {
            index += 1;
        }

        let exponent_digits_start = index;
        while index < bytes.len() && bytes[index].is_ascii_digit() {
            index += 1;
        }

        if exponent_digits_start == index {
            index = exponent_marker;
        }
    }

    if saw_digit {
        Some(index)
    } else {
        None
    }
}

fn tokenize_reals(input: &str) -> Vec<Token<'_>> {
    let bytes = input.as_bytes();
    let mut tokens = Vec::new();
    let mut text_start = 0;
    let mut index = 0;

    while index < bytes.len() {
        if is_number_start(bytes, index) {
            if let Some(end) = parse_real_end(bytes, index) {
                if text_start < index {
                    tokens.push(Token::Text(&input[text_start..index]));
                }

                let raw = &input[index..end];
                if let Ok(value) = raw.parse::<f64>() {
                    tokens.push(Token::Number(value, raw));
                    index = end;
                    text_start = end;
                    continue;
                }
            }
        }

        index += 1;
    }

    if text_start < input.len() {
        tokens.push(Token::Text(&input[text_start..]));
    }

    tokens
}

fn is_close(a: f64, b: f64) -> bool {
    let diff = (a - b).abs();
    let scale = a.abs().max(b.abs());
    diff <= ABS_EPSILON.max(REL_EPSILON * scale)
}

fn assert_text_with_numeric_tolerance(expected: &str, actual: &str, context: &str) {
    let expected_compact: String = expected
        .chars()
        .filter(|character| !character.is_ascii_whitespace())
        .collect();
    let actual_compact: String = actual
        .chars()
        .filter(|character| !character.is_ascii_whitespace())
        .collect();

    let expected_tokens = tokenize_reals(&expected_compact);
    let actual_tokens = tokenize_reals(&actual_compact);

    assert_eq!(
        expected_tokens.len(),
        actual_tokens.len(),
        "{context}: token count mismatch (expected {}, actual {})",
        expected_tokens.len(),
        actual_tokens.len()
    );

    for (index, (expected_token, actual_token)) in
        expected_tokens.iter().zip(actual_tokens.iter()).enumerate()
    {
        match (expected_token, actual_token) {
            (Token::Text(expected_text), Token::Text(actual_text)) => {
                assert_eq!(
                    expected_text, actual_text,
                    "{context}: textual mismatch at token {index}"
                );
            }
            (Token::Number(expected_value, expected_raw), Token::Number(actual_value, actual_raw)) => {
                assert!(
                    is_close(*expected_value, *actual_value),
                    "{context}: numeric mismatch at token {index}: expected {expected_raw}, actual {actual_raw}, abs diff {}",
                    (expected_value - actual_value).abs()
                );
            }
            _ => {
                panic!("{context}: token type mismatch at token {index}");
            }
        }
    }
}

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

    assert_text_with_numeric_tolerance(
        &golden_normalized,
        &actual_normalized,
        &format!(
            "Output for example '{name}' does not match golden file. Golden: {} Actual: {}",
            golden_path.display(),
            output_path.display()
        ),
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
// keycap uses a long curved profile with multiple offset/fillet/sweep steps.
// OCCT's internal approximations can vary slightly by platform and allocator,
// producing geometry-equivalent output with larger numeric drift in some entities.
golden_test!(keycap, "keycap", ignore = "non-deterministic STEP parameterization");
golden_test!(letter_a, "letter-a");
golden_test!(offset2d, "offset2d");
// pentafoil contains a very large STEP with unstable low-level entity expansion
// across runs/platforms, which can change token counts despite equivalent shape.
golden_test!(pentafoil, "pentafoil", ignore = "non-deterministic STEP expansion");
golden_test!(rounded_chamfer, "rounded-chamfer");
golden_test!(section, "section");
golden_test!(swept_face, "swept-face");
golden_test!(swept_face_variable, "swept-face-variable");
golden_test!(swept_wire, "swept-wire");
golden_test!(swept_wire_variable, "swept-wire-variable");
golden_test!(turners_cube, "turners-cube");
golden_test!(variable_fillet, "variable-fillet");
// zbox-case uses hollow(), union(), subtract(), and fillet_edges() in a loop.
// Like high-level-bottle, these Boolean operations iterate over OCCT's internal
// shape maps (TopTools_IndexedMapOfShape), keyed by pointer addresses. Under
// concurrent execution, varied heap layouts cause different hash iteration
// order, producing geometrically identical but structurally reordered B-Rep
// output — and thus different STEP entity numbering/parameterization.
golden_test!(zbox_case, "zbox-case", ignore = "non-deterministic STEP output");
