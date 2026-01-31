#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_gp.hxx");
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "gp_XYZ"]
        type gp_XYZ;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "gp_Ax1"]
        type gp_Ax1;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "gp_Ax2"]
        type gp_Ax2;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "gp_Trsf"]
        type gp_Trsf;
        #[doc = "Defines a 3D cartesian point."]
        #[cxx_name = "gp_Pnt"]
        type Pnt;
        #[doc = "Creates a point with zero coordinates."]
        #[cxx_name = "gp_Pnt_ctor"]
        fn Pnt_ctor() -> UniquePtr<Pnt>;
        #[doc = "Creates a point from a XYZ object."]
        #[cxx_name = "gp_Pnt_ctor_xyz"]
        fn Pnt_ctor_xyz(the_coord: &gp_XYZ) -> UniquePtr<Pnt>;
        #[doc = "Creates a  point with its 3 cartesian's coordinates : theXp, theYp, theZp."]
        #[cxx_name = "gp_Pnt_ctor_real_real_real"]
        fn Pnt_ctor_real_real_real(the_xp: f64, the_yp: f64, the_zp: f64) -> UniquePtr<Pnt>;
        #[doc = "Changes the coordinate of range theIndex : theIndex = 1 => X is modified theIndex = 2 => Y is modified theIndex = 3 => Z is modified Raised if theIndex != {1, 2, 3}."]
        #[cxx_name = "SetCoord"]
        fn set_coordint(self: Pin<&mut Pnt>, the_index: i32, the_xi: f64);
        #[doc = "For this point, assigns  the values theXp, theYp and theZp to its three coordinates."]
        #[cxx_name = "SetCoord"]
        fn set_coordreal_2(self: Pin<&mut Pnt>, the_xp: f64, the_yp: f64, the_zp: f64);
        #[doc = "Assigns the given value to the X coordinate of this point."]
        #[cxx_name = "SetX"]
        fn set_x(self: Pin<&mut Pnt>, the_x: f64);
        #[doc = "Assigns the given value to the Y coordinate of this point."]
        #[cxx_name = "SetY"]
        fn set_y(self: Pin<&mut Pnt>, the_y: f64);
        #[doc = "Assigns the given value to the Z coordinate of this point."]
        #[cxx_name = "SetZ"]
        fn set_z(self: Pin<&mut Pnt>, the_z: f64);
        #[doc = "Assigns the three coordinates of theCoord to this point."]
        #[cxx_name = "SetXYZ"]
        fn set_xyz(self: Pin<&mut Pnt>, the_coord: &gp_XYZ);
        #[doc = "Returns the coordinate of corresponding to the value of theIndex : theIndex = 1 => X is returned theIndex = 2 => Y is returned theIndex = 3 => Z is returned Raises OutOfRange if theIndex != {1, 2, 3}. Raised if theIndex != {1, 2, 3}."]
        #[cxx_name = "Coord"]
        fn coordint(self: &Pnt, the_index: i32) -> f64;
        #[doc = "For this point gives its three coordinates theXp, theYp and theZp."]
        #[cxx_name = "Coord"]
        fn coordreal_2(self: &Pnt, the_xp: &mut f64, the_yp: &mut f64, the_zp: &mut f64);
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
        fn xyz(self: &Pnt) -> &gp_XYZ;
        #[doc = "For this point, returns its three coordinates as a XYZ object."]
        #[cxx_name = "Coord"]
        fn coord3(self: &Pnt) -> &gp_XYZ;
        #[doc = "Returns the coordinates of this point. Note: This syntax allows direct modification of the returned value."]
        #[cxx_name = "ChangeCoord"]
        fn change_coord(self: Pin<&mut Pnt>) -> Pin<&mut gp_XYZ>;
        #[doc = "Assigns the result of the following expression to this point (theAlpha*this + theBeta*theP) / (theAlpha + theBeta)"]
        #[cxx_name = "BaryCenter"]
        fn bary_center(self: Pin<&mut Pnt>, the_alpha: f64, the_p: &Pnt, the_beta: f64);
        #[doc = "Comparison Returns True if the distance between the two points is lower or equal to theLinearTolerance."]
        #[cxx_name = "IsEqual"]
        fn is_equal(self: &Pnt, the_other: &Pnt, the_linear_tolerance: f64) -> bool;
        #[doc = "Computes the distance between two points."]
        #[cxx_name = "Distance"]
        fn distance(self: &Pnt, the_other: &Pnt) -> f64;
        #[doc = "Computes the square distance between two points."]
        #[cxx_name = "SquareDistance"]
        fn square_distance(self: &Pnt, the_other: &Pnt) -> f64;
        #[doc = "Performs the symmetrical transformation of a point with respect to the point theP which is the center of the  symmetry."]
        #[cxx_name = "Mirror"]
        fn mirrorpnt(self: Pin<&mut Pnt>, the_p: &Pnt);
        #[cxx_name = "Mirror"]
        fn mirrorax1_2(self: Pin<&mut Pnt>, the_a1: &gp_Ax1);
        #[cxx_name = "Mirror"]
        fn mirrorax2_3(self: Pin<&mut Pnt>, the_a2: &gp_Ax2);
        #[cxx_name = "Rotate"]
        fn rotate(self: Pin<&mut Pnt>, the_a1: &gp_Ax1, the_ang: f64);
        #[doc = "Scales a point. theS is the scaling value."]
        #[cxx_name = "Scale"]
        fn scale(self: Pin<&mut Pnt>, the_p: &Pnt, the_s: f64);
        #[doc = "Transforms a point with the transformation T."]
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Pnt>, the_t: &gp_Trsf);
        #[doc = "Translates a point in the direction of the vector theV. The magnitude of the translation is the vector's magnitude."]
        #[cxx_name = "Translate"]
        fn translatevec(self: Pin<&mut Pnt>, the_v: &Vec_);
        #[doc = "Translates a point from the point theP1 to the point theP2."]
        #[cxx_name = "Translate"]
        fn translatepnt_2(self: Pin<&mut Pnt>, the_p1: &Pnt, the_p2: &Pnt);
        #[doc = "Performs the symmetrical transformation of a point with respect to an axis placement which is the axis of the symmetry."]
        #[cxx_name = "gp_Pnt_Mirrored"]
        fn Pnt_mirroredpnt(self_: &Pnt, the_p: &Pnt) -> UniquePtr<Pnt>;
        #[doc = "Performs the symmetrical transformation of a point with respect to a plane. The axis placement theA2 locates the plane of the symmetry : (Location, XDirection, YDirection)."]
        #[cxx_name = "gp_Pnt_Mirrored"]
        fn Pnt_mirroredax1_2(self_: &Pnt, the_a1: &gp_Ax1) -> UniquePtr<Pnt>;
        #[doc = "Rotates a point. theA1 is the axis of the rotation. theAng is the angular value of the rotation in radians."]
        #[cxx_name = "gp_Pnt_Mirrored"]
        fn Pnt_mirroredax2_3(self_: &Pnt, the_a2: &gp_Ax2) -> UniquePtr<Pnt>;
        #[cxx_name = "gp_Pnt_Rotated"]
        fn Pnt_rotated(self_: &Pnt, the_a1: &gp_Ax1, the_ang: f64) -> UniquePtr<Pnt>;
        #[cxx_name = "gp_Pnt_Scaled"]
        fn Pnt_scaled(self_: &Pnt, the_p: &Pnt, the_s: f64) -> UniquePtr<Pnt>;
        #[cxx_name = "gp_Pnt_Transformed"]
        fn Pnt_transformed(self_: &Pnt, the_t: &gp_Trsf) -> UniquePtr<Pnt>;
        #[cxx_name = "gp_Pnt_Translated"]
        fn Pnt_translatedvec(self_: &Pnt, the_v: &Vec_) -> UniquePtr<Pnt>;
        #[cxx_name = "gp_Pnt_Translated"]
        fn Pnt_translatedpnt_2(self_: &Pnt, the_p1: &Pnt, the_p2: &Pnt) -> UniquePtr<Pnt>;
        #[doc = "Defines a non-persistent vector in 3D space."]
        #[cxx_name = "gp_Vec"]
        type Vec_;
        #[doc = "Creates a zero vector."]
        #[cxx_name = "gp_Vec_ctor"]
        fn Vec__ctor() -> UniquePtr<Vec_>;
        #[doc = "Creates a unitary vector from a direction theV."]
        #[cxx_name = "gp_Vec_ctor_dir"]
        fn Vec__ctor_dir(the_v: &Dir) -> UniquePtr<Vec_>;
        #[doc = "Creates a vector with a triplet of coordinates."]
        #[cxx_name = "gp_Vec_ctor_xyz"]
        fn Vec__ctor_xyz(the_coord: &gp_XYZ) -> UniquePtr<Vec_>;
        #[doc = "Creates a point with its three cartesian coordinates."]
        #[cxx_name = "gp_Vec_ctor_real_real_real"]
        fn Vec__ctor_real_real_real(the_xv: f64, the_yv: f64, the_zv: f64) -> UniquePtr<Vec_>;
        #[doc = "Creates a vector from two points. The length of the vector is the distance between theP1 and theP2"]
        #[cxx_name = "gp_Vec_ctor_pnt_pnt"]
        fn Vec__ctor_pnt_pnt(the_p1: &Pnt, the_p2: &Pnt) -> UniquePtr<Vec_>;
        #[doc = "Changes the coordinate of range theIndex theIndex = 1 => X is modified theIndex = 2 => Y is modified theIndex = 3 => Z is modified Raised if theIndex != {1, 2, 3}."]
        #[cxx_name = "SetCoord"]
        fn set_coordint(self: Pin<&mut Vec_>, the_index: i32, the_xi: f64);
        #[doc = "For this vector, assigns -   the values theXv, theYv and theZv to its three coordinates."]
        #[cxx_name = "SetCoord"]
        fn set_coordreal_2(self: Pin<&mut Vec_>, the_xv: f64, the_yv: f64, the_zv: f64);
        #[doc = "Assigns the given value to the X coordinate of this vector."]
        #[cxx_name = "SetX"]
        fn set_x(self: Pin<&mut Vec_>, the_x: f64);
        #[doc = "Assigns the given value to the X coordinate of this vector."]
        #[cxx_name = "SetY"]
        fn set_y(self: Pin<&mut Vec_>, the_y: f64);
        #[doc = "Assigns the given value to the X coordinate of this vector."]
        #[cxx_name = "SetZ"]
        fn set_z(self: Pin<&mut Vec_>, the_z: f64);
        #[doc = "Assigns the three coordinates of theCoord to this vector."]
        #[cxx_name = "SetXYZ"]
        fn set_xyz(self: Pin<&mut Vec_>, the_coord: &gp_XYZ);
        #[doc = "Returns the coordinate of range theIndex : theIndex = 1 => X is returned theIndex = 2 => Y is returned theIndex = 3 => Z is returned Raised if theIndex != {1, 2, 3}."]
        #[cxx_name = "Coord"]
        fn coordint(self: &Vec_, the_index: i32) -> f64;
        #[doc = "For this vector returns its three coordinates theXv, theYv, and theZv inline"]
        #[cxx_name = "Coord"]
        fn coordreal_2(self: &Vec_, the_xv: &mut f64, the_yv: &mut f64, the_zv: &mut f64);
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
        fn xyz(self: &Vec_) -> &gp_XYZ;
        #[doc = "Returns True if the two vectors have the same magnitude value and the same direction. The precision values are theLinearTolerance for the magnitude and theAngularTolerance for the direction."]
        #[cxx_name = "IsEqual"]
        fn is_equal(
            self: &Vec_,
            the_other: &Vec_,
            the_linear_tolerance: f64,
            the_angular_tolerance: f64,
        ) -> bool;
        #[doc = "Returns True if abs(<me>.Angle(theOther) - PI/2.) <= theAngularTolerance Raises VectorWithNullMagnitude if <me>.Magnitude() <= Resolution or theOther.Magnitude() <= Resolution from gp"]
        #[cxx_name = "IsNormal"]
        fn is_normal(self: &Vec_, the_other: &Vec_, the_angular_tolerance: f64) -> bool;
        #[doc = "Returns True if PI - <me>.Angle(theOther) <= theAngularTolerance Raises VectorWithNullMagnitude if <me>.Magnitude() <= Resolution or Other.Magnitude() <= Resolution from gp"]
        #[cxx_name = "IsOpposite"]
        fn is_opposite(self: &Vec_, the_other: &Vec_, the_angular_tolerance: f64) -> bool;
        #[doc = "Returns True if Angle(<me>, theOther) <= theAngularTolerance or PI - Angle(<me>, theOther) <= theAngularTolerance This definition means that two parallel vectors cannot define a plane but two vectors with opposite directions are considered as parallel. Raises VectorWithNullMagnitude if <me>.Magnitude() <= Resolution or Other.Magnitude() <= Resolution from gp"]
        #[cxx_name = "IsParallel"]
        fn is_parallel(self: &Vec_, the_other: &Vec_, the_angular_tolerance: f64) -> bool;
        #[doc = "Computes the angular value between <me> and <theOther> Returns the angle value between 0 and PI in radian. Raises VectorWithNullMagnitude if <me>.Magnitude() <= Resolution from gp or theOther.Magnitude() <= Resolution because the angular value is indefinite if one of the vectors has a null magnitude."]
        #[cxx_name = "Angle"]
        fn angle(self: &Vec_, the_other: &Vec_) -> f64;
        #[doc = "Computes the angle, in radians, between this vector and vector theOther. The result is a value between -Pi and Pi. For this, theVRef defines the positive sense of rotation: the angular value is positive, if the cross product this ^ theOther has the same orientation as theVRef relative to the plane defined by the vectors this and theOther. Otherwise, the angular value is negative. Exceptions gp_VectorWithNullMagnitude if the magnitude of this vector, the vector theOther, or the vector theVRef is less than or equal to gp::Resolution(). Standard_DomainError if this vector, the vector theOther, and the vector theVRef are coplanar, unless this vector and the vector theOther are parallel."]
        #[cxx_name = "AngleWithRef"]
        fn angle_with_ref(self: &Vec_, the_other: &Vec_, the_v_ref: &Vec_) -> f64;
        #[doc = "Computes the magnitude of this vector."]
        #[cxx_name = "Magnitude"]
        fn magnitude(self: &Vec_) -> f64;
        #[doc = "Computes the square magnitude of this vector."]
        #[cxx_name = "SquareMagnitude"]
        fn square_magnitude(self: &Vec_) -> f64;
        #[doc = "Adds two vectors"]
        #[cxx_name = "Add"]
        fn add(self: Pin<&mut Vec_>, the_other: &Vec_);
        #[doc = "Subtracts two vectors"]
        #[cxx_name = "Subtract"]
        fn subtract(self: Pin<&mut Vec_>, the_right: &Vec_);
        #[doc = "Multiplies a vector by a scalar"]
        #[cxx_name = "Multiply"]
        fn multiply(self: Pin<&mut Vec_>, the_scalar: f64);
        #[doc = "Divides a vector by a scalar"]
        #[cxx_name = "Divide"]
        fn divide(self: Pin<&mut Vec_>, the_scalar: f64);
        #[doc = "computes the cross product between two vectors"]
        #[cxx_name = "Cross"]
        fn cross(self: Pin<&mut Vec_>, the_right: &Vec_);
        #[doc = "Computes the magnitude of the cross product between <me> and theRight. Returns || <me> ^ theRight ||"]
        #[cxx_name = "CrossMagnitude"]
        fn cross_magnitude(self: &Vec_, the_right: &Vec_) -> f64;
        #[doc = "Computes the square magnitude of the cross product between <me> and theRight. Returns || <me> ^ theRight ||**2"]
        #[cxx_name = "CrossSquareMagnitude"]
        fn cross_square_magnitude(self: &Vec_, the_right: &Vec_) -> f64;
        #[doc = "Computes the triple vector product. <me> ^= (theV1 ^ theV2)"]
        #[cxx_name = "CrossCross"]
        fn cross_cross(self: Pin<&mut Vec_>, the_v1: &Vec_, the_v2: &Vec_);
        #[doc = "computes the scalar product"]
        #[cxx_name = "Dot"]
        fn dot(self: &Vec_, the_other: &Vec_) -> f64;
        #[doc = "Computes the triple scalar product <me> * (theV1 ^ theV2)."]
        #[cxx_name = "DotCross"]
        fn dot_cross(self: &Vec_, the_v1: &Vec_, the_v2: &Vec_) -> f64;
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
            the_a1: f64,
            the_v1: &Vec_,
            the_a2: f64,
            the_v2: &Vec_,
            the_a3: f64,
            the_v3: &Vec_,
            the_v4: &Vec_,
        );
        #[doc = "<me> is set to the following linear form : theA1 * theV1 + theA2 * theV2 + theA3 * theV3"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formreal_2(
            self: Pin<&mut Vec_>,
            the_a1: f64,
            the_v1: &Vec_,
            the_a2: f64,
            the_v2: &Vec_,
            the_a3: f64,
            the_v3: &Vec_,
        );
        #[doc = "<me> is set to the following linear form : theA1 * theV1 + theA2 * theV2 + theV3"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formreal_3(
            self: Pin<&mut Vec_>,
            the_a1: f64,
            the_v1: &Vec_,
            the_a2: f64,
            the_v2: &Vec_,
            the_v3: &Vec_,
        );
        #[doc = "<me> is set to the following linear form : theA1 * theV1 + theA2 * theV2"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formreal_4(
            self: Pin<&mut Vec_>,
            the_a1: f64,
            the_v1: &Vec_,
            the_a2: f64,
            the_v2: &Vec_,
        );
        #[doc = "<me> is set to the following linear form : theA1 * theV1 + theV2"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formreal_5(self: Pin<&mut Vec_>, the_a1: f64, the_v1: &Vec_, the_v2: &Vec_);
        #[doc = "<me> is set to the following linear form : theV1 + theV2"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formvec_6(self: Pin<&mut Vec_>, the_v1: &Vec_, the_v2: &Vec_);
        #[cxx_name = "Mirror"]
        fn mirrorvec(self: Pin<&mut Vec_>, the_v: &Vec_);
        #[cxx_name = "Mirror"]
        fn mirrorax1_2(self: Pin<&mut Vec_>, the_a1: &gp_Ax1);
        #[cxx_name = "Mirror"]
        fn mirrorax2_3(self: Pin<&mut Vec_>, the_a2: &gp_Ax2);
        #[cxx_name = "Rotate"]
        fn rotate(self: Pin<&mut Vec_>, the_a1: &gp_Ax1, the_ang: f64);
        #[cxx_name = "Scale"]
        fn scale(self: Pin<&mut Vec_>, the_s: f64);
        #[doc = "Transforms a vector with the transformation theT."]
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Vec_>, the_t: &gp_Trsf);
        #[doc = "Adds two vectors"]
        #[cxx_name = "gp_Vec_Added"]
        fn Vec__added(self_: &Vec_, the_other: &Vec_) -> UniquePtr<Vec_>;
        #[doc = "Subtracts two vectors"]
        #[cxx_name = "gp_Vec_Subtracted"]
        fn Vec__subtracted(self_: &Vec_, the_right: &Vec_) -> UniquePtr<Vec_>;
        #[doc = "Multiplies a vector by a scalar"]
        #[cxx_name = "gp_Vec_Multiplied"]
        fn Vec__multiplied(self_: &Vec_, the_scalar: f64) -> UniquePtr<Vec_>;
        #[doc = "Divides a vector by a scalar"]
        #[cxx_name = "gp_Vec_Divided"]
        fn Vec__divided(self_: &Vec_, the_scalar: f64) -> UniquePtr<Vec_>;
        #[doc = "computes the cross product between two vectors"]
        #[cxx_name = "gp_Vec_Crossed"]
        fn Vec__crossed(self_: &Vec_, the_right: &Vec_) -> UniquePtr<Vec_>;
        #[doc = "Computes the triple vector product. <me> ^ (theV1 ^ theV2)"]
        #[cxx_name = "gp_Vec_CrossCrossed"]
        fn Vec__cross_crossed(self_: &Vec_, the_v1: &Vec_, the_v2: &Vec_) -> UniquePtr<Vec_>;
        #[doc = "normalizes a vector Raises an exception if the magnitude of the vector is lower or equal to Resolution from gp."]
        #[cxx_name = "gp_Vec_Normalized"]
        fn Vec__normalized(self_: &Vec_) -> UniquePtr<Vec_>;
        #[doc = "Reverses the direction of a vector"]
        #[cxx_name = "gp_Vec_Reversed"]
        fn Vec__reversed(self_: &Vec_) -> UniquePtr<Vec_>;
        #[doc = "Performs the symmetrical transformation of a vector with respect to the vector theV which is the center of the  symmetry."]
        #[cxx_name = "gp_Vec_Mirrored"]
        fn Vec__mirroredvec(self_: &Vec_, the_v: &Vec_) -> UniquePtr<Vec_>;
        #[doc = "Performs the symmetrical transformation of a vector with respect to an axis placement which is the axis of the symmetry."]
        #[cxx_name = "gp_Vec_Mirrored"]
        fn Vec__mirroredax1_2(self_: &Vec_, the_a1: &gp_Ax1) -> UniquePtr<Vec_>;
        #[doc = "Performs the symmetrical transformation of a vector with respect to a plane. The axis placement theA2 locates the plane of the symmetry : (Location, XDirection, YDirection)."]
        #[cxx_name = "gp_Vec_Mirrored"]
        fn Vec__mirroredax2_3(self_: &Vec_, the_a2: &gp_Ax2) -> UniquePtr<Vec_>;
        #[doc = "Rotates a vector. theA1 is the axis of the rotation. theAng is the angular value of the rotation in radians."]
        #[cxx_name = "gp_Vec_Rotated"]
        fn Vec__rotated(self_: &Vec_, the_a1: &gp_Ax1, the_ang: f64) -> UniquePtr<Vec_>;
        #[doc = "Scales a vector. theS is the scaling value."]
        #[cxx_name = "gp_Vec_Scaled"]
        fn Vec__scaled(self_: &Vec_, the_s: f64) -> UniquePtr<Vec_>;
        #[doc = "Transforms a vector with the transformation theT."]
        #[cxx_name = "gp_Vec_Transformed"]
        fn Vec__transformed(self_: &Vec_, the_t: &gp_Trsf) -> UniquePtr<Vec_>;
        #[doc = "Describes a unit vector in 3D space. This unit vector is also called \"Direction\". See Also gce_MakeDir which provides functions for more complex unit vector constructions Geom_Direction which provides additional functions for constructing unit vectors and works, in particular, with the parametric equations of unit vectors."]
        #[cxx_name = "gp_Dir"]
        type Dir;
        #[doc = "Creates a direction corresponding to X axis."]
        #[cxx_name = "gp_Dir_ctor"]
        fn Dir_ctor() -> UniquePtr<Dir>;
        #[doc = "Normalizes the vector theV and creates a direction. Raises ConstructionError if theV.Magnitude() <= Resolution."]
        #[cxx_name = "gp_Dir_ctor_vec"]
        fn Dir_ctor_vec(the_v: &Vec_) -> UniquePtr<Dir>;
        #[doc = "Creates a direction from a triplet of coordinates. Raises ConstructionError if theCoord.Modulus() <= Resolution from gp."]
        #[cxx_name = "gp_Dir_ctor_xyz"]
        fn Dir_ctor_xyz(the_coord: &gp_XYZ) -> UniquePtr<Dir>;
        #[doc = "Creates a direction with its 3 cartesian coordinates. Raises ConstructionError if Sqrt(theXv*theXv + theYv*theYv + theZv*theZv) <= Resolution Modification of the direction's coordinates If Sqrt (theXv*theXv + theYv*theYv + theZv*theZv) <= Resolution from gp where theXv, theYv ,theZv are the new coordinates it is not possible to construct the direction and the method raises the exception ConstructionError."]
        #[cxx_name = "gp_Dir_ctor_real_real_real"]
        fn Dir_ctor_real_real_real(the_xv: f64, the_yv: f64, the_zv: f64) -> UniquePtr<Dir>;
        #[doc = "For this unit vector,  assigns the value Xi to: -   the X coordinate if theIndex is 1, or -   the Y coordinate if theIndex is 2, or -   the Z coordinate if theIndex is 3, and then normalizes it. Warning Remember that all the coordinates of a unit vector are implicitly modified when any single one is changed directly. Exceptions Standard_OutOfRange if theIndex is not 1, 2, or 3. Standard_ConstructionError if either of the following is less than or equal to gp::Resolution(): -   Sqrt(Xv*Xv + Yv*Yv + Zv*Zv), or -   the modulus of the number triple formed by the new value theXi and the two other coordinates of this vector that were not directly modified."]
        #[cxx_name = "SetCoord"]
        fn set_coordint(self: Pin<&mut Dir>, the_index: i32, the_xi: f64);
        #[doc = "For this unit vector,  assigns the values theXv, theYv and theZv to its three coordinates. Remember that all the coordinates of a unit vector are implicitly modified when any single one is changed directly."]
        #[cxx_name = "SetCoord"]
        fn set_coordreal_2(self: Pin<&mut Dir>, the_xv: f64, the_yv: f64, the_zv: f64);
        #[doc = "Assigns the given value to the X coordinate of this   unit vector."]
        #[cxx_name = "SetX"]
        fn set_x(self: Pin<&mut Dir>, the_x: f64);
        #[doc = "Assigns the given value to the Y coordinate of this   unit vector."]
        #[cxx_name = "SetY"]
        fn set_y(self: Pin<&mut Dir>, the_y: f64);
        #[doc = "Assigns the given value to the Z  coordinate of this   unit vector."]
        #[cxx_name = "SetZ"]
        fn set_z(self: Pin<&mut Dir>, the_z: f64);
        #[doc = "Assigns the three coordinates of theCoord to this unit vector."]
        #[cxx_name = "SetXYZ"]
        fn set_xyz(self: Pin<&mut Dir>, the_coord: &gp_XYZ);
        #[doc = "Returns the coordinate of range theIndex : theIndex = 1 => X is returned Ithendex = 2 => Y is returned theIndex = 3 => Z is returned Exceptions Standard_OutOfRange if theIndex is not 1, 2, or 3."]
        #[cxx_name = "Coord"]
        fn coordint(self: &Dir, the_index: i32) -> f64;
        #[doc = "Returns for the  unit vector  its three coordinates theXv, theYv, and theZv."]
        #[cxx_name = "Coord"]
        fn coordreal_2(self: &Dir, the_xv: &mut f64, the_yv: &mut f64, the_zv: &mut f64);
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
        fn xyz(self: &Dir) -> &gp_XYZ;
        #[doc = "Returns True if the angle between the two directions is lower or equal to theAngularTolerance."]
        #[cxx_name = "IsEqual"]
        fn is_equal(self: &Dir, the_other: &Dir, the_angular_tolerance: f64) -> bool;
        #[doc = "Returns True if  the angle between this unit vector and the unit vector theOther is equal to Pi/2 (normal)."]
        #[cxx_name = "IsNormal"]
        fn is_normal(self: &Dir, the_other: &Dir, the_angular_tolerance: f64) -> bool;
        #[doc = "Returns True if  the angle between this unit vector and the unit vector theOther is equal to Pi (opposite)."]
        #[cxx_name = "IsOpposite"]
        fn is_opposite(self: &Dir, the_other: &Dir, the_angular_tolerance: f64) -> bool;
        #[doc = "Returns true if the angle between this unit vector and the unit vector theOther is equal to 0 or to Pi. Note: the tolerance criterion is given by theAngularTolerance."]
        #[cxx_name = "IsParallel"]
        fn is_parallel(self: &Dir, the_other: &Dir, the_angular_tolerance: f64) -> bool;
        #[doc = "Computes the angular value in radians between <me> and <theOther>. This value is always positive in 3D space. Returns the angle in the range [0, PI]"]
        #[cxx_name = "Angle"]
        fn angle(self: &Dir, the_other: &Dir) -> f64;
        #[doc = "Computes the angular value between <me> and <theOther>. <theVRef> is the direction of reference normal to <me> and <theOther> and its orientation gives the positive sense of rotation. If the cross product <me> ^ <theOther> has the same orientation as <theVRef> the angular value is positive else negative. Returns the angular value in the range -PI and PI (in radians). Raises  DomainError if <me> and <theOther> are not parallel this exception is raised when <theVRef> is in the same plane as <me> and <theOther> The tolerance criterion is Resolution from package gp."]
        #[cxx_name = "AngleWithRef"]
        fn angle_with_ref(self: &Dir, the_other: &Dir, the_v_ref: &Dir) -> f64;
        #[doc = "Computes the cross product between two directions Raises the exception ConstructionError if the two directions are parallel because the computed vector cannot be normalized to create a direction."]
        #[cxx_name = "Cross"]
        fn cross(self: Pin<&mut Dir>, the_right: &Dir);
        #[cxx_name = "CrossCross"]
        fn cross_cross(self: Pin<&mut Dir>, the_v1: &Dir, the_v2: &Dir);
        #[doc = "Computes the scalar product"]
        #[cxx_name = "Dot"]
        fn dot(self: &Dir, the_other: &Dir) -> f64;
        #[doc = "Computes the triple scalar product <me> * (theV1 ^ theV2). Warnings : The computed vector theV1' = theV1 ^ theV2 is not normalized to create a unitary vector. So this method never raises an exception even if theV1 and theV2 are parallel."]
        #[cxx_name = "DotCross"]
        fn dot_cross(self: &Dir, the_v1: &Dir, the_v2: &Dir) -> f64;
        #[cxx_name = "Reverse"]
        fn reverse(self: Pin<&mut Dir>);
        #[cxx_name = "Mirror"]
        fn mirrordir(self: Pin<&mut Dir>, the_v: &Dir);
        #[cxx_name = "Mirror"]
        fn mirrorax1_2(self: Pin<&mut Dir>, the_a1: &gp_Ax1);
        #[cxx_name = "Mirror"]
        fn mirrorax2_3(self: Pin<&mut Dir>, the_a2: &gp_Ax2);
        #[cxx_name = "Rotate"]
        fn rotate(self: Pin<&mut Dir>, the_a1: &gp_Ax1, the_ang: f64);
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Dir>, the_t: &gp_Trsf);
        #[doc = "Computes the triple vector product. <me> ^ (V1 ^ V2) Raises the exception ConstructionError if V1 and V2 are parallel or <me> and (V1^V2) are parallel because the computed vector can't be normalized to create a direction."]
        #[cxx_name = "gp_Dir_Crossed"]
        fn Dir_crossed(self_: &Dir, the_right: &Dir) -> UniquePtr<Dir>;
        #[doc = "Computes the double vector product this ^ (theV1 ^ theV2). -   CrossCrossed creates a new unit vector. Exceptions Standard_ConstructionError if: -   theV1 and theV2 are parallel, or -   this unit vector and (theV1 ^ theV2) are parallel. This is because, in these conditions, the computed vector is null and cannot be normalized."]
        #[cxx_name = "gp_Dir_CrossCrossed"]
        fn Dir_cross_crossed(self_: &Dir, the_v1: &Dir, the_v2: &Dir) -> UniquePtr<Dir>;
        #[doc = "Reverses the orientation of a direction geometric transformations Performs the symmetrical transformation of a direction with respect to the direction V which is the center of the  symmetry.]"]
        #[cxx_name = "gp_Dir_Reversed"]
        fn Dir_reversed(self_: &Dir) -> UniquePtr<Dir>;
        #[doc = "Performs the symmetrical transformation of a direction with respect to the direction theV which is the center of the  symmetry."]
        #[cxx_name = "gp_Dir_Mirrored"]
        fn Dir_mirroreddir(self_: &Dir, the_v: &Dir) -> UniquePtr<Dir>;
        #[doc = "Performs the symmetrical transformation of a direction with respect to an axis placement which is the axis of the symmetry."]
        #[cxx_name = "gp_Dir_Mirrored"]
        fn Dir_mirroredax1_2(self_: &Dir, the_a1: &gp_Ax1) -> UniquePtr<Dir>;
        #[doc = "Performs the symmetrical transformation of a direction with respect to a plane. The axis placement theA2 locates the plane of the symmetry : (Location, XDirection, YDirection)."]
        #[cxx_name = "gp_Dir_Mirrored"]
        fn Dir_mirroredax2_3(self_: &Dir, the_a2: &gp_Ax2) -> UniquePtr<Dir>;
        #[doc = "Rotates a direction. theA1 is the axis of the rotation. theAng is the angular value of the rotation in radians."]
        #[cxx_name = "gp_Dir_Rotated"]
        fn Dir_rotated(self_: &Dir, the_a1: &gp_Ax1, the_ang: f64) -> UniquePtr<Dir>;
        #[doc = "Transforms a direction with a \"Trsf\" from gp. Warnings : If the scale factor of the \"Trsf\" theT is negative then the direction <me> is reversed."]
        #[cxx_name = "gp_Dir_Transformed"]
        fn Dir_transformed(self_: &Dir, the_t: &gp_Trsf) -> UniquePtr<Dir>;
    }
    impl UniquePtr<Pnt> {}
    impl UniquePtr<Vec_> {}
    impl UniquePtr<Dir> {}
}
pub use ffi::Dir;
pub use ffi::Dir_ctor;
pub use ffi::Dir_ctor_real_real_real;
pub use ffi::Dir_ctor_vec;
pub use ffi::Dir_ctor_xyz;
pub use ffi::Pnt;
pub use ffi::Pnt_ctor;
pub use ffi::Pnt_ctor_real_real_real;
pub use ffi::Pnt_ctor_xyz;
pub use ffi::Vec_ as Vec;
pub use ffi::Vec__ctor;
pub use ffi::Vec__ctor_dir;
pub use ffi::Vec__ctor_pnt_pnt;
pub use ffi::Vec__ctor_real_real_real;
pub use ffi::Vec__ctor_xyz;
