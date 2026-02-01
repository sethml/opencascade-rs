#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_step_control.hxx");
        #[doc = "ProgressRange from message module"]
        type Message_ProgressRange = crate::message::ffi::ProgressRange;
        #[doc = "ReturnStatus from if_select module"]
        type IFSelect_ReturnStatus = crate::if_select::ffi::IFSelect_ReturnStatus;
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
        #[cxx_name = "XSControl_WorkSession"]
        type XSControl_WorkSession;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "DE_ShapeFixParameters"]
        type DE_ShapeFixParameters;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "DESTEP_Parameters"]
        type DESTEP_Parameters;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TColStd_SequenceOfAsciiString"]
        type TColStd_SequenceOfAsciiString;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "StepData_StepModel"]
        type StepData_StepModel;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "STEPControl_StepModelType"]
        type STEPControl_StepModelType;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleXSControlWorkSession"]
        type HandleXSControlWorkSession;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleStepDataStepModel"]
        type HandleStepDataStepModel;
        #[doc = " ======================== STEPControl_Reader ========================"]
        #[doc = "Reads STEP files, checks them and translates their contents into Open CASCADE models. The STEP data can be that of a whole model or that of a specific list of entities in the model. As in XSControl_Reader, you specify the list using a selection. For the translation of iges files it is possible to use next sequence: To change translation parameters class Interface_Static should be used before beginning of translation  (see STEP Parameters and General Parameters) Creation of reader - STEPControl_Reader reader; To load s file in a model use method reader.ReadFile(\"filename.stp\") To print load results reader.PrintCheckLoad(failsonly,mode) where mode is equal to the value of enumeration IFSelect_PrintCount For definition number of candidates : Standard_Integer nbroots = reader. NbRootsForTransfer(); To transfer entities from a model the following methods can be used: for the whole model - reader.TransferRoots(); to transfer a list of entities: reader.TransferList(list); to transfer one entity Handle(Standard_Transient) ent = reader.RootForTransfer(num); reader.TransferEntity(ent), or reader.TransferOneRoot(num), or reader.TransferOne(num), or reader.TransferRoot(num) To obtain the result the following method can be used: reader.NbShapes() and reader.Shape(num); or reader.OneShape(); To print the results of transfer use method: reader.PrintCheckTransfer(failwarn,mode); where printfail is equal to the value of enumeration IFSelect_PrintFail, mode see above; or reader.PrintStatsTransfer(); Gets correspondence between a STEP entity and a result shape obtained from it. Handle(XSControl_WorkSession) WS = reader.WS(); if ( WS->TransferReader()->HasResult(ent) ) TopoDS_Shape shape = WS->TransferReader()->ShapeResult(ent);"]
        #[cxx_name = "STEPControl_Reader"]
        type Reader;
        #[doc = "Creates a reader object with an empty STEP model."]
        #[cxx_name = "STEPControl_Reader_ctor"]
        fn Reader_ctor() -> UniquePtr<Reader>;
        #[doc = "Creates a Reader for STEP from an already existing Session Clears the session if it was not yet set for STEP"]
        #[cxx_name = "STEPControl_Reader_ctor_handleworksession_bool"]
        fn Reader_ctor_handleworksession_bool(
            WS: &HandleXSControlWorkSession,
            scratch: bool,
        ) -> UniquePtr<Reader>;
        #[doc = "Transfers a root given its rank in the list of candidate roots Default is the first one Returns True if a shape has resulted, false else Same as inherited TransferOneRoot, kept for compatibility"]
        #[cxx_name = "TransferRoot"]
        fn transfer_root(
            self: Pin<&mut Reader>,
            num: i32,
            theProgress: &Message_ProgressRange,
        ) -> bool;
        #[doc = "Determines the list of root entities from Model which are candidate for a transfer to a Shape (type of entities is PRODUCT)"]
        #[cxx_name = "NbRootsForTransfer"]
        fn nb_roots_for_transfer(self: Pin<&mut Reader>) -> i32;
        #[doc = "Returns sequence of all unit names for shape representations found in file"]
        #[cxx_name = "FileUnits"]
        fn file_units(
            self: Pin<&mut Reader>,
            theUnitLengthNames: Pin<&mut TColStd_SequenceOfAsciiString>,
            theUnitAngleNames: Pin<&mut TColStd_SequenceOfAsciiString>,
            theUnitSolidAngleNames: Pin<&mut TColStd_SequenceOfAsciiString>,
        );
        #[doc = "Sets system length unit used by transfer process. Performs only if a model is not NULL"]
        #[cxx_name = "SetSystemLengthUnit"]
        fn set_system_length_unit(self: Pin<&mut Reader>, theLengthUnit: f64);
        #[doc = "Returns system length unit used by transfer process. Performs only if a model is not NULL"]
        #[cxx_name = "SystemLengthUnit"]
        fn system_length_unit(self: &Reader) -> f64;
        #[doc = "Returns the model as a StepModel. It can then be consulted (header, product)"]
        #[cxx_name = "STEPControl_Reader_StepModel"]
        fn Reader_step_model(self_: &Reader) -> UniquePtr<HandleStepDataStepModel>;
        #[doc = "Loads a file and returns the read status Zero for a Model which compies with the Controller"]
        #[cxx_name = "STEPControl_Reader_ReadFile"]
        fn Reader_read_filecharptr(
            self_: Pin<&mut Reader>,
            filename: &str,
        ) -> IFSelect_ReturnStatus;
        #[doc = "Loads a file and returns the read status Zero for a Model which compies with the Controller"]
        #[cxx_name = "STEPControl_Reader_ReadFile"]
        fn Reader_read_filecharptr_2(
            self_: Pin<&mut Reader>,
            filename: &str,
            theParams: &DESTEP_Parameters,
        ) -> IFSelect_ReturnStatus;
        #[doc = " ======================== STEPControl_Writer ========================"]
        #[doc = "This class creates and writes STEP files from Open CASCADE models. A STEP file can be written to an existing STEP file or to a new one. Translation can be performed in one or several operations. Each translation operation outputs a distinct root entity in the STEP file."]
        #[cxx_name = "STEPControl_Writer"]
        type Writer;
        #[doc = "Creates a Writer from scratch"]
        #[cxx_name = "STEPControl_Writer_ctor"]
        fn Writer_ctor() -> UniquePtr<Writer>;
        #[doc = "Creates a Writer from an already existing Session If <scratch> is True (D), clears already recorded data"]
        #[cxx_name = "STEPControl_Writer_ctor_handleworksession_bool"]
        fn Writer_ctor_handleworksession_bool(
            WS: &HandleXSControlWorkSession,
            scratch: bool,
        ) -> UniquePtr<Writer>;
        #[doc = "Sets a length-measure value that will be written to uncertainty-measure-with-unit when the next shape is translated."]
        #[cxx_name = "SetTolerance"]
        fn set_tolerance(self: Pin<&mut Writer>, Tol: f64);
        #[doc = "Unsets the tolerance formerly forced by SetTolerance"]
        #[cxx_name = "UnsetTolerance"]
        fn unset_tolerance(self: Pin<&mut Writer>);
        #[doc = "Sets a specific session to <me>"]
        #[cxx_name = "SetWS"]
        fn set_ws(self: Pin<&mut Writer>, WS: &HandleXSControlWorkSession, scratch: bool);
        #[doc = "Displays the statistics for the last translation. what defines the kind of statistics that are displayed: - 0 gives general statistics   (number of translated roots, number of warnings, number of   fail messages), - 1 gives root results, - 2 gives statistics for all checked entities, - 3 gives the list of translated entities, - 4 gives warning and fail messages, - 5 gives fail messages only. mode is used according to the use of what. If what is 0, mode is ignored. If what is 1, 2 or 3, mode defines the following: - 0 lists the numbers of STEP entities in a STEP model, - 1 gives the number, identifier, type and result type for each STEP entity and/or its status (fail, warning, etc.), - 2 gives maximum information for each STEP entity (i.e. checks), - 3 gives the number of entities by the type of a STEP entity, - 4 gives the number of of STEP entities per result type and/or status, - 5 gives the number of pairs (STEP or result type and status), - 6 gives the number of pairs (STEP or result type and status) AND the list of entity numbers in the STEP model."]
        #[cxx_name = "PrintStatsTransfer"]
        fn print_stats_transfer(self: &Writer, what: i32, mode: i32);
        #[doc = "Returns the session used in <me>"]
        #[cxx_name = "STEPControl_Writer_WS"]
        fn Writer_ws(self_: &Writer) -> UniquePtr<HandleXSControlWorkSession>;
        #[doc = "Returns the produced model. Produces a new one if not yet done or if <newone> is True This method allows for instance to edit product or header data before writing."]
        #[cxx_name = "STEPControl_Writer_Model"]
        fn Writer_model(
            self_: Pin<&mut Writer>,
            newone: bool,
        ) -> UniquePtr<HandleStepDataStepModel>;
        #[doc = "Writes a STEP model in the file identified by filename."]
        #[cxx_name = "STEPControl_Writer_Write"]
        fn Writer_write(self_: Pin<&mut Writer>, theFileName: &str) -> IFSelect_ReturnStatus;
    }
    impl UniquePtr<Reader> {}
    impl UniquePtr<Writer> {}
}
pub use ffi::Reader;
impl Reader {
    #[doc = "Creates a reader object with an empty STEP model."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Reader_ctor()
    }

    #[doc = "Creates a Reader for STEP from an already existing Session Clears the session if it was not yet set for STEP"]
    pub fn new_handleworksession_bool(
        WS: &ffi::HandleXSControlWorkSession,
        scratch: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::Reader_ctor_handleworksession_bool(WS, scratch)
    }
}
pub use ffi::Writer;
impl Writer {
    #[doc = "Creates a Writer from scratch"]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Writer_ctor()
    }

    #[doc = "Creates a Writer from an already existing Session If <scratch> is True (D), clears already recorded data"]
    pub fn new_handleworksession_bool(
        WS: &ffi::HandleXSControlWorkSession,
        scratch: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::Writer_ctor_handleworksession_bool(WS, scratch)
    }
}
