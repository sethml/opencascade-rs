use crate::primitives::Shape;
use cxx::UniquePtr;
use opencascade_sys::b_rep_algo_api;

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
        let list = self.inner.pin_mut().section_edges();
        let mut iter = list.iter();
        let mut edges = Vec::new();
        loop {
            let shape = iter.pin_mut().next();
            if shape.is_null() {
                break;
            }
            edges.push(Shape::from_shape(&shape));
        }
        edges
    }
}

/// Creates a `Section` from two shapes, performs the intersection, and returns the resulting edges.
pub fn edges(target: &Shape, tool: &Shape) -> Vec<Shape> {
    Section::new(target, tool).section_edges()
}
