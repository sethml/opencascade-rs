//! C++ exception handling for the OCCT FFI boundary.
//!
//! When a C++ OCCT function throws an exception (typically a `Standard_Failure`
//! subclass), the generated wrapper catches it and stores the exception info
//! in a thread-local. After every FFI call, the generated Rust wrappers call
//! `check_exception()` which panics if an exception was caught.

use std::ffi::{c_char, CStr};

extern "C" {
    fn occt_has_pending_exception() -> bool;
    fn occt_pending_exception_message() -> *const c_char;
    fn occt_pending_exception_type() -> *const c_char;
    fn occt_clear_exception();
}

/// Check for pending OCCT C++ exceptions and panic if one is found.
///
/// Called after every FFI call to convert C++ exceptions to Rust panics.
/// The panic message includes the C++ exception type and message.
/// Set `RUST_BACKTRACE=1` to see the full stack trace.
#[inline]
pub fn check_exception() {
    if unsafe { occt_has_pending_exception() } {
        let msg = unsafe {
            let ptr = occt_pending_exception_message();
            if ptr.is_null() {
                "<null>"
            } else {
                CStr::from_ptr(ptr).to_str().unwrap_or("<invalid UTF-8>")
            }
        };
        let ty = unsafe {
            let ptr = occt_pending_exception_type();
            if ptr.is_null() {
                "<null>"
            } else {
                CStr::from_ptr(ptr).to_str().unwrap_or("<invalid UTF-8>")
            }
        };
        unsafe { occt_clear_exception() };
        panic!("OCCT C++ exception: {ty}: {msg}");
    }
}
