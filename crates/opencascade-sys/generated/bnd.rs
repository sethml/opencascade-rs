#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_bnd.hxx");
        #[doc = "HArray1OfPnt from t_colgp module"]
        type TColgp_HArray1OfPnt = crate::t_colgp::ffi::HArray1OfPnt;
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
        #[cxx_name = "TColgp_Array1OfPnt"]
        type TColgp_Array1OfPnt;
        #[doc = " ======================== Bnd_Box ========================"]
        #[doc = "Describes a bounding box in 3D space. A bounding box is parallel to the axes of the coordinates system. If it is finite, it is defined by the three intervals: -   [ Xmin,Xmax ], -   [ Ymin,Ymax ], -   [ Zmin,Zmax ]. A bounding box may be infinite (i.e. open) in one or more directions. It is said to be: -   OpenXmin if it is infinite on the negative side of the   \"X Direction\"; -   OpenXmax if it is infinite on the positive side of the \"X Direction\"; -   OpenYmin if it is infinite on the negative side of the   \"Y Direction\"; -   OpenYmax if it is infinite on the positive side of the \"Y Direction\"; -   OpenZmin if it is infinite on the negative side of the   \"Z Direction\"; -   OpenZmax if it is infinite on the positive side of the \"Z Direction\"; -   WholeSpace if it is infinite in all six directions. In this case, any point of the space is inside the box; -   Void if it is empty. In this case, there is no point included in the box. A bounding box is defined by: -   six bounds (Xmin, Xmax, Ymin, Ymax, Zmin and Zmax) which limit the bounding box if it is finite, -   eight flags (OpenXmin, OpenXmax, OpenYmin, OpenYmax, OpenZmin, OpenZmax, WholeSpace and Void) which describe the bounding box if it is infinite or empty, and -   a gap, which is included on both sides in any direction when consulting the finite bounds of the box."]
        #[cxx_name = "Bnd_Box"]
        type Box_;
        #[doc = "Creates an empty Box. The constructed box is qualified Void. Its gap is null."]
        #[cxx_name = "Bnd_Box_ctor"]
        fn Box__ctor() -> UniquePtr<Box_>;
        #[doc = "Creates a bounding box, it contains: -   minimum/maximum point of bounding box, The constructed box is qualified Void. Its gap is null."]
        #[cxx_name = "Bnd_Box_ctor_pnt2"]
        fn Box__ctor_pnt2(theMin: &gp_Pnt, theMax: &gp_Pnt) -> UniquePtr<Box_>;
        #[doc = "Sets this bounding box so that it covers the whole of 3D space. It is infinitely long in all directions."]
        #[cxx_name = "SetWhole"]
        fn set_whole(self: Pin<&mut Box_>);
        #[doc = "Sets this bounding box so that it is empty. All points are outside a void box."]
        #[cxx_name = "SetVoid"]
        fn set_void(self: Pin<&mut Box_>);
        #[doc = "Sets this bounding box so that it bounds -   the point P. This involves first setting this bounding box to be void and then adding the point P."]
        #[cxx_name = "Set"]
        fn setpnt(self: Pin<&mut Box_>, P: &gp_Pnt);
        #[doc = "Sets this bounding box so that it bounds the half-line defined by point P and direction D, i.e. all points M defined by M=P+u*D, where u is greater than or equal to 0, are inside the bounding volume. This involves first setting this box to be void and then adding   the half-line."]
        #[cxx_name = "Set"]
        fn setpnt_2(self: Pin<&mut Box_>, P: &gp_Pnt, D: &gp_Dir);
        #[doc = "Enlarges this bounding box, if required, so that it contains at least: -   interval [ aXmin,aXmax ] in the \"X Direction\", -   interval [ aYmin,aYmax ] in the \"Y Direction\", -   interval [ aZmin,aZmax ] in the \"Z Direction\";"]
        #[cxx_name = "Update"]
        fn updatereal(
            self: Pin<&mut Box_>,
            aXmin: f64,
            aYmin: f64,
            aZmin: f64,
            aXmax: f64,
            aYmax: f64,
            aZmax: f64,
        );
        #[doc = "Adds a point of coordinates (X,Y,Z) to this bounding box."]
        #[cxx_name = "Update"]
        fn updatereal_2(self: Pin<&mut Box_>, X: f64, Y: f64, Z: f64);
        #[doc = "Returns the gap of this bounding box."]
        #[cxx_name = "GetGap"]
        fn get_gap(self: &Box_) -> f64;
        #[doc = "Set the gap of this bounding box to abs(Tol)."]
        #[cxx_name = "SetGap"]
        fn set_gap(self: Pin<&mut Box_>, Tol: f64);
        #[doc = "Enlarges the      box    with    a   tolerance   value. (minvalues-Abs(<tol>) and maxvalues+Abs(<tol>)) This means that the minimum values of its X, Y and Z intervals of definition, when they are finite, are reduced by the absolute value of Tol, while the maximum values are increased by the same amount."]
        #[cxx_name = "Enlarge"]
        fn enlarge(self: Pin<&mut Box_>, Tol: f64);
        #[doc = "Returns the bounds of this bounding box. The gap is included. If this bounding box is infinite (i.e. \"open\"), returned values may be equal to +/- Precision::Infinite(). Standard_ConstructionError exception will be thrown if the box is void. if IsVoid()"]
        #[cxx_name = "Get"]
        fn get(
            self: &Box_,
            theXmin: &mut f64,
            theYmin: &mut f64,
            theZmin: &mut f64,
            theXmax: &mut f64,
            theYmax: &mut f64,
            theZmax: &mut f64,
        );
        #[doc = "The   Box will be   infinitely   long  in the Xmin direction."]
        #[cxx_name = "OpenXmin"]
        fn open_xmin(self: Pin<&mut Box_>);
        #[doc = "The   Box will be   infinitely   long  in the Xmax direction."]
        #[cxx_name = "OpenXmax"]
        fn open_xmax(self: Pin<&mut Box_>);
        #[doc = "The   Box will be   infinitely   long  in the Ymin direction."]
        #[cxx_name = "OpenYmin"]
        fn open_ymin(self: Pin<&mut Box_>);
        #[doc = "The   Box will be   infinitely   long  in the Ymax direction."]
        #[cxx_name = "OpenYmax"]
        fn open_ymax(self: Pin<&mut Box_>);
        #[doc = "The   Box will be   infinitely   long  in the Zmin direction."]
        #[cxx_name = "OpenZmin"]
        fn open_zmin(self: Pin<&mut Box_>);
        #[doc = "The   Box will be   infinitely   long  in the Zmax direction."]
        #[cxx_name = "OpenZmax"]
        fn open_zmax(self: Pin<&mut Box_>);
        #[doc = "Returns true if this bounding box has at least one open direction."]
        #[cxx_name = "IsOpen"]
        fn is_open(self: &Box_) -> bool;
        #[doc = "Returns true if this bounding box is open in the  Xmin direction."]
        #[cxx_name = "IsOpenXmin"]
        fn is_open_xmin(self: &Box_) -> bool;
        #[doc = "Returns true if this bounding box is open in the  Xmax direction."]
        #[cxx_name = "IsOpenXmax"]
        fn is_open_xmax(self: &Box_) -> bool;
        #[doc = "Returns true if this bounding box is open in the  Ymix direction."]
        #[cxx_name = "IsOpenYmin"]
        fn is_open_ymin(self: &Box_) -> bool;
        #[doc = "Returns true if this bounding box is open in the  Ymax direction."]
        #[cxx_name = "IsOpenYmax"]
        fn is_open_ymax(self: &Box_) -> bool;
        #[doc = "Returns true if this bounding box is open in the  Zmin direction."]
        #[cxx_name = "IsOpenZmin"]
        fn is_open_zmin(self: &Box_) -> bool;
        #[doc = "Returns true if this bounding box is open in the  Zmax  direction."]
        #[cxx_name = "IsOpenZmax"]
        fn is_open_zmax(self: &Box_) -> bool;
        #[doc = "Returns true if this bounding box is infinite in all 6 directions (WholeSpace flag)."]
        #[cxx_name = "IsWhole"]
        fn is_whole(self: &Box_) -> bool;
        #[doc = "Returns true if this bounding box is empty (Void flag)."]
        #[cxx_name = "IsVoid"]
        fn is_void(self: &Box_) -> bool;
        #[doc = "true if xmax-xmin < tol."]
        #[cxx_name = "IsXThin"]
        fn is_x_thin(self: &Box_, tol: f64) -> bool;
        #[doc = "true if ymax-ymin < tol."]
        #[cxx_name = "IsYThin"]
        fn is_y_thin(self: &Box_, tol: f64) -> bool;
        #[doc = "true if zmax-zmin < tol."]
        #[cxx_name = "IsZThin"]
        fn is_z_thin(self: &Box_, tol: f64) -> bool;
        #[doc = "Returns true if IsXThin, IsYThin and IsZThin are all true, i.e. if the box is thin in all three dimensions."]
        #[cxx_name = "IsThin"]
        fn is_thin(self: &Box_, tol: f64) -> bool;
        #[doc = "Adds the box <Other> to <me>."]
        #[cxx_name = "Add"]
        fn addbox(self: Pin<&mut Box_>, Other: &Box_);
        #[doc = "Adds a Pnt to the box."]
        #[cxx_name = "Add"]
        fn addpnt_2(self: Pin<&mut Box_>, P: &gp_Pnt);
        #[doc = "Extends  <me> from the Pnt <P> in the direction <D>."]
        #[cxx_name = "Add"]
        fn addpnt_3(self: Pin<&mut Box_>, P: &gp_Pnt, D: &gp_Dir);
        #[doc = "Extends the Box  in the given Direction, i.e. adds an  half-line. The   box  may become   infinite in 1,2 or 3 directions."]
        #[cxx_name = "Add"]
        fn adddir_4(self: Pin<&mut Box_>, D: &gp_Dir);
        #[doc = "Returns True if the Pnt is out the box."]
        #[cxx_name = "IsOut"]
        fn is_outpnt(self: &Box_, P: &gp_Pnt) -> bool;
        #[doc = "Returns False if the line intersects the box."]
        #[cxx_name = "IsOut"]
        fn is_outlin_2(self: &Box_, L: &gp_Lin) -> bool;
        #[doc = "Returns False if the plane intersects the box."]
        #[cxx_name = "IsOut"]
        fn is_outpln_3(self: &Box_, P: &gp_Pln) -> bool;
        #[doc = "Returns False if the <Box> intersects or is inside <me>."]
        #[cxx_name = "IsOut"]
        fn is_outbox_4(self: &Box_, Other: &Box_) -> bool;
        #[doc = "Returns False if  the transformed <Box> intersects or  is inside <me>."]
        #[cxx_name = "IsOut"]
        fn is_outbox_5(self: &Box_, Other: &Box_, T: &gp_Trsf) -> bool;
        #[doc = "Returns False  if the transformed <Box> intersects or  is inside the transformed box <me>."]
        #[cxx_name = "IsOut"]
        fn is_outtrsf_6(self: &Box_, T1: &gp_Trsf, Other: &Box_, T2: &gp_Trsf) -> bool;
        #[doc = "Returns False  if the flat band lying between two parallel lines represented by their reference points <P1>, <P2> and direction <D> intersects the box."]
        #[cxx_name = "IsOut"]
        fn is_outpnt_7(self: &Box_, P1: &gp_Pnt, P2: &gp_Pnt, D: &gp_Dir) -> bool;
        #[doc = "Computes the minimum distance between two boxes."]
        #[cxx_name = "Distance"]
        fn distance(self: &Box_, Other: &Box_) -> f64;
        #[cxx_name = "Dump"]
        fn dump(self: &Box_);
        #[doc = "Computes the squared diagonal of me."]
        #[cxx_name = "SquareExtent"]
        fn square_extent(self: &Box_) -> f64;
        #[doc = "Returns TRUE if this box has finite part."]
        #[cxx_name = "HasFinitePart"]
        fn has_finite_part(self: &Box_) -> bool;
        #[doc = "Returns the lower corner of this bounding box. The gap is included. If this bounding box is infinite (i.e. \"open\"), returned values may be equal to +/- Precision::Infinite(). Standard_ConstructionError exception will be thrown if the box is void. if IsVoid()"]
        #[cxx_name = "Bnd_Box_CornerMin"]
        fn Box__corner_min(self_: &Box_) -> UniquePtr<gp_Pnt>;
        #[doc = "Returns the upper corner of this bounding box. The gap is included. If this bounding box is infinite (i.e. \"open\"), returned values may be equal to +/- Precision::Infinite(). Standard_ConstructionError exception will be thrown if the box is void. if IsVoid()"]
        #[cxx_name = "Bnd_Box_CornerMax"]
        fn Box__corner_max(self_: &Box_) -> UniquePtr<gp_Pnt>;
        #[doc = "Returns a bounding box which is the result of applying the transformation T to this bounding box. Warning Applying a geometric transformation (for example, a rotation) to a bounding box generally increases its dimensions. This is not optimal for algorithms which use it."]
        #[cxx_name = "Bnd_Box_Transformed"]
        fn Box__transformed(self_: &Box_, T: &gp_Trsf) -> UniquePtr<Box_>;
        #[doc = "Returns a finite part of an infinite bounding box (returns self if this is already finite box). This can be a Void box in case if its sides has been defined as infinite (Open) without adding any finite points. WARNING! This method relies on Open flags, the infinite points added using Add() method will be returned as is."]
        #[cxx_name = "Bnd_Box_FinitePart"]
        fn Box__finite_part(self_: &Box_) -> UniquePtr<Box_>;
        #[doc = " ======================== Bnd_OBB ========================"]
        #[doc = "The class describes the Oriented Bounding Box (OBB), much tighter enclosing volume for the shape than the Axis Aligned Bounding Box (AABB). The OBB is defined by a center of the box, the axes and the halves of its three dimensions. The OBB can be used more effectively than AABB as a rejection mechanism for non-interfering objects."]
        #[cxx_name = "Bnd_OBB"]
        type OBB;
        #[doc = "Empty constructor"]
        #[cxx_name = "Bnd_OBB_ctor"]
        fn OBB_ctor() -> UniquePtr<OBB>;
        #[doc = "Constructor taking all defining parameters"]
        #[cxx_name = "Bnd_OBB_ctor_pnt_dir3_real3"]
        fn OBB_ctor_pnt_dir3_real3(
            theCenter: &gp_Pnt,
            theXDirection: &gp_Dir,
            theYDirection: &gp_Dir,
            theZDirection: &gp_Dir,
            theHXSize: f64,
            theHYSize: f64,
            theHZSize: f64,
        ) -> UniquePtr<OBB>;
        #[doc = "Constructor to create OBB from AABB."]
        #[cxx_name = "Bnd_OBB_ctor_box"]
        fn OBB_ctor_box(theBox: &Box_) -> UniquePtr<OBB>;
        #[doc = "Sets the center of OBB"]
        #[cxx_name = "SetCenter"]
        fn set_center(self: Pin<&mut OBB>, theCenter: &gp_Pnt);
        #[doc = "Sets the X component of OBB - direction and size"]
        #[cxx_name = "SetXComponent"]
        fn set_x_component(self: Pin<&mut OBB>, theXDirection: &gp_Dir, theHXSize: f64);
        #[doc = "Sets the Y component of OBB - direction and size"]
        #[cxx_name = "SetYComponent"]
        fn set_y_component(self: Pin<&mut OBB>, theYDirection: &gp_Dir, theHYSize: f64);
        #[doc = "Sets the Z component of OBB - direction and size"]
        #[cxx_name = "SetZComponent"]
        fn set_z_component(self: Pin<&mut OBB>, theZDirection: &gp_Dir, theHZSize: f64);
        #[doc = "Returns the center of OBB"]
        #[cxx_name = "Center"]
        fn center(self: &OBB) -> &gp_XYZ;
        #[doc = "Returns the X Direction of OBB"]
        #[cxx_name = "XDirection"]
        fn x_direction(self: &OBB) -> &gp_XYZ;
        #[doc = "Returns the Y Direction of OBB"]
        #[cxx_name = "YDirection"]
        fn y_direction(self: &OBB) -> &gp_XYZ;
        #[doc = "Returns the Z Direction of OBB"]
        #[cxx_name = "ZDirection"]
        fn z_direction(self: &OBB) -> &gp_XYZ;
        #[doc = "Returns the X Dimension of OBB"]
        #[cxx_name = "XHSize"]
        fn xh_size(self: &OBB) -> f64;
        #[doc = "Returns the Y Dimension of OBB"]
        #[cxx_name = "YHSize"]
        fn yh_size(self: &OBB) -> f64;
        #[doc = "Returns the Z Dimension of OBB"]
        #[cxx_name = "ZHSize"]
        fn zh_size(self: &OBB) -> f64;
        #[doc = "Checks if the box is empty."]
        #[cxx_name = "IsVoid"]
        fn is_void(self: &OBB) -> bool;
        #[doc = "Clears this box"]
        #[cxx_name = "SetVoid"]
        fn set_void(self: Pin<&mut OBB>);
        #[doc = "Sets the flag for axes aligned box"]
        #[cxx_name = "SetAABox"]
        fn set_aa_box(self: Pin<&mut OBB>, theFlag: &bool);
        #[doc = "Returns TRUE if the box is axes aligned"]
        #[cxx_name = "IsAABox"]
        fn is_aa_box(self: &OBB) -> bool;
        #[doc = "Enlarges the box with the given value"]
        #[cxx_name = "Enlarge"]
        fn enlarge(self: Pin<&mut OBB>, theGapAdd: f64);
        #[doc = "Returns square diagonal of this box"]
        #[cxx_name = "SquareExtent"]
        fn square_extent(self: &OBB) -> f64;
        #[doc = "Check if the box do not interfere the other box."]
        #[cxx_name = "IsOut"]
        fn is_outobb(self: &OBB, theOther: &OBB) -> bool;
        #[doc = "Check if the point is inside of <this>."]
        #[cxx_name = "IsOut"]
        fn is_outpnt_2(self: &OBB, theP: &gp_Pnt) -> bool;
        #[doc = "Check if the theOther is completely inside *this."]
        #[cxx_name = "IsCompletelyInside"]
        fn is_completely_inside(self: &OBB, theOther: &OBB) -> bool;
        #[doc = "Rebuilds this in order to include all previous objects (which it was created from) and theOther."]
        #[cxx_name = "Add"]
        fn addobb(self: Pin<&mut OBB>, theOther: &OBB);
        #[doc = "Rebuilds this in order to include all previous objects (which it was created from) and theP."]
        #[cxx_name = "Add"]
        fn addpnt_2(self: Pin<&mut OBB>, theP: &gp_Pnt);
        #[doc = "Returns the local coordinates system of this oriented box. So that applying it to axis-aligned box ((-XHSize, -YHSize, -ZHSize), (XHSize, YHSize, ZHSize)) will produce this oriented box. @code gp_Trsf aLoc; aLoc.SetTransformation (theOBB.Position(), gp::XOY()); @endcode"]
        #[cxx_name = "Bnd_OBB_Position"]
        fn OBB_position(self_: &OBB) -> UniquePtr<gp_Ax3>;
    }
    impl UniquePtr<Box_> {}
    impl UniquePtr<OBB> {}
}
pub use ffi::Box_ as Box;
impl Box {
    #[doc = "Creates an empty Box. The constructed box is qualified Void. Its gap is null."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Box__ctor()
    }

    #[doc = "Creates a bounding box, it contains: -   minimum/maximum point of bounding box, The constructed box is qualified Void. Its gap is null."]
    pub fn new_pnt2(theMin: &ffi::gp_Pnt, theMax: &ffi::gp_Pnt) -> cxx::UniquePtr<Self> {
        ffi::Box__ctor_pnt2(theMin, theMax)
    }
}
pub use ffi::OBB;
impl OBB {
    #[doc = "Empty constructor"]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::OBB_ctor()
    }

    #[doc = "Constructor taking all defining parameters"]
    pub fn new_pnt_dir3_real3(
        theCenter: &ffi::gp_Pnt,
        theXDirection: &ffi::gp_Dir,
        theYDirection: &ffi::gp_Dir,
        theZDirection: &ffi::gp_Dir,
        theHXSize: f64,
        theHYSize: f64,
        theHZSize: f64,
    ) -> cxx::UniquePtr<Self> {
        ffi::OBB_ctor_pnt_dir3_real3(
            theCenter,
            theXDirection,
            theYDirection,
            theZDirection,
            theHXSize,
            theHYSize,
            theHZSize,
        )
    }

    #[doc = "Constructor to create OBB from AABB."]
    pub fn new_box(theBox: &ffi::Box_) -> cxx::UniquePtr<Self> {
        ffi::OBB_ctor_box(theBox)
    }
}
