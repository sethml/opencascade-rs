#pragma once
#include "rust/cxx.h"
#include <memory>

#include <STEPControl_Reader.hxx>
#include <STEPControl_Writer.hxx>

#include <IFSelect_ReturnStatus.hxx>
#include <TopoDS_Shape.hxx>

// Data Import
inline IFSelect_ReturnStatus read_step(STEPControl_Reader &reader, rust::String theFileName) {
  return reader.ReadFile(theFileName.c_str());
}

// Data Export
inline IFSelect_ReturnStatus transfer_shape(STEPControl_Writer &writer, const TopoDS_Shape &theShape) {
  return writer.Transfer(theShape, STEPControl_AsIs);
}

inline IFSelect_ReturnStatus write_step(STEPControl_Writer &writer, rust::String theFileName) {
  return writer.Write(theFileName.c_str());
}

