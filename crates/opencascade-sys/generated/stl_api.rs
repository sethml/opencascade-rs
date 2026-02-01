#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_stl_api.hxx");
        #[doc = "ProgressRange from message module"]
        type Message_ProgressRange = crate::message::ffi::ProgressRange;
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
        #[doc = " ======================== StlAPI_Writer ========================"]
        #[doc = "This class creates and writes STL files from Open CASCADE shapes. An STL file can be written to an existing STL file or to a new one."]
        #[cxx_name = "StlAPI_Writer"]
        type Writer;
        #[doc = "Creates a writer object with default parameters: ASCIIMode."]
        #[cxx_name = "StlAPI_Writer_ctor"]
        fn Writer_ctor() -> UniquePtr<Writer>;
        #[doc = "Returns the address to the flag defining the mode for writing the file. This address may be used to either read or change the flag. If the mode returns True (default value) the generated file is an ASCII file. If the mode returns False, the generated file is a binary file."]
        #[cxx_name = "ASCIIMode"]
        fn ascii_mode(self: Pin<&mut Writer>) -> &mut bool;
        #[doc = "Converts a given shape to STL format and writes it to file with a given filename. \\return the error state."]
        #[cxx_name = "StlAPI_Writer_Write"]
        fn Writer_write(
            self_: Pin<&mut Writer>,
            theShape: &TopoDS_Shape,
            theFileName: &str,
            theProgress: &Message_ProgressRange,
        ) -> bool;
    }
    impl UniquePtr<Writer> {}
}
pub use ffi::Writer;
impl Writer {
    #[doc = "Creates a writer object with default parameters: ASCIIMode."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Writer_ctor()
    }
}
