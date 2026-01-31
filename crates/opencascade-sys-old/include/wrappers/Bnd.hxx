#pragma once
#include "rust/cxx.h"
#include <memory>

#include <Bnd_Box.hxx>

// Bnd_Box
inline std::unique_ptr<Bnd_Box> Bnd_Box_ctor() { return std::unique_ptr<Bnd_Box>(new Bnd_Box()); }
inline std::unique_ptr<gp_Pnt> Bnd_Box_CornerMin(const Bnd_Box &box) {
  auto p = box.CornerMin();
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(p));
}
inline std::unique_ptr<gp_Pnt> Bnd_Box_CornerMax(const Bnd_Box &box) {
  auto p = box.CornerMax();
  return std::unique_ptr<gp_Pnt>(new gp_Pnt(p));
}

