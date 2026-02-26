//! Tests for C++ exception handling across the FFI boundary.
//!
//! These tests verify that OCCT C++ exceptions are caught by the generated
//! wrappers and converted into Rust panics with meaningful messages.

/// Calling `TopoDS::Face()` on a non-Face shape throws `Standard_TypeMismatch`
/// in C++. Verify that this is caught and turned into a Rust panic.
#[test]
#[should_panic(expected = "OCCT C++ exception: Standard_TypeMismatch: TopoDS::Face")]
fn face_shape_on_compound_panics_with_type_mismatch() {
    // Create a box — its .shape() returns a Compound (not a Face)
    let mut make_box = opencascade_sys::b_rep_prim_api::MakeBox::new_real3(10.0, 20.0, 30.0);
    let compound_shape = make_box.shape();

    // This should panic because compound_shape is a Compound, not a Face
    let _face = opencascade_sys::topo_ds::face_shape(compound_shape);
}