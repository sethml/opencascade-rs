#pragma once
#include "rust/cxx.h"
#include <memory>

#include <GProp_GProps.hxx>

// Shape Properties
inline std::unique_ptr<gp_Pnt> GProp_GProps_CentreOfMass(const GProp_GProps &props) {
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(props.CentreOfMass()));
}

