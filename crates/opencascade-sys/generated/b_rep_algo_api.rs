#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_b_rep_algo_api.hxx");
        #[doc = "BezierCurve from geom module"]
        type Geom_BezierCurve = crate::geom::ffi::BezierCurve;
        #[doc = "BSplineCurve from geom module"]
        type Geom_BSplineCurve = crate::geom::ffi::BSplineCurve;
        #[doc = "Curve from geom module"]
        type Geom_Curve = crate::geom::ffi::Curve;
        #[doc = "CylindricalSurface from geom module"]
        type Geom_CylindricalSurface = crate::geom::ffi::CylindricalSurface;
        #[doc = "Plane from geom module"]
        type Geom_Plane = crate::geom::ffi::Plane;
        #[doc = "Surface from geom module"]
        type Geom_Surface = crate::geom::ffi::Surface;
        #[doc = "TrimmedCurve from geom module"]
        type Geom_TrimmedCurve = crate::geom::ffi::TrimmedCurve;
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
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "BOPAlgo_PaveFiller"]
        type BOPAlgo_PaveFiller;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleGeomSurface"]
        type HandleGeomSurface;
        #[doc = " ======================== BRepAlgoAPI_Common ========================"]
        #[doc = "The class provides Boolean common operation between arguments and tools (Boolean Intersection)."]
        #[cxx_name = "BRepAlgoAPI_Common"]
        type Common;
        #[doc = "Empty constructor"]
        #[cxx_name = "BRepAlgoAPI_Common_ctor"]
        fn Common_ctor() -> UniquePtr<Common>;
        #[doc = "Empty constructor <PF> - PaveFiller object that is carried out"]
        #[cxx_name = "BRepAlgoAPI_Common_ctor_pavefiller"]
        fn Common_ctor_pavefiller(PF: &BOPAlgo_PaveFiller) -> UniquePtr<Common>;
        #[doc = "Constructor with two shapes <S1>  -argument <S2>  -tool <anOperation> - the type of the operation Obsolete"]
        #[cxx_name = "BRepAlgoAPI_Common_ctor_shape2_progressrange"]
        fn Common_ctor_shape2_progressrange(
            S1: &TopoDS_Shape,
            S2: &TopoDS_Shape,
            theRange: &Message_ProgressRange,
        ) -> UniquePtr<Common>;
        #[doc = "Constructor with two shapes <S1>  -argument <S2>  -tool <anOperation> - the type of the operation <PF> - PaveFiller object that is carried out Obsolete"]
        #[cxx_name = "BRepAlgoAPI_Common_ctor_shape2_pavefiller_progressrange"]
        fn Common_ctor_shape2_pavefiller_progressrange(
            S1: &TopoDS_Shape,
            S2: &TopoDS_Shape,
            PF: &BOPAlgo_PaveFiller,
            theRange: &Message_ProgressRange,
        ) -> UniquePtr<Common>;
        #[doc = " ======================== BRepAlgoAPI_Cut ========================"]
        #[doc = "The class Cut provides Boolean cut operation between arguments and tools (Boolean Subtraction)."]
        #[cxx_name = "BRepAlgoAPI_Cut"]
        type Cut;
        #[doc = "Empty constructor"]
        #[cxx_name = "BRepAlgoAPI_Cut_ctor"]
        fn Cut_ctor() -> UniquePtr<Cut>;
        #[doc = "Empty constructor <PF> - PaveFiller object that is carried out"]
        #[cxx_name = "BRepAlgoAPI_Cut_ctor_pavefiller"]
        fn Cut_ctor_pavefiller(PF: &BOPAlgo_PaveFiller) -> UniquePtr<Cut>;
        #[doc = "Constructor with two shapes <S1>  -argument <S2>  -tool <anOperation> - the type of the operation Obsolete"]
        #[cxx_name = "BRepAlgoAPI_Cut_ctor_shape2_progressrange"]
        fn Cut_ctor_shape2_progressrange(
            S1: &TopoDS_Shape,
            S2: &TopoDS_Shape,
            theRange: &Message_ProgressRange,
        ) -> UniquePtr<Cut>;
        #[doc = "Constructor with two shapes <S1>  -argument <S2>  -tool <anOperation> - the type of the operation <PF> - PaveFiller object that is carried out Obsolete"]
        #[cxx_name = "BRepAlgoAPI_Cut_ctor_shape2_pavefiller_bool_progressrange"]
        fn Cut_ctor_shape2_pavefiller_bool_progressrange(
            S1: &TopoDS_Shape,
            S2: &TopoDS_Shape,
            aDSF: &BOPAlgo_PaveFiller,
            bFWD: bool,
            theRange: &Message_ProgressRange,
        ) -> UniquePtr<Cut>;
        #[doc = " ======================== BRepAlgoAPI_Fuse ========================"]
        #[doc = "The class provides Boolean fusion operation between arguments and tools  (Boolean Union)."]
        #[cxx_name = "BRepAlgoAPI_Fuse"]
        type Fuse;
        #[doc = "Empty constructor"]
        #[cxx_name = "BRepAlgoAPI_Fuse_ctor"]
        fn Fuse_ctor() -> UniquePtr<Fuse>;
        #[doc = "Empty constructor <PF> - PaveFiller object that is carried out"]
        #[cxx_name = "BRepAlgoAPI_Fuse_ctor_pavefiller"]
        fn Fuse_ctor_pavefiller(PF: &BOPAlgo_PaveFiller) -> UniquePtr<Fuse>;
        #[doc = "Constructor with two shapes <S1>  -argument <S2>  -tool <anOperation> - the type of the operation Obsolete"]
        #[cxx_name = "BRepAlgoAPI_Fuse_ctor_shape2_progressrange"]
        fn Fuse_ctor_shape2_progressrange(
            S1: &TopoDS_Shape,
            S2: &TopoDS_Shape,
            theRange: &Message_ProgressRange,
        ) -> UniquePtr<Fuse>;
        #[doc = "Constructor with two shapes <S1>  -argument <S2>  -tool <anOperation> - the type of the operation <PF> - PaveFiller object that is carried out Obsolete"]
        #[cxx_name = "BRepAlgoAPI_Fuse_ctor_shape2_pavefiller_progressrange"]
        fn Fuse_ctor_shape2_pavefiller_progressrange(
            S1: &TopoDS_Shape,
            S2: &TopoDS_Shape,
            aDSF: &BOPAlgo_PaveFiller,
            theRange: &Message_ProgressRange,
        ) -> UniquePtr<Fuse>;
        #[doc = " ======================== BRepAlgoAPI_Section ========================"]
        #[doc = "The algorithm is to build a Section operation between arguments and tools. The result of Section operation consists of vertices and edges. The result of Section operation contains: 1. new vertices that are subjects of V/V, E/E, E/F, F/F interferences 2. vertices that are subjects of V/E, V/F interferences 3. new edges that are subjects of F/F interferences 4. edges that are Common Blocks"]
        #[cxx_name = "BRepAlgoAPI_Section"]
        type Section;
        #[doc = "Empty constructor"]
        #[cxx_name = "BRepAlgoAPI_Section_ctor"]
        fn Section_ctor() -> UniquePtr<Section>;
        #[doc = "Empty constructor <PF> - PaveFiller object that is carried out"]
        #[cxx_name = "BRepAlgoAPI_Section_ctor_pavefiller"]
        fn Section_ctor_pavefiller(PF: &BOPAlgo_PaveFiller) -> UniquePtr<Section>;
        #[doc = "Constructor with two shapes <S1>  -argument <S2>  -tool <PerformNow> - the flag: if <PerformNow>=True - the algorithm is performed immediately Obsolete"]
        #[cxx_name = "BRepAlgoAPI_Section_ctor_shape2_bool"]
        fn Section_ctor_shape2_bool(
            S1: &TopoDS_Shape,
            S2: &TopoDS_Shape,
            PerformNow: bool,
        ) -> UniquePtr<Section>;
        #[doc = "Constructor with two shapes <S1>  -argument <S2>  -tool <PF> - PaveFiller object that is carried out <PerformNow> - the flag: if <PerformNow>=True - the algorithm is performed immediately Obsolete"]
        #[cxx_name = "BRepAlgoAPI_Section_ctor_shape2_pavefiller_bool"]
        fn Section_ctor_shape2_pavefiller_bool(
            S1: &TopoDS_Shape,
            S2: &TopoDS_Shape,
            aDSF: &BOPAlgo_PaveFiller,
            PerformNow: bool,
        ) -> UniquePtr<Section>;
        #[doc = "Constructor with two shapes <S1>  - argument <Pl>  - tool <PerformNow> - the flag: if <PerformNow>=True - the algorithm is performed immediately Obsolete"]
        #[cxx_name = "BRepAlgoAPI_Section_ctor_shape_pln_bool"]
        fn Section_ctor_shape_pln_bool(
            S1: &TopoDS_Shape,
            Pl: &gp_Pln,
            PerformNow: bool,
        ) -> UniquePtr<Section>;
        #[doc = "Constructor with two shapes <S1>  - argument <Sf>  - tool <PerformNow> - the flag: if <PerformNow>=True - the algorithm is performed immediately Obsolete"]
        #[cxx_name = "BRepAlgoAPI_Section_ctor_shape_handlesurface_bool"]
        fn Section_ctor_shape_handlesurface_bool(
            S1: &TopoDS_Shape,
            Sf: &HandleGeomSurface,
            PerformNow: bool,
        ) -> UniquePtr<Section>;
        #[doc = "Constructor with two shapes <Sf>  - argument <S2>  - tool <PerformNow> - the flag: if <PerformNow>=True - the algorithm is performed immediately Obsolete"]
        #[cxx_name = "BRepAlgoAPI_Section_ctor_handlesurface_shape_bool"]
        fn Section_ctor_handlesurface_shape_bool(
            Sf: &HandleGeomSurface,
            S2: &TopoDS_Shape,
            PerformNow: bool,
        ) -> UniquePtr<Section>;
        #[doc = "Constructor with two shapes <Sf1>  - argument <Sf2>  - tool <PerformNow> - the flag: if <PerformNow>=True - the algorithm is performed immediately Obsolete"]
        #[cxx_name = "BRepAlgoAPI_Section_ctor_handlesurface2_bool"]
        fn Section_ctor_handlesurface2_bool(
            Sf1: &HandleGeomSurface,
            Sf2: &HandleGeomSurface,
            PerformNow: bool,
        ) -> UniquePtr<Section>;
        #[doc = "initialize the argument <S1>  - argument Obsolete"]
        #[cxx_name = "Init1"]
        fn init1shape(self: Pin<&mut Section>, S1: &TopoDS_Shape);
        #[doc = "initialize the argument <Pl>  - argument Obsolete"]
        #[cxx_name = "Init1"]
        fn init1pln_2(self: Pin<&mut Section>, Pl: &gp_Pln);
        #[doc = "initialize the argument <Sf>  - argument Obsolete"]
        #[cxx_name = "Init1"]
        fn init1handlesurface_3(self: Pin<&mut Section>, Sf: &HandleGeomSurface);
        #[doc = "initialize the tool <S2>  - tool Obsolete"]
        #[cxx_name = "Init2"]
        fn init2shape(self: Pin<&mut Section>, S2: &TopoDS_Shape);
        #[doc = "initialize the tool <Pl>  - tool Obsolete"]
        #[cxx_name = "Init2"]
        fn init2pln_2(self: Pin<&mut Section>, Pl: &gp_Pln);
        #[doc = "initialize the tool <Sf>  - tool Obsolete"]
        #[cxx_name = "Init2"]
        fn init2handlesurface_3(self: Pin<&mut Section>, Sf: &HandleGeomSurface);
        #[cxx_name = "Approximation"]
        fn approximation(self: Pin<&mut Section>, B: bool);
        #[doc = "Indicates whether the P-Curve should be (or not) performed on the argument. By default, no parametric 2D curve (pcurve) is defined for the edges of the result. If ComputePCurve1 equals true, further computations performed to attach an P-Curve in the parametric space of the argument to the constructed edges. Obsolete"]
        #[cxx_name = "ComputePCurveOn1"]
        fn compute_p_curve_on1(self: Pin<&mut Section>, B: bool);
        #[doc = "Indicates whether the P-Curve should be (or not) performed on the tool. By default, no parametric 2D curve (pcurve) is defined for the edges of the result. If ComputePCurve1 equals true, further computations performed to attach an P-Curve in the parametric space of the tool to the constructed edges. Obsolete"]
        #[cxx_name = "ComputePCurveOn2"]
        fn compute_p_curve_on2(self: Pin<&mut Section>, B: bool);
        #[doc = "Performs the algorithm Filling interference Data Structure (if it is necessary) Building the result of the operation."]
        #[cxx_name = "Build"]
        fn build(self: Pin<&mut Section>, theRange: &Message_ProgressRange);
        #[doc = "get the face of the first part giving section edge <E>. Returns True on the 3 following conditions : 1/ <E> is an edge returned by the Shape() metwod. 2/ First part of section performed is a shape. 3/ <E> is built on a intersection curve (i.e <E> is not the result of common edges) When False, F remains untouched. Obsolete"]
        #[cxx_name = "HasAncestorFaceOn1"]
        fn has_ancestor_face_on1(
            self: &Section,
            E: &TopoDS_Shape,
            F: Pin<&mut TopoDS_Shape>,
        ) -> bool;
        #[doc = "Identifies the ancestor faces of the intersection edge E resulting from the last computation performed in this framework, that is, the faces of the two original shapes on which the edge E lies: -      HasAncestorFaceOn1 gives the ancestor face in the first shape, and -      HasAncestorFaceOn2 gives the ancestor face in the second shape. These functions return true if an ancestor face F is found, or false if not. An ancestor face is identifiable for the edge E if the following conditions are satisfied: -  the first part on which this algorithm performed its last computation is a shape, that is, it was not given as a surface or a plane at the time of construction of this algorithm or at a later time by the Init1 function, - E is one of the elementary edges built by the last computation of this section algorithm. To use these functions properly, you have to test the returned Boolean value before using the ancestor face: F is significant only if the returned Boolean value equals true. Obsolete"]
        #[cxx_name = "HasAncestorFaceOn2"]
        fn has_ancestor_face_on2(
            self: &Section,
            E: &TopoDS_Shape,
            F: Pin<&mut TopoDS_Shape>,
        ) -> bool;
    }
    impl UniquePtr<Common> {}
    impl UniquePtr<Cut> {}
    impl UniquePtr<Fuse> {}
    impl UniquePtr<Section> {}
}
pub use ffi::Common;
impl Common {
    #[doc = "Empty constructor"]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Common_ctor()
    }

    #[doc = "Empty constructor <PF> - PaveFiller object that is carried out"]
    pub fn new_pavefiller(PF: &ffi::BOPAlgo_PaveFiller) -> cxx::UniquePtr<Self> {
        ffi::Common_ctor_pavefiller(PF)
    }

    #[doc = "Constructor with two shapes <S1>  -argument <S2>  -tool <anOperation> - the type of the operation Obsolete"]
    pub fn new_shape2_progressrange(
        S1: &ffi::TopoDS_Shape,
        S2: &ffi::TopoDS_Shape,
        theRange: &ffi::Message_ProgressRange,
    ) -> cxx::UniquePtr<Self> {
        ffi::Common_ctor_shape2_progressrange(S1, S2, theRange)
    }

    #[doc = "Constructor with two shapes <S1>  -argument <S2>  -tool <anOperation> - the type of the operation <PF> - PaveFiller object that is carried out Obsolete"]
    pub fn new_shape2_pavefiller_progressrange(
        S1: &ffi::TopoDS_Shape,
        S2: &ffi::TopoDS_Shape,
        PF: &ffi::BOPAlgo_PaveFiller,
        theRange: &ffi::Message_ProgressRange,
    ) -> cxx::UniquePtr<Self> {
        ffi::Common_ctor_shape2_pavefiller_progressrange(S1, S2, PF, theRange)
    }
}
pub use ffi::Cut;
impl Cut {
    #[doc = "Empty constructor"]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Cut_ctor()
    }

    #[doc = "Empty constructor <PF> - PaveFiller object that is carried out"]
    pub fn new_pavefiller(PF: &ffi::BOPAlgo_PaveFiller) -> cxx::UniquePtr<Self> {
        ffi::Cut_ctor_pavefiller(PF)
    }

    #[doc = "Constructor with two shapes <S1>  -argument <S2>  -tool <anOperation> - the type of the operation Obsolete"]
    pub fn new_shape2_progressrange(
        S1: &ffi::TopoDS_Shape,
        S2: &ffi::TopoDS_Shape,
        theRange: &ffi::Message_ProgressRange,
    ) -> cxx::UniquePtr<Self> {
        ffi::Cut_ctor_shape2_progressrange(S1, S2, theRange)
    }

    #[doc = "Constructor with two shapes <S1>  -argument <S2>  -tool <anOperation> - the type of the operation <PF> - PaveFiller object that is carried out Obsolete"]
    pub fn new_shape2_pavefiller_bool_progressrange(
        S1: &ffi::TopoDS_Shape,
        S2: &ffi::TopoDS_Shape,
        aDSF: &ffi::BOPAlgo_PaveFiller,
        bFWD: bool,
        theRange: &ffi::Message_ProgressRange,
    ) -> cxx::UniquePtr<Self> {
        ffi::Cut_ctor_shape2_pavefiller_bool_progressrange(S1, S2, aDSF, bFWD, theRange)
    }
}
pub use ffi::Fuse;
impl Fuse {
    #[doc = "Empty constructor"]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Fuse_ctor()
    }

    #[doc = "Empty constructor <PF> - PaveFiller object that is carried out"]
    pub fn new_pavefiller(PF: &ffi::BOPAlgo_PaveFiller) -> cxx::UniquePtr<Self> {
        ffi::Fuse_ctor_pavefiller(PF)
    }

    #[doc = "Constructor with two shapes <S1>  -argument <S2>  -tool <anOperation> - the type of the operation Obsolete"]
    pub fn new_shape2_progressrange(
        S1: &ffi::TopoDS_Shape,
        S2: &ffi::TopoDS_Shape,
        theRange: &ffi::Message_ProgressRange,
    ) -> cxx::UniquePtr<Self> {
        ffi::Fuse_ctor_shape2_progressrange(S1, S2, theRange)
    }

    #[doc = "Constructor with two shapes <S1>  -argument <S2>  -tool <anOperation> - the type of the operation <PF> - PaveFiller object that is carried out Obsolete"]
    pub fn new_shape2_pavefiller_progressrange(
        S1: &ffi::TopoDS_Shape,
        S2: &ffi::TopoDS_Shape,
        aDSF: &ffi::BOPAlgo_PaveFiller,
        theRange: &ffi::Message_ProgressRange,
    ) -> cxx::UniquePtr<Self> {
        ffi::Fuse_ctor_shape2_pavefiller_progressrange(S1, S2, aDSF, theRange)
    }
}
pub use ffi::Section;
impl Section {
    #[doc = "Empty constructor"]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Section_ctor()
    }

    #[doc = "Empty constructor <PF> - PaveFiller object that is carried out"]
    pub fn new_pavefiller(PF: &ffi::BOPAlgo_PaveFiller) -> cxx::UniquePtr<Self> {
        ffi::Section_ctor_pavefiller(PF)
    }

    #[doc = "Constructor with two shapes <S1>  -argument <S2>  -tool <PerformNow> - the flag: if <PerformNow>=True - the algorithm is performed immediately Obsolete"]
    pub fn new_shape2_bool(
        S1: &ffi::TopoDS_Shape,
        S2: &ffi::TopoDS_Shape,
        PerformNow: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::Section_ctor_shape2_bool(S1, S2, PerformNow)
    }

    #[doc = "Constructor with two shapes <S1>  -argument <S2>  -tool <PF> - PaveFiller object that is carried out <PerformNow> - the flag: if <PerformNow>=True - the algorithm is performed immediately Obsolete"]
    pub fn new_shape2_pavefiller_bool(
        S1: &ffi::TopoDS_Shape,
        S2: &ffi::TopoDS_Shape,
        aDSF: &ffi::BOPAlgo_PaveFiller,
        PerformNow: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::Section_ctor_shape2_pavefiller_bool(S1, S2, aDSF, PerformNow)
    }

    #[doc = "Constructor with two shapes <S1>  - argument <Pl>  - tool <PerformNow> - the flag: if <PerformNow>=True - the algorithm is performed immediately Obsolete"]
    pub fn new_shape_pln_bool(
        S1: &ffi::TopoDS_Shape,
        Pl: &ffi::gp_Pln,
        PerformNow: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::Section_ctor_shape_pln_bool(S1, Pl, PerformNow)
    }

    #[doc = "Constructor with two shapes <S1>  - argument <Sf>  - tool <PerformNow> - the flag: if <PerformNow>=True - the algorithm is performed immediately Obsolete"]
    pub fn new_shape_handlesurface_bool(
        S1: &ffi::TopoDS_Shape,
        Sf: &ffi::HandleGeomSurface,
        PerformNow: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::Section_ctor_shape_handlesurface_bool(S1, Sf, PerformNow)
    }

    #[doc = "Constructor with two shapes <Sf>  - argument <S2>  - tool <PerformNow> - the flag: if <PerformNow>=True - the algorithm is performed immediately Obsolete"]
    pub fn new_handlesurface_shape_bool(
        Sf: &ffi::HandleGeomSurface,
        S2: &ffi::TopoDS_Shape,
        PerformNow: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::Section_ctor_handlesurface_shape_bool(Sf, S2, PerformNow)
    }

    #[doc = "Constructor with two shapes <Sf1>  - argument <Sf2>  - tool <PerformNow> - the flag: if <PerformNow>=True - the algorithm is performed immediately Obsolete"]
    pub fn new_handlesurface2_bool(
        Sf1: &ffi::HandleGeomSurface,
        Sf2: &ffi::HandleGeomSurface,
        PerformNow: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::Section_ctor_handlesurface2_bool(Sf1, Sf2, PerformNow)
    }
}
