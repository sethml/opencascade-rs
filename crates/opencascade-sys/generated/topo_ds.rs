#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_topo_ds.hxx");
        #[doc = "Location from top_loc module"]
        type TopLoc_Location = crate::top_loc::ffi::Location;
        #[doc = "ShapeEnum from top_abs module"]
        type TopAbs_ShapeEnum = crate::top_abs::ffi::TopAbs_ShapeEnum;
        #[doc = "Orientation from top_abs module"]
        type TopAbs_Orientation = crate::top_abs::ffi::TopAbs_Orientation;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "TopoDS_TShape"]
        type TopoDS_TShape;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleTopoDSTShape"]
        type HandleTopoDSTShape;
        #[doc = " ======================== TopoDS_Shape ========================"]
        #[doc = "Describes a shape which - references an underlying shape with the potential to be given a location and an orientation - has a location for the underlying shape, giving its placement in the local coordinate system - has an orientation for the underlying shape, in terms of its geometry (as opposed to orientation in relation to other shapes). Note: A Shape is empty if it references an underlying shape which has an empty list of shapes."]
        #[cxx_name = "TopoDS_Shape"]
        type Shape;
        #[doc = "Creates a NULL Shape referring to nothing."]
        #[cxx_name = "TopoDS_Shape_ctor"]
        fn Shape_ctor() -> UniquePtr<Shape>;
        #[doc = "Returns true if this shape is null. In other words, it references no underlying shape with the potential to be given a location and an orientation."]
        #[cxx_name = "IsNull"]
        fn is_null(self: &Shape) -> bool;
        #[doc = "Destroys the reference to the underlying shape stored in this shape. As a result, this shape becomes null."]
        #[cxx_name = "Nullify"]
        fn nullify(self: Pin<&mut Shape>);
        #[doc = "Returns the shape local coordinate system."]
        #[cxx_name = "Location"]
        fn location(self: &Shape) -> &TopLoc_Location;
        #[doc = "Sets the shape local coordinate system. @param theLoc the new local coordinate system. @param theRaiseExc flag to raise exception in case of transformation with scale or negative."]
        #[cxx_name = "Location"]
        fn locationlocation_2(self: Pin<&mut Shape>, theLoc: &TopLoc_Location, theRaiseExc: bool);
        #[doc = "Returns the shape orientation."]
        #[cxx_name = "Orientation"]
        fn orientation(self: &Shape) -> TopAbs_Orientation;
        #[doc = "Returns a handle to the actual shape implementation."]
        #[cxx_name = "TShape"]
        fn t_shape(self: &Shape) -> &HandleTopoDSTShape;
        #[doc = "Returns the value of the TopAbs_ShapeEnum enumeration that corresponds to this shape, for example VERTEX, EDGE, and so on. Exceptions Standard_NullObject if this shape is null."]
        #[cxx_name = "ShapeType"]
        fn shape_type(self: &Shape) -> TopAbs_ShapeEnum;
        #[doc = "Returns the free flag."]
        #[cxx_name = "Free"]
        fn free(self: &Shape) -> bool;
        #[doc = "Sets the free flag."]
        #[cxx_name = "Free"]
        fn freebool_2(self: Pin<&mut Shape>, theIsFree: bool);
        #[doc = "Returns the locked flag."]
        #[cxx_name = "Locked"]
        fn locked(self: &Shape) -> bool;
        #[doc = "Sets the locked flag."]
        #[cxx_name = "Locked"]
        fn lockedbool_2(self: Pin<&mut Shape>, theIsLocked: bool);
        #[doc = "Returns the modification flag."]
        #[cxx_name = "Modified"]
        fn modified(self: &Shape) -> bool;
        #[doc = "Sets the modification flag."]
        #[cxx_name = "Modified"]
        fn modifiedbool_2(self: Pin<&mut Shape>, theIsModified: bool);
        #[doc = "Returns the checked flag."]
        #[cxx_name = "Checked"]
        fn checked(self: &Shape) -> bool;
        #[doc = "Sets the checked flag."]
        #[cxx_name = "Checked"]
        fn checkedbool_2(self: Pin<&mut Shape>, theIsChecked: bool);
        #[doc = "Returns the orientability flag."]
        #[cxx_name = "Orientable"]
        fn orientable(self: &Shape) -> bool;
        #[doc = "Sets the orientability flag."]
        #[cxx_name = "Orientable"]
        fn orientablebool_2(self: Pin<&mut Shape>, theIsOrientable: bool);
        #[doc = "Returns the closedness flag."]
        #[cxx_name = "Closed"]
        fn closed(self: &Shape) -> bool;
        #[doc = "Sets the closedness flag."]
        #[cxx_name = "Closed"]
        fn closedbool_2(self: Pin<&mut Shape>, theIsClosed: bool);
        #[doc = "Returns the infinity flag."]
        #[cxx_name = "Infinite"]
        fn infinite(self: &Shape) -> bool;
        #[doc = "Sets the infinity flag."]
        #[cxx_name = "Infinite"]
        fn infinitebool_2(self: Pin<&mut Shape>, theIsInfinite: bool);
        #[doc = "Returns the convexness flag."]
        #[cxx_name = "Convex"]
        fn convex(self: &Shape) -> bool;
        #[doc = "Sets the convexness flag."]
        #[cxx_name = "Convex"]
        fn convexbool_2(self: Pin<&mut Shape>, theIsConvex: bool);
        #[doc = "Multiplies the Shape location by thePosition. @param thePosition the transformation to apply. @param theRaiseExc flag to raise exception in case of transformation with scale or negative."]
        #[cxx_name = "Move"]
        fn move_(self: Pin<&mut Shape>, thePosition: &TopLoc_Location, theRaiseExc: bool);
        #[doc = "Reverses the orientation, using the Reverse method from the TopAbs package."]
        #[cxx_name = "Reverse"]
        fn reverse(self: Pin<&mut Shape>);
        #[doc = "Complements the orientation, using the  Complement method from the TopAbs package."]
        #[cxx_name = "Complement"]
        fn complement(self: Pin<&mut Shape>);
        #[doc = "Returns the number of direct sub-shapes (children). @sa TopoDS_Iterator for accessing sub-shapes"]
        #[cxx_name = "NbChildren"]
        fn nb_children(self: &Shape) -> i32;
        #[doc = "Returns True if two shapes  are partners, i.e.  if they   share   the   same  TShape.  Locations  and Orientations may differ."]
        #[cxx_name = "IsPartner"]
        fn is_partner(self: &Shape, theOther: &Shape) -> bool;
        #[doc = "Returns True if two shapes are same, i.e.  if they share  the  same TShape  with the same  Locations. Orientations may differ."]
        #[cxx_name = "IsSame"]
        fn is_same(self: &Shape, theOther: &Shape) -> bool;
        #[doc = "Returns True if two shapes are equal, i.e. if they share the same TShape with  the same Locations and Orientations."]
        #[cxx_name = "IsEqual"]
        fn is_equal(self: &Shape, theOther: &Shape) -> bool;
        #[doc = "Negation of the IsEqual method."]
        #[cxx_name = "IsNotEqual"]
        fn is_not_equal(self: &Shape, theOther: &Shape) -> bool;
        #[doc = "Replace   <me> by  a  new   Shape with the    same Orientation and Location and a new TShape with the same geometry and no sub-shapes."]
        #[cxx_name = "EmptyCopy"]
        fn empty_copy(self: Pin<&mut Shape>);
        #[cxx_name = "TShape"]
        fn t_shapehandletshape_2(self: Pin<&mut Shape>, theTShape: &HandleTopoDSTShape);
        #[doc = "Returns a  shape  similar to <me> with the local coordinate system set to <Loc>. @param theLoc the new local coordinate system. @param theRaiseExc flag to raise exception in case of transformation with scale or negative. @return the located shape."]
        #[cxx_name = "TopoDS_Shape_Located"]
        fn Shape_located(
            self_: &Shape,
            theLoc: &TopLoc_Location,
            theRaiseExc: bool,
        ) -> UniquePtr<Shape>;
        #[doc = "Returns a shape similar to <me> with a location multiplied by thePosition. @param thePosition the transformation to apply. @param theRaiseExc flag to raise exception in case of transformation with scale or negative. @return the moved shape."]
        #[cxx_name = "TopoDS_Shape_Moved"]
        fn Shape_moved(
            self_: &Shape,
            thePosition: &TopLoc_Location,
            theRaiseExc: bool,
        ) -> UniquePtr<Shape>;
        #[doc = "Returns    a shape  similar    to  <me>  with  the orientation  reversed, using  the   Reverse method from the TopAbs package."]
        #[cxx_name = "TopoDS_Shape_Reversed"]
        fn Shape_reversed(self_: &Shape) -> UniquePtr<Shape>;
        #[doc = "Returns  a   shape  similar  to   <me>   with  the orientation complemented,  using   the  Complement method from the TopAbs package."]
        #[cxx_name = "TopoDS_Shape_Complemented"]
        fn Shape_complemented(self_: &Shape) -> UniquePtr<Shape>;
        #[doc = "Returns a new Shape with the  same Orientation and Location and  a new TShape  with the same geometry and no sub-shapes."]
        #[cxx_name = "TopoDS_Shape_EmptyCopied"]
        fn Shape_empty_copied(self_: &Shape) -> UniquePtr<Shape>;
        #[doc = " ======================== TopoDS_Vertex ========================"]
        #[doc = "Describes a vertex which - references an underlying vertex with the potential to be given a location and an orientation - has a location for the underlying vertex, giving its placement in the local coordinate system - has an orientation for the underlying vertex, in terms of its geometry (as opposed to orientation in relation to other shapes)."]
        #[cxx_name = "TopoDS_Vertex"]
        type Vertex;
        #[doc = "Undefined Vertex."]
        #[cxx_name = "TopoDS_Vertex_ctor"]
        fn Vertex_ctor() -> UniquePtr<Vertex>;
        #[doc = " ======================== TopoDS_Edge ========================"]
        #[doc = "Describes an edge which - references an underlying edge with the potential to be given a location and an orientation - has a location for the underlying edge, giving its placement in the local coordinate system - has an orientation for the underlying edge, in terms of its geometry (as opposed to orientation in relation to other shapes)."]
        #[cxx_name = "TopoDS_Edge"]
        type Edge;
        #[doc = "Undefined Edge."]
        #[cxx_name = "TopoDS_Edge_ctor"]
        fn Edge_ctor() -> UniquePtr<Edge>;
        #[doc = " ======================== TopoDS_Wire ========================"]
        #[doc = "Describes a wire which - references an underlying wire with the potential to be given a location and an orientation - has a location for the underlying wire, giving its placement in the local coordinate system - has an orientation for the underlying wire, in terms of its geometry (as opposed to orientation in relation to other shapes)."]
        #[cxx_name = "TopoDS_Wire"]
        type Wire;
        #[doc = "Undefined Wire."]
        #[cxx_name = "TopoDS_Wire_ctor"]
        fn Wire_ctor() -> UniquePtr<Wire>;
        #[doc = " ======================== TopoDS_Face ========================"]
        #[doc = "Describes a face which - references an underlying face with the potential to be given a location and an orientation - has a location for the underlying face, giving its placement in the local coordinate system - has an orientation for the underlying face, in terms of its geometry (as opposed to orientation in relation to other shapes)."]
        #[cxx_name = "TopoDS_Face"]
        type Face;
        #[doc = "Undefined Face."]
        #[cxx_name = "TopoDS_Face_ctor"]
        fn Face_ctor() -> UniquePtr<Face>;
        #[doc = " ======================== TopoDS_Shell ========================"]
        #[doc = "Describes a shell which - references an underlying shell with the potential to be given a location and an orientation - has a location for the underlying shell, giving its placement in the local coordinate system - has an orientation for the underlying shell, in terms of its geometry (as opposed to orientation in relation to other shapes)."]
        #[cxx_name = "TopoDS_Shell"]
        type Shell;
        #[doc = "Constructs an Undefined Shell."]
        #[cxx_name = "TopoDS_Shell_ctor"]
        fn Shell_ctor() -> UniquePtr<Shell>;
        #[doc = " ======================== TopoDS_Solid ========================"]
        #[doc = "Describes a solid shape which - references an underlying solid shape with the potential to be given a location and an orientation - has a location for the underlying shape, giving its placement in the local coordinate system - has an orientation for the underlying shape, in terms of its geometry (as opposed to orientation in relation to other shapes)."]
        #[cxx_name = "TopoDS_Solid"]
        type Solid;
        #[doc = "Constructs an Undefined Solid."]
        #[cxx_name = "TopoDS_Solid_ctor"]
        fn Solid_ctor() -> UniquePtr<Solid>;
        #[doc = " ======================== TopoDS_Compound ========================"]
        #[doc = "Describes a compound which - references an underlying compound with the potential to be given a location and an orientation - has a location for the underlying compound, giving its placement in the local coordinate system - has an orientation for the underlying compound, in terms of its geometry (as opposed to orientation in relation to other shapes). Casts shape S to the more specialized return type, Compound."]
        #[cxx_name = "TopoDS_Compound"]
        type Compound;
        #[doc = "Constructs an Undefined Compound."]
        #[cxx_name = "TopoDS_Compound_ctor"]
        fn Compound_ctor() -> UniquePtr<Compound>;
        #[doc = " ======================== TopoDS_CompSolid ========================"]
        #[doc = "Describes a composite solid which - references an underlying composite solid with the potential to be given a location and an orientation - has a location for the underlying composite solid, giving its placement in the local coordinate system - has an orientation for the underlying composite solid, in terms of its geometry (as opposed to orientation in relation to other shapes). Casts shape S to the more specialized return type, CompSolid."]
        #[cxx_name = "TopoDS_CompSolid"]
        type CompSolid;
        #[doc = "Constructs an Undefined CompSolid."]
        #[cxx_name = "TopoDS_CompSolid_ctor"]
        fn CompSolid_ctor() -> UniquePtr<CompSolid>;
        #[doc = " ======================== TopoDS_Builder ========================"]
        #[doc = "A  Builder is used   to  create  Topological  Data Structures.It is the root of the Builder class hierarchy. There are three groups of methods in the Builder : The Make methods create Shapes. The Add method includes a Shape in another Shape. The Remove  method  removes a  Shape from an other Shape. The methods in Builder are not static. They can be redefined in inherited builders. This   Builder does not  provide   methods to Make Vertices,  Edges, Faces,  Shells  or Solids. These methods are  provided  in  the inherited  Builders as they must provide the geometry. The Add method check for the following rules : - Any SHAPE can be added in a COMPOUND. - Only SOLID can be added in a COMPSOLID. - Only SHELL, EDGE and VERTEX can be added in a SOLID. EDGE and VERTEX as to be INTERNAL or EXTERNAL. - Only FACE can be added in a SHELL. - Only WIRE and VERTEX can be added in a FACE. VERTEX as to be INTERNAL or EXTERNAL. - Only EDGE can be added in a WIRE. - Only VERTEX can be added in an EDGE. - Nothing can be added in a VERTEX."]
        #[cxx_name = "TopoDS_Builder"]
        type Builder;
        #[doc = "Make an empty Wire."]
        #[cxx_name = "MakeWire"]
        fn make_wire(self: &Builder, W: Pin<&mut Wire>);
        #[doc = "Make an empty Shell."]
        #[cxx_name = "MakeShell"]
        fn make_shell(self: &Builder, S: Pin<&mut Shell>);
        #[doc = "Make a Solid covering the whole 3D space."]
        #[cxx_name = "MakeSolid"]
        fn make_solid(self: &Builder, S: Pin<&mut Solid>);
        #[doc = "Make an empty Composite Solid."]
        #[cxx_name = "MakeCompSolid"]
        fn make_comp_solid(self: &Builder, C: Pin<&mut CompSolid>);
        #[doc = "Make an empty Compound."]
        #[cxx_name = "MakeCompound"]
        fn make_compound(self: &Builder, C: Pin<&mut Compound>);
        #[doc = "Add the Shape C in the Shape S. Exceptions - TopoDS_FrozenShape if S is not free and cannot be modified. - TopoDS__UnCompatibleShapes if S and C are not compatible."]
        #[cxx_name = "Add"]
        fn add(self: &Builder, S: Pin<&mut Shape>, C: &Shape);
        #[doc = "Remove the Shape C from the Shape S. Exceptions TopoDS_FrozenShape if S is frozen and cannot be modified."]
        #[cxx_name = "Remove"]
        fn remove(self: &Builder, S: Pin<&mut Shape>, C: &Shape);
    }
    impl UniquePtr<Shape> {}
    impl UniquePtr<Vertex> {}
    impl UniquePtr<Edge> {}
    impl UniquePtr<Wire> {}
    impl UniquePtr<Face> {}
    impl UniquePtr<Shell> {}
    impl UniquePtr<Solid> {}
    impl UniquePtr<Compound> {}
    impl UniquePtr<CompSolid> {}
    impl UniquePtr<Builder> {}
}
pub use ffi::Shape;
impl Shape {
    #[doc = "Creates a NULL Shape referring to nothing."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Shape_ctor()
    }
}
pub use ffi::Vertex;
impl Vertex {
    #[doc = "Undefined Vertex."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Vertex_ctor()
    }
}
pub use ffi::Edge;
impl Edge {
    #[doc = "Undefined Edge."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Edge_ctor()
    }
}
pub use ffi::Wire;
impl Wire {
    #[doc = "Undefined Wire."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Wire_ctor()
    }
}
pub use ffi::Face;
impl Face {
    #[doc = "Undefined Face."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Face_ctor()
    }
}
pub use ffi::Shell;
impl Shell {
    #[doc = "Constructs an Undefined Shell."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Shell_ctor()
    }
}
pub use ffi::Solid;
impl Solid {
    #[doc = "Constructs an Undefined Solid."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Solid_ctor()
    }
}
pub use ffi::Compound;
impl Compound {
    #[doc = "Constructs an Undefined Compound."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Compound_ctor()
    }
}
pub use ffi::CompSolid;
impl CompSolid {
    #[doc = "Constructs an Undefined CompSolid."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::CompSolid_ctor()
    }
}
pub use ffi::Builder;
