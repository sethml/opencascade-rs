// Manual binding for BRepFill_AdvancedEvolved::SetTemporaryDirectory
//
// The C++ method takes `const Standard_CString&` (const char* const&),
// which can't be auto-generated. See BRepFill_AdvancedEvolved.hxx:64.

extern "C" {
    fn BRepFill_AdvancedEvolved_set_temporary_directory_str(
        self_: *mut crate::ffi::BRepFill_AdvancedEvolved,
        path_ptr: *const u8,
        path_len: usize,
    );
}

impl AdvancedEvolved {
    /// Sets directory where the debug shapes will be saved.
    ///
    /// # Memory
    ///
    /// This allocates a copy of the path string via `malloc` that is
    /// intentionally leaked. The C++ class stores the raw `const char*`
    /// pointer with no ownership transfer mechanism. Since this method
    /// is typically called at most once with a short path, the leak is
    /// acceptable.
    pub fn set_temporary_directory(&mut self, path: &str) {
        unsafe {
            BRepFill_AdvancedEvolved_set_temporary_directory_str(
                self as *mut Self,
                path.as_ptr(),
                path.len(),
            );
        }
    }
}
