// Manual binding for Transfer_Finder::GetStringAttribute
//
// The C++ method has a `Standard_CString&` output parameter (const char*&),
// which can't be auto-generated. See Transfer_Finder.hxx:118.

extern "C" {
    fn Transfer_Finder_get_string_attribute_str(
        self_: *const crate::ffi::Transfer_Finder,
        name: *const std::ffi::c_char,
    ) -> *const std::ffi::c_char;
}

impl Finder {
    /// Returns a string attribute by name.
    ///
    /// Returns `Some(value)` if the attribute exists and is a string,
    /// `None` otherwise.
    pub fn get_string_attribute(&self, name: &str) -> Option<String> {
        let c_name = std::ffi::CString::new(name).unwrap();
        unsafe {
            let ptr = Transfer_Finder_get_string_attribute_str(
                self as *const Self,
                c_name.as_ptr(),
            );
            if ptr.is_null() {
                None
            } else {
                // ptr points into the Finder's attribute map; valid for the
                // lifetime of self. Copy it into an owned String immediately.
                Some(std::ffi::CStr::from_ptr(ptr).to_string_lossy().into_owned())
            }
        }
    }
}
