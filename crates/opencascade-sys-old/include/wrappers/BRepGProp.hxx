#pragma once
#include "rust/cxx.h"
#include <memory>

#include <BRepGProp.hxx>
#include <BRepGProp_Face.hxx>

#include <GProp_GProps.hxx>
#include <TopoDS_Shape.hxx>

inline void BRepGProp_LinearProperties(const TopoDS_Shape &shape, GProp_GProps &props) {
  BRepGProp::LinearProperties(shape, props);
}

inline void BRepGProp_SurfaceProperties(const TopoDS_Shape &shape, GProp_GProps &props) {
  BRepGProp::SurfaceProperties(shape, props);
}

inline void BRepGProp_VolumeProperties(const TopoDS_Shape &shape, GProp_GProps &props) {
  BRepGProp::VolumeProperties(shape, props);
}

