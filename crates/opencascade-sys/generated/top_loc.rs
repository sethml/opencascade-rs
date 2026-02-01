#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_top_loc.hxx");
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
        #[cxx_name = "TopLoc_Datum3D"]
        type TopLoc_Datum3D;
        #[doc = r" Handle to OCCT object"]
        #[cxx_name = "HandleTopLocDatum3D"]
        type HandleTopLocDatum3D;
        #[doc = " ======================== TopLoc_Location ========================"]
        #[doc = "A Location is a composite transition. It comprises a series of elementary reference coordinates, i.e. objects of type TopLoc_Datum3D, and the powers to which these objects are raised."]
        #[cxx_name = "TopLoc_Location"]
        type Location;
        #[doc = "Constructs an empty local coordinate system object. Note: A Location constructed from a default datum is said to be \"empty\"."]
        #[cxx_name = "TopLoc_Location_ctor"]
        fn Location_ctor() -> UniquePtr<Location>;
        #[doc = "Constructs the local coordinate system object defined by the transformation T. T invokes in turn, a TopLoc_Datum3D object."]
        #[cxx_name = "TopLoc_Location_ctor_trsf"]
        fn Location_ctor_trsf(T: &gp_Trsf) -> UniquePtr<Location>;
        #[doc = "Constructs the local coordinate system object defined by the 3D datum D. Exceptions Standard_ConstructionError if the transformation T does not represent a 3D coordinate system."]
        #[cxx_name = "TopLoc_Location_ctor_handledatum3d"]
        fn Location_ctor_handledatum3d(D: &HandleTopLocDatum3D) -> UniquePtr<Location>;
        #[doc = "Returns true if this location is equal to the Identity transformation."]
        #[cxx_name = "IsIdentity"]
        fn is_identity(self: &Location) -> bool;
        #[doc = "Resets this location to the Identity transformation."]
        #[cxx_name = "Identity"]
        fn identity(self: Pin<&mut Location>);
        #[doc = "Returns    the  first   elementary  datum  of  the Location.  Use the NextLocation function recursively to access the other data comprising this location. Exceptions Standard_NoSuchObject if this location is empty."]
        #[cxx_name = "FirstDatum"]
        fn first_datum(self: &Location) -> &HandleTopLocDatum3D;
        #[doc = "Returns   the  power  elevation  of    the   first elementary datum. Exceptions Standard_NoSuchObject if this location is empty."]
        #[cxx_name = "FirstPower"]
        fn first_power(self: &Location) -> i32;
        #[doc = "Returns  a Location representing  <me> without the first datum. We have the relation : <me> = NextLocation() * FirstDatum() ^ FirstPower() Exceptions Standard_NoSuchObject if this location is empty."]
        #[cxx_name = "NextLocation"]
        fn next_location(self: &Location) -> &Location;
        #[doc = "Returns  the transformation    associated  to  the coordinate system."]
        #[cxx_name = "Transformation"]
        fn transformation(self: &Location) -> &gp_Trsf;
        #[doc = "Returns a hashed value for this local coordinate system. This value is used, with map tables, to store and retrieve the object easily @return a computed hash code"]
        #[cxx_name = "HashCode"]
        fn hash_code(self: &Location) -> usize;
        #[doc = "Returns true if this location and the location Other have the same elementary data, i.e. contain the same series of TopLoc_Datum3D and respective powers. This method is an alias for operator ==."]
        #[cxx_name = "IsEqual"]
        fn is_equal(self: &Location, Other: &Location) -> bool;
        #[doc = "Returns true if this location and the location Other do not have the same elementary data, i.e. do not contain the same series of TopLoc_Datum3D and respective powers. This method is an alias for operator !=."]
        #[cxx_name = "IsDifferent"]
        fn is_different(self: &Location, Other: &Location) -> bool;
        #[doc = "Clear myItems"]
        #[cxx_name = "Clear"]
        fn clear(self: Pin<&mut Location>);
        #[doc = "Returns the inverse of <me>. <me> * Inverted() is an Identity."]
        #[cxx_name = "TopLoc_Location_Inverted"]
        fn Location_inverted(self_: &Location) -> UniquePtr<Location>;
        #[doc = "Returns <me> * <Other>, the  elementary datums are concatenated."]
        #[cxx_name = "TopLoc_Location_Multiplied"]
        fn Location_multiplied(self_: &Location, Other: &Location) -> UniquePtr<Location>;
        #[doc = "Returns  <me> / <Other>."]
        #[cxx_name = "TopLoc_Location_Divided"]
        fn Location_divided(self_: &Location, Other: &Location) -> UniquePtr<Location>;
        #[doc = "Returns <Other>.Inverted() * <me>."]
        #[cxx_name = "TopLoc_Location_Predivided"]
        fn Location_predivided(self_: &Location, Other: &Location) -> UniquePtr<Location>;
        #[doc = "Returns me at the power <pwr>.   If <pwr>  is zero returns  Identity.  <pwr> can  be lower  than zero (usual meaning for powers)."]
        #[cxx_name = "TopLoc_Location_Powered"]
        fn Location_powered(self_: &Location, pwr: i32) -> UniquePtr<Location>;
        #[cxx_name = "TopLoc_Location_ScalePrec"]
        fn Location_scale_prec() -> f64;
    }
    impl UniquePtr<Location> {}
}
pub use ffi::Location;
impl Location {
    #[doc = "Constructs an empty local coordinate system object. Note: A Location constructed from a default datum is said to be \"empty\"."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Location_ctor()
    }

    #[doc = "Constructs the local coordinate system object defined by the transformation T. T invokes in turn, a TopLoc_Datum3D object."]
    pub fn new_trsf(T: &ffi::gp_Trsf) -> cxx::UniquePtr<Self> {
        ffi::Location_ctor_trsf(T)
    }

    #[doc = "Constructs the local coordinate system object defined by the 3D datum D. Exceptions Standard_ConstructionError if the transformation T does not represent a 3D coordinate system."]
    pub fn new_handledatum3d(D: &ffi::HandleTopLocDatum3D) -> cxx::UniquePtr<Self> {
        ffi::Location_ctor_handledatum3d(D)
    }
}
