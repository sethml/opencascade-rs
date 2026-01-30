#pragma once
#include "rust/cxx.h"
#include <memory>

#include <BRepBndLib.hxx>

#include <Bnd_Box.hxx>
#include <Standard_Boolean.hxx>
#include <TopoDS_Shape.hxx>

// BRepBndLib
inline void BRepBndLib_Add(const TopoDS_Shape &shape, Bnd_Box &box, const Standard_Boolean useTriangulation) {
  BRepBndLib::Add(shape, box, useTriangulation);
}
