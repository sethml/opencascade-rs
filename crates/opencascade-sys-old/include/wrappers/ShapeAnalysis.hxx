#pragma once
#include "rust/cxx.h"
#include <memory>

#include <ShapeAnalysis_FreeBounds.hxx>

#include <Standard_Boolean.hxx>
#include <Standard_Real.hxx>

inline void connect_edges_to_wires(HandleTopTools_HSequenceOfShape &edges, const Standard_Real toler,
                                   const Standard_Boolean shared, HandleTopTools_HSequenceOfShape &wires) {
  ShapeAnalysis_FreeBounds::ConnectEdgesToWires(edges, toler, shared, wires);
}

