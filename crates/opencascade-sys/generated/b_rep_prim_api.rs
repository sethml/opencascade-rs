#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_b_rep_prim_api.hxx");
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
        #[doc = "FormatVersion from top_tools module"]
        type TopTools_FormatVersion = crate::top_tools::ffi::TopTools_FormatVersion;
        #[doc = "ProgressRange from message module"]
        type Message_ProgressRange = crate::message::ffi::ProgressRange;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "BRepPrim_Torus"]
        type BRepPrim_Torus;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "BRepPrim_Wedge"]
        type BRepPrim_Wedge;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "BRepPrim_Cone"]
        type BRepPrim_Cone;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "BRepSweep_Revol"]
        type BRepSweep_Revol;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "BRepPrim_Sphere"]
        type BRepPrim_Sphere;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "BRepPrim_Cylinder"]
        type BRepPrim_Cylinder;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TopTools_ListOfShape"]
        type TopTools_ListOfShape;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "BRepSweep_Prism"]
        type BRepSweep_Prism;
        #[doc = " ======================== BRepPrimAPI_MakeBox ========================"]
        #[doc = "Describes functions to build parallelepiped boxes. A MakeBox object provides a framework for: -   defining the construction of a box, -   implementing the construction algorithm, and -   consulting the result. Constructs a box such that its sides are parallel to the axes of -   the global coordinate system, or -   the local coordinate system Axis. and -   with a corner at (0, 0, 0) and of size (dx, dy, dz), or -   with a corner at point P and of size (dx, dy, dz), or -   with corners at points P1 and P2. Exceptions Standard_DomainError if: dx, dy, dz are less than or equal to Precision::Confusion(), or -   the vector joining the points P1 and P2 has a component projected onto the global coordinate system less than or equal to Precision::Confusion(). In these cases, the box would be flat."]
        #[cxx_name = "BRepPrimAPI_MakeBox"]
        type MakeBox;
        #[doc = "Default constructor"]
        #[cxx_name = "BRepPrimAPI_MakeBox_ctor"]
        fn MakeBox_ctor() -> UniquePtr<MakeBox>;
        #[doc = "Make a box with a corner at 0,0,0 and the other dx,dy,dz"]
        #[cxx_name = "BRepPrimAPI_MakeBox_ctor_real3"]
        fn MakeBox_ctor_real3(dx: f64, dy: f64, dz: f64) -> UniquePtr<MakeBox>;
        #[doc = "Make a box with a corner at P and size dx, dy, dz."]
        #[cxx_name = "BRepPrimAPI_MakeBox_ctor_pnt_real3"]
        fn MakeBox_ctor_pnt_real3(P: &gp_Pnt, dx: f64, dy: f64, dz: f64) -> UniquePtr<MakeBox>;
        #[doc = "Make a box with corners P1,P2."]
        #[cxx_name = "BRepPrimAPI_MakeBox_ctor_pnt2"]
        fn MakeBox_ctor_pnt2(P1: &gp_Pnt, P2: &gp_Pnt) -> UniquePtr<MakeBox>;
        #[doc = "Make a box with Ax2 (the left corner and the axis) and size dx, dy, dz."]
        #[cxx_name = "BRepPrimAPI_MakeBox_ctor_ax2_real3"]
        fn MakeBox_ctor_ax2_real3(Axes: &gp_Ax2, dx: f64, dy: f64, dz: f64) -> UniquePtr<MakeBox>;
        #[doc = "Init a box with a corner at 0,0,0 and the other theDX, theDY, theDZ"]
        #[cxx_name = "Init"]
        fn initreal(self: Pin<&mut MakeBox>, theDX: f64, theDY: f64, theDZ: f64);
        #[doc = "Init a box with a corner at thePnt and size theDX, theDY, theDZ."]
        #[cxx_name = "Init"]
        fn initpnt_2(self: Pin<&mut MakeBox>, thePnt: &gp_Pnt, theDX: f64, theDY: f64, theDZ: f64);
        #[doc = "Init a box with corners thePnt1, thePnt2."]
        #[cxx_name = "Init"]
        fn initpnt_3(self: Pin<&mut MakeBox>, thePnt1: &gp_Pnt, thePnt2: &gp_Pnt);
        #[doc = "Init a box with Ax2 (the left corner and the theAxes) and size theDX, theDY, theDZ."]
        #[cxx_name = "Init"]
        fn initax2_4(self: Pin<&mut MakeBox>, theAxes: &gp_Ax2, theDX: f64, theDY: f64, theDZ: f64);
        #[doc = "Returns the internal algorithm."]
        #[cxx_name = "Wedge"]
        fn wedge(self: Pin<&mut MakeBox>) -> Pin<&mut BRepPrim_Wedge>;
        #[doc = "Stores the solid in myShape."]
        #[cxx_name = "Build"]
        fn build(self: Pin<&mut MakeBox>, theRange: &Message_ProgressRange);
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
        #[doc = " ======================== BRepPrimAPI_MakeCone ========================"]
        #[doc = "Describes functions to build cones or portions of cones. A MakeCone object provides a framework for: -   defining the construction of a cone, -   implementing the construction algorithm, and -   consulting the result."]
        #[cxx_name = "BRepPrimAPI_MakeCone"]
        type MakeCone;
        #[doc = "Make a cone. @param[in] R1  cone bottom radius, may be null (z = 0) @param[in] R2  cone top radius, may be null (z = H) @param[in] H   cone height"]
        #[cxx_name = "BRepPrimAPI_MakeCone_ctor_real3"]
        fn MakeCone_ctor_real3(R1: f64, R2: f64, H: f64) -> UniquePtr<MakeCone>;
        #[doc = "Make a cone. @param[in] R1     cone bottom radius, may be null (z = 0) @param[in] R2     cone top radius, may be null (z = H) @param[in] H      cone height @param[in] angle  angle to create a part cone"]
        #[cxx_name = "BRepPrimAPI_MakeCone_ctor_real4"]
        fn MakeCone_ctor_real4(R1: f64, R2: f64, H: f64, angle: f64) -> UniquePtr<MakeCone>;
        #[doc = "Make a cone. @param[in] axes  coordinate system for the construction of the cone @param[in] R1    cone bottom radius, may be null (z = 0) @param[in] R2    cone top radius, may be null (z = H) @param[in] H     cone height"]
        #[cxx_name = "BRepPrimAPI_MakeCone_ctor_ax2_real3"]
        fn MakeCone_ctor_ax2_real3(Axes: &gp_Ax2, R1: f64, R2: f64, H: f64) -> UniquePtr<MakeCone>;
        #[doc = "Make a cone of height H radius R1 in the plane z = 0, R2 in the plane Z = H. R1 and R2 may be null. Take a section of <angle> Constructs a cone, or a portion of a cone, of height H, and radius R1 in the plane z = 0 and R2 in the plane z = H. The result is a sharp cone if R1 or R2 is equal to 0. The cone is constructed about the \"Z Axis\" of either: -   the global coordinate system, or -   the local coordinate system Axes. It is limited in these coordinate systems as follows: -   in the v parametric direction (the Z coordinate), by the two parameter values 0 and H, -   and in the u parametric direction (defined by the angle of rotation around the Z axis), in the case of a portion of a cone, by the two parameter values 0 and angle. Angle is given in radians. The resulting shape is composed of: -   a lateral conical face -   two planar faces in the planes z = 0 and z = H, or only one planar face in one of these two planes if a radius value is null (in the case of a complete cone, these faces are circles), and -   and in the case of a portion of a cone, two planar faces to close the shape. (either two parallelograms or two triangles, in the planes u = 0 and u = angle). Exceptions Standard_DomainError if: -   H is less than or equal to Precision::Confusion(), or -   the half-angle at the apex of the cone, defined by R1, R2 and H, is less than Precision::Confusion()/H, or greater than (Pi/2)-Precision::Confusion()/H.f"]
        #[cxx_name = "BRepPrimAPI_MakeCone_ctor_ax2_real4"]
        fn MakeCone_ctor_ax2_real4(
            Axes: &gp_Ax2,
            R1: f64,
            R2: f64,
            H: f64,
            angle: f64,
        ) -> UniquePtr<MakeCone>;
        #[doc = "Returns the algorithm."]
        #[cxx_name = "Cone"]
        fn cone(self: Pin<&mut MakeCone>) -> Pin<&mut BRepPrim_Cone>;
        #[doc = " ======================== BRepPrimAPI_MakeCylinder ========================"]
        #[doc = "Describes functions to build cylinders or portions of  cylinders. A MakeCylinder object provides a framework for: -   defining the construction of a cylinder, -   implementing the construction algorithm, and -   consulting the result."]
        #[cxx_name = "BRepPrimAPI_MakeCylinder"]
        type MakeCylinder;
        #[doc = "Make a cylinder. @param[in] R  cylinder radius @param[in] H  cylinder height"]
        #[cxx_name = "BRepPrimAPI_MakeCylinder_ctor_real2"]
        fn MakeCylinder_ctor_real2(R: f64, H: f64) -> UniquePtr<MakeCylinder>;
        #[doc = "Make a cylinder (part cylinder). @param[in] R      cylinder radius @param[in] H      cylinder height @param[in] Angle  defines the missing portion of the cylinder"]
        #[cxx_name = "BRepPrimAPI_MakeCylinder_ctor_real3"]
        fn MakeCylinder_ctor_real3(R: f64, H: f64, Angle: f64) -> UniquePtr<MakeCylinder>;
        #[doc = "Make a cylinder of radius R and length H. @param[in] Axes  coordinate system for the construction of the cylinder @param[in] R     cylinder radius @param[in] H     cylinder height"]
        #[cxx_name = "BRepPrimAPI_MakeCylinder_ctor_ax2_real2"]
        fn MakeCylinder_ctor_ax2_real2(Axes: &gp_Ax2, R: f64, H: f64) -> UniquePtr<MakeCylinder>;
        #[doc = "Make a cylinder   of  radius R  and  length H with angle  H. Constructs -   a cylinder of radius R and height H, or -   a portion of cylinder of radius R and height H, and of the angle Angle defining the missing portion of the cylinder. The cylinder is constructed about the \"Z Axis\" of either: -   the global coordinate system, or -   the local coordinate system Axes. It is limited in this coordinate system as follows: -   in the v parametric direction (the Z axis), by the two parameter values 0 and H, -   and in the u parametric direction (the rotation angle around the Z Axis), in the case of a portion of a cylinder, by the two parameter values 0 and Angle. Angle is given in radians. The resulting shape is composed of: -   a lateral cylindrical face, -   two planar faces in the planes z = 0 and z = H (in the case of a complete cylinder, these faces are circles), and -   in case of a portion of a cylinder, two additional planar faces to close the shape.(two rectangles in the planes u = 0 and u = Angle). Exceptions Standard_DomainError if: -   R is less than or equal to Precision::Confusion(), or -   H is less than or equal to Precision::Confusion()."]
        #[cxx_name = "BRepPrimAPI_MakeCylinder_ctor_ax2_real3"]
        fn MakeCylinder_ctor_ax2_real3(
            Axes: &gp_Ax2,
            R: f64,
            H: f64,
            Angle: f64,
        ) -> UniquePtr<MakeCylinder>;
        #[doc = "Returns the algorithm."]
        #[cxx_name = "Cylinder"]
        fn cylinder(self: Pin<&mut MakeCylinder>) -> Pin<&mut BRepPrim_Cylinder>;
        #[doc = " ======================== BRepPrimAPI_MakePrism ========================"]
        #[doc = "Describes functions to build linear swept topologies, called prisms. A prism is defined by: -   a basis shape, which is swept, and -   a sweeping direction, which is: -   a vector for finite prisms, or -   a direction for infinite or semi-infinite prisms. The basis shape must not contain any solids. The profile generates objects according to the following rules: -   Vertices generate Edges -   Edges generate Faces. -   Wires generate Shells. -   Faces generate Solids. -   Shells generate Composite Solids A MakePrism object provides a framework for: -   defining the construction of a prism, -   implementing the construction algorithm, and -   consulting the result."]
        #[cxx_name = "BRepPrimAPI_MakePrism"]
        type MakePrism;
        #[doc = "Builds the prism of base S and vector V. If C is true, S is copied. If Canonize is true then generated surfaces are attempted to be canonized in simple types"]
        #[cxx_name = "BRepPrimAPI_MakePrism_ctor_shape_vec_bool2"]
        fn MakePrism_ctor_shape_vec_bool2(
            S: &TopoDS_Shape,
            V: &gp_Vec,
            Copy: bool,
            Canonize: bool,
        ) -> UniquePtr<MakePrism>;
        #[doc = "Builds a semi-infinite or an infinite prism of base S. If Inf is true the prism  is infinite, if Inf is false the prism is semi-infinite (in the direction D).  If C is true S is copied (for semi-infinite prisms). If Canonize is true then generated surfaces are attempted to be canonized in simple types"]
        #[cxx_name = "BRepPrimAPI_MakePrism_ctor_shape_dir_bool3"]
        fn MakePrism_ctor_shape_dir_bool3(
            S: &TopoDS_Shape,
            D: &gp_Dir,
            Inf: bool,
            Copy: bool,
            Canonize: bool,
        ) -> UniquePtr<MakePrism>;
        #[doc = "Returns the internal sweeping algorithm."]
        #[cxx_name = "Prism"]
        fn prism(self: &MakePrism) -> &BRepSweep_Prism;
        #[doc = "Builds the resulting shape (redefined from MakeShape)."]
        #[cxx_name = "Build"]
        fn build(self: Pin<&mut MakePrism>, theRange: &Message_ProgressRange);
        #[doc = "Returns ListOfShape from TopTools."]
        #[cxx_name = "Generated"]
        fn generated(self: Pin<&mut MakePrism>, S: &TopoDS_Shape) -> &TopTools_ListOfShape;
        #[doc = "Returns true if the shape S has been deleted."]
        #[cxx_name = "IsDeleted"]
        fn is_deleted(self: Pin<&mut MakePrism>, S: &TopoDS_Shape) -> bool;
        #[doc = "Returns the  TopoDS  Shape of the bottom of the prism."]
        #[cxx_name = "BRepPrimAPI_MakePrism_FirstShape"]
        fn MakePrism_first_shape(self_: Pin<&mut MakePrism>) -> UniquePtr<TopoDS_Shape>;
        #[doc = "Returns the TopoDS Shape of the top of the prism. In the case of a finite prism, FirstShape returns the basis of the prism, in other words, S if Copy is false; otherwise, the copy of S belonging to the prism. LastShape returns the copy of S translated by V at the time of construction."]
        #[cxx_name = "BRepPrimAPI_MakePrism_LastShape"]
        fn MakePrism_last_shape(self_: Pin<&mut MakePrism>) -> UniquePtr<TopoDS_Shape>;
        #[doc = "Returns the TopoDS Shape of the bottom  of the  prism. generated  with  theShape (subShape of the  generating shape)."]
        #[cxx_name = "BRepPrimAPI_MakePrism_FirstShape"]
        fn MakePrism_first_shapeshape_2(
            self_: Pin<&mut MakePrism>,
            theShape: &TopoDS_Shape,
        ) -> UniquePtr<TopoDS_Shape>;
        #[doc = "Returns the  TopoDS  Shape of the top  of  the  prism. generated  with  theShape (subShape of the  generating shape)."]
        #[cxx_name = "BRepPrimAPI_MakePrism_LastShape"]
        fn MakePrism_last_shapeshape_2(
            self_: Pin<&mut MakePrism>,
            theShape: &TopoDS_Shape,
        ) -> UniquePtr<TopoDS_Shape>;
        #[doc = " ======================== BRepPrimAPI_MakeRevol ========================"]
        #[doc = "Class to make revolved sweep topologies. a revolved sweep is defined by : * A basis topology which is swept. The   basis topology  must   not  contain solids (neither composite solids.). The basis topology  may be copied  or  shared in the result. * A rotation axis and angle : - The axis is an Ax1 from gp. - The angle is in [0, 2*Pi]. - The angle default value is 2*Pi. The result is a topology with a higher dimension : - Vertex -> Edge. - Edge   -> Face. - Wire   -> Shell. - Face   -> Solid. - Shell  -> CompSolid. Sweeping a Compound sweeps  the elements  of the compound  and creates    a  compound with    the results."]
        #[cxx_name = "BRepPrimAPI_MakeRevol"]
        type MakeRevol;
        #[doc = "Builds the Revol of base S, axis  A and angle  D. If C is true, S is copied."]
        #[cxx_name = "BRepPrimAPI_MakeRevol_ctor_shape_ax1_real_bool"]
        fn MakeRevol_ctor_shape_ax1_real_bool(
            S: &TopoDS_Shape,
            A: &gp_Ax1,
            D: f64,
            Copy: bool,
        ) -> UniquePtr<MakeRevol>;
        #[doc = "Builds the Revol of base S, axis  A and angle 2*Pi. If C is true, S is copied."]
        #[cxx_name = "BRepPrimAPI_MakeRevol_ctor_shape_ax1_bool"]
        fn MakeRevol_ctor_shape_ax1_bool(
            S: &TopoDS_Shape,
            A: &gp_Ax1,
            Copy: bool,
        ) -> UniquePtr<MakeRevol>;
        #[doc = "Returns the internal sweeping algorithm."]
        #[cxx_name = "Revol"]
        fn revol(self: &MakeRevol) -> &BRepSweep_Revol;
        #[doc = "Builds the resulting shape (redefined from MakeShape)."]
        #[cxx_name = "Build"]
        fn build(self: Pin<&mut MakeRevol>, theRange: &Message_ProgressRange);
        #[doc = "Returns list of shape generated from shape S Warning: shape S must be shape of type VERTEX, EDGE, FACE, SOLID. For shapes of other types method always returns empty list"]
        #[cxx_name = "Generated"]
        fn generated(self: Pin<&mut MakeRevol>, S: &TopoDS_Shape) -> &TopTools_ListOfShape;
        #[doc = "Returns true if the shape S has been deleted."]
        #[cxx_name = "IsDeleted"]
        fn is_deleted(self: Pin<&mut MakeRevol>, S: &TopoDS_Shape) -> bool;
        #[doc = "Check if there are degenerated edges in the result."]
        #[cxx_name = "HasDegenerated"]
        fn has_degenerated(self: &MakeRevol) -> bool;
        #[doc = "Returns the list of degenerated edges"]
        #[cxx_name = "Degenerated"]
        fn degenerated(self: &MakeRevol) -> &TopTools_ListOfShape;
        #[doc = "Returns the first shape of the revol  (coinciding with the generating shape)."]
        #[cxx_name = "BRepPrimAPI_MakeRevol_FirstShape"]
        fn MakeRevol_first_shape(self_: Pin<&mut MakeRevol>) -> UniquePtr<TopoDS_Shape>;
        #[doc = "Returns the TopoDS Shape of the end of the revol."]
        #[cxx_name = "BRepPrimAPI_MakeRevol_LastShape"]
        fn MakeRevol_last_shape(self_: Pin<&mut MakeRevol>) -> UniquePtr<TopoDS_Shape>;
        #[doc = "Returns the TopoDS Shape of the beginning of the revolution, generated with theShape  (subShape of the generating shape)."]
        #[cxx_name = "BRepPrimAPI_MakeRevol_FirstShape"]
        fn MakeRevol_first_shapeshape_2(
            self_: Pin<&mut MakeRevol>,
            theShape: &TopoDS_Shape,
        ) -> UniquePtr<TopoDS_Shape>;
        #[doc = "Returns the TopoDS Shape of the end of the revolution, generated with  theShape (subShape of the  generating shape)."]
        #[cxx_name = "BRepPrimAPI_MakeRevol_LastShape"]
        fn MakeRevol_last_shapeshape_2(
            self_: Pin<&mut MakeRevol>,
            theShape: &TopoDS_Shape,
        ) -> UniquePtr<TopoDS_Shape>;
        #[doc = " ======================== BRepPrimAPI_MakeSphere ========================"]
        #[doc = "Describes functions to build spheres or portions of spheres. A MakeSphere object provides a framework for: -   defining the construction of a sphere, -   implementing the construction algorithm, and -   consulting the result."]
        #[cxx_name = "BRepPrimAPI_MakeSphere"]
        type MakeSphere;
        #[doc = "Make a sphere. @param[in] R  sphere radius"]
        #[cxx_name = "BRepPrimAPI_MakeSphere_ctor_real"]
        fn MakeSphere_ctor_real(R: f64) -> UniquePtr<MakeSphere>;
        #[doc = "Make a sphere (spherical wedge). @param[in] R      sphere radius @param[in] angle  angle between the radii lying within the bounding semidisks"]
        #[cxx_name = "BRepPrimAPI_MakeSphere_ctor_real2"]
        fn MakeSphere_ctor_real2(R: f64, angle: f64) -> UniquePtr<MakeSphere>;
        #[doc = "Make a sphere (spherical segment). @param[in] R  sphere radius @param[in] angle1  first angle defining a spherical segment @param[in] angle2  second angle defining a spherical segment"]
        #[cxx_name = "BRepPrimAPI_MakeSphere_ctor_real3"]
        fn MakeSphere_ctor_real3(R: f64, angle1: f64, angle2: f64) -> UniquePtr<MakeSphere>;
        #[doc = "Make a sphere (spherical segment). @param[in] R       sphere radius @param[in] angle1  first angle defining a spherical segment @param[in] angle2  second angle defining a spherical segment @param[in] angle3  angle between the radii lying within the bounding semidisks"]
        #[cxx_name = "BRepPrimAPI_MakeSphere_ctor_real4"]
        fn MakeSphere_ctor_real4(
            R: f64,
            angle1: f64,
            angle2: f64,
            angle3: f64,
        ) -> UniquePtr<MakeSphere>;
        #[doc = "Make a sphere. @param[in] Center  sphere center coordinates @param[in] R       sphere radius"]
        #[cxx_name = "BRepPrimAPI_MakeSphere_ctor_pnt_real"]
        fn MakeSphere_ctor_pnt_real(Center: &gp_Pnt, R: f64) -> UniquePtr<MakeSphere>;
        #[doc = "Make a sphere (spherical wedge). @param[in] Center  sphere center coordinates @param[in] R       sphere radius @param[in] angle   angle between the radii lying within the bounding semidisks"]
        #[cxx_name = "BRepPrimAPI_MakeSphere_ctor_pnt_real2"]
        fn MakeSphere_ctor_pnt_real2(Center: &gp_Pnt, R: f64, angle: f64) -> UniquePtr<MakeSphere>;
        #[doc = "Make a sphere (spherical segment). @param[in] Center  sphere center coordinates @param[in] R       sphere radius @param[in] angle1  first angle defining a spherical segment @param[in] angle2  second angle defining a spherical segment"]
        #[cxx_name = "BRepPrimAPI_MakeSphere_ctor_pnt_real3"]
        fn MakeSphere_ctor_pnt_real3(
            Center: &gp_Pnt,
            R: f64,
            angle1: f64,
            angle2: f64,
        ) -> UniquePtr<MakeSphere>;
        #[doc = "Make a sphere (spherical segment). @param[in] Center  sphere center coordinates @param[in] R       sphere radius @param[in] angle1  first angle defining a spherical segment @param[in] angle2  second angle defining a spherical segment @param[in] angle3  angle between the radii lying within the bounding semidisks"]
        #[cxx_name = "BRepPrimAPI_MakeSphere_ctor_pnt_real4"]
        fn MakeSphere_ctor_pnt_real4(
            Center: &gp_Pnt,
            R: f64,
            angle1: f64,
            angle2: f64,
            angle3: f64,
        ) -> UniquePtr<MakeSphere>;
        #[doc = "Make a sphere. @param[in] Axis  coordinate system for the construction of the sphere @param[in] R     sphere radius"]
        #[cxx_name = "BRepPrimAPI_MakeSphere_ctor_ax2_real"]
        fn MakeSphere_ctor_ax2_real(Axis: &gp_Ax2, R: f64) -> UniquePtr<MakeSphere>;
        #[doc = "Make a sphere (spherical wedge). @param[in] Axis   coordinate system for the construction of the sphere @param[in] R      sphere radius @param[in] angle  angle between the radii lying within the bounding semidisks"]
        #[cxx_name = "BRepPrimAPI_MakeSphere_ctor_ax2_real2"]
        fn MakeSphere_ctor_ax2_real2(Axis: &gp_Ax2, R: f64, angle: f64) -> UniquePtr<MakeSphere>;
        #[doc = "Make a sphere (spherical segment). @param[in] Axis    coordinate system for the construction of the sphere @param[in] R       sphere radius @param[in] angle1  first angle defining a spherical segment @param[in] angle2  second angle defining a spherical segment"]
        #[cxx_name = "BRepPrimAPI_MakeSphere_ctor_ax2_real3"]
        fn MakeSphere_ctor_ax2_real3(
            Axis: &gp_Ax2,
            R: f64,
            angle1: f64,
            angle2: f64,
        ) -> UniquePtr<MakeSphere>;
        #[doc = "Make a sphere of radius R. For all algorithms The resulting shape is composed of -   a lateral spherical face, -   two planar faces parallel to the plane z = 0 if the sphere is truncated in the v parametric direction, or only one planar face if angle1 is equal to -p/2 or if angle2 is equal to p/2 (these faces are circles in case of a complete truncated sphere), -   and in case of a portion of sphere, two planar faces to shut the shape.(in the planes u = 0 and u = angle)."]
        #[cxx_name = "BRepPrimAPI_MakeSphere_ctor_ax2_real4"]
        fn MakeSphere_ctor_ax2_real4(
            Axis: &gp_Ax2,
            R: f64,
            angle1: f64,
            angle2: f64,
            angle3: f64,
        ) -> UniquePtr<MakeSphere>;
        #[doc = "Returns the algorithm."]
        #[cxx_name = "Sphere"]
        fn sphere(self: Pin<&mut MakeSphere>) -> Pin<&mut BRepPrim_Sphere>;
        #[doc = " ======================== BRepPrimAPI_MakeTorus ========================"]
        #[doc = "Describes functions to build tori or portions of tori. A MakeTorus object provides a framework for: -   defining the construction of a torus, -   implementing the construction algorithm, and -   consulting the result."]
        #[cxx_name = "BRepPrimAPI_MakeTorus"]
        type MakeTorus;
        #[doc = "Make a torus. @param[in] R1  distance from the center of the pipe to the center of the torus @param[in] R2  radius of the pipe"]
        #[cxx_name = "BRepPrimAPI_MakeTorus_ctor_real2"]
        fn MakeTorus_ctor_real2(R1: f64, R2: f64) -> UniquePtr<MakeTorus>;
        #[doc = "Make a section of a torus. @param[in] R1     distance from the center of the pipe to the center of the torus @param[in] R2     radius of the pipe @param[in] angle  angle to create a torus pipe segment"]
        #[cxx_name = "BRepPrimAPI_MakeTorus_ctor_real3"]
        fn MakeTorus_ctor_real3(R1: f64, R2: f64, angle: f64) -> UniquePtr<MakeTorus>;
        #[doc = "Make  a torus with angles on the small circle. @param[in] R1      distance from the center of the pipe to the center of the torus @param[in] R2      radius of the pipe @param[in] angle1  first  angle to create a torus ring segment @param[in] angle2  second angle to create a torus ring segment"]
        #[cxx_name = "BRepPrimAPI_MakeTorus_ctor_real4"]
        fn MakeTorus_ctor_real4(R1: f64, R2: f64, angle1: f64, angle2: f64)
            -> UniquePtr<MakeTorus>;
        #[doc = "Make  a torus with angles on the small circle. @param[in] R1      distance from the center of the pipe to the center of the torus @param[in] R2      radius of the pipe @param[in] angle1  first  angle to create a torus ring segment @param[in] angle2  second angle to create a torus ring segment @param[in] angle   angle to create a torus pipe segment"]
        #[cxx_name = "BRepPrimAPI_MakeTorus_ctor_real5"]
        fn MakeTorus_ctor_real5(
            R1: f64,
            R2: f64,
            angle1: f64,
            angle2: f64,
            angle: f64,
        ) -> UniquePtr<MakeTorus>;
        #[doc = "Make a torus. @param[in] Axes  coordinate system for the construction of the sphere @param[in] R1    distance from the center of the pipe to the center of the torus @param[in] R2    radius of the pipe"]
        #[cxx_name = "BRepPrimAPI_MakeTorus_ctor_ax2_real2"]
        fn MakeTorus_ctor_ax2_real2(Axes: &gp_Ax2, R1: f64, R2: f64) -> UniquePtr<MakeTorus>;
        #[doc = "Make a section of a torus. @param[in] Axes   coordinate system for the construction of the sphere @param[in] R1     distance from the center of the pipe to the center of the torus @param[in] R2     radius of the pipe @param[in] angle  angle to create a torus pipe segment"]
        #[cxx_name = "BRepPrimAPI_MakeTorus_ctor_ax2_real3"]
        fn MakeTorus_ctor_ax2_real3(
            Axes: &gp_Ax2,
            R1: f64,
            R2: f64,
            angle: f64,
        ) -> UniquePtr<MakeTorus>;
        #[doc = "Make a torus. @param[in] Axes    coordinate system for the construction of the sphere @param[in] R1      distance from the center of the pipe to the center of the torus @param[in] R2      radius of the pipe @param[in] angle1  first  angle to create a torus ring segment @param[in] angle2  second angle to create a torus ring segment"]
        #[cxx_name = "BRepPrimAPI_MakeTorus_ctor_ax2_real4"]
        fn MakeTorus_ctor_ax2_real4(
            Axes: &gp_Ax2,
            R1: f64,
            R2: f64,
            angle1: f64,
            angle2: f64,
        ) -> UniquePtr<MakeTorus>;
        #[doc = "Make a section of a torus of radii R1 R2. For all algorithms The resulting shape is composed of -      a lateral toroidal face, -      two conical faces (defined  by the equation v = angle1 and v = angle2) if the sphere is truncated in the v parametric direction (they may be cylindrical faces in some particular conditions), and in case of a portion of torus, two planar faces to close the shape.(in the planes u = 0 and u = angle). Notes: -      The u parameter corresponds to a rotation angle around the Z axis. -      The circle whose radius is equal to the minor radius, located in the plane defined by the X axis and the Z axis, centered on the X axis, on its positive side, and positioned at a distance from the origin equal to the major radius, is the reference circle of the torus. The rotation around an axis parallel to the Y axis and passing through the center of the reference circle gives the v parameter on the reference circle. The X axis gives the origin of the v parameter. Near 0, as v increases, the Z coordinate decreases."]
        #[cxx_name = "BRepPrimAPI_MakeTorus_ctor_ax2_real5"]
        fn MakeTorus_ctor_ax2_real5(
            Axes: &gp_Ax2,
            R1: f64,
            R2: f64,
            angle1: f64,
            angle2: f64,
            angle: f64,
        ) -> UniquePtr<MakeTorus>;
        #[doc = "Returns the algorithm."]
        #[cxx_name = "Torus"]
        fn torus(self: Pin<&mut MakeTorus>) -> Pin<&mut BRepPrim_Torus>;
    }
    impl UniquePtr<MakeBox> {}
    impl UniquePtr<MakeCone> {}
    impl UniquePtr<MakeCylinder> {}
    impl UniquePtr<MakePrism> {}
    impl UniquePtr<MakeRevol> {}
    impl UniquePtr<MakeSphere> {}
    impl UniquePtr<MakeTorus> {}
}
pub use ffi::MakeBox;
impl MakeBox {
    #[doc = "Default constructor"]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::MakeBox_ctor()
    }

    #[doc = "Make a box with a corner at 0,0,0 and the other dx,dy,dz"]
    pub fn new_real3(dx: f64, dy: f64, dz: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeBox_ctor_real3(dx, dy, dz)
    }

    #[doc = "Make a box with a corner at P and size dx, dy, dz."]
    pub fn new_pnt_real3(P: &ffi::gp_Pnt, dx: f64, dy: f64, dz: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeBox_ctor_pnt_real3(P, dx, dy, dz)
    }

    #[doc = "Make a box with corners P1,P2."]
    pub fn new_pnt2(P1: &ffi::gp_Pnt, P2: &ffi::gp_Pnt) -> cxx::UniquePtr<Self> {
        ffi::MakeBox_ctor_pnt2(P1, P2)
    }

    #[doc = "Make a box with Ax2 (the left corner and the axis) and size dx, dy, dz."]
    pub fn new_ax2_real3(Axes: &ffi::gp_Ax2, dx: f64, dy: f64, dz: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeBox_ctor_ax2_real3(Axes, dx, dy, dz)
    }
}
pub use ffi::MakeCone;
impl MakeCone {
    #[doc = "Make a cone. @param[in] R1  cone bottom radius, may be null (z = 0) @param[in] R2  cone top radius, may be null (z = H) @param[in] H   cone height"]
    pub fn new_real3(R1: f64, R2: f64, H: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeCone_ctor_real3(R1, R2, H)
    }

    #[doc = "Make a cone. @param[in] R1     cone bottom radius, may be null (z = 0) @param[in] R2     cone top radius, may be null (z = H) @param[in] H      cone height @param[in] angle  angle to create a part cone"]
    pub fn new_real4(R1: f64, R2: f64, H: f64, angle: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeCone_ctor_real4(R1, R2, H, angle)
    }

    #[doc = "Make a cone. @param[in] axes  coordinate system for the construction of the cone @param[in] R1    cone bottom radius, may be null (z = 0) @param[in] R2    cone top radius, may be null (z = H) @param[in] H     cone height"]
    pub fn new_ax2_real3(Axes: &ffi::gp_Ax2, R1: f64, R2: f64, H: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeCone_ctor_ax2_real3(Axes, R1, R2, H)
    }

    #[doc = "Make a cone of height H radius R1 in the plane z = 0, R2 in the plane Z = H. R1 and R2 may be null. Take a section of <angle> Constructs a cone, or a portion of a cone, of height H, and radius R1 in the plane z = 0 and R2 in the plane z = H. The result is a sharp cone if R1 or R2 is equal to 0. The cone is constructed about the \"Z Axis\" of either: -   the global coordinate system, or -   the local coordinate system Axes. It is limited in these coordinate systems as follows: -   in the v parametric direction (the Z coordinate), by the two parameter values 0 and H, -   and in the u parametric direction (defined by the angle of rotation around the Z axis), in the case of a portion of a cone, by the two parameter values 0 and angle. Angle is given in radians. The resulting shape is composed of: -   a lateral conical face -   two planar faces in the planes z = 0 and z = H, or only one planar face in one of these two planes if a radius value is null (in the case of a complete cone, these faces are circles), and -   and in the case of a portion of a cone, two planar faces to close the shape. (either two parallelograms or two triangles, in the planes u = 0 and u = angle). Exceptions Standard_DomainError if: -   H is less than or equal to Precision::Confusion(), or -   the half-angle at the apex of the cone, defined by R1, R2 and H, is less than Precision::Confusion()/H, or greater than (Pi/2)-Precision::Confusion()/H.f"]
    pub fn new_ax2_real4(
        Axes: &ffi::gp_Ax2,
        R1: f64,
        R2: f64,
        H: f64,
        angle: f64,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakeCone_ctor_ax2_real4(Axes, R1, R2, H, angle)
    }
}
pub use ffi::MakeCylinder;
impl MakeCylinder {
    #[doc = "Make a cylinder. @param[in] R  cylinder radius @param[in] H  cylinder height"]
    pub fn new_real2(R: f64, H: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeCylinder_ctor_real2(R, H)
    }

    #[doc = "Make a cylinder (part cylinder). @param[in] R      cylinder radius @param[in] H      cylinder height @param[in] Angle  defines the missing portion of the cylinder"]
    pub fn new_real3(R: f64, H: f64, Angle: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeCylinder_ctor_real3(R, H, Angle)
    }

    #[doc = "Make a cylinder of radius R and length H. @param[in] Axes  coordinate system for the construction of the cylinder @param[in] R     cylinder radius @param[in] H     cylinder height"]
    pub fn new_ax2_real2(Axes: &ffi::gp_Ax2, R: f64, H: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeCylinder_ctor_ax2_real2(Axes, R, H)
    }

    #[doc = "Make a cylinder   of  radius R  and  length H with angle  H. Constructs -   a cylinder of radius R and height H, or -   a portion of cylinder of radius R and height H, and of the angle Angle defining the missing portion of the cylinder. The cylinder is constructed about the \"Z Axis\" of either: -   the global coordinate system, or -   the local coordinate system Axes. It is limited in this coordinate system as follows: -   in the v parametric direction (the Z axis), by the two parameter values 0 and H, -   and in the u parametric direction (the rotation angle around the Z Axis), in the case of a portion of a cylinder, by the two parameter values 0 and Angle. Angle is given in radians. The resulting shape is composed of: -   a lateral cylindrical face, -   two planar faces in the planes z = 0 and z = H (in the case of a complete cylinder, these faces are circles), and -   in case of a portion of a cylinder, two additional planar faces to close the shape.(two rectangles in the planes u = 0 and u = Angle). Exceptions Standard_DomainError if: -   R is less than or equal to Precision::Confusion(), or -   H is less than or equal to Precision::Confusion()."]
    pub fn new_ax2_real3(Axes: &ffi::gp_Ax2, R: f64, H: f64, Angle: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeCylinder_ctor_ax2_real3(Axes, R, H, Angle)
    }
}
pub use ffi::MakePrism;
impl MakePrism {
    #[doc = "Builds the prism of base S and vector V. If C is true, S is copied. If Canonize is true then generated surfaces are attempted to be canonized in simple types"]
    pub fn new_shape_vec_bool2(
        S: &ffi::TopoDS_Shape,
        V: &ffi::gp_Vec,
        Copy: bool,
        Canonize: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakePrism_ctor_shape_vec_bool2(S, V, Copy, Canonize)
    }

    #[doc = "Builds a semi-infinite or an infinite prism of base S. If Inf is true the prism  is infinite, if Inf is false the prism is semi-infinite (in the direction D).  If C is true S is copied (for semi-infinite prisms). If Canonize is true then generated surfaces are attempted to be canonized in simple types"]
    pub fn new_shape_dir_bool3(
        S: &ffi::TopoDS_Shape,
        D: &ffi::gp_Dir,
        Inf: bool,
        Copy: bool,
        Canonize: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakePrism_ctor_shape_dir_bool3(S, D, Inf, Copy, Canonize)
    }
}
pub use ffi::MakeRevol;
impl MakeRevol {
    #[doc = "Builds the Revol of base S, axis  A and angle  D. If C is true, S is copied."]
    pub fn new_shape_ax1_real_bool(
        S: &ffi::TopoDS_Shape,
        A: &ffi::gp_Ax1,
        D: f64,
        Copy: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakeRevol_ctor_shape_ax1_real_bool(S, A, D, Copy)
    }

    #[doc = "Builds the Revol of base S, axis  A and angle 2*Pi. If C is true, S is copied."]
    pub fn new_shape_ax1_bool(
        S: &ffi::TopoDS_Shape,
        A: &ffi::gp_Ax1,
        Copy: bool,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakeRevol_ctor_shape_ax1_bool(S, A, Copy)
    }
}
pub use ffi::MakeSphere;
impl MakeSphere {
    #[doc = "Make a sphere. @param[in] R  sphere radius"]
    pub fn new_real(R: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeSphere_ctor_real(R)
    }

    #[doc = "Make a sphere (spherical wedge). @param[in] R      sphere radius @param[in] angle  angle between the radii lying within the bounding semidisks"]
    pub fn new_real2(R: f64, angle: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeSphere_ctor_real2(R, angle)
    }

    #[doc = "Make a sphere (spherical segment). @param[in] R  sphere radius @param[in] angle1  first angle defining a spherical segment @param[in] angle2  second angle defining a spherical segment"]
    pub fn new_real3(R: f64, angle1: f64, angle2: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeSphere_ctor_real3(R, angle1, angle2)
    }

    #[doc = "Make a sphere (spherical segment). @param[in] R       sphere radius @param[in] angle1  first angle defining a spherical segment @param[in] angle2  second angle defining a spherical segment @param[in] angle3  angle between the radii lying within the bounding semidisks"]
    pub fn new_real4(R: f64, angle1: f64, angle2: f64, angle3: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeSphere_ctor_real4(R, angle1, angle2, angle3)
    }

    #[doc = "Make a sphere. @param[in] Center  sphere center coordinates @param[in] R       sphere radius"]
    pub fn new_pnt_real(Center: &ffi::gp_Pnt, R: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeSphere_ctor_pnt_real(Center, R)
    }

    #[doc = "Make a sphere (spherical wedge). @param[in] Center  sphere center coordinates @param[in] R       sphere radius @param[in] angle   angle between the radii lying within the bounding semidisks"]
    pub fn new_pnt_real2(Center: &ffi::gp_Pnt, R: f64, angle: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeSphere_ctor_pnt_real2(Center, R, angle)
    }

    #[doc = "Make a sphere (spherical segment). @param[in] Center  sphere center coordinates @param[in] R       sphere radius @param[in] angle1  first angle defining a spherical segment @param[in] angle2  second angle defining a spherical segment"]
    pub fn new_pnt_real3(
        Center: &ffi::gp_Pnt,
        R: f64,
        angle1: f64,
        angle2: f64,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakeSphere_ctor_pnt_real3(Center, R, angle1, angle2)
    }

    #[doc = "Make a sphere (spherical segment). @param[in] Center  sphere center coordinates @param[in] R       sphere radius @param[in] angle1  first angle defining a spherical segment @param[in] angle2  second angle defining a spherical segment @param[in] angle3  angle between the radii lying within the bounding semidisks"]
    pub fn new_pnt_real4(
        Center: &ffi::gp_Pnt,
        R: f64,
        angle1: f64,
        angle2: f64,
        angle3: f64,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakeSphere_ctor_pnt_real4(Center, R, angle1, angle2, angle3)
    }

    #[doc = "Make a sphere. @param[in] Axis  coordinate system for the construction of the sphere @param[in] R     sphere radius"]
    pub fn new_ax2_real(Axis: &ffi::gp_Ax2, R: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeSphere_ctor_ax2_real(Axis, R)
    }

    #[doc = "Make a sphere (spherical wedge). @param[in] Axis   coordinate system for the construction of the sphere @param[in] R      sphere radius @param[in] angle  angle between the radii lying within the bounding semidisks"]
    pub fn new_ax2_real2(Axis: &ffi::gp_Ax2, R: f64, angle: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeSphere_ctor_ax2_real2(Axis, R, angle)
    }

    #[doc = "Make a sphere (spherical segment). @param[in] Axis    coordinate system for the construction of the sphere @param[in] R       sphere radius @param[in] angle1  first angle defining a spherical segment @param[in] angle2  second angle defining a spherical segment"]
    pub fn new_ax2_real3(
        Axis: &ffi::gp_Ax2,
        R: f64,
        angle1: f64,
        angle2: f64,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakeSphere_ctor_ax2_real3(Axis, R, angle1, angle2)
    }

    #[doc = "Make a sphere of radius R. For all algorithms The resulting shape is composed of -   a lateral spherical face, -   two planar faces parallel to the plane z = 0 if the sphere is truncated in the v parametric direction, or only one planar face if angle1 is equal to -p/2 or if angle2 is equal to p/2 (these faces are circles in case of a complete truncated sphere), -   and in case of a portion of sphere, two planar faces to shut the shape.(in the planes u = 0 and u = angle)."]
    pub fn new_ax2_real4(
        Axis: &ffi::gp_Ax2,
        R: f64,
        angle1: f64,
        angle2: f64,
        angle3: f64,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakeSphere_ctor_ax2_real4(Axis, R, angle1, angle2, angle3)
    }
}
pub use ffi::MakeTorus;
impl MakeTorus {
    #[doc = "Make a torus. @param[in] R1  distance from the center of the pipe to the center of the torus @param[in] R2  radius of the pipe"]
    pub fn new_real2(R1: f64, R2: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeTorus_ctor_real2(R1, R2)
    }

    #[doc = "Make a section of a torus. @param[in] R1     distance from the center of the pipe to the center of the torus @param[in] R2     radius of the pipe @param[in] angle  angle to create a torus pipe segment"]
    pub fn new_real3(R1: f64, R2: f64, angle: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeTorus_ctor_real3(R1, R2, angle)
    }

    #[doc = "Make  a torus with angles on the small circle. @param[in] R1      distance from the center of the pipe to the center of the torus @param[in] R2      radius of the pipe @param[in] angle1  first  angle to create a torus ring segment @param[in] angle2  second angle to create a torus ring segment"]
    pub fn new_real4(R1: f64, R2: f64, angle1: f64, angle2: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeTorus_ctor_real4(R1, R2, angle1, angle2)
    }

    #[doc = "Make  a torus with angles on the small circle. @param[in] R1      distance from the center of the pipe to the center of the torus @param[in] R2      radius of the pipe @param[in] angle1  first  angle to create a torus ring segment @param[in] angle2  second angle to create a torus ring segment @param[in] angle   angle to create a torus pipe segment"]
    pub fn new_real5(
        R1: f64,
        R2: f64,
        angle1: f64,
        angle2: f64,
        angle: f64,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakeTorus_ctor_real5(R1, R2, angle1, angle2, angle)
    }

    #[doc = "Make a torus. @param[in] Axes  coordinate system for the construction of the sphere @param[in] R1    distance from the center of the pipe to the center of the torus @param[in] R2    radius of the pipe"]
    pub fn new_ax2_real2(Axes: &ffi::gp_Ax2, R1: f64, R2: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeTorus_ctor_ax2_real2(Axes, R1, R2)
    }

    #[doc = "Make a section of a torus. @param[in] Axes   coordinate system for the construction of the sphere @param[in] R1     distance from the center of the pipe to the center of the torus @param[in] R2     radius of the pipe @param[in] angle  angle to create a torus pipe segment"]
    pub fn new_ax2_real3(Axes: &ffi::gp_Ax2, R1: f64, R2: f64, angle: f64) -> cxx::UniquePtr<Self> {
        ffi::MakeTorus_ctor_ax2_real3(Axes, R1, R2, angle)
    }

    #[doc = "Make a torus. @param[in] Axes    coordinate system for the construction of the sphere @param[in] R1      distance from the center of the pipe to the center of the torus @param[in] R2      radius of the pipe @param[in] angle1  first  angle to create a torus ring segment @param[in] angle2  second angle to create a torus ring segment"]
    pub fn new_ax2_real4(
        Axes: &ffi::gp_Ax2,
        R1: f64,
        R2: f64,
        angle1: f64,
        angle2: f64,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakeTorus_ctor_ax2_real4(Axes, R1, R2, angle1, angle2)
    }

    #[doc = "Make a section of a torus of radii R1 R2. For all algorithms The resulting shape is composed of -      a lateral toroidal face, -      two conical faces (defined  by the equation v = angle1 and v = angle2) if the sphere is truncated in the v parametric direction (they may be cylindrical faces in some particular conditions), and in case of a portion of torus, two planar faces to close the shape.(in the planes u = 0 and u = angle). Notes: -      The u parameter corresponds to a rotation angle around the Z axis. -      The circle whose radius is equal to the minor radius, located in the plane defined by the X axis and the Z axis, centered on the X axis, on its positive side, and positioned at a distance from the origin equal to the major radius, is the reference circle of the torus. The rotation around an axis parallel to the Y axis and passing through the center of the reference circle gives the v parameter on the reference circle. The X axis gives the origin of the v parameter. Near 0, as v increases, the Z coordinate decreases."]
    pub fn new_ax2_real5(
        Axes: &ffi::gp_Ax2,
        R1: f64,
        R2: f64,
        angle1: f64,
        angle2: f64,
        angle: f64,
    ) -> cxx::UniquePtr<Self> {
        ffi::MakeTorus_ctor_ax2_real5(Axes, R1, R2, angle1, angle2, angle)
    }
}
