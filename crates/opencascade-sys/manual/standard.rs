// Manual bindings for C++ iostream global objects (std::cout, std::cerr, etc.)
//
// These provide access to C++ standard output/error streams, which can be
// passed to OCCT methods that take Standard_OStream& parameters (e.g., Dump,
// DumpJson, Print).
//
// This file is include!()'d into the generated standard.rs module, which
// already provides OStream/IStream type aliases via its re-exports section.

extern "C" {
    fn iostream_cout() -> *mut crate::ffi_types::Standard_OStream;
    fn iostream_cerr() -> *mut crate::ffi_types::Standard_OStream;
    fn iostream_clog() -> *mut crate::ffi_types::Standard_OStream;
    fn iostream_cin() -> *mut crate::ffi_types::Standard_IStream;
}

/// Returns a mutable reference to `std::cout` (C++ standard output stream).
///
/// # Safety
///
/// The returned reference points to the global C++ `std::cout` object.
/// Concurrent writes from multiple threads are safe but may result in
/// interleaved output. The reference is valid for the entire program lifetime.
///
/// # Example
///
/// ```no_run
/// let cout = opencascade_sys::standard::cout();
/// // Pass to OCCT methods that take &mut OStream:
/// // some_shape.dump_json(cout, 0);
/// ```
pub fn cout() -> &'static mut OStream {
    unsafe { &mut *iostream_cout() }
}

/// Returns a mutable reference to `std::cerr` (C++ standard error stream).
///
/// # Safety
///
/// Same safety requirements as [`cout()`].
pub fn cerr() -> &'static mut OStream {
    unsafe { &mut *iostream_cerr() }
}

/// Returns a mutable reference to `std::clog` (C++ standard log stream).
///
/// # Safety
///
/// Same safety requirements as [`cout()`].
pub fn clog() -> &'static mut OStream {
    unsafe { &mut *iostream_clog() }
}

/// Returns a mutable reference to `std::cin` (C++ standard input stream).
///
/// # Safety
///
/// Same safety requirements as [`cout()`].
pub fn cin() -> &'static mut IStream {
    unsafe { &mut *iostream_cin() }
}
