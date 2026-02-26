//! Exception handling for the OCCT FFI boundary.
//!
//! Non-void C++ wrappers return `OcctResult<T>` — a generic struct containing
//! the return value and a null-terminated exception string pointer. Void wrappers
//! return `*const c_char` directly (null = success, non-null = exception).
//!
//! Generated wrappers call `check_result()` or `check_void_result()` which
//! handle the null check and panic on the error path.

/// Result type returned from non-void C++ FFI wrappers.
///
/// On success: `exc` is null, `ret` holds the return value.
/// On exception: `exc` points to a null-terminated UTF-8 message,
/// allocated by `occt_alloc_exception` using Rust's global allocator.
#[repr(C)]
pub struct OcctResult<T> {
    pub ret: T,
    pub exc: *const std::ffi::c_char,
}

/// Check a non-void FFI result; panic if there was an exception.
#[inline(always)]
#[track_caller]
pub fn check_result<T>(result: OcctResult<T>) -> T {
    if !result.exc.is_null() {
        wrapper_threw_exception(result.exc);
    }
    result.ret
}

/// Check a void FFI result; panic if there was an exception.
#[inline(always)]
#[track_caller]
pub fn check_void_result(exc: *const std::ffi::c_char) {
    if !exc.is_null() {
        wrapper_threw_exception(exc);
    }
}

/// Called from C++ to allocate a null-terminated copy of the exception message.
///
/// The C++ side formats "TypeName: message" and passes it here.
/// We copy into Rust-allocated memory so `wrapper_threw_exception`
/// can free it with the Rust allocator.
#[no_mangle]
extern "C" fn occt_alloc_exception(ptr: *const u8, len: usize) -> *const std::ffi::c_char {
    unsafe {
        let layout = std::alloc::Layout::from_size_align_unchecked(len + 1, 1);
        let copy = std::alloc::alloc(layout);
        if copy.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        std::ptr::copy_nonoverlapping(ptr, copy, len);
        *copy.add(len) = 0; // null terminator
        copy as *const std::ffi::c_char
    }
}

/// Panic with the message from a C++ exception.
///
/// Called when a generated Rust wrapper detects `exc != null`.
/// Reads the message, deallocates the buffer, and panics.
/// `#[track_caller]` ensures the panic location points to the
/// generated wrapper, not this function.
#[cold]
#[inline(never)]
#[track_caller]
pub fn wrapper_threw_exception(exc: *const std::ffi::c_char) -> ! {
    let cstr = unsafe { std::ffi::CStr::from_ptr(exc) };
    let bytes = cstr.to_bytes();
    let msg = std::str::from_utf8(bytes).unwrap_or("<invalid UTF-8>");
    let owned = msg.to_string();
    // Free the Rust-allocated buffer (len + 1 for null terminator)
    unsafe {
        let layout = std::alloc::Layout::from_size_align_unchecked(bytes.len() + 1, 1);
        std::alloc::dealloc(exc as *mut u8, layout);
    }
    panic!("OCCT C++ exception: {owned}");
}