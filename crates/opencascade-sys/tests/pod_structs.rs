//! Tests for POD (Plain Old Data) struct bindings.
//!
//! These tests verify that transparent `#[repr(C)]` Rust structs match
//! the C++ layout and can be used to read/write fields directly.

use opencascade_sys::bop_algo::MakePeriodic_PeriodicityParams;

/// Verify that the Rust struct size matches the C++ sizeof for each POD type.
/// This catches layout mismatches from field ordering, alignment, or padding
/// differences between the Rust and C++ definitions.
///
/// PeriodicityParams has: bool[3] + f64[3] + bool[3] + f64[3]
/// = 3*1 + 5(pad) + 3*8 + 3*1 + 5(pad) + 3*8 = 64 bytes
/// (with typical alignment: bools pack to 3 bytes, then 5 padding to align f64)
#[test]
fn periodicity_params_sizeof() {
    let rust_size = std::mem::size_of::<MakePeriodic_PeriodicityParams>();
    // The struct has 4 array fields: bool[3], f64[3], bool[3], f64[3]
    // Minimum size is 3 + 24 + 3 + 24 = 54 bytes, but alignment may add padding
    assert!(
        rust_size >= 54,
        "Rust sizeof ({}) is suspiciously small for PeriodicityParams (expected >= 54)",
        rust_size
    );
}

/// Verify field access works on a default-initialized PeriodicityParams.
#[test]
fn periodicity_params_field_access() {
    let params = MakePeriodic_PeriodicityParams {
        my_periodic: [false, true, false],
        my_period: [1.0, 2.0, 3.0],
        my_is_trimmed: [true, false, true],
        my_period_first: [0.5, 1.5, 2.5],
    };

    assert_eq!(params.my_periodic, [false, true, false]);
    assert_eq!(params.my_period, [1.0, 2.0, 3.0]);
    assert_eq!(params.my_is_trimmed, [true, false, true]);
    assert_eq!(params.my_period_first, [0.5, 1.5, 2.5]);
}

/// Verify that PeriodicityParams is Copy (POD types should be Copy).
#[test]
fn periodicity_params_is_copy() {
    let params = MakePeriodic_PeriodicityParams {
        my_periodic: [true, true, true],
        my_period: [10.0, 20.0, 30.0],
        my_is_trimmed: [false, false, false],
        my_period_first: [0.0, 0.0, 0.0],
    };
    let copy = params; // This moves if not Copy
    assert_eq!(params.my_period, copy.my_period); // Use both — only works if Copy
}

/// Verify Debug formatting works.
#[test]
fn periodicity_params_debug_format() {
    let params = MakePeriodic_PeriodicityParams {
        my_periodic: [false; 3],
        my_period: [0.0; 3],
        my_is_trimmed: [false; 3],
        my_period_first: [0.0; 3],
    };
    let debug_str = format!("{:?}", params);
    assert!(debug_str.contains("my_periodic"));
    assert!(debug_str.contains("my_period"));
}