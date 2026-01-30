#pragma once
#include "rust/cxx.h"
#include <memory>

#include <IGESControl_Reader.hxx>
#include <IGESControl_Writer.hxx>

#include <IFSelect_ReturnStatus.hxx>
#include <TopoDS_Shape.hxx>

inline IFSelect_ReturnStatus read_iges(IGESControl_Reader &reader, rust::String theFileName) {
  return reader.ReadFile(theFileName.c_str());
}

inline void compute_model(IGESControl_Writer &writer) { writer.ComputeModel(); }

inline bool add_shape(IGESControl_Writer &writer, const TopoDS_Shape &theShape) { return writer.AddShape(theShape); }

inline bool write_iges(IGESControl_Writer &writer, rust::String theFileName) {
  return writer.Write(theFileName.c_str());
}

