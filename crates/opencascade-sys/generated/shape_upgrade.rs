#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_shape_upgrade.hxx");
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
        #[doc = "BRepTools from b_rep_tools module"]
        type BRepTools = crate::b_rep_tools::ffi::BRepTools;
        #[doc = "FormatVersion from top_tools module"]
        type TopTools_FormatVersion = crate::top_tools::ffi::TopTools_FormatVersion;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "BRepTools_History"]
        type BRepTools_History;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Standard_Type"]
        type Standard_Type;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TopTools_MapOfShape"]
        type TopTools_MapOfShape;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleBRepToolsHistory"]
        type HandleBRepToolsHistory;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleStandardType"]
        type HandleStandardType;
        #[doc = " ======================== ShapeUpgrade_UnifySameDomain ========================"]
        #[doc = "This tool tries to unify faces and edges of the shape which lie on the same geometry. Faces/edges are considering as 'same-domain' if a group of neighbouring faces/edges are lying on coincident surfaces/curves. In this case these faces/edges can be unified into one face/edge. ShapeUpgrade_UnifySameDomain is initialized by a shape and the next optional parameters: UnifyFaces - tries to unify all possible faces UnifyEdges - tries to unify all possible edges ConcatBSplines - if this flag is set to true then all neighbouring edges, which lay on BSpline or Bezier curves with C1 continuity on their common vertices, will be merged into one common edge. The input shape can be of any type containing faces or edges - compsolid, solid, shell, wire, compound of any kind of shapes. The algorithm preserves the structure of compsolids, solids, shells and wires. E.g., if two shells have a common edge and the faces sharing this edge lie on the same surface the algorithm will not unify these faces, otherwise the structure of shells would be broken. However, if such faces belong to different compounds of faces they will be unified. The output result of the tool is the unified shape. All the modifications of initial shape are recorded during unifying. Methods History are intended to: <br> - set a place holder for the history of modifications of sub-shapes of the initial shape; <br> - get the collected history. <br> The algorithm provides a place holder for the history and collects the history by default. To avoid collecting of the history the place holder should be set to null handle."]
        #[cxx_name = "ShapeUpgrade_UnifySameDomain"]
        type UnifySameDomain;
        #[doc = "Empty constructor"]
        #[cxx_name = "ShapeUpgrade_UnifySameDomain_ctor"]
        fn UnifySameDomain_ctor() -> UniquePtr<UnifySameDomain>;
        #[doc = "Constructor defining input shape and necessary flags. It does not perform unification."]
        #[cxx_name = "ShapeUpgrade_UnifySameDomain_ctor_shape_bool3"]
        fn UnifySameDomain_ctor_shape_bool3(
            aShape: &TopoDS_Shape,
            UnifyEdges: bool,
            UnifyFaces: bool,
            ConcatBSplines: bool,
        ) -> UniquePtr<UnifySameDomain>;
        #[doc = "Initializes with a shape and necessary flags. It does not perform unification. If you intend to nullify the History place holder do it after initialization."]
        #[cxx_name = "Initialize"]
        fn initialize(
            self: Pin<&mut UnifySameDomain>,
            aShape: &TopoDS_Shape,
            UnifyEdges: bool,
            UnifyFaces: bool,
            ConcatBSplines: bool,
        );
        #[doc = "Sets the flag defining whether it is allowed to create internal edges inside merged faces in the case of non-manifold topology. Without this flag merging through multi connected edge is forbidden. Default value is false."]
        #[cxx_name = "AllowInternalEdges"]
        fn allow_internal_edges(self: Pin<&mut UnifySameDomain>, theValue: bool);
        #[doc = "Sets the shape for avoid merging of the faces/edges. This shape can be vertex or edge. If the shape is a vertex it forbids merging of connected edges. If the shape is a edge it forbids merging of connected faces. This method can be called several times to keep several shapes."]
        #[cxx_name = "KeepShape"]
        fn keep_shape(self: Pin<&mut UnifySameDomain>, theShape: &TopoDS_Shape);
        #[doc = "Sets the map of shapes for avoid merging of the faces/edges. It allows passing a ready to use map instead of calling many times the method KeepShape."]
        #[cxx_name = "KeepShapes"]
        fn keep_shapes(self: Pin<&mut UnifySameDomain>, theShapes: &TopTools_MapOfShape);
        #[doc = "Sets the flag defining the behavior of the algorithm regarding modification of input shape. If this flag is equal to True then the input (original) shape can't be modified during modification process. Default value is true."]
        #[cxx_name = "SetSafeInputMode"]
        fn set_safe_input_mode(self: Pin<&mut UnifySameDomain>, theValue: bool);
        #[doc = "Sets the linear tolerance. It plays the role of chord error when taking decision about merging of shapes. Default value is Precision::Confusion()."]
        #[cxx_name = "SetLinearTolerance"]
        fn set_linear_tolerance(self: Pin<&mut UnifySameDomain>, theValue: f64);
        #[doc = "Sets the angular tolerance. If two shapes form a connection angle greater than this value they will not be merged. Default value is Precision::Angular()."]
        #[cxx_name = "SetAngularTolerance"]
        fn set_angular_tolerance(self: Pin<&mut UnifySameDomain>, theValue: f64);
        #[doc = "Performs unification and builds the resulting shape."]
        #[cxx_name = "Build"]
        fn build(self: Pin<&mut UnifySameDomain>);
        #[doc = "Gives the resulting shape"]
        #[cxx_name = "Shape"]
        fn shape(self: &UnifySameDomain) -> &TopoDS_Shape;
        #[doc = "Returns the history of the processed shapes."]
        #[cxx_name = "History"]
        fn history(self: &UnifySameDomain) -> &HandleBRepToolsHistory;
        #[doc = "Returns the history of the processed shapes."]
        #[cxx_name = "History"]
        fn history2(self: Pin<&mut UnifySameDomain>) -> Pin<&mut HandleBRepToolsHistory>;
        #[cxx_name = "DynamicType"]
        fn dynamic_type(self: &UnifySameDomain) -> &HandleStandardType;
        #[cxx_name = "ShapeUpgrade_UnifySameDomain_get_type_name"]
        fn UnifySameDomain_get_type_name() -> String;
    }
    impl UniquePtr<UnifySameDomain> {}
}
pub use ffi::UnifySameDomain;
impl UnifySameDomain {
    #[doc = "Empty constructor"]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::UnifySameDomain_ctor()
    }

    #[doc = "Constructor defining input shape and necessary flags. It does not perform unification."]
    pub fn new_shape_bool3(
        aShape: &ffi::TopoDS_Shape,
        UnifyEdges: bool,
        UnifyFaces: bool,
        ConcatBSplines: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::UnifySameDomain_ctor_shape_bool3(aShape, UnifyEdges, UnifyFaces, ConcatBSplines)
    }
}
