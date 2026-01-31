#pragma once
#include "rust/cxx.h"
#include <memory>

#include <GeomAPI_Interpolate.hxx>
#include <GeomAPI_ProjectPointOnSurf.hxx>

#include <Geom_BSplineCurve.hxx>

inline std::unique_ptr<HandleGeomBSplineCurve> GeomAPI_Interpolate_Curve(const GeomAPI_Interpolate &interpolate) {
  return std::unique_ptr<HandleGeomBSplineCurve>(new opencascade::handle<Geom_BSplineCurve>(interpolate.Curve()));
}

