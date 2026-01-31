#pragma once
#include "rust/cxx.h"
#include <memory>

#include <BRepLib.hxx>
#include <BRepLib_ToolTriangulatedShape.hxx>

#include <Poly_Triangulation.hxx>
#include <TopoDS_Face.hxx>
#include <TopoDS_Shape.hxx>

// BRepLib
inline bool BRepLibBuildCurves3d(const TopoDS_Shape &shape) { return BRepLib::BuildCurves3d(shape); }

inline void compute_normals(const TopoDS_Face &face, const Handle(Poly_Triangulation) & triangulation) {
  BRepLib_ToolTriangulatedShape::ComputeNormals(face, triangulation);
}

