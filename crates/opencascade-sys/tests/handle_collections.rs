//! Tests for collections whose element type is an OCCT handle.
//!
//! Collection discovery previously rejected `NCollection_List<occ::handle<T>>` and
//! friends, so every method mentioning one was skipped. These tests exercise a
//! handle-element collection end to end — store handles, read them back, and
//! confirm the round-tripped handle still refers to a live object — rather than
//! only checking that the bindings compile and link.

use opencascade_sys::geom::{Handle_Geom_Curve, Line};
use opencascade_sys::gp;
use opencascade_sys::n_collection::Sequence_Handle_Geom_Curve;
use opencascade_sys::OwnedPtr;

/// A `Handle(Geom_Curve)` for the line through (0, y, 0) along +X.
fn line_at_y(y: f64) -> OwnedPtr<Handle_Geom_Curve> {
    let p = gp::Pnt::new_real3(0.0, y, 0.0);
    let d = gp::Dir::new_real3(1.0, 0.0, 0.0);
    Line::to_handle(Line::new_pnt_dir(&p, &d)).to_handle_curve()
}

#[test]
fn sequence_of_handles_round_trips() {
    let mut seq = Sequence_Handle_Geom_Curve::new();
    assert_eq!(seq.size(), 0);

    seq.append(&line_at_y(0.0));
    seq.append(&line_at_y(7.0));
    assert_eq!(seq.size(), 2, "both handles should be stored");

    // A read-back handle must yield a live curve, not a null or dangling one:
    // evaluate it and check the point we expect on that line.
    let second = seq.value(2);
    assert!(!second.is_null(), "stored handle should not be null");
    let p = second.get().value(0.0);
    assert!(
        (p.y() - 7.0).abs() < 1e-12,
        "round-tripped handle should be the second line (y=7), got y={}",
        p.y()
    );
}

#[test]
fn sequence_of_handles_owns_its_elements() {
    let mut seq = Sequence_Handle_Geom_Curve::new();
    // Each handle is dropped right after being appended, so the collection must
    // hold its own reference for the curves to survive.
    for y in [1.0, 2.0, 3.0] {
        let h = line_at_y(y);
        seq.append(&h);
    }
    assert_eq!(seq.size(), 3);

    let mut ys: Vec<f64> = seq
        .iter()
        .map(|h| {
            assert!(!h.is_null(), "iterated handle should not be null");
            h.get().value(0.0).y()
        })
        .collect();
    ys.sort_by(|a, b| a.partial_cmp(b).unwrap());

    assert_eq!(ys.len(), 3, "iterator should yield every element");
    for (got, want) in ys.iter().zip([1.0, 2.0, 3.0]) {
        assert!(
            (got - want).abs() < 1e-12,
            "expected a line at y={want}, got {got}"
        );
    }

    seq.clear();
    assert_eq!(seq.size(), 0);
}
