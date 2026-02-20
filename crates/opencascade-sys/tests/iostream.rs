//! Tests for iostream (Standard_OStream / Standard_IStream) bindings.
//!
//! These tests verify that the manual iostream bindings work correctly
//! and that OCCT methods accepting Standard_OStream& can be called with
//! the provided cout/cerr accessors.

/// Verify that cout() returns a valid reference and can be passed to
/// an OCCT method that takes &mut Standard_OStream.
#[test]
fn dump_shape_to_cout() {
    // Create a simple box shape
    let mut make_box = opencascade_sys::b_rep_prim_api::MakeBox::new_real3(10.0, 20.0, 30.0);
    let shape = make_box.shape();

    // Get a reference to std::cout
    let cout = opencascade_sys::standard::cout();

    // Dump the shape's topological structure to stdout.
    // This exercises the full path: manual C++ wrapper -> extern "C" -> Rust FFI.
    opencascade_sys::b_rep_tools::dump_shape_ostream(shape, cout);
}

/// Verify that cerr() also works.
#[test]
fn cerr_accessor_works() {
    let cerr = opencascade_sys::standard::cerr();
    // Just verify we can obtain the reference without crashing.
    // The type is Standard_OStream (opaque), so there's not much to assert,
    // but the fact that the extern "C" call resolved and returned a non-null
    // pointer (converted to a reference) proves the C++ wrapper links correctly.
    let _ptr: *const opencascade_sys::standard::OStream = cerr;
}
