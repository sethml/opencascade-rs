#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_b_rep_mesh.hxx");
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
        #[doc = "ProgressRange from message module"]
        type Message_ProgressRange = crate::message::ffi::ProgressRange;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Standard_Type"]
        type Standard_Type;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "IMeshTools_Context"]
        type IMeshTools_Context;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "IMeshTools_Parameters"]
        type IMeshTools_Parameters;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleIMeshToolsContext"]
        type HandleIMeshToolsContext;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleStandardType"]
        type HandleStandardType;
        #[doc = " ======================== BRepMesh_IncrementalMesh ========================"]
        #[doc = "Builds the mesh of a shape with respect of their correctly triangulated parts"]
        #[cxx_name = "BRepMesh_IncrementalMesh"]
        type IncrementalMesh;
        #[doc = "@name mesher API Default constructor"]
        #[cxx_name = "BRepMesh_IncrementalMesh_ctor"]
        fn IncrementalMesh_ctor() -> UniquePtr<IncrementalMesh>;
        #[doc = "Constructor. Automatically calls method Perform. @param theShape shape to be meshed. @param theLinDeflection linear deflection. @param isRelative if TRUE deflection used for discretization of each edge will be <theLinDeflection> * <size of edge>. Deflection used for the faces will be the maximum deflection of their edges. @param theAngDeflection angular deflection. @param isInParallel if TRUE shape will be meshed in parallel."]
        #[cxx_name = "BRepMesh_IncrementalMesh_ctor_shape_real_bool_real_bool"]
        fn IncrementalMesh_ctor_shape_real_bool_real_bool(
            theShape: &TopoDS_Shape,
            theLinDeflection: f64,
            isRelative: bool,
            theAngDeflection: f64,
            isInParallel: bool,
        ) -> UniquePtr<IncrementalMesh>;
        #[doc = "Constructor. Automatically calls method Perform. @param theShape shape to be meshed. @param theParameters - parameters of meshing"]
        #[cxx_name = "BRepMesh_IncrementalMesh_ctor_shape_parameters_progressrange"]
        fn IncrementalMesh_ctor_shape_parameters_progressrange(
            theShape: &TopoDS_Shape,
            theParameters: &IMeshTools_Parameters,
            theRange: &Message_ProgressRange,
        ) -> UniquePtr<IncrementalMesh>;
        #[doc = "Performs meshing of the shape."]
        #[cxx_name = "Perform"]
        fn performprogressrange(self: Pin<&mut IncrementalMesh>, theRange: &Message_ProgressRange);
        #[doc = "Performs meshing using custom context;"]
        #[cxx_name = "Perform"]
        fn performhandlecontext_2(
            self: Pin<&mut IncrementalMesh>,
            theContext: &HandleIMeshToolsContext,
            theRange: &Message_ProgressRange,
        );
        #[doc = "@name accessing to parameters. Returns meshing parameters"]
        #[cxx_name = "Parameters"]
        fn parameters(self: &IncrementalMesh) -> &IMeshTools_Parameters;
        #[doc = "Returns modifiable meshing parameters"]
        #[cxx_name = "ChangeParameters"]
        fn change_parameters(self: Pin<&mut IncrementalMesh>) -> Pin<&mut IMeshTools_Parameters>;
        #[doc = "Returns modified flag."]
        #[cxx_name = "IsModified"]
        fn is_modified(self: &IncrementalMesh) -> bool;
        #[doc = "Returns accumulated status flags faced during meshing."]
        #[cxx_name = "GetStatusFlags"]
        fn get_status_flags(self: &IncrementalMesh) -> i32;
        #[cxx_name = "DynamicType"]
        fn dynamic_type(self: &IncrementalMesh) -> &HandleStandardType;
        #[doc = "Returns multi-threading usage flag set by default in Discret() static method (thus applied only to Mesh Factories)."]
        #[cxx_name = "BRepMesh_IncrementalMesh_IsParallelDefault"]
        fn IncrementalMesh_is_parallel_default() -> bool;
        #[doc = "Setup multi-threading usage flag set by default in Discret() static method (thus applied only to Mesh Factories)."]
        #[cxx_name = "BRepMesh_IncrementalMesh_SetParallelDefault"]
        fn IncrementalMesh_set_parallel_default(isInParallel: bool);
        #[cxx_name = "BRepMesh_IncrementalMesh_get_type_name"]
        fn IncrementalMesh_get_type_name() -> String;
    }
    impl UniquePtr<IncrementalMesh> {}
}
pub use ffi::IncrementalMesh;
impl IncrementalMesh {
    #[doc = "@name mesher API Default constructor"]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::IncrementalMesh_ctor()
    }

    #[doc = "Constructor. Automatically calls method Perform. @param theShape shape to be meshed. @param theLinDeflection linear deflection. @param isRelative if TRUE deflection used for discretization of each edge will be <theLinDeflection> * <size of edge>. Deflection used for the faces will be the maximum deflection of their edges. @param theAngDeflection angular deflection. @param isInParallel if TRUE shape will be meshed in parallel."]
    pub fn new_shape_real_bool_real_bool(
        theShape: &ffi::TopoDS_Shape,
        theLinDeflection: f64,
        isRelative: bool,
        theAngDeflection: f64,
        isInParallel: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::IncrementalMesh_ctor_shape_real_bool_real_bool(
            theShape,
            theLinDeflection,
            isRelative,
            theAngDeflection,
            isInParallel,
        )
    }

    #[doc = "Constructor. Automatically calls method Perform. @param theShape shape to be meshed. @param theParameters - parameters of meshing"]
    pub fn new_shape_parameters_progressrange(
        theShape: &ffi::TopoDS_Shape,
        theParameters: &ffi::IMeshTools_Parameters,
        theRange: &ffi::Message_ProgressRange,
    ) -> cxx::UniquePtr<Self> {
        ffi::IncrementalMesh_ctor_shape_parameters_progressrange(theShape, theParameters, theRange)
    }
}
