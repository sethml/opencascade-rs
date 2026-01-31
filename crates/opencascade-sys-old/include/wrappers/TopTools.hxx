#pragma once
#include "rust/cxx.h"
#include <memory>

#include <TopTools_HSequenceOfShape.hxx>
#include <TopTools_ListOfShape.hxx>

#include <TopoDS_Face.hxx>
#include <TopoDS_Shape.hxx>

// Typedefs
typedef opencascade::handle<TopTools_HSequenceOfShape> HandleTopTools_HSequenceOfShape;

// Collections
inline void shape_list_append_face(TopTools_ListOfShape &list, const TopoDS_Face &face) { list.Append(face); }

inline std::unique_ptr<HandleTopTools_HSequenceOfShape> new_HandleTopTools_HSequenceOfShape() {
  auto sequence = new TopTools_HSequenceOfShape();
  auto handle = new opencascade::handle<TopTools_HSequenceOfShape>(sequence);
  return std::unique_ptr<HandleTopTools_HSequenceOfShape>(handle);
}

inline void TopTools_HSequenceOfShape_append(HandleTopTools_HSequenceOfShape &handle, const TopoDS_Shape &shape) {
  handle->Append(shape);
}

inline Standard_Integer TopTools_HSequenceOfShape_length(const HandleTopTools_HSequenceOfShape &handle) {
  return handle->Length();
}

inline const TopoDS_Shape &TopTools_HSequenceOfShape_value(const HandleTopTools_HSequenceOfShape &handle,
                                                           Standard_Integer index) {
  return handle->Value(index);
}

