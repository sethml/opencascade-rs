#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_top_exp.hxx");
        #[doc = "ShapeEnum from top_abs module"]
        type TopAbs_ShapeEnum = crate::top_abs::ffi::TopAbs_ShapeEnum;
        #[doc = "Orientation from top_abs module"]
        type TopAbs_Orientation = crate::top_abs::ffi::TopAbs_Orientation;
        #[doc = "Shape from topo_ds module"]
        type TopoDS_Shape = crate::topo_ds::ffi::Shape;
        #[doc = "Vertex from topo_ds module"]
        type TopoDS_Vertex = crate::topo_ds::ffi::Vertex;
        #[doc = "Edge from topo_ds module"]
        type TopoDS_Edge = crate::topo_ds::ffi::Edge;
        #[doc = "Wire from topo_ds module"]
        type TopoDS_Wire = crate::topo_ds::ffi::Wire;
        #[doc = "Face from topo_ds module"]
        type TopoDS_Face = crate::topo_ds::ffi::Face;
        #[doc = "Shell from topo_ds module"]
        type TopoDS_Shell = crate::topo_ds::ffi::Shell;
        #[doc = "Solid from topo_ds module"]
        type TopoDS_Solid = crate::topo_ds::ffi::Solid;
        #[doc = "Compound from topo_ds module"]
        type TopoDS_Compound = crate::topo_ds::ffi::Compound;
        #[doc = "CompSolid from topo_ds module"]
        type TopoDS_CompSolid = crate::topo_ds::ffi::CompSolid;
        #[doc = "Builder from topo_ds module"]
        type TopoDS_Builder = crate::topo_ds::ffi::Builder;
        #[doc = " ======================== TopExp_Explorer ========================"]
        #[doc = "An Explorer is a Tool to visit  a Topological Data Structure form the TopoDS package. An Explorer is built with : * The Shape to explore. * The type of Shapes to find : e.g VERTEX, EDGE. This type cannot be SHAPE. * The type of Shapes to avoid. e.g  SHELL, EDGE. By default   this type is  SHAPE which  means no restriction on the exploration. The Explorer  visits  all the  structure   to find shapes of the   requested  type  which   are   not contained in the type to avoid. Example to find all the Faces in the Shape S : TopExp_Explorer Ex; for (Ex.Init(S,TopAbs_FACE); Ex.More(); Ex.Next()) { ProcessFace(Ex.Current()); } // an other way TopExp_Explorer Ex(S,TopAbs_FACE); while (Ex.More()) { ProcessFace(Ex.Current()); Ex.Next(); } To find all the vertices which are not in an edge : for (Ex.Init(S,TopAbs_VERTEX,TopAbs_EDGE); ...) To  find all the faces  in   a SHELL, then all the faces not in a SHELL : TopExp_Explorer Ex1, Ex2; for (Ex1.Init(S,TopAbs_SHELL),...) { // visit all shells for (Ex2.Init(Ex1.Current(),TopAbs_FACE),...) { // visit all the faces of the current shell } } for (Ex1.Init(S,TopAbs_FACE,TopAbs_SHELL),...) { // visit all faces not in a shell } If   the type  to avoid  is   the same  or is less complex than the type to find it has no effect. For example searching edges  not in a vertex  does not make a difference."]
        #[cxx_name = "TopExp_Explorer"]
        type Explorer;
        #[doc = "Creates an empty explorer, becomes useful after Init."]
        #[cxx_name = "TopExp_Explorer_ctor"]
        fn Explorer_ctor() -> UniquePtr<Explorer>;
        #[doc = "Returns True if there are more shapes in the exploration."]
        #[cxx_name = "More"]
        fn more(self: &Explorer) -> bool;
        #[doc = "Moves to the next Shape in the exploration. Exceptions Standard_NoMoreObject if there are no more shapes to explore."]
        #[cxx_name = "Next"]
        fn next(self: Pin<&mut Explorer>);
        #[doc = "Returns the current shape in the exploration. Exceptions Standard_NoSuchObject if this explorer has no more shapes to explore."]
        #[cxx_name = "Value"]
        fn value(self: &Explorer) -> &TopoDS_Shape;
        #[doc = "Returns the current shape in the exploration. Exceptions Standard_NoSuchObject if this explorer has no more shapes to explore."]
        #[cxx_name = "Current"]
        fn current(self: &Explorer) -> &TopoDS_Shape;
        #[doc = "Reinitialize the exploration with the original arguments."]
        #[cxx_name = "ReInit"]
        fn re_init(self: Pin<&mut Explorer>);
        #[doc = "Return explored shape."]
        #[cxx_name = "ExploredShape"]
        fn explored_shape(self: &Explorer) -> &TopoDS_Shape;
        #[doc = "Returns the current depth of the exploration. 0 is the shape to explore itself."]
        #[cxx_name = "Depth"]
        fn depth(self: &Explorer) -> i32;
        #[doc = "Clears the content of the explorer. It will return False on More()."]
        #[cxx_name = "Clear"]
        fn clear(self: Pin<&mut Explorer>);
    }
    impl UniquePtr<Explorer> {}
}
pub use ffi::Explorer;
impl Explorer {
    #[doc = "Creates an empty explorer, becomes useful after Init."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Explorer_ctor()
    }
}
