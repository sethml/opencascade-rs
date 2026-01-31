// Module TopTools
use cxx::UniquePtr;
#[allow(unused_imports)]
use crate::ffi::*;

// Re-export symbols from FFI
pub use crate::ffi::{
    HandleTopTools_HSequenceOfShape_Get,
    TopTools_HSequenceOfShape_append as HSequenceOfShape_append,
    TopTools_HSequenceOfShape_length as HSequenceOfShape_length,
    TopTools_HSequenceOfShape_value as HSequenceOfShape_value,
    map_shapes,
    map_shapes_and_ancestors,
    map_shapes_and_unique_ancestors,
    new_HandleTopTools_HSequenceOfShape,
    new_indexed_data_map_of_shape_list_of_shape,
    new_indexed_map_of_shape,
    new_list_of_shape,
    shape_list_append_face,
    shape_list_to_vector,
};

// Type aliases
pub use crate::ffi::HandleTopTools_HSequenceOfShape;
pub type HSequenceOfShape = crate::ffi::TopTools_HSequenceOfShape;
pub type IndexedDataMapOfShapeListOfShape = crate::ffi::TopTools_IndexedDataMapOfShapeListOfShape;
pub type IndexedMapOfShape = crate::ffi::TopTools_IndexedMapOfShape;
pub type ListOfShape = crate::ffi::TopTools_ListOfShape;
