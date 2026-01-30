#pragma once
#include "rust/cxx.h"
#include <memory>

#include <GCE2d_MakeSegment.hxx>

#include <Geom2d_TrimmedCurve.hxx>

inline std::unique_ptr<HandleGeom2d_TrimmedCurve> GCE2d_MakeSegment_point_point(const gp_Pnt2d &p1,
                                                                                const gp_Pnt2d &p2) {
  return std::unique_ptr<HandleGeom2d_TrimmedCurve>(
      new opencascade::handle<Geom2d_TrimmedCurve>(GCE2d_MakeSegment(p1, p2)));
}

