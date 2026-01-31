#pragma once
#include "rust/cxx.h"
#include <memory>

#include <BRepAdaptor_Curve.hxx>

#include <Standard_Real.hxx>

inline std::unique_ptr<gp_Pnt> BRepAdaptor_Curve_value(const BRepAdaptor_Curve &curve, const Standard_Real U) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(curve.Value(U)));
}

