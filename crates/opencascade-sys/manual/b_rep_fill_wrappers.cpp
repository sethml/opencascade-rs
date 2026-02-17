#include <BRepFill_AdvancedEvolved.hxx>
#include <cstdlib>
#include <cstring>

// Manual binding for SetTemporaryDirectory.
// The C++ signature is: void SetTemporaryDirectory(const Standard_CString& thePath)
// where Standard_CString = const char*, so the param is const char* const&.
// The class stores the raw pointer in myDebugShapesPath (a Standard_CString field),
// so the pointed-to string must outlive the object.
extern "C" void BRepFill_AdvancedEvolved_set_temporary_directory_str(
    BRepFill_AdvancedEvolved* self_,
    const char* path_ptr,
    size_t path_len
) {
    // Allocate a null-terminated copy. This is an intentional memory leak:
    // the C++ object stores the raw pointer, and there's no cleanup hook.
    // Acceptable because this is typically called at most once with a short path.
    char* buf = static_cast<char*>(std::malloc(path_len + 1));
    std::memcpy(buf, path_ptr, path_len);
    buf[path_len] = '\0';
    self_->SetTemporaryDirectory(buf);
}
