//! Tests for the NCollection map lookup methods (`find_index` / `contains`).
//!
//! The C++ wrappers for these have always been generated, but the Rust side
//! omitted the declarations, so they were unreachable. These tests exercise the
//! real OCCT containers to confirm the bindings link and return OCCT's semantics
//! (1-based indices, 0 for "not found").

use opencascade_sys::b_rep_prim_api::MakeBox;
use opencascade_sys::n_collection::{
    IndexedMap_TopoDS_Shape_TopTools_ShapeMapHasher, Map_TopoDS_Shape_TopTools_ShapeMapHasher,
};

/// `find_index` is the inverse of `find_key`: it maps a shape back to its
/// 1-based position, which is the whole reason to prefer an IndexedMap over a
/// linear scan.
#[test]
fn indexed_map_find_index_round_trips() {
    let mut box_a = MakeBox::new_real3(10.0, 10.0, 10.0);
    let shape_a = box_a.as_b_rep_builder_api_make_shape_mut().shape();
    let mut box_b = MakeBox::new_real3(20.0, 5.0, 5.0);
    let shape_b = box_b.as_b_rep_builder_api_make_shape_mut().shape();

    let mut map = IndexedMap_TopoDS_Shape_TopTools_ShapeMapHasher::new();
    let idx_a = map.add(shape_a);
    let idx_b = map.add(shape_b);

    assert_eq!(map.size(), 2, "both shapes should be stored");
    assert_eq!(idx_a, 1, "OCCT indices are 1-based");
    assert_eq!(idx_b, 2);

    // find_index recovers the index that add() returned.
    assert_eq!(map.find_index(shape_a), idx_a);
    assert_eq!(map.find_index(shape_b), idx_b);

    // ...and round-trips through find_key back to the same entry.
    let recovered = map.find_key(idx_b);
    assert_eq!(
        map.find_index(recovered),
        idx_b,
        "find_key(i) should map back to i"
    );
}

/// OCCT returns 0 (not an error) for a key that is absent.
#[test]
fn indexed_map_find_index_returns_zero_when_absent() {
    let mut box_a = MakeBox::new_real3(10.0, 10.0, 10.0);
    let shape_a = box_a.as_b_rep_builder_api_make_shape_mut().shape();
    let mut box_b = MakeBox::new_real3(20.0, 5.0, 5.0);
    let shape_b = box_b.as_b_rep_builder_api_make_shape_mut().shape();

    let mut map = IndexedMap_TopoDS_Shape_TopTools_ShapeMapHasher::new();
    map.add(shape_a);

    assert_eq!(
        map.find_index(shape_b),
        0,
        "absent key must yield 0, not a panic or a stale index"
    );
}

/// A plain Map has no index, so membership is the only lookup it offers.
#[test]
fn map_contains_reports_membership() {
    let mut box_a = MakeBox::new_real3(10.0, 10.0, 10.0);
    let shape_a = box_a.as_b_rep_builder_api_make_shape_mut().shape();
    let mut box_b = MakeBox::new_real3(20.0, 5.0, 5.0);
    let shape_b = box_b.as_b_rep_builder_api_make_shape_mut().shape();

    let mut map = Map_TopoDS_Shape_TopTools_ShapeMapHasher::new();
    map.add(shape_a);

    assert!(map.contains(shape_a), "added shape should be present");
    assert!(!map.contains(shape_b), "unadded shape should be absent");

    map.add(shape_b);
    assert!(map.contains(shape_b));
    assert_eq!(map.size(), 2);
}
