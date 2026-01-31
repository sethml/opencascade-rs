#pragma once
#include "rust/cxx.h"
#include <memory>

#include <GCPnts_TangentialDeflection.hxx>

#include <Standard_Integer.hxx>

inline std::unique_ptr<gp_Pnt> GCPnts_TangentialDeflection_Value(const GCPnts_TangentialDeflection &approximator,
                                                                 Standard_Integer i) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(approximator.Value(i)));
}

