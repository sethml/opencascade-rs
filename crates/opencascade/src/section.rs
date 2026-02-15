use crate::primitives::Shape;
use opencascade_sys::b_rep_algo_api;

/// A wrapper around the `BRepAlgoAPI_Section` class.
pub struct Section {
    pub(crate) inner: opencascade_sys::OwnedPtr<b_rep_algo_api::Section>,
}
impl Section {
    /// Create a new `Section` to intersect `target` by `tool`.
    pub fn new(target: &Shape, tool: &Shape) -> Section {
        let perform_now = true;
        Section {
            inner: b_rep_algo_api::Section::new_shape2_bool(
                &target.inner,
                &tool.inner,
                perform_now,
            ),
        }
    }

    /// Get the edges of the resulting intersection.
    pub fn section_edges(mut self) -> Vec<Shape> {
        let list = self.inner.section_edges();
        let mut shapes = Vec::new();
        let mut iter = list.iter();
        loop {
            let shape = iter.next();
            if shape.is_null() {
                break;
            }
            shapes.push(Shape::from_shape(&shape));
        }
        shapes
    }
}

/// Creates a `Section` from two shapes, performs the intersection, and returns the resulting edges.
pub fn edges(target: &Shape, tool: &Shape) -> Vec<Shape> {
    let section = Section::new(target, tool);
    section.section_edges()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        primitives::{IntoShape, ShapeType},
        workplane::Workplane,
    };
    use glam::dvec3;

    #[test]
    fn section_new() {
        let a = Workplane::xy().rect(1.0, 1.0).to_face();
        let b = Workplane::yz().rect(1.0, 1.0).to_face();

        let s = Section::new(&a.into_shape(), &b.into_shape());

        let edges = s.section_edges();
        assert_eq!(edges.len(), 1);

        let s = edges.first().unwrap();

        assert_eq!(s.shape_type(), ShapeType::Edge);

        let e = s.edges().next().expect("There should be only one edge");

        assert_eq!(e.start_point(), dvec3(0.0, -0.5, 0.0));
        assert_eq!(e.end_point(), dvec3(0.0, 0.5, 0.0));
    }
}
