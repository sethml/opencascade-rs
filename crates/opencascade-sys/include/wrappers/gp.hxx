#pragma once
#include <memory>

#include <gp.hxx>
#include <gp_Ax2.hxx>
#include <gp_Ax3.hxx>
#include <gp_Circ.hxx>
#include <gp_Lin.hxx>
#include <gp_Pnt.hxx>
#include <gp_Trsf.hxx>
#include <gp_Vec.hxx>

// Geometric processing
inline const gp_Ax1 &gp_OX() { return gp::OX(); }
inline const gp_Ax1 &gp_OY() { return gp::OY(); }
inline const gp_Ax1 &gp_OZ() { return gp::OZ(); }

inline const gp_Dir &gp_DZ() { return gp::DZ(); }

inline std::unique_ptr<gp_Ax1> gp_Ax1_ctor(const gp_Pnt &origin, const gp_Dir &main_dir) {
  return std::unique_ptr<gp_Ax1>(new gp_Ax1(origin, main_dir));
}

inline std::unique_ptr<gp_Ax2> gp_Ax2_ctor(const gp_Pnt &origin, const gp_Dir &main_dir) {
  return std::unique_ptr<gp_Ax2>(new gp_Ax2(origin, main_dir));
}

inline std::unique_ptr<gp_Ax3> gp_Ax3_from_gp_Ax2(const gp_Ax2 &axis) {
  return std::unique_ptr<gp_Ax3>(new gp_Ax3(axis));
}

inline std::unique_ptr<gp_Dir> gp_Dir_ctor(double x, double y, double z) {
  return std::unique_ptr<gp_Dir>(new gp_Dir(x, y, z));
}

inline std::unique_ptr<gp_Dir2d> gp_Dir2d_ctor(double x, double y) {
  return std::unique_ptr<gp_Dir2d>(new gp_Dir2d(x, y));
}

inline std::unique_ptr<gp_Ax2d> gp_Ax2d_ctor(const gp_Pnt2d &point, const gp_Dir2d &dir) {
  return std::unique_ptr<gp_Ax2d>(new gp_Ax2d(point, dir));
}
