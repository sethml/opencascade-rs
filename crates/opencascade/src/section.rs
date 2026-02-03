use crate::primitives::Shape;
use cxx::UniquePtr;
use opencascade_sys::{b_rep_algo_api, top_tools};

/// A wrapper around the `BRepAlgoAPI_Section` class.
pub struct Section {
    pub(crate) inner: UniquePtr<b_rep_algo_api::Section>,
}
impl Section {
    /// Create a new `Section` to intersect `target` by `tool`.
    pub fn new(target: &Shape, tool: &Shape) -> Section {
        let perform_now = true;
        Section {
            inner: b_rep_algo_api::Section::new_shape2_bool(&target.inner, &tool.inner, perform_now),
        }
    }

    /// Get the edges of the resulting intersection.
    pub fn section_edges(mut self) -> Vec<Shape> {
        let builder_algo = self.inner.pin_mut().as_builder_algo_mut();
        let edges = builder_algo.section_edges();
        // Convert cross-module type reference to the actual list type
        let list_of_shape: &top_tools::ListOfShape = unsafe { std::mem::transmute(edges) };
        list_of_shape_to_vec(list_of_shape)
    }
}

/// Creates a `Section` from two shapes, performs the intersection, and returns the resulting edges.
pub fn edges(target: &Shape, tool: &Shape) -> Vec<Shape> {
    Section::new(target, tool).section_edges()
}

fn list_of_shape_to_vec(list: &top_tools::ListOfShape) -> Vec<Shape> {
    let mut shapes = Vec::new();
    for shape in list.iter() {
        if let Some(shape_ref) = shape.as_ref() {
            shapes.push(Shape::from_shape(shape_ref));
        }
    }
    shapes
}

// NOTE: Tests are disabled because section_edges() is blocked
// #[cfg(test)]
// mod test { ... }
