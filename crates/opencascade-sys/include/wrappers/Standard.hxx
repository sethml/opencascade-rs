#pragma once
#include "rust/cxx.h"
#include <memory>

#include <Standard_Type.hxx>

// Handles
typedef opencascade::handle<Standard_Type> HandleStandardType;

inline rust::String type_name(const HandleStandardType &handle) { return std::string(handle->Name()); }
