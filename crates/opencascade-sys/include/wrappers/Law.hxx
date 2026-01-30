#pragma once
#include "rust/cxx.h"
#include <memory>

#include <Law_Function.hxx>
#include <Law_Interpol.hxx>

// Law_Function stuff
inline std::unique_ptr<HandleLawFunction> Law_Function_to_handle(std::unique_ptr<Law_Function> law_function) {
  return std::unique_ptr<HandleLawFunction>(new HandleLawFunction(law_function.release()));
}

// Law_Interpol stuff
inline std::unique_ptr<Law_Function> Law_Interpol_into_Law_Function(std::unique_ptr<Law_Interpol> law_interpol) {
  return std::unique_ptr<Law_Function>(law_interpol.release());
}

