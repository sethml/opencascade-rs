#pragma once
#include "rust/cxx.h"
#include <memory>

#include <StlAPI_Writer.hxx>

#include <TopoDS_Shape.hxx>

inline bool write_stl(StlAPI_Writer &writer, const TopoDS_Shape &theShape, rust::String theFileName) {
  return writer.Write(theShape, theFileName.c_str());
}

