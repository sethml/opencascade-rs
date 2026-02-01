#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_b_rep_feat.hxx");
        #[doc = "FormatVersion from top_tools module"]
        type TopTools_FormatVersion = crate::top_tools::ffi::TopTools_FormatVersion;
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
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TopTools_ListOfShape"]
        type TopTools_ListOfShape;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TColGeom_SequenceOfCurve"]
        type TColGeom_SequenceOfCurve;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "BRepFeat_Status"]
        type BRepFeat_Status;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleGeomCurve"]
        type HandleGeomCurve;
        #[doc = " ======================== BRepFeat_MakeCylindricalHole ========================"]
        #[doc = "Provides a tool to make cylindrical holes on a shape."]
        #[cxx_name = "BRepFeat_MakeCylindricalHole"]
        type MakeCylindricalHole;
        #[doc = "Empty constructor."]
        #[cxx_name = "BRepFeat_MakeCylindricalHole_ctor"]
        fn MakeCylindricalHole_ctor() -> UniquePtr<MakeCylindricalHole>;
        #[doc = "Sets the axis of the hole(s)."]
        #[cxx_name = "Init"]
        fn initax1(self: Pin<&mut MakeCylindricalHole>, Axis: &gp_Ax1);
        #[doc = "Sets the shape and  axis on which hole(s)  will be performed."]
        #[cxx_name = "Init"]
        fn initshape_2(self: Pin<&mut MakeCylindricalHole>, S: &TopoDS_Shape, Axis: &gp_Ax1);
        #[doc = "Performs every  hole of    radius  <Radius>.  This command  has the  same effect as   a cut operation with an  infinite cylinder   defined by the  given axis and <Radius>."]
        #[cxx_name = "Perform"]
        fn performreal(self: Pin<&mut MakeCylindricalHole>, Radius: f64);
        #[doc = "Performs every  hole  of  radius  <Radius> located between PFrom  and  PTo  on the  given  axis.   If <WithControl> is set  to Standard_False no control are  done  on   the  resulting  shape   after  the operation is performed."]
        #[cxx_name = "Perform"]
        fn performreal_2(
            self: Pin<&mut MakeCylindricalHole>,
            Radius: f64,
            PFrom: f64,
            PTo: f64,
            WithControl: bool,
        );
        #[doc = "Performs the first hole of radius <Radius>, in the direction of  the defined axis. First hole signify first encountered after the origin of the axis. If <WithControl> is set  to Standard_False no control are  done  on   the  resulting  shape   after  the operation is performed."]
        #[cxx_name = "PerformThruNext"]
        fn perform_thru_next(self: Pin<&mut MakeCylindricalHole>, Radius: f64, WithControl: bool);
        #[doc = "Performs every  hole of   radius  <Radius> located after  the   origin  of   the given    axis.    If <WithControl> is  set to Standard_False no control are done   on   the  resulting  shape   after  the operation is performed."]
        #[cxx_name = "PerformUntilEnd"]
        fn perform_until_end(self: Pin<&mut MakeCylindricalHole>, Radius: f64, WithControl: bool);
        #[doc = "Performs a  blind   hole of radius    <Radius> and length <Length>.  The length is  measured from the origin of the given  axis. If <WithControl> is set to  Standard_False no  control  are done after the operation is performed."]
        #[cxx_name = "PerformBlind"]
        fn perform_blind(
            self: Pin<&mut MakeCylindricalHole>,
            Radius: f64,
            Length: f64,
            WithControl: bool,
        );
        #[doc = "Builds the    resulting shape  (redefined     from MakeShape). Invalidates the  given parts  of tools if  any,   and performs the  result   of the local operation."]
        #[cxx_name = "Build"]
        fn build(self: Pin<&mut MakeCylindricalHole>);
        #[doc = "Returns the status after a hole is performed."]
        #[cxx_name = "BRepFeat_MakeCylindricalHole_Status"]
        fn MakeCylindricalHole_status(self_: &MakeCylindricalHole) -> UniquePtr<BRepFeat_Status>;
        #[doc = " ======================== BRepFeat_MakeDPrism ========================"]
        #[doc = "Describes functions to build draft prism topologies from basis shape surfaces. These can be depressions or protrusions. The semantics of draft prism feature creation is based on the construction of shapes: -          along a length -          up to a limiting face -          from a limiting face to a height. The shape defining construction of the draft prism feature can be either the supporting edge or the concerned area of a face. In case of the supporting edge, this contour can be attached to a face of the basis shape by binding. When the contour is bound to this face, the information that the contour will slide on the face becomes available to the relevant class methods. In case of the concerned area of a face, you could, for example, cut it out and move it to a different height which will define the limiting face of a protrusion or depression."]
        #[cxx_name = "BRepFeat_MakeDPrism"]
        type MakeDPrism;
        #[doc = "A face Pbase is selected in the shape Sbase to serve as the basis for the draft prism. The draft will be defined by the angle Angle and Fuse offers a choice between: - removing matter with a Boolean cut using the setting 0 - adding matter with Boolean fusion using the setting 1. The sketch face Skface serves to determine the type of operation. If it is inside the basis shape, a local operation such as glueing can be performed. Initializes the draft prism class"]
        #[cxx_name = "BRepFeat_MakeDPrism_ctor_shape_face2_real_int_bool"]
        fn MakeDPrism_ctor_shape_face2_real_int_bool(
            Sbase: &TopoDS_Shape,
            Pbase: &TopoDS_Face,
            Skface: &TopoDS_Face,
            Angle: f64,
            Fuse: i32,
            Modify: bool,
        ) -> UniquePtr<MakeDPrism>;
        #[cxx_name = "BRepFeat_MakeDPrism_ctor"]
        fn MakeDPrism_ctor() -> UniquePtr<MakeDPrism>;
        #[doc = "Initializes this algorithm for building draft prisms along surfaces. A face Pbase is selected in the basis shape Sbase to serve as the basis from the draft prism. The draft will be defined by the angle Angle and Fuse offers a choice between: -   removing matter with a Boolean cut using the setting 0 -   adding matter with Boolean fusion using the setting  1. The sketch face Skface serves to determine the type of operation. If it is inside the basis shape, a local operation such as glueing can be performed."]
        #[cxx_name = "Init"]
        fn init(
            self: Pin<&mut MakeDPrism>,
            Sbase: &TopoDS_Shape,
            Pbase: &TopoDS_Face,
            Skface: &TopoDS_Face,
            Angle: f64,
            Fuse: i32,
            Modify: bool,
        );
        #[doc = "Indicates that the edge <E> will slide on the face <OnFace>. Raises ConstructionError if the  face does not belong to the basis shape, or the edge to the prismed shape."]
        #[cxx_name = "Add"]
        fn add(self: Pin<&mut MakeDPrism>, E: &TopoDS_Edge, OnFace: &TopoDS_Face);
        #[cxx_name = "Perform"]
        fn performreal(self: Pin<&mut MakeDPrism>, Height: f64);
        #[cxx_name = "Perform"]
        fn performshape_2(self: Pin<&mut MakeDPrism>, Until: &TopoDS_Shape);
        #[doc = "Assigns one of the following semantics -   to a height Height -   to a face Until -   from a face From to a height Until. Reconstructs the feature topologically according to the semantic option chosen."]
        #[cxx_name = "Perform"]
        fn performshape_3(self: Pin<&mut MakeDPrism>, From: &TopoDS_Shape, Until: &TopoDS_Shape);
        #[doc = "Realizes a semi-infinite prism, limited by the position of the prism base."]
        #[cxx_name = "PerformUntilEnd"]
        fn perform_until_end(self: Pin<&mut MakeDPrism>);
        #[doc = "Realizes a semi-infinite prism, limited by the face Funtil."]
        #[cxx_name = "PerformFromEnd"]
        fn perform_from_end(self: Pin<&mut MakeDPrism>, FUntil: &TopoDS_Shape);
        #[doc = "Builds an infinite prism. The infinite descendants will not be kept in the result."]
        #[cxx_name = "PerformThruAll"]
        fn perform_thru_all(self: Pin<&mut MakeDPrism>);
        #[doc = "Assigns both a limiting shape, Until from TopoDS_Shape, and a height, Height at which to stop generation of the prism feature."]
        #[cxx_name = "PerformUntilHeight"]
        fn perform_until_height(self: Pin<&mut MakeDPrism>, Until: &TopoDS_Shape, Height: f64);
        #[cxx_name = "Curves"]
        fn curves(self: Pin<&mut MakeDPrism>, S: Pin<&mut TColGeom_SequenceOfCurve>);
        #[doc = "Determination of TopEdges and LatEdges. sig = 1 -> TopEdges = FirstShape of the DPrism sig = 2 -> TOpEdges = LastShape of the DPrism"]
        #[cxx_name = "BossEdges"]
        fn boss_edges(self: Pin<&mut MakeDPrism>, sig: i32);
        #[doc = "Returns the list of TopoDS Edges of the top of the boss."]
        #[cxx_name = "TopEdges"]
        fn top_edges(self: Pin<&mut MakeDPrism>) -> &TopTools_ListOfShape;
        #[doc = "Returns the list of TopoDS Edges of the bottom of the boss."]
        #[cxx_name = "LatEdges"]
        fn lat_edges(self: Pin<&mut MakeDPrism>) -> &TopTools_ListOfShape;
        #[cxx_name = "BRepFeat_MakeDPrism_BarycCurve"]
        fn MakeDPrism_baryc_curve(self_: Pin<&mut MakeDPrism>) -> UniquePtr<HandleGeomCurve>;
    }
    impl UniquePtr<MakeCylindricalHole> {}
    impl UniquePtr<MakeDPrism> {}
}
pub use ffi::MakeCylindricalHole;
impl MakeCylindricalHole {
    #[doc = "Empty constructor."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::MakeCylindricalHole_ctor()
    }
}
pub use ffi::MakeDPrism;
impl MakeDPrism {
    #[doc = "A face Pbase is selected in the shape Sbase to serve as the basis for the draft prism. The draft will be defined by the angle Angle and Fuse offers a choice between: - removing matter with a Boolean cut using the setting 0 - adding matter with Boolean fusion using the setting 1. The sketch face Skface serves to determine the type of operation. If it is inside the basis shape, a local operation such as glueing can be performed. Initializes the draft prism class"]
    pub fn new_shape_face2_real_int_bool(
        Sbase: &ffi::TopoDS_Shape,
        Pbase: &ffi::TopoDS_Face,
        Skface: &ffi::TopoDS_Face,
        Angle: f64,
        Fuse: i32,
        Modify: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakeDPrism_ctor_shape_face2_real_int_bool(Sbase, Pbase, Skface, Angle, Fuse, Modify)
    }

    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::MakeDPrism_ctor()
    }
}
