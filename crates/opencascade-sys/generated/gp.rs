#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_gp.hxx");
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "gp_Trsf"]
        type gp_Trsf;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "gp_Mat"]
        type gp_Mat;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "gp_Ax1"]
        type gp_Ax1;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "gp_Ax2"]
        type gp_Ax2;
        #[doc = " ======================== gp_Pnt ========================"]
        #[doc = "Defines a 3D cartesian point."]
        #[cxx_name = "gp_Pnt"]
        type Pnt;
        #[doc = "Creates a point with zero coordinates."]
        #[cxx_name = "gp_Pnt_ctor"]
        fn Pnt_ctor() -> UniquePtr<Pnt>;
        #[doc = "Creates a point from a XYZ object."]
        #[cxx_name = "gp_Pnt_ctor_xyz"]
        fn Pnt_ctor_xyz(theCoord: &XYZ) -> UniquePtr<Pnt>;
        #[doc = "Creates a  point with its 3 cartesian's coordinates : theXp, theYp, theZp."]
        #[cxx_name = "gp_Pnt_ctor_real3"]
        fn Pnt_ctor_real3(theXp: f64, theYp: f64, theZp: f64) -> UniquePtr<Pnt>;
        #[doc = "Changes the coordinate of range theIndex : theIndex = 1 => X is modified theIndex = 2 => Y is modified theIndex = 3 => Z is modified Raised if theIndex != {1, 2, 3}."]
        #[cxx_name = "SetCoord"]
        fn set_coordint(self: Pin<&mut Pnt>, theIndex: i32, theXi: f64);
        #[doc = "For this point, assigns  the values theXp, theYp and theZp to its three coordinates."]
        #[cxx_name = "SetCoord"]
        fn set_coordreal_2(self: Pin<&mut Pnt>, theXp: f64, theYp: f64, theZp: f64);
        #[doc = "Assigns the given value to the X coordinate of this point."]
        #[cxx_name = "SetX"]
        fn set_x(self: Pin<&mut Pnt>, theX: f64);
        #[doc = "Assigns the given value to the Y coordinate of this point."]
        #[cxx_name = "SetY"]
        fn set_y(self: Pin<&mut Pnt>, theY: f64);
        #[doc = "Assigns the given value to the Z coordinate of this point."]
        #[cxx_name = "SetZ"]
        fn set_z(self: Pin<&mut Pnt>, theZ: f64);
        #[doc = "Assigns the three coordinates of theCoord to this point."]
        #[cxx_name = "SetXYZ"]
        fn set_xyz(self: Pin<&mut Pnt>, theCoord: &XYZ);
        #[doc = "Returns the coordinate of corresponding to the value of theIndex : theIndex = 1 => X is returned theIndex = 2 => Y is returned theIndex = 3 => Z is returned Raises OutOfRange if theIndex != {1, 2, 3}. Raised if theIndex != {1, 2, 3}."]
        #[cxx_name = "Coord"]
        fn coordint(self: &Pnt, theIndex: i32) -> f64;
        #[doc = "For this point gives its three coordinates theXp, theYp and theZp."]
        #[cxx_name = "Coord"]
        fn coordreal_2(self: &Pnt, theXp: &mut f64, theYp: &mut f64, theZp: &mut f64);
        #[doc = "For this point, returns its X coordinate."]
        #[cxx_name = "X"]
        fn x(self: &Pnt) -> f64;
        #[doc = "For this point, returns its Y coordinate."]
        #[cxx_name = "Y"]
        fn y(self: &Pnt) -> f64;
        #[doc = "For this point, returns its Z coordinate."]
        #[cxx_name = "Z"]
        fn z(self: &Pnt) -> f64;
        #[doc = "For this point, returns its three coordinates as a XYZ object."]
        #[cxx_name = "XYZ"]
        fn xyz(self: &Pnt) -> &XYZ;
        #[doc = "For this point, returns its three coordinates as a XYZ object."]
        #[cxx_name = "Coord"]
        fn coord3(self: &Pnt) -> &XYZ;
        #[doc = "Returns the coordinates of this point. Note: This syntax allows direct modification of the returned value."]
        #[cxx_name = "ChangeCoord"]
        fn change_coord(self: Pin<&mut Pnt>) -> Pin<&mut XYZ>;
        #[doc = "Assigns the result of the following expression to this point (theAlpha*this + theBeta*theP) / (theAlpha + theBeta)"]
        #[cxx_name = "BaryCenter"]
        fn bary_center(self: Pin<&mut Pnt>, theAlpha: f64, theP: &Pnt, theBeta: f64);
        #[doc = "Comparison Returns True if the distance between the two points is lower or equal to theLinearTolerance."]
        #[cxx_name = "IsEqual"]
        fn is_equal(self: &Pnt, theOther: &Pnt, theLinearTolerance: f64) -> bool;
        #[doc = "Computes the distance between two points."]
        #[cxx_name = "Distance"]
        fn distance(self: &Pnt, theOther: &Pnt) -> f64;
        #[doc = "Computes the square distance between two points."]
        #[cxx_name = "SquareDistance"]
        fn square_distance(self: &Pnt, theOther: &Pnt) -> f64;
        #[doc = "Performs the symmetrical transformation of a point with respect to the point theP which is the center of the  symmetry."]
        #[cxx_name = "Mirror"]
        fn mirrorpnt(self: Pin<&mut Pnt>, theP: &Pnt);
        #[cxx_name = "Mirror"]
        fn mirrorax1_2(self: Pin<&mut Pnt>, theA1: &gp_Ax1);
        #[cxx_name = "Mirror"]
        fn mirrorax2_3(self: Pin<&mut Pnt>, theA2: &gp_Ax2);
        #[cxx_name = "Rotate"]
        fn rotate(self: Pin<&mut Pnt>, theA1: &gp_Ax1, theAng: f64);
        #[doc = "Scales a point. theS is the scaling value."]
        #[cxx_name = "Scale"]
        fn scale(self: Pin<&mut Pnt>, theP: &Pnt, theS: f64);
        #[doc = "Transforms a point with the transformation T."]
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Pnt>, theT: &gp_Trsf);
        #[doc = "Translates a point in the direction of the vector theV. The magnitude of the translation is the vector's magnitude."]
        #[cxx_name = "Translate"]
        fn translatevec(self: Pin<&mut Pnt>, theV: &Vec_);
        #[doc = "Translates a point from the point theP1 to the point theP2."]
        #[cxx_name = "Translate"]
        fn translatepnt_2(self: Pin<&mut Pnt>, theP1: &Pnt, theP2: &Pnt);
        #[doc = "Performs the symmetrical transformation of a point with respect to an axis placement which is the axis of the symmetry."]
        #[cxx_name = "gp_Pnt_Mirrored"]
        fn Pnt_mirroredpnt(self_: &Pnt, theP: &Pnt) -> UniquePtr<Pnt>;
        #[doc = "Performs the symmetrical transformation of a point with respect to a plane. The axis placement theA2 locates the plane of the symmetry : (Location, XDirection, YDirection)."]
        #[cxx_name = "gp_Pnt_Mirrored"]
        fn Pnt_mirroredax1_2(self_: &Pnt, theA1: &gp_Ax1) -> UniquePtr<Pnt>;
        #[doc = "Rotates a point. theA1 is the axis of the rotation. theAng is the angular value of the rotation in radians."]
        #[cxx_name = "gp_Pnt_Mirrored"]
        fn Pnt_mirroredax2_3(self_: &Pnt, theA2: &gp_Ax2) -> UniquePtr<Pnt>;
        #[cxx_name = "gp_Pnt_Rotated"]
        fn Pnt_rotated(self_: &Pnt, theA1: &gp_Ax1, theAng: f64) -> UniquePtr<Pnt>;
        #[cxx_name = "gp_Pnt_Scaled"]
        fn Pnt_scaled(self_: &Pnt, theP: &Pnt, theS: f64) -> UniquePtr<Pnt>;
        #[cxx_name = "gp_Pnt_Transformed"]
        fn Pnt_transformed(self_: &Pnt, theT: &gp_Trsf) -> UniquePtr<Pnt>;
        #[cxx_name = "gp_Pnt_Translated"]
        fn Pnt_translatedvec(self_: &Pnt, theV: &Vec_) -> UniquePtr<Pnt>;
        #[cxx_name = "gp_Pnt_Translated"]
        fn Pnt_translatedpnt_2(self_: &Pnt, theP1: &Pnt, theP2: &Pnt) -> UniquePtr<Pnt>;
        #[doc = " ======================== gp_Vec ========================"]
        #[doc = "Defines a non-persistent vector in 3D space."]
        #[cxx_name = "gp_Vec"]
        type Vec_;
        #[doc = "Creates a zero vector."]
        #[cxx_name = "gp_Vec_ctor"]
        fn Vec__ctor() -> UniquePtr<Vec_>;
        #[doc = "Creates a unitary vector from a direction theV."]
        #[cxx_name = "gp_Vec_ctor_dir"]
        fn Vec__ctor_dir(theV: &Dir) -> UniquePtr<Vec_>;
        #[doc = "Creates a vector with a triplet of coordinates."]
        #[cxx_name = "gp_Vec_ctor_xyz"]
        fn Vec__ctor_xyz(theCoord: &XYZ) -> UniquePtr<Vec_>;
        #[doc = "Creates a point with its three cartesian coordinates."]
        #[cxx_name = "gp_Vec_ctor_real3"]
        fn Vec__ctor_real3(theXv: f64, theYv: f64, theZv: f64) -> UniquePtr<Vec_>;
        #[doc = "Creates a vector from two points. The length of the vector is the distance between theP1 and theP2"]
        #[cxx_name = "gp_Vec_ctor_pnt2"]
        fn Vec__ctor_pnt2(theP1: &Pnt, theP2: &Pnt) -> UniquePtr<Vec_>;
        #[doc = "Changes the coordinate of range theIndex theIndex = 1 => X is modified theIndex = 2 => Y is modified theIndex = 3 => Z is modified Raised if theIndex != {1, 2, 3}."]
        #[cxx_name = "SetCoord"]
        fn set_coordint(self: Pin<&mut Vec_>, theIndex: i32, theXi: f64);
        #[doc = "For this vector, assigns -   the values theXv, theYv and theZv to its three coordinates."]
        #[cxx_name = "SetCoord"]
        fn set_coordreal_2(self: Pin<&mut Vec_>, theXv: f64, theYv: f64, theZv: f64);
        #[doc = "Assigns the given value to the X coordinate of this vector."]
        #[cxx_name = "SetX"]
        fn set_x(self: Pin<&mut Vec_>, theX: f64);
        #[doc = "Assigns the given value to the X coordinate of this vector."]
        #[cxx_name = "SetY"]
        fn set_y(self: Pin<&mut Vec_>, theY: f64);
        #[doc = "Assigns the given value to the X coordinate of this vector."]
        #[cxx_name = "SetZ"]
        fn set_z(self: Pin<&mut Vec_>, theZ: f64);
        #[doc = "Assigns the three coordinates of theCoord to this vector."]
        #[cxx_name = "SetXYZ"]
        fn set_xyz(self: Pin<&mut Vec_>, theCoord: &XYZ);
        #[doc = "Returns the coordinate of range theIndex : theIndex = 1 => X is returned theIndex = 2 => Y is returned theIndex = 3 => Z is returned Raised if theIndex != {1, 2, 3}."]
        #[cxx_name = "Coord"]
        fn coordint(self: &Vec_, theIndex: i32) -> f64;
        #[doc = "For this vector returns its three coordinates theXv, theYv, and theZv inline"]
        #[cxx_name = "Coord"]
        fn coordreal_2(self: &Vec_, theXv: &mut f64, theYv: &mut f64, theZv: &mut f64);
        #[doc = "For this vector, returns its X coordinate."]
        #[cxx_name = "X"]
        fn x(self: &Vec_) -> f64;
        #[doc = "For this vector, returns its Y coordinate."]
        #[cxx_name = "Y"]
        fn y(self: &Vec_) -> f64;
        #[doc = "For this vector, returns its Z  coordinate."]
        #[cxx_name = "Z"]
        fn z(self: &Vec_) -> f64;
        #[doc = "For this vector, returns -   its three coordinates as a number triple"]
        #[cxx_name = "XYZ"]
        fn xyz(self: &Vec_) -> &XYZ;
        #[doc = "Returns True if the two vectors have the same magnitude value and the same direction. The precision values are theLinearTolerance for the magnitude and theAngularTolerance for the direction."]
        #[cxx_name = "IsEqual"]
        fn is_equal(
            self: &Vec_,
            theOther: &Vec_,
            theLinearTolerance: f64,
            theAngularTolerance: f64,
        ) -> bool;
        #[doc = "Returns True if abs(<me>.Angle(theOther) - PI/2.) <= theAngularTolerance Raises VectorWithNullMagnitude if <me>.Magnitude() <= Resolution or theOther.Magnitude() <= Resolution from gp"]
        #[cxx_name = "IsNormal"]
        fn is_normal(self: &Vec_, theOther: &Vec_, theAngularTolerance: f64) -> bool;
        #[doc = "Returns True if PI - <me>.Angle(theOther) <= theAngularTolerance Raises VectorWithNullMagnitude if <me>.Magnitude() <= Resolution or Other.Magnitude() <= Resolution from gp"]
        #[cxx_name = "IsOpposite"]
        fn is_opposite(self: &Vec_, theOther: &Vec_, theAngularTolerance: f64) -> bool;
        #[doc = "Returns True if Angle(<me>, theOther) <= theAngularTolerance or PI - Angle(<me>, theOther) <= theAngularTolerance This definition means that two parallel vectors cannot define a plane but two vectors with opposite directions are considered as parallel. Raises VectorWithNullMagnitude if <me>.Magnitude() <= Resolution or Other.Magnitude() <= Resolution from gp"]
        #[cxx_name = "IsParallel"]
        fn is_parallel(self: &Vec_, theOther: &Vec_, theAngularTolerance: f64) -> bool;
        #[doc = "Computes the angular value between <me> and <theOther> Returns the angle value between 0 and PI in radian. Raises VectorWithNullMagnitude if <me>.Magnitude() <= Resolution from gp or theOther.Magnitude() <= Resolution because the angular value is indefinite if one of the vectors has a null magnitude."]
        #[cxx_name = "Angle"]
        fn angle(self: &Vec_, theOther: &Vec_) -> f64;
        #[doc = "Computes the angle, in radians, between this vector and vector theOther. The result is a value between -Pi and Pi. For this, theVRef defines the positive sense of rotation: the angular value is positive, if the cross product this ^ theOther has the same orientation as theVRef relative to the plane defined by the vectors this and theOther. Otherwise, the angular value is negative. Exceptions gp_VectorWithNullMagnitude if the magnitude of this vector, the vector theOther, or the vector theVRef is less than or equal to gp::Resolution(). Standard_DomainError if this vector, the vector theOther, and the vector theVRef are coplanar, unless this vector and the vector theOther are parallel."]
        #[cxx_name = "AngleWithRef"]
        fn angle_with_ref(self: &Vec_, theOther: &Vec_, theVRef: &Vec_) -> f64;
        #[doc = "Computes the magnitude of this vector."]
        #[cxx_name = "Magnitude"]
        fn magnitude(self: &Vec_) -> f64;
        #[doc = "Computes the square magnitude of this vector."]
        #[cxx_name = "SquareMagnitude"]
        fn square_magnitude(self: &Vec_) -> f64;
        #[doc = "Adds two vectors"]
        #[cxx_name = "Add"]
        fn add(self: Pin<&mut Vec_>, theOther: &Vec_);
        #[doc = "Subtracts two vectors"]
        #[cxx_name = "Subtract"]
        fn subtract(self: Pin<&mut Vec_>, theRight: &Vec_);
        #[doc = "Multiplies a vector by a scalar"]
        #[cxx_name = "Multiply"]
        fn multiply(self: Pin<&mut Vec_>, theScalar: f64);
        #[doc = "Divides a vector by a scalar"]
        #[cxx_name = "Divide"]
        fn divide(self: Pin<&mut Vec_>, theScalar: f64);
        #[doc = "computes the cross product between two vectors"]
        #[cxx_name = "Cross"]
        fn cross(self: Pin<&mut Vec_>, theRight: &Vec_);
        #[doc = "Computes the magnitude of the cross product between <me> and theRight. Returns || <me> ^ theRight ||"]
        #[cxx_name = "CrossMagnitude"]
        fn cross_magnitude(self: &Vec_, theRight: &Vec_) -> f64;
        #[doc = "Computes the square magnitude of the cross product between <me> and theRight. Returns || <me> ^ theRight ||**2"]
        #[cxx_name = "CrossSquareMagnitude"]
        fn cross_square_magnitude(self: &Vec_, theRight: &Vec_) -> f64;
        #[doc = "Computes the triple vector product. <me> ^= (theV1 ^ theV2)"]
        #[cxx_name = "CrossCross"]
        fn cross_cross(self: Pin<&mut Vec_>, theV1: &Vec_, theV2: &Vec_);
        #[doc = "computes the scalar product"]
        #[cxx_name = "Dot"]
        fn dot(self: &Vec_, theOther: &Vec_) -> f64;
        #[doc = "Computes the triple scalar product <me> * (theV1 ^ theV2)."]
        #[cxx_name = "DotCross"]
        fn dot_cross(self: &Vec_, theV1: &Vec_, theV2: &Vec_) -> f64;
        #[doc = "normalizes a vector Raises an exception if the magnitude of the vector is lower or equal to Resolution from gp."]
        #[cxx_name = "Normalize"]
        fn normalize(self: Pin<&mut Vec_>);
        #[doc = "Reverses the direction of a vector"]
        #[cxx_name = "Reverse"]
        fn reverse(self: Pin<&mut Vec_>);
        #[doc = "<me> is set to the following linear form : theA1 * theV1 + theA2 * theV2 + theA3 * theV3 + theV4"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formreal(
            self: Pin<&mut Vec_>,
            theA1: f64,
            theV1: &Vec_,
            theA2: f64,
            theV2: &Vec_,
            theA3: f64,
            theV3: &Vec_,
            theV4: &Vec_,
        );
        #[doc = "<me> is set to the following linear form : theA1 * theV1 + theA2 * theV2 + theA3 * theV3"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formreal_2(
            self: Pin<&mut Vec_>,
            theA1: f64,
            theV1: &Vec_,
            theA2: f64,
            theV2: &Vec_,
            theA3: f64,
            theV3: &Vec_,
        );
        #[doc = "<me> is set to the following linear form : theA1 * theV1 + theA2 * theV2 + theV3"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formreal_3(
            self: Pin<&mut Vec_>,
            theA1: f64,
            theV1: &Vec_,
            theA2: f64,
            theV2: &Vec_,
            theV3: &Vec_,
        );
        #[doc = "<me> is set to the following linear form : theA1 * theV1 + theA2 * theV2"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formreal_4(
            self: Pin<&mut Vec_>,
            theA1: f64,
            theV1: &Vec_,
            theA2: f64,
            theV2: &Vec_,
        );
        #[doc = "<me> is set to the following linear form : theA1 * theV1 + theV2"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formreal_5(self: Pin<&mut Vec_>, theA1: f64, theV1: &Vec_, theV2: &Vec_);
        #[doc = "<me> is set to the following linear form : theV1 + theV2"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formvec_6(self: Pin<&mut Vec_>, theV1: &Vec_, theV2: &Vec_);
        #[cxx_name = "Mirror"]
        fn mirrorvec(self: Pin<&mut Vec_>, theV: &Vec_);
        #[cxx_name = "Mirror"]
        fn mirrorax1_2(self: Pin<&mut Vec_>, theA1: &gp_Ax1);
        #[cxx_name = "Mirror"]
        fn mirrorax2_3(self: Pin<&mut Vec_>, theA2: &gp_Ax2);
        #[cxx_name = "Rotate"]
        fn rotate(self: Pin<&mut Vec_>, theA1: &gp_Ax1, theAng: f64);
        #[cxx_name = "Scale"]
        fn scale(self: Pin<&mut Vec_>, theS: f64);
        #[doc = "Transforms a vector with the transformation theT."]
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Vec_>, theT: &gp_Trsf);
        #[doc = "Adds two vectors"]
        #[cxx_name = "gp_Vec_Added"]
        fn Vec__added(self_: &Vec_, theOther: &Vec_) -> UniquePtr<Vec_>;
        #[doc = "Subtracts two vectors"]
        #[cxx_name = "gp_Vec_Subtracted"]
        fn Vec__subtracted(self_: &Vec_, theRight: &Vec_) -> UniquePtr<Vec_>;
        #[doc = "Multiplies a vector by a scalar"]
        #[cxx_name = "gp_Vec_Multiplied"]
        fn Vec__multiplied(self_: &Vec_, theScalar: f64) -> UniquePtr<Vec_>;
        #[doc = "Divides a vector by a scalar"]
        #[cxx_name = "gp_Vec_Divided"]
        fn Vec__divided(self_: &Vec_, theScalar: f64) -> UniquePtr<Vec_>;
        #[doc = "computes the cross product between two vectors"]
        #[cxx_name = "gp_Vec_Crossed"]
        fn Vec__crossed(self_: &Vec_, theRight: &Vec_) -> UniquePtr<Vec_>;
        #[doc = "Computes the triple vector product. <me> ^ (theV1 ^ theV2)"]
        #[cxx_name = "gp_Vec_CrossCrossed"]
        fn Vec__cross_crossed(self_: &Vec_, theV1: &Vec_, theV2: &Vec_) -> UniquePtr<Vec_>;
        #[doc = "normalizes a vector Raises an exception if the magnitude of the vector is lower or equal to Resolution from gp."]
        #[cxx_name = "gp_Vec_Normalized"]
        fn Vec__normalized(self_: &Vec_) -> UniquePtr<Vec_>;
        #[doc = "Reverses the direction of a vector"]
        #[cxx_name = "gp_Vec_Reversed"]
        fn Vec__reversed(self_: &Vec_) -> UniquePtr<Vec_>;
        #[doc = "Performs the symmetrical transformation of a vector with respect to the vector theV which is the center of the  symmetry."]
        #[cxx_name = "gp_Vec_Mirrored"]
        fn Vec__mirroredvec(self_: &Vec_, theV: &Vec_) -> UniquePtr<Vec_>;
        #[doc = "Performs the symmetrical transformation of a vector with respect to an axis placement which is the axis of the symmetry."]
        #[cxx_name = "gp_Vec_Mirrored"]
        fn Vec__mirroredax1_2(self_: &Vec_, theA1: &gp_Ax1) -> UniquePtr<Vec_>;
        #[doc = "Performs the symmetrical transformation of a vector with respect to a plane. The axis placement theA2 locates the plane of the symmetry : (Location, XDirection, YDirection)."]
        #[cxx_name = "gp_Vec_Mirrored"]
        fn Vec__mirroredax2_3(self_: &Vec_, theA2: &gp_Ax2) -> UniquePtr<Vec_>;
        #[doc = "Rotates a vector. theA1 is the axis of the rotation. theAng is the angular value of the rotation in radians."]
        #[cxx_name = "gp_Vec_Rotated"]
        fn Vec__rotated(self_: &Vec_, theA1: &gp_Ax1, theAng: f64) -> UniquePtr<Vec_>;
        #[doc = "Scales a vector. theS is the scaling value."]
        #[cxx_name = "gp_Vec_Scaled"]
        fn Vec__scaled(self_: &Vec_, theS: f64) -> UniquePtr<Vec_>;
        #[doc = "Transforms a vector with the transformation theT."]
        #[cxx_name = "gp_Vec_Transformed"]
        fn Vec__transformed(self_: &Vec_, theT: &gp_Trsf) -> UniquePtr<Vec_>;
        #[doc = " ======================== gp_Dir ========================"]
        #[doc = "Describes a unit vector in 3D space. This unit vector is also called \"Direction\". See Also gce_MakeDir which provides functions for more complex unit vector constructions Geom_Direction which provides additional functions for constructing unit vectors and works, in particular, with the parametric equations of unit vectors."]
        #[cxx_name = "gp_Dir"]
        type Dir;
        #[doc = "Creates a direction corresponding to X axis."]
        #[cxx_name = "gp_Dir_ctor"]
        fn Dir_ctor() -> UniquePtr<Dir>;
        #[doc = "Normalizes the vector theV and creates a direction. Raises ConstructionError if theV.Magnitude() <= Resolution."]
        #[cxx_name = "gp_Dir_ctor_vec"]
        fn Dir_ctor_vec(theV: &Vec_) -> UniquePtr<Dir>;
        #[doc = "Creates a direction from a triplet of coordinates. Raises ConstructionError if theCoord.Modulus() <= Resolution from gp."]
        #[cxx_name = "gp_Dir_ctor_xyz"]
        fn Dir_ctor_xyz(theCoord: &XYZ) -> UniquePtr<Dir>;
        #[doc = "Creates a direction with its 3 cartesian coordinates. Raises ConstructionError if Sqrt(theXv*theXv + theYv*theYv + theZv*theZv) <= Resolution Modification of the direction's coordinates If Sqrt (theXv*theXv + theYv*theYv + theZv*theZv) <= Resolution from gp where theXv, theYv ,theZv are the new coordinates it is not possible to construct the direction and the method raises the exception ConstructionError."]
        #[cxx_name = "gp_Dir_ctor_real3"]
        fn Dir_ctor_real3(theXv: f64, theYv: f64, theZv: f64) -> UniquePtr<Dir>;
        #[doc = "For this unit vector,  assigns the value Xi to: -   the X coordinate if theIndex is 1, or -   the Y coordinate if theIndex is 2, or -   the Z coordinate if theIndex is 3, and then normalizes it. Warning Remember that all the coordinates of a unit vector are implicitly modified when any single one is changed directly. Exceptions Standard_OutOfRange if theIndex is not 1, 2, or 3. Standard_ConstructionError if either of the following is less than or equal to gp::Resolution(): -   Sqrt(Xv*Xv + Yv*Yv + Zv*Zv), or -   the modulus of the number triple formed by the new value theXi and the two other coordinates of this vector that were not directly modified."]
        #[cxx_name = "SetCoord"]
        fn set_coordint(self: Pin<&mut Dir>, theIndex: i32, theXi: f64);
        #[doc = "For this unit vector,  assigns the values theXv, theYv and theZv to its three coordinates. Remember that all the coordinates of a unit vector are implicitly modified when any single one is changed directly."]
        #[cxx_name = "SetCoord"]
        fn set_coordreal_2(self: Pin<&mut Dir>, theXv: f64, theYv: f64, theZv: f64);
        #[doc = "Assigns the given value to the X coordinate of this   unit vector."]
        #[cxx_name = "SetX"]
        fn set_x(self: Pin<&mut Dir>, theX: f64);
        #[doc = "Assigns the given value to the Y coordinate of this   unit vector."]
        #[cxx_name = "SetY"]
        fn set_y(self: Pin<&mut Dir>, theY: f64);
        #[doc = "Assigns the given value to the Z  coordinate of this   unit vector."]
        #[cxx_name = "SetZ"]
        fn set_z(self: Pin<&mut Dir>, theZ: f64);
        #[doc = "Assigns the three coordinates of theCoord to this unit vector."]
        #[cxx_name = "SetXYZ"]
        fn set_xyz(self: Pin<&mut Dir>, theCoord: &XYZ);
        #[doc = "Returns the coordinate of range theIndex : theIndex = 1 => X is returned Ithendex = 2 => Y is returned theIndex = 3 => Z is returned Exceptions Standard_OutOfRange if theIndex is not 1, 2, or 3."]
        #[cxx_name = "Coord"]
        fn coordint(self: &Dir, theIndex: i32) -> f64;
        #[doc = "Returns for the  unit vector  its three coordinates theXv, theYv, and theZv."]
        #[cxx_name = "Coord"]
        fn coordreal_2(self: &Dir, theXv: &mut f64, theYv: &mut f64, theZv: &mut f64);
        #[doc = "Returns the X coordinate for a  unit vector."]
        #[cxx_name = "X"]
        fn x(self: &Dir) -> f64;
        #[doc = "Returns the Y coordinate for a  unit vector."]
        #[cxx_name = "Y"]
        fn y(self: &Dir) -> f64;
        #[doc = "Returns the Z coordinate for a  unit vector."]
        #[cxx_name = "Z"]
        fn z(self: &Dir) -> f64;
        #[doc = "for this unit vector, returns  its three coordinates as a number triplea."]
        #[cxx_name = "XYZ"]
        fn xyz(self: &Dir) -> &XYZ;
        #[doc = "Returns True if the angle between the two directions is lower or equal to theAngularTolerance."]
        #[cxx_name = "IsEqual"]
        fn is_equal(self: &Dir, theOther: &Dir, theAngularTolerance: f64) -> bool;
        #[doc = "Returns True if  the angle between this unit vector and the unit vector theOther is equal to Pi/2 (normal)."]
        #[cxx_name = "IsNormal"]
        fn is_normal(self: &Dir, theOther: &Dir, theAngularTolerance: f64) -> bool;
        #[doc = "Returns True if  the angle between this unit vector and the unit vector theOther is equal to Pi (opposite)."]
        #[cxx_name = "IsOpposite"]
        fn is_opposite(self: &Dir, theOther: &Dir, theAngularTolerance: f64) -> bool;
        #[doc = "Returns true if the angle between this unit vector and the unit vector theOther is equal to 0 or to Pi. Note: the tolerance criterion is given by theAngularTolerance."]
        #[cxx_name = "IsParallel"]
        fn is_parallel(self: &Dir, theOther: &Dir, theAngularTolerance: f64) -> bool;
        #[doc = "Computes the angular value in radians between <me> and <theOther>. This value is always positive in 3D space. Returns the angle in the range [0, PI]"]
        #[cxx_name = "Angle"]
        fn angle(self: &Dir, theOther: &Dir) -> f64;
        #[doc = "Computes the angular value between <me> and <theOther>. <theVRef> is the direction of reference normal to <me> and <theOther> and its orientation gives the positive sense of rotation. If the cross product <me> ^ <theOther> has the same orientation as <theVRef> the angular value is positive else negative. Returns the angular value in the range -PI and PI (in radians). Raises  DomainError if <me> and <theOther> are not parallel this exception is raised when <theVRef> is in the same plane as <me> and <theOther> The tolerance criterion is Resolution from package gp."]
        #[cxx_name = "AngleWithRef"]
        fn angle_with_ref(self: &Dir, theOther: &Dir, theVRef: &Dir) -> f64;
        #[doc = "Computes the cross product between two directions Raises the exception ConstructionError if the two directions are parallel because the computed vector cannot be normalized to create a direction."]
        #[cxx_name = "Cross"]
        fn cross(self: Pin<&mut Dir>, theRight: &Dir);
        #[cxx_name = "CrossCross"]
        fn cross_cross(self: Pin<&mut Dir>, theV1: &Dir, theV2: &Dir);
        #[doc = "Computes the scalar product"]
        #[cxx_name = "Dot"]
        fn dot(self: &Dir, theOther: &Dir) -> f64;
        #[doc = "Computes the triple scalar product <me> * (theV1 ^ theV2). Warnings : The computed vector theV1' = theV1 ^ theV2 is not normalized to create a unitary vector. So this method never raises an exception even if theV1 and theV2 are parallel."]
        #[cxx_name = "DotCross"]
        fn dot_cross(self: &Dir, theV1: &Dir, theV2: &Dir) -> f64;
        #[cxx_name = "Reverse"]
        fn reverse(self: Pin<&mut Dir>);
        #[cxx_name = "Mirror"]
        fn mirrordir(self: Pin<&mut Dir>, theV: &Dir);
        #[cxx_name = "Mirror"]
        fn mirrorax1_2(self: Pin<&mut Dir>, theA1: &gp_Ax1);
        #[cxx_name = "Mirror"]
        fn mirrorax2_3(self: Pin<&mut Dir>, theA2: &gp_Ax2);
        #[cxx_name = "Rotate"]
        fn rotate(self: Pin<&mut Dir>, theA1: &gp_Ax1, theAng: f64);
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Dir>, theT: &gp_Trsf);
        #[doc = "Computes the triple vector product. <me> ^ (V1 ^ V2) Raises the exception ConstructionError if V1 and V2 are parallel or <me> and (V1^V2) are parallel because the computed vector can't be normalized to create a direction."]
        #[cxx_name = "gp_Dir_Crossed"]
        fn Dir_crossed(self_: &Dir, theRight: &Dir) -> UniquePtr<Dir>;
        #[doc = "Computes the double vector product this ^ (theV1 ^ theV2). -   CrossCrossed creates a new unit vector. Exceptions Standard_ConstructionError if: -   theV1 and theV2 are parallel, or -   this unit vector and (theV1 ^ theV2) are parallel. This is because, in these conditions, the computed vector is null and cannot be normalized."]
        #[cxx_name = "gp_Dir_CrossCrossed"]
        fn Dir_cross_crossed(self_: &Dir, theV1: &Dir, theV2: &Dir) -> UniquePtr<Dir>;
        #[doc = "Reverses the orientation of a direction geometric transformations Performs the symmetrical transformation of a direction with respect to the direction V which is the center of the  symmetry.]"]
        #[cxx_name = "gp_Dir_Reversed"]
        fn Dir_reversed(self_: &Dir) -> UniquePtr<Dir>;
        #[doc = "Performs the symmetrical transformation of a direction with respect to the direction theV which is the center of the  symmetry."]
        #[cxx_name = "gp_Dir_Mirrored"]
        fn Dir_mirroreddir(self_: &Dir, theV: &Dir) -> UniquePtr<Dir>;
        #[doc = "Performs the symmetrical transformation of a direction with respect to an axis placement which is the axis of the symmetry."]
        #[cxx_name = "gp_Dir_Mirrored"]
        fn Dir_mirroredax1_2(self_: &Dir, theA1: &gp_Ax1) -> UniquePtr<Dir>;
        #[doc = "Performs the symmetrical transformation of a direction with respect to a plane. The axis placement theA2 locates the plane of the symmetry : (Location, XDirection, YDirection)."]
        #[cxx_name = "gp_Dir_Mirrored"]
        fn Dir_mirroredax2_3(self_: &Dir, theA2: &gp_Ax2) -> UniquePtr<Dir>;
        #[doc = "Rotates a direction. theA1 is the axis of the rotation. theAng is the angular value of the rotation in radians."]
        #[cxx_name = "gp_Dir_Rotated"]
        fn Dir_rotated(self_: &Dir, theA1: &gp_Ax1, theAng: f64) -> UniquePtr<Dir>;
        #[doc = "Transforms a direction with a \"Trsf\" from gp. Warnings : If the scale factor of the \"Trsf\" theT is negative then the direction <me> is reversed."]
        #[cxx_name = "gp_Dir_Transformed"]
        fn Dir_transformed(self_: &Dir, theT: &gp_Trsf) -> UniquePtr<Dir>;
        #[doc = " ======================== gp_XYZ ========================"]
        #[doc = "This class describes a cartesian coordinate entity in 3D space {X,Y,Z}. This entity is used for algebraic calculation. This entity can be transformed with a \"Trsf\" or a  \"GTrsf\" from package \"gp\". It is used in vectorial computations or for holding this type of information in data structures."]
        #[cxx_name = "gp_XYZ"]
        type XYZ;
        #[doc = "Creates an XYZ object with zero coordinates (0,0,0)"]
        #[cxx_name = "gp_XYZ_ctor"]
        fn XYZ_ctor() -> UniquePtr<XYZ>;
        #[doc = "creates an XYZ with given coordinates"]
        #[cxx_name = "gp_XYZ_ctor_real3"]
        fn XYZ_ctor_real3(theX: f64, theY: f64, theZ: f64) -> UniquePtr<XYZ>;
        #[doc = "For this XYZ object, assigns the values theX, theY and theZ to its three coordinates"]
        #[cxx_name = "SetCoord"]
        fn set_coordreal(self: Pin<&mut XYZ>, theX: f64, theY: f64, theZ: f64);
        #[doc = "modifies the coordinate of range theIndex theIndex = 1 => X is modified theIndex = 2 => Y is modified theIndex = 3 => Z is modified Raises OutOfRange if theIndex != {1, 2, 3}."]
        #[cxx_name = "SetCoord"]
        fn set_coordint_2(self: Pin<&mut XYZ>, theIndex: i32, theXi: f64);
        #[doc = "Assigns the given value to the X coordinate"]
        #[cxx_name = "SetX"]
        fn set_x(self: Pin<&mut XYZ>, theX: f64);
        #[doc = "Assigns the given value to the Y coordinate"]
        #[cxx_name = "SetY"]
        fn set_y(self: Pin<&mut XYZ>, theY: f64);
        #[doc = "Assigns the given value to the Z coordinate"]
        #[cxx_name = "SetZ"]
        fn set_z(self: Pin<&mut XYZ>, theZ: f64);
        #[doc = "returns the coordinate of range theIndex : theIndex = 1 => X is returned theIndex = 2 => Y is returned theIndex = 3 => Z is returned Raises OutOfRange if theIndex != {1, 2, 3}."]
        #[cxx_name = "Coord"]
        fn coordint(self: &XYZ, theIndex: i32) -> f64;
        #[cxx_name = "ChangeCoord"]
        fn change_coord(self: Pin<&mut XYZ>, theIndex: i32) -> &mut f64;
        #[cxx_name = "Coord"]
        fn coordreal_2(self: &XYZ, theX: &mut f64, theY: &mut f64, theZ: &mut f64);
        #[doc = "Returns a const ptr to coordinates location. Is useful for algorithms, but DOES NOT PERFORM ANY CHECKS!"]
        #[cxx_name = "GetData"]
        fn get_data(self: &XYZ) -> *const f64;
        #[doc = "Returns a ptr to coordinates location. Is useful for algorithms, but DOES NOT PERFORM ANY CHECKS!"]
        #[cxx_name = "ChangeData"]
        fn change_data(self: Pin<&mut XYZ>) -> *mut f64;
        #[doc = "Returns the X coordinate"]
        #[cxx_name = "X"]
        fn x(self: &XYZ) -> f64;
        #[doc = "Returns the Y coordinate"]
        #[cxx_name = "Y"]
        fn y(self: &XYZ) -> f64;
        #[doc = "Returns the Z coordinate"]
        #[cxx_name = "Z"]
        fn z(self: &XYZ) -> f64;
        #[doc = "computes Sqrt (X*X + Y*Y + Z*Z) where X, Y and Z are the three coordinates of this XYZ object."]
        #[cxx_name = "Modulus"]
        fn modulus(self: &XYZ) -> f64;
        #[doc = "Computes X*X + Y*Y + Z*Z where X, Y and Z are the three coordinates of this XYZ object."]
        #[cxx_name = "SquareModulus"]
        fn square_modulus(self: &XYZ) -> f64;
        #[doc = "Returns True if he coordinates of this XYZ object are equal to the respective coordinates Other, within the specified tolerance theTolerance. I.e.: abs(<me>.X() - theOther.X()) <= theTolerance and abs(<me>.Y() - theOther.Y()) <= theTolerance and abs(<me>.Z() - theOther.Z()) <= theTolerance."]
        #[cxx_name = "IsEqual"]
        fn is_equal(self: &XYZ, theOther: &XYZ, theTolerance: f64) -> bool;
        #[doc = "@code <me>.X() = <me>.X() + theOther.X() <me>.Y() = <me>.Y() + theOther.Y() <me>.Z() = <me>.Z() + theOther.Z() @endcode"]
        #[cxx_name = "Add"]
        fn add(self: Pin<&mut XYZ>, theOther: &XYZ);
        #[doc = "@code <me>.X() = <me>.Y() * theOther.Z() - <me>.Z() * theOther.Y() <me>.Y() = <me>.Z() * theOther.X() - <me>.X() * theOther.Z() <me>.Z() = <me>.X() * theOther.Y() - <me>.Y() * theOther.X() @endcode"]
        #[cxx_name = "Cross"]
        fn cross(self: Pin<&mut XYZ>, theOther: &XYZ);
        #[doc = "Computes the magnitude of the cross product between <me> and theRight. Returns || <me> ^ theRight ||"]
        #[cxx_name = "CrossMagnitude"]
        fn cross_magnitude(self: &XYZ, theRight: &XYZ) -> f64;
        #[doc = "Computes the square magnitude of the cross product between <me> and theRight. Returns || <me> ^ theRight ||**2"]
        #[cxx_name = "CrossSquareMagnitude"]
        fn cross_square_magnitude(self: &XYZ, theRight: &XYZ) -> f64;
        #[doc = "Triple vector product Computes <me> = <me>.Cross(theCoord1.Cross(theCoord2))"]
        #[cxx_name = "CrossCross"]
        fn cross_cross(self: Pin<&mut XYZ>, theCoord1: &XYZ, theCoord2: &XYZ);
        #[doc = "divides <me> by a real."]
        #[cxx_name = "Divide"]
        fn divide(self: Pin<&mut XYZ>, theScalar: f64);
        #[doc = "computes the scalar product between <me> and theOther"]
        #[cxx_name = "Dot"]
        fn dot(self: &XYZ, theOther: &XYZ) -> f64;
        #[doc = "computes the triple scalar product"]
        #[cxx_name = "DotCross"]
        fn dot_cross(self: &XYZ, theCoord1: &XYZ, theCoord2: &XYZ) -> f64;
        #[doc = "@code <me>.X() = <me>.X() * theScalar; <me>.Y() = <me>.Y() * theScalar; <me>.Z() = <me>.Z() * theScalar; @endcode"]
        #[cxx_name = "Multiply"]
        fn multiplyreal(self: Pin<&mut XYZ>, theScalar: f64);
        #[doc = "@code <me>.X() = <me>.X() * theOther.X(); <me>.Y() = <me>.Y() * theOther.Y(); <me>.Z() = <me>.Z() * theOther.Z(); @endcode"]
        #[cxx_name = "Multiply"]
        fn multiplyxyz_2(self: Pin<&mut XYZ>, theOther: &XYZ);
        #[doc = "<me> = theMatrix * <me>"]
        #[cxx_name = "Multiply"]
        fn multiplymat_3(self: Pin<&mut XYZ>, theMatrix: &gp_Mat);
        #[doc = "@code <me>.X() = <me>.X()/ <me>.Modulus() <me>.Y() = <me>.Y()/ <me>.Modulus() <me>.Z() = <me>.Z()/ <me>.Modulus() @endcode Raised if <me>.Modulus() <= Resolution from gp"]
        #[cxx_name = "Normalize"]
        fn normalize(self: Pin<&mut XYZ>);
        #[doc = "@code <me>.X() = -<me>.X() <me>.Y() = -<me>.Y() <me>.Z() = -<me>.Z() @endcode"]
        #[cxx_name = "Reverse"]
        fn reverse(self: Pin<&mut XYZ>);
        #[doc = "@code <me>.X() = <me>.X() - theOther.X() <me>.Y() = <me>.Y() - theOther.Y() <me>.Z() = <me>.Z() - theOther.Z() @endcode"]
        #[cxx_name = "Subtract"]
        fn subtract(self: Pin<&mut XYZ>, theOther: &XYZ);
        #[doc = "<me> is set to the following linear form : @code theA1 * theXYZ1 + theA2 * theXYZ2 + theA3 * theXYZ3 + theXYZ4 @endcode"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formreal(
            self: Pin<&mut XYZ>,
            theA1: f64,
            theXYZ1: &XYZ,
            theA2: f64,
            theXYZ2: &XYZ,
            theA3: f64,
            theXYZ3: &XYZ,
            theXYZ4: &XYZ,
        );
        #[doc = "<me> is set to the following linear form : @code theA1 * theXYZ1 + theA2 * theXYZ2 + theA3 * theXYZ3 @endcode"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formreal_2(
            self: Pin<&mut XYZ>,
            theA1: f64,
            theXYZ1: &XYZ,
            theA2: f64,
            theXYZ2: &XYZ,
            theA3: f64,
            theXYZ3: &XYZ,
        );
        #[doc = "<me> is set to the following linear form : @code theA1 * theXYZ1 + theA2 * theXYZ2 + theXYZ3 @endcode"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formreal_3(
            self: Pin<&mut XYZ>,
            theA1: f64,
            theXYZ1: &XYZ,
            theA2: f64,
            theXYZ2: &XYZ,
            theXYZ3: &XYZ,
        );
        #[doc = "<me> is set to the following linear form : @code theA1 * theXYZ1 + theA2 * theXYZ2 @endcode"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formreal_4(
            self: Pin<&mut XYZ>,
            theA1: f64,
            theXYZ1: &XYZ,
            theA2: f64,
            theXYZ2: &XYZ,
        );
        #[doc = "<me> is set to the following linear form : @code theA1 * theXYZ1 + theXYZ2 @endcode"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formreal_5(self: Pin<&mut XYZ>, theA1: f64, theXYZ1: &XYZ, theXYZ2: &XYZ);
        #[doc = "<me> is set to the following linear form : @code theXYZ1 + theXYZ2 @endcode"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formxyz_6(self: Pin<&mut XYZ>, theXYZ1: &XYZ, theXYZ2: &XYZ);
        #[doc = "@code new.X() = <me>.X() + theOther.X() new.Y() = <me>.Y() + theOther.Y() new.Z() = <me>.Z() + theOther.Z() @endcode"]
        #[cxx_name = "gp_XYZ_Added"]
        fn XYZ_added(self_: &XYZ, theOther: &XYZ) -> UniquePtr<XYZ>;
        #[doc = "@code new.X() = <me>.Y() * theOther.Z() - <me>.Z() * theOther.Y() new.Y() = <me>.Z() * theOther.X() - <me>.X() * theOther.Z() new.Z() = <me>.X() * theOther.Y() - <me>.Y() * theOther.X() @endcode"]
        #[cxx_name = "gp_XYZ_Crossed"]
        fn XYZ_crossed(self_: &XYZ, theOther: &XYZ) -> UniquePtr<XYZ>;
        #[doc = "Triple vector product computes New = <me>.Cross(theCoord1.Cross(theCoord2))"]
        #[cxx_name = "gp_XYZ_CrossCrossed"]
        fn XYZ_cross_crossed(self_: &XYZ, theCoord1: &XYZ, theCoord2: &XYZ) -> UniquePtr<XYZ>;
        #[doc = "divides <me> by a real."]
        #[cxx_name = "gp_XYZ_Divided"]
        fn XYZ_divided(self_: &XYZ, theScalar: f64) -> UniquePtr<XYZ>;
        #[doc = "@code New.X() = <me>.X() * theScalar; New.Y() = <me>.Y() * theScalar; New.Z() = <me>.Z() * theScalar; @endcode"]
        #[cxx_name = "gp_XYZ_Multiplied"]
        fn XYZ_multipliedreal(self_: &XYZ, theScalar: f64) -> UniquePtr<XYZ>;
        #[doc = "@code new.X() = <me>.X() * theOther.X(); new.Y() = <me>.Y() * theOther.Y(); new.Z() = <me>.Z() * theOther.Z(); @endcode"]
        #[cxx_name = "gp_XYZ_Multiplied"]
        fn XYZ_multipliedxyz_2(self_: &XYZ, theOther: &XYZ) -> UniquePtr<XYZ>;
        #[doc = "New = theMatrix * <me>"]
        #[cxx_name = "gp_XYZ_Multiplied"]
        fn XYZ_multipliedmat_3(self_: &XYZ, theMatrix: &gp_Mat) -> UniquePtr<XYZ>;
        #[doc = "@code New.X() = <me>.X()/ <me>.Modulus() New.Y() = <me>.Y()/ <me>.Modulus() New.Z() = <me>.Z()/ <me>.Modulus() @endcode Raised if <me>.Modulus() <= Resolution from gp"]
        #[cxx_name = "gp_XYZ_Normalized"]
        fn XYZ_normalized(self_: &XYZ) -> UniquePtr<XYZ>;
        #[doc = "@code New.X() = -<me>.X() New.Y() = -<me>.Y() New.Z() = -<me>.Z() @endcode"]
        #[cxx_name = "gp_XYZ_Reversed"]
        fn XYZ_reversed(self_: &XYZ) -> UniquePtr<XYZ>;
        #[doc = "@code new.X() = <me>.X() - theOther.X() new.Y() = <me>.Y() - theOther.Y() new.Z() = <me>.Z() - theOther.Z() @endcode"]
        #[cxx_name = "gp_XYZ_Subtracted"]
        fn XYZ_subtracted(self_: &XYZ, theOther: &XYZ) -> UniquePtr<XYZ>;
    }
    impl UniquePtr<Pnt> {}
    impl UniquePtr<Vec_> {}
    impl UniquePtr<Dir> {}
    impl UniquePtr<XYZ> {}
}
pub use ffi::Pnt;
impl Pnt {
    #[doc = "Creates a point with zero coordinates."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Pnt_ctor()
    }

    #[doc = "Creates a point from a XYZ object."]
    pub fn new_xyz(theCoord: &ffi::XYZ) -> cxx::UniquePtr<Self> {
        ffi::Pnt_ctor_xyz(theCoord)
    }

    #[doc = "Creates a  point with its 3 cartesian's coordinates : theXp, theYp, theZp."]
    pub fn new_real3(theXp: f64, theYp: f64, theZp: f64) -> cxx::UniquePtr<Self> {
        ffi::Pnt_ctor_real3(theXp, theYp, theZp)
    }
}
pub use ffi::Vec_ as Vec;
impl Vec {
    #[doc = "Creates a zero vector."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Vec__ctor()
    }

    #[doc = "Creates a unitary vector from a direction theV."]
    pub fn new_dir(theV: &ffi::Dir) -> cxx::UniquePtr<Self> {
        ffi::Vec__ctor_dir(theV)
    }

    #[doc = "Creates a vector with a triplet of coordinates."]
    pub fn new_xyz(theCoord: &ffi::XYZ) -> cxx::UniquePtr<Self> {
        ffi::Vec__ctor_xyz(theCoord)
    }

    #[doc = "Creates a point with its three cartesian coordinates."]
    pub fn new_real3(theXv: f64, theYv: f64, theZv: f64) -> cxx::UniquePtr<Self> {
        ffi::Vec__ctor_real3(theXv, theYv, theZv)
    }

    #[doc = "Creates a vector from two points. The length of the vector is the distance between theP1 and theP2"]
    pub fn new_pnt2(theP1: &ffi::Pnt, theP2: &ffi::Pnt) -> cxx::UniquePtr<Self> {
        ffi::Vec__ctor_pnt2(theP1, theP2)
    }
}
pub use ffi::Dir;
impl Dir {
    #[doc = "Creates a direction corresponding to X axis."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Dir_ctor()
    }

    #[doc = "Normalizes the vector theV and creates a direction. Raises ConstructionError if theV.Magnitude() <= Resolution."]
    pub fn new_vec(theV: &ffi::Vec_) -> cxx::UniquePtr<Self> {
        ffi::Dir_ctor_vec(theV)
    }

    #[doc = "Creates a direction from a triplet of coordinates. Raises ConstructionError if theCoord.Modulus() <= Resolution from gp."]
    pub fn new_xyz(theCoord: &ffi::XYZ) -> cxx::UniquePtr<Self> {
        ffi::Dir_ctor_xyz(theCoord)
    }

    #[doc = "Creates a direction with its 3 cartesian coordinates. Raises ConstructionError if Sqrt(theXv*theXv + theYv*theYv + theZv*theZv) <= Resolution Modification of the direction's coordinates If Sqrt (theXv*theXv + theYv*theYv + theZv*theZv) <= Resolution from gp where theXv, theYv ,theZv are the new coordinates it is not possible to construct the direction and the method raises the exception ConstructionError."]
    pub fn new_real3(theXv: f64, theYv: f64, theZv: f64) -> cxx::UniquePtr<Self> {
        ffi::Dir_ctor_real3(theXv, theYv, theZv)
    }
}
pub use ffi::XYZ;
impl XYZ {
    #[doc = "Creates an XYZ object with zero coordinates (0,0,0)"]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::XYZ_ctor()
    }

    #[doc = "creates an XYZ with given coordinates"]
    pub fn new_real3(theX: f64, theY: f64, theZ: f64) -> cxx::UniquePtr<Self> {
        ffi::XYZ_ctor_real3(theX, theY, theZ)
    }
}
