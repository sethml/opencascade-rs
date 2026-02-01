#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_iges_control.hxx");
        #[doc = "ReturnStatus from if_select module"]
        type IFSelect_ReturnStatus = crate::if_select::ffi::IFSelect_ReturnStatus;
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
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "IGESData_IGESEntity"]
        type IGESData_IGESEntity;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "DE_ShapeFixParameters"]
        type DE_ShapeFixParameters;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "XSControl_WorkSession"]
        type XSControl_WorkSession;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "IFSelect_PrintCount"]
        type IFSelect_PrintCount;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "IGESData_IGESModel"]
        type IGESData_IGESModel;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Standard_Transient"]
        type Standard_Transient;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "IFSelect_PrintFail"]
        type IFSelect_PrintFail;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Transfer_FinderProcess"]
        type Transfer_FinderProcess;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleIGESDataIGESModel"]
        type HandleIGESDataIGESModel;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleStandardTransient"]
        type HandleStandardTransient;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleTransferFinderProcess"]
        type HandleTransferFinderProcess;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleXSControlWorkSession"]
        type HandleXSControlWorkSession;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleIGESDataIGESEntity"]
        type HandleIGESDataIGESEntity;
        #[doc = " ======================== IGESControl_Reader ========================"]
        #[doc = "Reads IGES files, checks them and translates their contents into Open CASCADE models. The IGES data can be that of a whole model or that of a specific list of entities in the model. As in XSControl_Reader, you specify the list using a selection. For translation of iges files it is possible to use the following sequence: To change parameters of translation class Interface_Static should be used before the beginning of translation (see IGES Parameters and General Parameters) Creation of reader IGESControl_Reader reader; To load a file in a model use method: reader.ReadFile(\"filename.igs\") To check a loading file use method Check: reader.Check(failsonly); where failsonly is equal to Standard_True or Standard_False; To print the results of load: reader.PrintCheckLoad(failsonly,mode) where mode is equal to the value of enumeration IFSelect_PrintCount To transfer entities from a model the following methods can be used: for the whole model reader.TransferRoots(onlyvisible); where onlyvisible is equal to Standard_True or Standard_False; To transfer a list of entities: reader.TransferList(list); To transfer one entity reader.TransferEntity(ent) or reader.Transfer(num); To obtain a result the following method can be used: reader.IsDone() reader.NbShapes() and reader.Shape(num); or reader.OneShape(); To print the results of transfer use method: reader.PrintTransferInfo(failwarn,mode); where printfail is equal to the value of enumeration IFSelect_PrintFail, mode see above. Gets correspondence between an IGES entity and a result shape obtained therefrom. reader.TransientProcess(); TopoDS_Shape shape = TransferBRep::ShapeResult(reader.TransientProcess(),ent);"]
        #[cxx_name = "IGESControl_Reader"]
        type Reader;
        #[doc = "Creates a Reader from scratch"]
        #[cxx_name = "IGESControl_Reader_ctor"]
        fn Reader_ctor() -> UniquePtr<Reader>;
        #[doc = "Creates a Reader from an already existing Session"]
        #[cxx_name = "IGESControl_Reader_ctor_handleworksession_bool"]
        fn Reader_ctor_handleworksession_bool(
            WS: &HandleXSControlWorkSession,
            scratch: bool,
        ) -> UniquePtr<Reader>;
        #[doc = "Set the transion of ALL Roots (if theReadOnlyVisible is False) or of Visible Roots (if theReadOnlyVisible is True)"]
        #[cxx_name = "SetReadVisible"]
        fn set_read_visible(self: Pin<&mut Reader>, ReadRoot: bool);
        #[cxx_name = "GetReadVisible"]
        fn get_read_visible(self: &Reader) -> bool;
        #[doc = "Determines the list of root entities from Model which are candidate for a transfer to a Shape (type of entities is PRODUCT) <theReadOnlyVisible> is taken into account to define roots"]
        #[cxx_name = "NbRootsForTransfer"]
        fn nb_roots_for_transfer(self: Pin<&mut Reader>) -> i32;
        #[doc = "Returns the model as a IGESModel. It can then be consulted (header, product)"]
        #[cxx_name = "IGESControl_Reader_IGESModel"]
        fn Reader_iges_model(self_: &Reader) -> UniquePtr<HandleIGESDataIGESModel>;
        #[doc = " ======================== IGESControl_Writer ========================"]
        #[doc = "This class creates and writes IGES files from CAS.CADE models. An IGES file can be written to an existing IGES file or to a new one. The translation can be performed in one or several operations. Each translation operation outputs a distinct root entity in the IGES file. To write an IGES file it is possible to use the following sequence: To modify the IGES file header or to change translation parameters it is necessary to use class Interface_Static (see IGESParameters and GeneralParameters)."]
        #[cxx_name = "IGESControl_Writer"]
        type Writer;
        #[doc = "Creates a writer object with the default unit (millimeters) and write mode (Face). IGESControl_Writer (const Standard_CString unit, const Standard_Integer modecr = 0);"]
        #[cxx_name = "IGESControl_Writer_ctor"]
        fn Writer_ctor() -> UniquePtr<Writer>;
        #[doc = "Creates a writer with given values for units and for write mode. theUnit may be any unit that is accepted by the IGES standard. By default, it is the millimeter. theModecr defines the write mode and may be: - 0: Faces (default) - 1: BRep."]
        #[cxx_name = "IGESControl_Writer_ctor_charptr_int"]
        fn Writer_ctor_charptr_int(theUnit: &str, theModecr: i32) -> UniquePtr<Writer>;
        #[doc = "Creates a writer object with the prepared IGES model theModel in write mode. theModecr defines the write mode and may be: - 0: Faces (default) - 1: BRep."]
        #[cxx_name = "IGESControl_Writer_ctor_handleigesmodel_int"]
        fn Writer_ctor_handleigesmodel_int(
            theModel: &HandleIGESDataIGESModel,
            theModecr: i32,
        ) -> UniquePtr<Writer>;
        #[doc = "Returns the IGES model to be written in output."]
        #[cxx_name = "Model"]
        fn model(self: &Writer) -> &HandleIGESDataIGESModel;
        #[cxx_name = "TransferProcess"]
        fn transfer_process(self: &Writer) -> &HandleTransferFinderProcess;
        #[doc = "Returns/Sets the TransferProcess : it contains final results and if some, check messages"]
        #[cxx_name = "SetTransferProcess"]
        fn set_transfer_process(self: Pin<&mut Writer>, TP: &HandleTransferFinderProcess);
        #[doc = "Translates a Shape to IGES Entities and adds them to the model Returns True if done, False if Shape not suitable for IGES or null"]
        #[cxx_name = "AddShape"]
        fn add_shape(
            self: Pin<&mut Writer>,
            sh: &TopoDS_Shape,
            theProgress: &Message_ProgressRange,
        ) -> bool;
        #[doc = "Translates a Geometry (Surface or Curve) to IGES Entities and adds them to the model Returns True if done, False if geom is neither a Surface or a Curve suitable for IGES or is null"]
        #[cxx_name = "AddGeom"]
        fn add_geom(self: Pin<&mut Writer>, geom: &HandleStandardTransient) -> bool;
        #[doc = "Adds an IGES entity (and the ones it references) to the model"]
        #[cxx_name = "AddEntity"]
        fn add_entity(self: Pin<&mut Writer>, ent: &HandleIGESDataIGESEntity) -> bool;
        #[doc = "Computes the entities found in the model, which is ready to be written. This contrasts with the default computation of headers only."]
        #[cxx_name = "ComputeModel"]
        fn compute_model(self: Pin<&mut Writer>);
        #[doc = "Prepares and writes an IGES model either to an OStream, S or to a file name,CString. Returns True if the operation was performed correctly and False if an error occurred (for instance, if the processor could not create the file)."]
        #[cxx_name = "IGESControl_Writer_Write"]
        fn Writer_write(self_: Pin<&mut Writer>, file: &str, fnes: bool) -> bool;
    }
    impl UniquePtr<Reader> {}
    impl UniquePtr<Writer> {}
}
pub use ffi::Reader;
impl Reader {
    #[doc = "Creates a Reader from scratch"]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Reader_ctor()
    }

    #[doc = "Creates a Reader from an already existing Session"]
    pub fn new_handleworksession_bool(
        WS: &ffi::HandleXSControlWorkSession,
        scratch: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::Reader_ctor_handleworksession_bool(WS, scratch)
    }
}
pub use ffi::Writer;
impl Writer {
    #[doc = "Creates a writer object with the default unit (millimeters) and write mode (Face). IGESControl_Writer (const Standard_CString unit, const Standard_Integer modecr = 0);"]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Writer_ctor()
    }

    #[doc = "Creates a writer with given values for units and for write mode. theUnit may be any unit that is accepted by the IGES standard. By default, it is the millimeter. theModecr defines the write mode and may be: - 0: Faces (default) - 1: BRep."]
    pub fn new_charptr_int(theUnit: &str, theModecr: i32) -> cxx::UniquePtr<Self> {
        ffi::Writer_ctor_charptr_int(theUnit, theModecr)
    }

    #[doc = "Creates a writer object with the prepared IGES model theModel in write mode. theModecr defines the write mode and may be: - 0: Faces (default) - 1: BRep."]
    pub fn new_handleigesmodel_int(
        theModel: &ffi::HandleIGESDataIGESModel,
        theModecr: i32,
    ) -> cxx::UniquePtr<Self> {
        ffi::Writer_ctor_handleigesmodel_int(theModel, theModecr)
    }
}
