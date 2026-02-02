// Common utilities for CXX bridge
// This file is hand-maintained, not generated.

#pragma once
#include "rust/cxx.h"
#include <memory>

// CXX's copy constructor support - when a Rust function has #[cxx_name = "construct_unique"],
// CXX generates code that calls this template to copy-construct the object.
// This is required for the to_owned() pattern used by TopoDS and gp types.
template <typename T, typename... Args>
std::unique_ptr<T> construct_unique(Args... args) {
    return std::make_unique<T>(args...);
}
