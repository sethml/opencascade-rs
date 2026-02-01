#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_b_rep_offset_api.hxx");
        #[doc = "MakeEdge from b_rep_builder_api module"]
        type BRepBuilderAPI_MakeEdge = crate::b_rep_builder_api::ffi::MakeEdge;
        #[doc = "MakeFace from b_rep_builder_api module"]
        type BRepBuilderAPI_MakeFace = crate::b_rep_builder_api::ffi::MakeFace;
        #[doc = "MakeSolid from b_rep_builder_api module"]
        type BRepBuilderAPI_MakeSolid = crate::b_rep_builder_api::ffi::MakeSolid;
        #[doc = "MakeVertex from b_rep_builder_api module"]
        type BRepBuilderAPI_MakeVertex = crate::b_rep_builder_api::ffi::MakeVertex;
        #[doc = "MakeWire from b_rep_builder_api module"]
        type BRepBuilderAPI_MakeWire = crate::b_rep_builder_api::ffi::MakeWire;
        #[doc = "Sewing from b_rep_builder_api module"]
        type BRepBuilderAPI_Sewing = crate::b_rep_builder_api::ffi::Sewing;
        #[doc = "Transform from b_rep_builder_api module"]
        type BRepBuilderAPI_Transform = crate::b_rep_builder_api::ffi::Transform;
        #[doc = "FormatVersion from top_tools module"]
        type TopTools_FormatVersion = crate::top_tools::ffi::TopTools_FormatVersion;
        #[doc = "Pnt from gp module"]
        type gp_Pnt = crate::gp::ffi::Pnt;
        #[doc = "Pnt2d from gp module"]
        type gp_Pnt2d = crate::gp::ffi::Pnt2d;
        #[doc = "Vec from gp module"]
        type gp_Vec = crate::gp::ffi::Vec_;
        #[doc = "Vec2d from gp module"]
        type gp_Vec2d = crate::gp::ffi::Vec2d;
        #[doc = "Dir from gp module"]
        type gp_Dir = crate::gp::ffi::Dir;
        #[doc = "Dir2d from gp module"]
        type gp_Dir2d = crate::gp::ffi::Dir2d;
        #[doc = "XYZ from gp module"]
        type gp_XYZ = crate::gp::ffi::XYZ;
        #[doc = "Ax1 from gp module"]
        type gp_Ax1 = crate::gp::ffi::Ax1;
        #[doc = "Ax2 from gp module"]
        type gp_Ax2 = crate::gp::ffi::Ax2;
        #[doc = "Ax2d from gp module"]
        type gp_Ax2d = crate::gp::ffi::Ax2d;
        #[doc = "Ax3 from gp module"]
        type gp_Ax3 = crate::gp::ffi::Ax3;
        #[doc = "Trsf from gp module"]
        type gp_Trsf = crate::gp::ffi::Trsf;
        #[doc = "Trsf2d from gp module"]
        type gp_Trsf2d = crate::gp::ffi::Trsf2d;
        #[doc = "GTrsf from gp module"]
        type gp_GTrsf = crate::gp::ffi::GTrsf;
        #[doc = "GTrsf2d from gp module"]
        type gp_GTrsf2d = crate::gp::ffi::GTrsf2d;
        #[doc = "Lin from gp module"]
        type gp_Lin = crate::gp::ffi::Lin;
        #[doc = "Circ from gp module"]
        type gp_Circ = crate::gp::ffi::Circ;
        #[doc = "Pln from gp module"]
        type gp_Pln = crate::gp::ffi::Pln;
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
        #[doc = "Function from law module"]
        type Law_Function = crate::law::ffi::Function;
        #[doc = "Interpol from law module"]
        type Law_Interpol = crate::law::ffi::Interpol;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "BRepFill_TypeOfContact"]
        type BRepFill_TypeOfContact;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TopTools_ListOfShape"]
        type TopTools_ListOfShape;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "BRepBuilderAPI_TransitionMode"]
        type BRepBuilderAPI_TransitionMode;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "GeomAbs_Shape"]
        type GeomAbs_Shape;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "BRepOffset_Mode"]
        type BRepOffset_Mode;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "BRepFill_ThruSectionErrorStatus"]
        type BRepFill_ThruSectionErrorStatus;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "BRepFill_Pipe"]
        type BRepFill_Pipe;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "GeomFill_Trihedron"]
        type GeomFill_Trihedron;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "BRepBuilderAPI_PipeError"]
        type BRepBuilderAPI_PipeError;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "GeomAbs_JoinType"]
        type GeomAbs_JoinType;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Approx_ParametrizationType"]
        type Approx_ParametrizationType;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleLawFunction"]
        type HandleLawFunction;
        #[doc = " ======================== BRepOffsetAPI_MakeOffset ========================"]
        #[doc = "Describes algorithms for offsetting wires from a set of wires contained in a planar face. A MakeOffset object provides a framework for: - defining the construction of an offset, - implementing the construction algorithm, and - consulting the result."]
        #[cxx_name = "BRepOffsetAPI_MakeOffset"]
        type MakeOffset;
        #[doc = "Constructs an algorithm for creating an empty offset"]
        #[cxx_name = "BRepOffsetAPI_MakeOffset_ctor"]
        fn MakeOffset_ctor() -> UniquePtr<MakeOffset>;
        #[doc = "Set approximation flag for conversion input contours into ones consisting of 2D circular arcs and 2D linear segments only."]
        #[cxx_name = "SetApprox"]
        fn set_approx(self: Pin<&mut MakeOffset>, ToApprox: bool);
        #[doc = "Initializes the algorithm to construct parallels to the wire Spine."]
        #[cxx_name = "AddWire"]
        fn add_wire(self: Pin<&mut MakeOffset>, Spine: &TopoDS_Wire);
        #[doc = "Computes a parallel to the spine at distance Offset and at an altitude Alt from the plane of the spine in relation to the normal to the spine. Exceptions: StdFail_NotDone if the offset is not built."]
        #[cxx_name = "Perform"]
        fn perform(self: Pin<&mut MakeOffset>, Offset: f64, Alt: f64);
        #[doc = "Builds the resulting shape (redefined from MakeShape)."]
        #[cxx_name = "Build"]
        fn build(self: Pin<&mut MakeOffset>, theRange: &Message_ProgressRange);
        #[doc = "returns a list of the created shapes from the shape <S>."]
        #[cxx_name = "Generated"]
        fn generated(self: Pin<&mut MakeOffset>, S: &TopoDS_Shape) -> &TopTools_ListOfShape;
        #[doc = "Converts each wire of the face into contour consisting only of arcs and segments. New 3D curves are built too."]
        #[cxx_name = "BRepOffsetAPI_MakeOffset_ConvertFace"]
        fn MakeOffset_convert_face(
            theFace: &TopoDS_Face,
            theAngleTolerance: f64,
        ) -> UniquePtr<TopoDS_Face>;
        #[doc = " ======================== BRepOffsetAPI_MakePipe ========================"]
        #[doc = "Describes functions to build pipes. A pipe is built a basis shape (called the profile) along a wire (called the spine) by sweeping. The profile must not contain solids. A MakePipe object provides a framework for: - defining the construction of a pipe, - implementing the construction algorithm, and - consulting the result. Warning The MakePipe class implements pipe constructions with G1 continuous spines only."]
        #[cxx_name = "BRepOffsetAPI_MakePipe"]
        type MakePipe;
        #[doc = "Constructs a pipe by sweeping the shape Profile along the wire Spine.The angle made by the spine with the profile is maintained along the length of the pipe. Warning Spine must be G1 continuous; that is, on the connection vertex of two edges of the wire, the tangent vectors on the left and on the right must have the same direction, though not necessarily the same magnitude. Exceptions Standard_DomainError if the profile is a solid or a composite solid."]
        #[cxx_name = "BRepOffsetAPI_MakePipe_ctor_wire_shape"]
        fn MakePipe_ctor_wire_shape(
            Spine: &TopoDS_Wire,
            Profile: &TopoDS_Shape,
        ) -> UniquePtr<MakePipe>;
        #[cxx_name = "Pipe"]
        fn pipe(self: &MakePipe) -> &BRepFill_Pipe;
        #[doc = "Builds the resulting shape (redefined from MakeShape)."]
        #[cxx_name = "Build"]
        fn build(self: Pin<&mut MakePipe>, theRange: &Message_ProgressRange);
        #[cxx_name = "Generated"]
        fn generatedshape(self: Pin<&mut MakePipe>, S: &TopoDS_Shape) -> &TopTools_ListOfShape;
        #[cxx_name = "ErrorOnSurface"]
        fn error_on_surface(self: &MakePipe) -> f64;
        #[doc = "Returns the  TopoDS  Shape of the bottom of the prism."]
        #[cxx_name = "BRepOffsetAPI_MakePipe_FirstShape"]
        fn MakePipe_first_shape(self_: Pin<&mut MakePipe>) -> UniquePtr<TopoDS_Shape>;
        #[doc = "Returns the TopoDS Shape of the top of the prism."]
        #[cxx_name = "BRepOffsetAPI_MakePipe_LastShape"]
        fn MakePipe_last_shape(self_: Pin<&mut MakePipe>) -> UniquePtr<TopoDS_Shape>;
        #[cxx_name = "BRepOffsetAPI_MakePipe_Generated"]
        fn MakePipe_generated(
            self_: Pin<&mut MakePipe>,
            SSpine: &TopoDS_Shape,
            SProfile: &TopoDS_Shape,
        ) -> UniquePtr<TopoDS_Shape>;
        #[doc = " ======================== BRepOffsetAPI_MakePipeShell ========================"]
        #[doc = "This class provides for a framework to construct a shell or a solid along a spine consisting in a wire. To produce a solid, the initial wire must be closed. Two approaches are used: - definition by section - by a section and a scaling law - by addition of successive intermediary sections - definition by sweep mode. - pseudo-Frenet - constant - binormal constant - normal defined by a surface support - normal defined by a guiding contour. The two global approaches can also be combined. You can also close the surface later in order to form a solid. Warning: some limitations exist -- Mode with auxiliary spine is incompatible with hometetic laws -- Mode with auxiliary spine and keep contact produce only CO surface."]
        #[cxx_name = "BRepOffsetAPI_MakePipeShell"]
        type MakePipeShell;
        #[doc = "Constructs the shell-generating framework defined by the wire Spine. Sets an sweep's mode If no mode are set, the mode use in MakePipe is used"]
        #[cxx_name = "BRepOffsetAPI_MakePipeShell_ctor_wire"]
        fn MakePipeShell_ctor_wire(Spine: &TopoDS_Wire) -> UniquePtr<MakePipeShell>;
        #[doc = "Sets a Frenet or a CorrectedFrenet trihedron to  perform  the  sweeping If IsFrenet is false, a corrected Frenet trihedron is used."]
        #[cxx_name = "SetMode"]
        fn set_modebool(self: Pin<&mut MakePipeShell>, IsFrenet: bool);
        #[doc = "Sets a Discrete trihedron to  perform  the  sweeping"]
        #[cxx_name = "SetDiscreteMode"]
        fn set_discrete_mode(self: Pin<&mut MakePipeShell>);
        #[doc = "Sets  a  fixed  trihedron  to  perform  the  sweeping all sections will be parallel."]
        #[cxx_name = "SetMode"]
        fn set_modeax2_2(self: Pin<&mut MakePipeShell>, Axe: &gp_Ax2);
        #[doc = "Sets a fixed BiNormal  direction to perform the -- sweeping.   Angular   relations   between  the section(s) and <BiNormal> will be constant"]
        #[cxx_name = "SetMode"]
        fn set_modedir_3(self: Pin<&mut MakePipeShell>, BiNormal: &gp_Dir);
        #[doc = "Sets support to the spine to define the BiNormal of the trihedron, like the normal  to the surfaces. Warning:  To be effective, Each  edge of the <spine> must have a representation on one face of<SpineSupport>"]
        #[cxx_name = "SetMode"]
        fn set_modeshape_4(self: Pin<&mut MakePipeShell>, SpineSupport: &TopoDS_Shape) -> bool;
        #[doc = "Adds the section Profile to this framework. First and last sections may be punctual, so the shape Profile may be both wire and vertex. Correspondent point on spine is computed automatically. If WithContact is true, the section is translated to be in contact with the spine. If WithCorrection is true, the section is rotated to be orthogonal to the spine?s tangent in the correspondent point. This option has no sense if the section is punctual (Profile is of type TopoDS_Vertex)."]
        #[cxx_name = "Add"]
        fn addshape(
            self: Pin<&mut MakePipeShell>,
            Profile: &TopoDS_Shape,
            WithContact: bool,
            WithCorrection: bool,
        );
        #[doc = "Adds the section Profile to this framework. Correspondent point on the spine is given by Location. Warning: To be effective, it is not recommended to combine methods Add and SetLaw."]
        #[cxx_name = "Add"]
        fn addshape_2(
            self: Pin<&mut MakePipeShell>,
            Profile: &TopoDS_Shape,
            Location: &TopoDS_Vertex,
            WithContact: bool,
            WithCorrection: bool,
        );
        #[doc = "Sets the evolution law defined by the wire Profile with its position (Location, WithContact, WithCorrection are the same options as in methods Add) and a homotetic law defined by the function L. Warning: To be effective, it is not recommended to combine methods Add and SetLaw."]
        #[cxx_name = "SetLaw"]
        fn set_lawshape(
            self: Pin<&mut MakePipeShell>,
            Profile: &TopoDS_Shape,
            L: &HandleLawFunction,
            WithContact: bool,
            WithCorrection: bool,
        );
        #[doc = "Sets the evolution law defined by the wire Profile with its position (Location, WithContact, WithCorrection are the same options as in methods Add) and a homotetic law defined by the function L. Warning: To be effective, it is not recommended to combine methods Add and SetLaw."]
        #[cxx_name = "SetLaw"]
        fn set_lawshape_2(
            self: Pin<&mut MakePipeShell>,
            Profile: &TopoDS_Shape,
            L: &HandleLawFunction,
            Location: &TopoDS_Vertex,
            WithContact: bool,
            WithCorrection: bool,
        );
        #[doc = "Removes the section Profile from this framework."]
        #[cxx_name = "Delete"]
        fn delete(self: Pin<&mut MakePipeShell>, Profile: &TopoDS_Shape);
        #[doc = "Returns true if this tool object is ready to build the shape, i.e. has a definition for the wire section Profile."]
        #[cxx_name = "IsReady"]
        fn is_ready(self: &MakePipeShell) -> bool;
        #[doc = "Sets the following tolerance values - 3D tolerance Tol3d - boundary tolerance BoundTol - angular tolerance TolAngular."]
        #[cxx_name = "SetTolerance"]
        fn set_tolerance(self: Pin<&mut MakePipeShell>, Tol3d: f64, BoundTol: f64, TolAngular: f64);
        #[doc = "Define the maximum V degree of resulting surface"]
        #[cxx_name = "SetMaxDegree"]
        fn set_max_degree(self: Pin<&mut MakePipeShell>, NewMaxDegree: i32);
        #[doc = "Define the maximum number of spans in V-direction on resulting surface"]
        #[cxx_name = "SetMaxSegments"]
        fn set_max_segments(self: Pin<&mut MakePipeShell>, NewMaxSegments: i32);
        #[doc = "Set the flag that indicates attempt to approximate a C1-continuous surface if a swept surface proved to be C0."]
        #[cxx_name = "SetForceApproxC1"]
        fn set_force_approx_c1(self: Pin<&mut MakePipeShell>, ForceApproxC1: bool);
        #[doc = "Simulates the resulting shape by calculating its cross-sections. The spine is divided by this cross-sections into (NumberOfSection - 1) equal parts, the number of cross-sections is NumberOfSection. The cross-sections are wires and they are returned in the list Result. This gives a rapid preview of the resulting shape, which will be obtained using the settings you have provided. Raises  NotDone if  <me> it is not Ready"]
        #[cxx_name = "Simulate"]
        fn simulate(
            self: Pin<&mut MakePipeShell>,
            NumberOfSection: i32,
            Result: Pin<&mut TopTools_ListOfShape>,
        );
        #[doc = "Builds the resulting shape (redefined from MakeShape)."]
        #[cxx_name = "Build"]
        fn build(self: Pin<&mut MakePipeShell>, theRange: &Message_ProgressRange);
        #[doc = "Transforms the sweeping Shell in Solid. If a propfile is not closed returns False"]
        #[cxx_name = "MakeSolid"]
        fn make_solid(self: Pin<&mut MakePipeShell>) -> bool;
        #[doc = "Returns a list of new shapes generated from the shape S by the shell-generating algorithm. This function is redefined from BRepOffsetAPI_MakeShape::Generated. S can be an edge or a vertex of a given Profile (see methods Add)."]
        #[cxx_name = "Generated"]
        fn generated(self: Pin<&mut MakePipeShell>, S: &TopoDS_Shape) -> &TopTools_ListOfShape;
        #[cxx_name = "ErrorOnSurface"]
        fn error_on_surface(self: &MakePipeShell) -> f64;
        #[doc = "Returns the list of original profiles"]
        #[cxx_name = "Profiles"]
        fn profiles(self: Pin<&mut MakePipeShell>, theProfiles: Pin<&mut TopTools_ListOfShape>);
        #[doc = "Returns the spine"]
        #[cxx_name = "Spine"]
        fn spine(self: Pin<&mut MakePipeShell>) -> &TopoDS_Wire;
        #[doc = "Get a status, when Simulate or Build failed.       It can be BRepBuilderAPI_PipeDone, BRepBuilderAPI_PipeNotDone, BRepBuilderAPI_PlaneNotIntersectGuide, BRepBuilderAPI_ImpossibleContact."]
        #[cxx_name = "BRepOffsetAPI_MakePipeShell_GetStatus"]
        fn MakePipeShell_get_status(self_: &MakePipeShell) -> UniquePtr<BRepBuilderAPI_PipeError>;
        #[doc = "Returns the  TopoDS  Shape of the bottom of the sweep."]
        #[cxx_name = "BRepOffsetAPI_MakePipeShell_FirstShape"]
        fn MakePipeShell_first_shape(self_: Pin<&mut MakePipeShell>) -> UniquePtr<TopoDS_Shape>;
        #[doc = "Returns the TopoDS Shape of the top of the sweep."]
        #[cxx_name = "BRepOffsetAPI_MakePipeShell_LastShape"]
        fn MakePipeShell_last_shape(self_: Pin<&mut MakePipeShell>) -> UniquePtr<TopoDS_Shape>;
        #[doc = " ======================== BRepOffsetAPI_MakeThickSolid ========================"]
        #[doc = "Describes functions to build hollowed solids. A hollowed solid is built from an initial solid and a set of faces on this solid, which are to be removed. The remaining faces of the solid become the walls of the hollowed solid, their thickness defined at the time of construction. the solid is built from an initial solid  <S> and a  set of  faces {Fi} from  <S>, builds a   solid  composed  by two shells closed  by the {Fi}. First shell <SS>   is composed by all the faces of <S> expected {Fi}.  Second shell is the offset shell of <SS>. A MakeThickSolid object provides a framework for: - defining the cross-section of a hollowed solid, - implementing the construction algorithm, and - consulting the result."]
        #[cxx_name = "BRepOffsetAPI_MakeThickSolid"]
        type MakeThickSolid;
        #[doc = "Constructor does nothing."]
        #[cxx_name = "BRepOffsetAPI_MakeThickSolid_ctor"]
        fn MakeThickSolid_ctor() -> UniquePtr<MakeThickSolid>;
        #[doc = "Constructs solid using simple algorithm. According to its nature it is not possible to set list of the closing faces. This algorithm does not support faces removing. It is caused by fact that intersections are not computed during offset creation. Non-closed shell or face is expected as input."]
        #[cxx_name = "MakeThickSolidBySimple"]
        fn make_thick_solid_by_simple(
            self: Pin<&mut MakeThickSolid>,
            theS: &TopoDS_Shape,
            theOffsetValue: f64,
        );
        #[cxx_name = "Build"]
        fn build(self: Pin<&mut MakeThickSolid>, theRange: &Message_ProgressRange);
        #[doc = "Returns the list  of shapes modified from the shape <S>."]
        #[cxx_name = "Modified"]
        fn modified(self: Pin<&mut MakeThickSolid>, S: &TopoDS_Shape) -> &TopTools_ListOfShape;
        #[doc = " ======================== BRepOffsetAPI_ThruSections ========================"]
        #[doc = "Describes functions to build a loft. This is a shell or a solid passing through a set of sections in a given sequence. Usually sections are wires, but the first and the last sections may be vertices (punctual sections)."]
        #[cxx_name = "BRepOffsetAPI_ThruSections"]
        type ThruSections;
        #[doc = "Initializes an algorithm for building a shell or a solid passing through a set of sections, where: -          isSolid is set to true if the construction algorithm is required to build a solid or to false if it is required to build a shell (the default value), -          ruled is set to true if the faces generated between the edges of two consecutive wires are ruled surfaces or to false (the default value) if they are smoothed out by approximation, -          pres3d defines the precision criterion used by the approximation algorithm; the default value is 1.0e-6. Use AddWire and AddVertex to define the successive sections of the shell or solid to be built."]
        #[cxx_name = "BRepOffsetAPI_ThruSections_ctor_bool2_real"]
        fn ThruSections_ctor_bool2_real(
            isSolid: bool,
            ruled: bool,
            pres3d: f64,
        ) -> UniquePtr<ThruSections>;
        #[doc = "Initializes this algorithm for building a shell or a solid passing through a set of sections, where: - isSolid is set to true if this construction algorithm is required to build a solid or to false if it is required to build a shell. false is the default value; - ruled is set to true if the faces generated between the edges of two consecutive wires are ruled surfaces or to false (the default value) if they are smoothed out by approximation, - pres3d defines the precision criterion used by the approximation algorithm; the default value is 1.0e-6. Use AddWire and AddVertex to define the successive sections of the shell or solid to be built."]
        #[cxx_name = "Init"]
        fn init(self: Pin<&mut ThruSections>, isSolid: bool, ruled: bool, pres3d: f64);
        #[doc = "Adds the wire wire to the set of sections through which the shell or solid is built. Use the Build function to construct the shape."]
        #[cxx_name = "AddWire"]
        fn add_wire(self: Pin<&mut ThruSections>, wire: &TopoDS_Wire);
        #[doc = "Adds the vertex Vertex (punctual section) to the set of sections through which the shell or solid is built. A vertex may be added to the set of sections only as first or last section. At least one wire must be added to the set of sections by the method AddWire. Use the Build function to construct the shape."]
        #[cxx_name = "AddVertex"]
        fn add_vertex(self: Pin<&mut ThruSections>, aVertex: &TopoDS_Vertex);
        #[doc = "Sets/unsets the option to compute origin and orientation on wires to avoid twisted results and update wires to have same number of edges."]
        #[cxx_name = "CheckCompatibility"]
        fn check_compatibility(self: Pin<&mut ThruSections>, check: bool);
        #[doc = "Define the approximation algorithm"]
        #[cxx_name = "SetSmoothing"]
        fn set_smoothing(self: Pin<&mut ThruSections>, UseSmoothing: bool);
        #[doc = "define the Weights  associed to the criterium used in the  optimization. if Wi <= 0"]
        #[cxx_name = "SetCriteriumWeight"]
        fn set_criterium_weight(self: Pin<&mut ThruSections>, W1: f64, W2: f64, W3: f64);
        #[doc = "Define the maximal U degree of result surface"]
        #[cxx_name = "SetMaxDegree"]
        fn set_max_degree(self: Pin<&mut ThruSections>, MaxDeg: i32);
        #[doc = "returns the maximal U degree of result surface"]
        #[cxx_name = "MaxDegree"]
        fn max_degree(self: &ThruSections) -> i32;
        #[doc = "Define the approximation algorithm"]
        #[cxx_name = "UseSmoothing"]
        fn use_smoothing(self: &ThruSections) -> bool;
        #[doc = "returns the Weights associed  to the criterium used in the  optimization."]
        #[cxx_name = "CriteriumWeight"]
        fn criterium_weight(self: &ThruSections, W1: &mut f64, W2: &mut f64, W3: &mut f64);
        #[cxx_name = "Build"]
        fn build(self: Pin<&mut ThruSections>, theRange: &Message_ProgressRange);
        #[doc = "Returns the TopoDS Shape of the bottom of the loft if solid"]
        #[cxx_name = "FirstShape"]
        fn first_shape(self: &ThruSections) -> &TopoDS_Shape;
        #[doc = "Returns the TopoDS Shape of the top of the loft if solid"]
        #[cxx_name = "LastShape"]
        fn last_shape(self: &ThruSections) -> &TopoDS_Shape;
        #[doc = "Sets the mutable input state. If true then the input profile can be modified inside the thrusection operation. Default value is true."]
        #[cxx_name = "SetMutableInput"]
        fn set_mutable_input(self: Pin<&mut ThruSections>, theIsMutableInput: bool);
        #[doc = "Returns a list of new shapes generated from the shape S by the shell-generating algorithm. This function is redefined from BRepBuilderAPI_MakeShape::Generated. S can be an edge or a vertex of a given Profile (see methods AddWire and AddVertex)."]
        #[cxx_name = "Generated"]
        fn generated(self: Pin<&mut ThruSections>, S: &TopoDS_Shape) -> &TopTools_ListOfShape;
        #[doc = "Returns the list of original wires"]
        #[cxx_name = "Wires"]
        fn wires(self: &ThruSections) -> &TopTools_ListOfShape;
        #[doc = "Returns the current mutable input state"]
        #[cxx_name = "IsMutableInput"]
        fn is_mutable_input(self: &ThruSections) -> bool;
        #[doc = "returns the type of parametrization used in the approximation"]
        #[cxx_name = "BRepOffsetAPI_ThruSections_ParType"]
        fn ThruSections_par_type(self_: &ThruSections) -> UniquePtr<Approx_ParametrizationType>;
        #[doc = "returns the Continuity used in the approximation"]
        #[cxx_name = "BRepOffsetAPI_ThruSections_Continuity"]
        fn ThruSections_continuity(self_: &ThruSections) -> UniquePtr<GeomAbs_Shape>;
        #[doc = "if Ruled Returns the Face generated by each edge except the last wire if smoothed Returns the Face generated by each edge of the first wire"]
        #[cxx_name = "BRepOffsetAPI_ThruSections_GeneratedFace"]
        fn ThruSections_generated_face(
            self_: &ThruSections,
            Edge: &TopoDS_Shape,
        ) -> UniquePtr<TopoDS_Shape>;
        #[doc = "Returns the status of thrusection operation"]
        #[cxx_name = "BRepOffsetAPI_ThruSections_GetStatus"]
        fn ThruSections_get_status(
            self_: &ThruSections,
        ) -> UniquePtr<BRepFill_ThruSectionErrorStatus>;
    }
    impl UniquePtr<MakeOffset> {}
    impl UniquePtr<MakePipe> {}
    impl UniquePtr<MakePipeShell> {}
    impl UniquePtr<MakeThickSolid> {}
    impl UniquePtr<ThruSections> {}
}
pub use ffi::MakeOffset;
impl MakeOffset {
    #[doc = "Constructs an algorithm for creating an empty offset"]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::MakeOffset_ctor()
    }
}
pub use ffi::MakePipe;
impl MakePipe {
    #[doc = "Constructs a pipe by sweeping the shape Profile along the wire Spine.The angle made by the spine with the profile is maintained along the length of the pipe. Warning Spine must be G1 continuous; that is, on the connection vertex of two edges of the wire, the tangent vectors on the left and on the right must have the same direction, though not necessarily the same magnitude. Exceptions Standard_DomainError if the profile is a solid or a composite solid."]
    pub fn new_wire_shape(
        Spine: &ffi::TopoDS_Wire,
        Profile: &ffi::TopoDS_Shape,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakePipe_ctor_wire_shape(Spine, Profile)
    }
}
pub use ffi::MakePipeShell;
impl MakePipeShell {
    #[doc = "Constructs the shell-generating framework defined by the wire Spine. Sets an sweep's mode If no mode are set, the mode use in MakePipe is used"]
    pub fn new_wire(Spine: &ffi::TopoDS_Wire) -> cxx::UniquePtr<Self> {
        ffi::MakePipeShell_ctor_wire(Spine)
    }
}
pub use ffi::MakeThickSolid;
impl MakeThickSolid {
    #[doc = "Constructor does nothing."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::MakeThickSolid_ctor()
    }
}
pub use ffi::ThruSections;
impl ThruSections {
    #[doc = "Initializes an algorithm for building a shell or a solid passing through a set of sections, where: -          isSolid is set to true if the construction algorithm is required to build a solid or to false if it is required to build a shell (the default value), -          ruled is set to true if the faces generated between the edges of two consecutive wires are ruled surfaces or to false (the default value) if they are smoothed out by approximation, -          pres3d defines the precision criterion used by the approximation algorithm; the default value is 1.0e-6. Use AddWire and AddVertex to define the successive sections of the shell or solid to be built."]
    pub fn new_bool2_real(isSolid: bool, ruled: bool, pres3d: f64) -> cxx::UniquePtr<Self> {
        ffi::ThruSections_ctor_bool2_real(isSolid, ruled, pres3d)
    }
}
