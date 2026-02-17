#include <Transfer_Finder.hxx>

// Manual binding for GetStringAttribute.
// C++ signature: Standard_Boolean GetStringAttribute(Standard_CString name, Standard_CString& val)
// val points into the Finder's attribute map and is valid for the lifetime of the Finder.
// We return it directly; Rust copies it into a String immediately.
extern "C" const char* Transfer_Finder_get_string_attribute_str(
    const Transfer_Finder* self_,
    const char* name
) {
    Standard_CString val = nullptr;
    Standard_Boolean result = self_->GetStringAttribute(name, val);

    // Return the pointer into the Finder's map directly; nullptr signals not-found.
    return result ? val : nullptr;
}
