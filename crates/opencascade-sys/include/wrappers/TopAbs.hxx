#pragma once
#include "rust/cxx.h"
#include <memory>

#include <TopAbs_ShapeEnum.hxx>

#include <TopExp.hxx>
#include <TopTools_IndexedDataMapOfShapeListOfShape.hxx>
#include <TopTools_IndexedMapOfShape.hxx>
#include <TopoDS_Shape.hxx>

// Collections
inline void map_shapes(const TopoDS_Shape &S, const TopAbs_ShapeEnum T, TopTools_IndexedMapOfShape &M) {
  TopExp::MapShapes(S, T, M);
}

inline void map_shapes_and_ancestors(const TopoDS_Shape &S, const TopAbs_ShapeEnum TS, const TopAbs_ShapeEnum TA,
                                     TopTools_IndexedDataMapOfShapeListOfShape &M) {
  TopExp::MapShapesAndAncestors(S, TS, TA, M);
}

inline void map_shapes_and_unique_ancestors(const TopoDS_Shape &S, const TopAbs_ShapeEnum TS, const TopAbs_ShapeEnum TA,
                                            TopTools_IndexedDataMapOfShapeListOfShape &M) {
  TopExp::MapShapesAndUniqueAncestors(S, TS, TA, M);
}

