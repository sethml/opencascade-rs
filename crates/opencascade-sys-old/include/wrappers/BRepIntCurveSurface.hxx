#pragma once
#include "rust/cxx.h"
#include <memory>

#include <BRepIntCurveSurface_Inter.hxx>

inline std::unique_ptr<gp_Pnt> BRepIntCurveSurface_Inter_point(const BRepIntCurveSurface_Inter &intersector) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(intersector.Pnt()));
}

