#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_b_rep_prim_api.hxx");
        #[doc = "Pnt from gp module"]
        type gp_Pnt = crate::gp::ffi::Pnt;
        #[doc = "Vec from gp module"]
        type gp_Vec = crate::gp::ffi::Vec_;
        #[doc = "Dir from gp module"]
        type gp_Dir = crate::gp::ffi::Dir;
        #[doc = "Shape from topo_ds module"]
        type TopoDS_Shape = crate::topo_ds::ffi::Shape;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TopoDS_Face"]
        type TopoDS_Face;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "BRepPrim_Wedge"]
        type BRepPrim_Wedge;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "Message_ProgressRange"]
        type Message_ProgressRange;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TopoDS_Solid"]
        type TopoDS_Solid;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "gp_Ax2"]
        type gp_Ax2;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TopoDS_Shell"]
        type TopoDS_Shell;
        #[doc = "Describes functions to build parallelepiped boxes. A MakeBox object provides a framework for: -   defining the construction of a box, -   implementing the construction algorithm, and -   consulting the result. Constructs a box such that its sides are parallel to the axes of -   the global coordinate system, or -   the local coordinate system Axis. and -   with a corner at (0, 0, 0) and of size (dx, dy, dz), or -   with a corner at point P and of size (dx, dy, dz), or -   with corners at points P1 and P2. Exceptions Standard_DomainError if: dx, dy, dz are less than or equal to Precision::Confusion(), or -   the vector joining the points P1 and P2 has a component projected onto the global coordinate system less than or equal to Precision::Confusion(). In these cases, the box would be flat."]
        #[cxx_name = "BRepPrimAPI_MakeBox"]
        type MakeBox;
        #[doc = "Default constructor"]
        #[cxx_name = "BRepPrimAPI_MakeBox_ctor"]
        fn MakeBox_ctor() -> UniquePtr<MakeBox>;
        #[doc = "Make a box with a corner at 0,0,0 and the other dx,dy,dz"]
        #[cxx_name = "BRepPrimAPI_MakeBox_ctor_real_real_real"]
        fn MakeBox_ctor_real_real_real(dx: f64, dy: f64, dz: f64) -> UniquePtr<MakeBox>;
        #[doc = "Make a box with a corner at P and size dx, dy, dz."]
        #[cxx_name = "BRepPrimAPI_MakeBox_ctor_pnt_real_real_real"]
        fn MakeBox_ctor_pnt_real_real_real(
            p: &gp_Pnt,
            dx: f64,
            dy: f64,
            dz: f64,
        ) -> UniquePtr<MakeBox>;
        #[doc = "Make a box with corners P1,P2."]
        #[cxx_name = "BRepPrimAPI_MakeBox_ctor_pnt_pnt"]
        fn MakeBox_ctor_pnt_pnt(p1: &gp_Pnt, p2: &gp_Pnt) -> UniquePtr<MakeBox>;
        #[doc = "Make a box with Ax2 (the left corner and the axis) and size dx, dy, dz."]
        #[cxx_name = "BRepPrimAPI_MakeBox_ctor_ax2_real_real_real"]
        fn MakeBox_ctor_ax2_real_real_real(
            axes: &gp_Ax2,
            dx: f64,
            dy: f64,
            dz: f64,
        ) -> UniquePtr<MakeBox>;
        #[doc = "Init a box with a corner at 0,0,0 and the other theDX, theDY, theDZ"]
        #[cxx_name = "Init"]
        fn initreal(self: Pin<&mut MakeBox>, the_dx: f64, the_dy: f64, the_dz: f64);
        #[doc = "Init a box with a corner at thePnt and size theDX, theDY, theDZ."]
        #[cxx_name = "Init"]
        fn initpnt_2(
            self: Pin<&mut MakeBox>,
            the_pnt: &gp_Pnt,
            the_dx: f64,
            the_dy: f64,
            the_dz: f64,
        );
        #[doc = "Init a box with corners thePnt1, thePnt2."]
        #[cxx_name = "Init"]
        fn initpnt_3(self: Pin<&mut MakeBox>, the_pnt1: &gp_Pnt, the_pnt2: &gp_Pnt);
        #[doc = "Init a box with Ax2 (the left corner and the theAxes) and size theDX, theDY, theDZ."]
        #[cxx_name = "Init"]
        fn initax2_4(
            self: Pin<&mut MakeBox>,
            the_axes: &gp_Ax2,
            the_dx: f64,
            the_dy: f64,
            the_dz: f64,
        );
        #[doc = "Returns the internal algorithm."]
        #[cxx_name = "Wedge"]
        fn wedge(self: Pin<&mut MakeBox>) -> Pin<&mut BRepPrim_Wedge>;
        #[doc = "Stores the solid in myShape."]
        #[cxx_name = "Build"]
        fn build(self: Pin<&mut MakeBox>, the_range: &Message_ProgressRange);
        #[doc = "Returns the constructed box as a shell."]
        #[cxx_name = "Shell"]
        fn shell(self: Pin<&mut MakeBox>) -> &TopoDS_Shell;
        #[doc = "Returns the constructed box as a solid."]
        #[cxx_name = "Solid"]
        fn solid(self: Pin<&mut MakeBox>) -> &TopoDS_Solid;
        #[doc = "Returns ZMin face"]
        #[cxx_name = "BottomFace"]
        fn bottom_face(self: Pin<&mut MakeBox>) -> &TopoDS_Face;
        #[doc = "Returns XMin face"]
        #[cxx_name = "BackFace"]
        fn back_face(self: Pin<&mut MakeBox>) -> &TopoDS_Face;
        #[doc = "Returns XMax face"]
        #[cxx_name = "FrontFace"]
        fn front_face(self: Pin<&mut MakeBox>) -> &TopoDS_Face;
        #[doc = "Returns YMin face"]
        #[cxx_name = "LeftFace"]
        fn left_face(self: Pin<&mut MakeBox>) -> &TopoDS_Face;
        #[doc = "Returns YMax face"]
        #[cxx_name = "RightFace"]
        fn right_face(self: Pin<&mut MakeBox>) -> &TopoDS_Face;
        #[doc = "Returns ZMax face"]
        #[cxx_name = "TopFace"]
        fn top_face(self: Pin<&mut MakeBox>) -> &TopoDS_Face;
    }
    impl UniquePtr<MakeBox> {}
}
pub use ffi::MakeBox;
pub use ffi::MakeBox_ctor;
pub use ffi::MakeBox_ctor_ax2_real_real_real;
pub use ffi::MakeBox_ctor_pnt_pnt;
pub use ffi::MakeBox_ctor_pnt_real_real_real;
pub use ffi::MakeBox_ctor_real_real_real;
