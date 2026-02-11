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
    ///
    /// NOTE: Currently unimplemented - requires SectionEdges() binding which returns
    /// TopTools_ListOfShape (a collection type not yet supported in unified FFI).
    pub fn section_edges(self) -> Vec<Shape> {
        // TODO: Bind BRepAlgoAPI_BooleanOperation::SectionEdges() and implement
        // collection iteration for TopTools_ListOfShape
        unimplemented!("section_edges requires SectionEdges() binding for TopTools_ListOfShape")
    }
}

/// Creates a `Section` from two shapes, performs the intersection, and returns the resulting edges.
pub fn edges(target: &Shape, tool: &Shape) -> Vec<Shape> {
    Section::new(target, tool).section_edges()
}

// NOTE: Tests are disabled because section_edges() is blocked
// #[cfg(test)]
// mod test { ... }
