#pragma once
#include "rust/cxx.h"
#include <memory>

#include <TopLoc_Location.hxx>

inline std::unique_ptr<gp_Trsf> TopLoc_Location_Transformation(const TopLoc_Location &location) {
  return std::unique_ptr<gp_Trsf>(new gp_Trsf(location.Transformation()));
}

