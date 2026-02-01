#![doc = r" Generated CXX bridge for OCCT module"]
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]
#[cxx::bridge]
pub(crate) mod ffi {
    unsafe extern "C++" {
        include!("wrapper_gp.hxx");
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "gp_Mat2d"]
        type gp_Mat2d;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "gp_Quaternion"]
        type gp_Quaternion;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "gp_TrsfForm"]
        type gp_TrsfForm;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "gp_Mat"]
        type gp_Mat;
        #[doc = r" Referenced type from C++"]
        #[cxx_name = "gp_XY"]
        type gp_XY;
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
        fn mirrorax1_2(self: Pin<&mut Pnt>, theA1: &Ax1);
        #[cxx_name = "Mirror"]
        fn mirrorax2_3(self: Pin<&mut Pnt>, theA2: &Ax2);
        #[cxx_name = "Rotate"]
        fn rotate(self: Pin<&mut Pnt>, theA1: &Ax1, theAng: f64);
        #[doc = "Scales a point. theS is the scaling value."]
        #[cxx_name = "Scale"]
        fn scale(self: Pin<&mut Pnt>, theP: &Pnt, theS: f64);
        #[doc = "Transforms a point with the transformation T."]
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Pnt>, theT: &Trsf);
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
        fn Pnt_mirroredax1_2(self_: &Pnt, theA1: &Ax1) -> UniquePtr<Pnt>;
        #[doc = "Rotates a point. theA1 is the axis of the rotation. theAng is the angular value of the rotation in radians."]
        #[cxx_name = "gp_Pnt_Mirrored"]
        fn Pnt_mirroredax2_3(self_: &Pnt, theA2: &Ax2) -> UniquePtr<Pnt>;
        #[cxx_name = "gp_Pnt_Rotated"]
        fn Pnt_rotated(self_: &Pnt, theA1: &Ax1, theAng: f64) -> UniquePtr<Pnt>;
        #[cxx_name = "gp_Pnt_Scaled"]
        fn Pnt_scaled(self_: &Pnt, theP: &Pnt, theS: f64) -> UniquePtr<Pnt>;
        #[cxx_name = "gp_Pnt_Transformed"]
        fn Pnt_transformed(self_: &Pnt, theT: &Trsf) -> UniquePtr<Pnt>;
        #[cxx_name = "gp_Pnt_Translated"]
        fn Pnt_translatedvec(self_: &Pnt, theV: &Vec_) -> UniquePtr<Pnt>;
        #[cxx_name = "gp_Pnt_Translated"]
        fn Pnt_translatedpnt_2(self_: &Pnt, theP1: &Pnt, theP2: &Pnt) -> UniquePtr<Pnt>;
        #[doc = " ======================== gp_Pnt2d ========================"]
        #[doc = "Defines  a non-persistent 2D cartesian point."]
        #[cxx_name = "gp_Pnt2d"]
        type Pnt2d;
        #[doc = "Creates a point with zero coordinates."]
        #[cxx_name = "gp_Pnt2d_ctor"]
        fn Pnt2d_ctor() -> UniquePtr<Pnt2d>;
        #[doc = "Creates a point with a doublet of coordinates."]
        #[cxx_name = "gp_Pnt2d_ctor_xy"]
        fn Pnt2d_ctor_xy(theCoord: &gp_XY) -> UniquePtr<Pnt2d>;
        #[doc = "Creates a  point with its 2 cartesian's coordinates : theXp, theYp."]
        #[cxx_name = "gp_Pnt2d_ctor_real2"]
        fn Pnt2d_ctor_real2(theXp: f64, theYp: f64) -> UniquePtr<Pnt2d>;
        #[doc = "Assigns the value Xi to the coordinate that corresponds to theIndex: theIndex = 1 => X is modified theIndex = 2 => Y is modified Raises OutOfRange if theIndex != {1, 2}."]
        #[cxx_name = "SetCoord"]
        fn set_coordint(self: Pin<&mut Pnt2d>, theIndex: i32, theXi: f64);
        #[doc = "For this point, assigns the values theXp and theYp to its two coordinates"]
        #[cxx_name = "SetCoord"]
        fn set_coordreal_2(self: Pin<&mut Pnt2d>, theXp: f64, theYp: f64);
        #[doc = "Assigns the given value to the X  coordinate of this point."]
        #[cxx_name = "SetX"]
        fn set_x(self: Pin<&mut Pnt2d>, theX: f64);
        #[doc = "Assigns the given value to the Y  coordinate of this point."]
        #[cxx_name = "SetY"]
        fn set_y(self: Pin<&mut Pnt2d>, theY: f64);
        #[doc = "Assigns the two coordinates of Coord to this point."]
        #[cxx_name = "SetXY"]
        fn set_xy(self: Pin<&mut Pnt2d>, theCoord: &gp_XY);
        #[doc = "Returns the coordinate of range theIndex : theIndex = 1 => X is returned theIndex = 2 => Y is returned Raises OutOfRange if theIndex != {1, 2}."]
        #[cxx_name = "Coord"]
        fn coordint(self: &Pnt2d, theIndex: i32) -> f64;
        #[doc = "For this point returns its two coordinates as a number pair."]
        #[cxx_name = "Coord"]
        fn coordreal_2(self: &Pnt2d, theXp: &mut f64, theYp: &mut f64);
        #[doc = "For this point, returns its X  coordinate."]
        #[cxx_name = "X"]
        fn x(self: &Pnt2d) -> f64;
        #[doc = "For this point, returns its Y coordinate."]
        #[cxx_name = "Y"]
        fn y(self: &Pnt2d) -> f64;
        #[doc = "For this point, returns its two coordinates as a number pair."]
        #[cxx_name = "XY"]
        fn xy(self: &Pnt2d) -> &gp_XY;
        #[doc = "For this point, returns its two coordinates as a number pair."]
        #[cxx_name = "Coord"]
        fn coord3(self: &Pnt2d) -> &gp_XY;
        #[doc = "Returns the coordinates of this point. Note: This syntax allows direct modification of the returned value."]
        #[cxx_name = "ChangeCoord"]
        fn change_coord(self: Pin<&mut Pnt2d>) -> Pin<&mut gp_XY>;
        #[doc = "Comparison Returns True if the distance between the two points is lower or equal to theLinearTolerance."]
        #[cxx_name = "IsEqual"]
        fn is_equal(self: &Pnt2d, theOther: &Pnt2d, theLinearTolerance: f64) -> bool;
        #[doc = "Computes the distance between two points."]
        #[cxx_name = "Distance"]
        fn distance(self: &Pnt2d, theOther: &Pnt2d) -> f64;
        #[doc = "Computes the square distance between two points."]
        #[cxx_name = "SquareDistance"]
        fn square_distance(self: &Pnt2d, theOther: &Pnt2d) -> f64;
        #[doc = "Performs the symmetrical transformation of a point with respect to the point theP which is the center of the  symmetry."]
        #[cxx_name = "Mirror"]
        fn mirrorpnt2d(self: Pin<&mut Pnt2d>, theP: &Pnt2d);
        #[cxx_name = "Mirror"]
        fn mirrorax2d_2(self: Pin<&mut Pnt2d>, theA: &Ax2d);
        #[doc = "Rotates a point. theA1 is the axis of the rotation. Ang is the angular value of the rotation in radians."]
        #[cxx_name = "Rotate"]
        fn rotate(self: Pin<&mut Pnt2d>, theP: &Pnt2d, theAng: f64);
        #[doc = "Scales a point. theS is the scaling value."]
        #[cxx_name = "Scale"]
        fn scale(self: Pin<&mut Pnt2d>, theP: &Pnt2d, theS: f64);
        #[doc = "Transforms a point with the transformation theT."]
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Pnt2d>, theT: &Trsf2d);
        #[doc = "Translates a point in the direction of the vector theV. The magnitude of the translation is the vector's magnitude."]
        #[cxx_name = "Translate"]
        fn translatevec2d(self: Pin<&mut Pnt2d>, theV: &Vec2d);
        #[doc = "Translates a point from the point theP1 to the point theP2."]
        #[cxx_name = "Translate"]
        fn translatepnt2d_2(self: Pin<&mut Pnt2d>, theP1: &Pnt2d, theP2: &Pnt2d);
        #[doc = "Performs the symmetrical transformation of a point with respect to an axis placement which is the axis"]
        #[cxx_name = "gp_Pnt2d_Mirrored"]
        fn Pnt2d_mirroredpnt2d(self_: &Pnt2d, theP: &Pnt2d) -> UniquePtr<Pnt2d>;
        #[cxx_name = "gp_Pnt2d_Mirrored"]
        fn Pnt2d_mirroredax2d_2(self_: &Pnt2d, theA: &Ax2d) -> UniquePtr<Pnt2d>;
        #[cxx_name = "gp_Pnt2d_Rotated"]
        fn Pnt2d_rotated(self_: &Pnt2d, theP: &Pnt2d, theAng: f64) -> UniquePtr<Pnt2d>;
        #[cxx_name = "gp_Pnt2d_Scaled"]
        fn Pnt2d_scaled(self_: &Pnt2d, theP: &Pnt2d, theS: f64) -> UniquePtr<Pnt2d>;
        #[cxx_name = "gp_Pnt2d_Transformed"]
        fn Pnt2d_transformed(self_: &Pnt2d, theT: &Trsf2d) -> UniquePtr<Pnt2d>;
        #[cxx_name = "gp_Pnt2d_Translated"]
        fn Pnt2d_translatedvec2d(self_: &Pnt2d, theV: &Vec2d) -> UniquePtr<Pnt2d>;
        #[cxx_name = "gp_Pnt2d_Translated"]
        fn Pnt2d_translatedpnt2d_2(self_: &Pnt2d, theP1: &Pnt2d, theP2: &Pnt2d)
            -> UniquePtr<Pnt2d>;
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
        fn mirrorax1_2(self: Pin<&mut Vec_>, theA1: &Ax1);
        #[cxx_name = "Mirror"]
        fn mirrorax2_3(self: Pin<&mut Vec_>, theA2: &Ax2);
        #[cxx_name = "Rotate"]
        fn rotate(self: Pin<&mut Vec_>, theA1: &Ax1, theAng: f64);
        #[cxx_name = "Scale"]
        fn scale(self: Pin<&mut Vec_>, theS: f64);
        #[doc = "Transforms a vector with the transformation theT."]
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Vec_>, theT: &Trsf);
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
        fn Vec__mirroredax1_2(self_: &Vec_, theA1: &Ax1) -> UniquePtr<Vec_>;
        #[doc = "Performs the symmetrical transformation of a vector with respect to a plane. The axis placement theA2 locates the plane of the symmetry : (Location, XDirection, YDirection)."]
        #[cxx_name = "gp_Vec_Mirrored"]
        fn Vec__mirroredax2_3(self_: &Vec_, theA2: &Ax2) -> UniquePtr<Vec_>;
        #[doc = "Rotates a vector. theA1 is the axis of the rotation. theAng is the angular value of the rotation in radians."]
        #[cxx_name = "gp_Vec_Rotated"]
        fn Vec__rotated(self_: &Vec_, theA1: &Ax1, theAng: f64) -> UniquePtr<Vec_>;
        #[doc = "Scales a vector. theS is the scaling value."]
        #[cxx_name = "gp_Vec_Scaled"]
        fn Vec__scaled(self_: &Vec_, theS: f64) -> UniquePtr<Vec_>;
        #[doc = "Transforms a vector with the transformation theT."]
        #[cxx_name = "gp_Vec_Transformed"]
        fn Vec__transformed(self_: &Vec_, theT: &Trsf) -> UniquePtr<Vec_>;
        #[doc = " ======================== gp_Vec2d ========================"]
        #[doc = "Defines a non-persistent vector in 2D space."]
        #[cxx_name = "gp_Vec2d"]
        type Vec2d;
        #[doc = "Creates a zero vector."]
        #[cxx_name = "gp_Vec2d_ctor"]
        fn Vec2d_ctor() -> UniquePtr<Vec2d>;
        #[doc = "Creates a unitary vector from a direction theV."]
        #[cxx_name = "gp_Vec2d_ctor_dir2d"]
        fn Vec2d_ctor_dir2d(theV: &Dir2d) -> UniquePtr<Vec2d>;
        #[doc = "Creates a vector with a doublet of coordinates."]
        #[cxx_name = "gp_Vec2d_ctor_xy"]
        fn Vec2d_ctor_xy(theCoord: &gp_XY) -> UniquePtr<Vec2d>;
        #[doc = "Creates a point with its two Cartesian coordinates."]
        #[cxx_name = "gp_Vec2d_ctor_real2"]
        fn Vec2d_ctor_real2(theXv: f64, theYv: f64) -> UniquePtr<Vec2d>;
        #[doc = "Creates a vector from two points. The length of the vector is the distance between theP1 and theP2"]
        #[cxx_name = "gp_Vec2d_ctor_pnt2d2"]
        fn Vec2d_ctor_pnt2d2(theP1: &Pnt2d, theP2: &Pnt2d) -> UniquePtr<Vec2d>;
        #[doc = "Changes the coordinate of range theIndex theIndex = 1 => X is modified theIndex = 2 => Y is modified Raises OutOfRange if theIndex != {1, 2}."]
        #[cxx_name = "SetCoord"]
        fn set_coordint(self: Pin<&mut Vec2d>, theIndex: i32, theXi: f64);
        #[doc = "For this vector, assigns the values theXv and theYv to its two coordinates"]
        #[cxx_name = "SetCoord"]
        fn set_coordreal_2(self: Pin<&mut Vec2d>, theXv: f64, theYv: f64);
        #[doc = "Assigns the given value to the X coordinate of this vector."]
        #[cxx_name = "SetX"]
        fn set_x(self: Pin<&mut Vec2d>, theX: f64);
        #[doc = "Assigns the given value to the Y coordinate of this vector."]
        #[cxx_name = "SetY"]
        fn set_y(self: Pin<&mut Vec2d>, theY: f64);
        #[doc = "Assigns the two coordinates of theCoord to this vector."]
        #[cxx_name = "SetXY"]
        fn set_xy(self: Pin<&mut Vec2d>, theCoord: &gp_XY);
        #[doc = "Returns the coordinate of range theIndex : theIndex = 1 => X is returned theIndex = 2 => Y is returned Raised if theIndex != {1, 2}."]
        #[cxx_name = "Coord"]
        fn coordint(self: &Vec2d, theIndex: i32) -> f64;
        #[doc = "For this vector, returns  its two coordinates theXv and theYv"]
        #[cxx_name = "Coord"]
        fn coordreal_2(self: &Vec2d, theXv: &mut f64, theYv: &mut f64);
        #[doc = "For this vector, returns its X  coordinate."]
        #[cxx_name = "X"]
        fn x(self: &Vec2d) -> f64;
        #[doc = "For this vector, returns its Y  coordinate."]
        #[cxx_name = "Y"]
        fn y(self: &Vec2d) -> f64;
        #[doc = "For this vector, returns its two coordinates as a number pair"]
        #[cxx_name = "XY"]
        fn xy(self: &Vec2d) -> &gp_XY;
        #[doc = "Returns True if the two vectors have the same magnitude value and the same direction. The precision values are theLinearTolerance for the magnitude and theAngularTolerance for the direction."]
        #[cxx_name = "IsEqual"]
        fn is_equal(
            self: &Vec2d,
            theOther: &Vec2d,
            theLinearTolerance: f64,
            theAngularTolerance: f64,
        ) -> bool;
        #[doc = "Returns True if abs(Abs(<me>.Angle(theOther)) - PI/2.) <= theAngularTolerance Raises VectorWithNullMagnitude if <me>.Magnitude() <= Resolution or theOther.Magnitude() <= Resolution from gp."]
        #[cxx_name = "IsNormal"]
        fn is_normal(self: &Vec2d, theOther: &Vec2d, theAngularTolerance: f64) -> bool;
        #[doc = "Returns True if PI - Abs(<me>.Angle(theOther)) <= theAngularTolerance Raises VectorWithNullMagnitude if <me>.Magnitude() <= Resolution or theOther.Magnitude() <= Resolution from gp."]
        #[cxx_name = "IsOpposite"]
        fn is_opposite(self: &Vec2d, theOther: &Vec2d, theAngularTolerance: f64) -> bool;
        #[doc = "Returns true if Abs(Angle(<me>, theOther)) <= theAngularTolerance or PI - Abs(Angle(<me>, theOther)) <= theAngularTolerance Two vectors with opposite directions are considered as parallel. Raises VectorWithNullMagnitude if <me>.Magnitude() <= Resolution or theOther.Magnitude() <= Resolution from gp"]
        #[cxx_name = "IsParallel"]
        fn is_parallel(self: &Vec2d, theOther: &Vec2d, theAngularTolerance: f64) -> bool;
        #[doc = "Computes the angular value between <me> and <theOther> returns the angle value between -PI and PI in radian. The orientation is from <me> to theOther. The positive sense is the trigonometric sense. Raises VectorWithNullMagnitude if <me>.Magnitude() <= Resolution from gp or theOther.Magnitude() <= Resolution because the angular value is indefinite if one of the vectors has a null magnitude."]
        #[cxx_name = "Angle"]
        fn angle(self: &Vec2d, theOther: &Vec2d) -> f64;
        #[doc = "Computes the magnitude of this vector."]
        #[cxx_name = "Magnitude"]
        fn magnitude(self: &Vec2d) -> f64;
        #[doc = "Computes the square magnitude of this vector."]
        #[cxx_name = "SquareMagnitude"]
        fn square_magnitude(self: &Vec2d) -> f64;
        #[cxx_name = "Add"]
        fn add(self: Pin<&mut Vec2d>, theOther: &Vec2d);
        #[doc = "Computes the crossing product between two vectors"]
        #[cxx_name = "Crossed"]
        fn crossed(self: &Vec2d, theRight: &Vec2d) -> f64;
        #[doc = "Computes the magnitude of the cross product between <me> and theRight. Returns || <me> ^ theRight ||"]
        #[cxx_name = "CrossMagnitude"]
        fn cross_magnitude(self: &Vec2d, theRight: &Vec2d) -> f64;
        #[doc = "Computes the square magnitude of the cross product between <me> and theRight. Returns || <me> ^ theRight ||**2"]
        #[cxx_name = "CrossSquareMagnitude"]
        fn cross_square_magnitude(self: &Vec2d, theRight: &Vec2d) -> f64;
        #[cxx_name = "Divide"]
        fn divide(self: Pin<&mut Vec2d>, theScalar: f64);
        #[doc = "Computes the scalar product"]
        #[cxx_name = "Dot"]
        fn dot(self: &Vec2d, theOther: &Vec2d) -> f64;
        #[cxx_name = "Multiply"]
        fn multiply(self: Pin<&mut Vec2d>, theScalar: f64);
        #[cxx_name = "Normalize"]
        fn normalize(self: Pin<&mut Vec2d>);
        #[cxx_name = "Reverse"]
        fn reverse(self: Pin<&mut Vec2d>);
        #[doc = "Subtracts two vectors"]
        #[cxx_name = "Subtract"]
        fn subtract(self: Pin<&mut Vec2d>, theRight: &Vec2d);
        #[doc = "<me> is set to the following linear form : theA1 * theV1 + theA2 * theV2 + theV3"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formreal(
            self: Pin<&mut Vec2d>,
            theA1: f64,
            theV1: &Vec2d,
            theA2: f64,
            theV2: &Vec2d,
            theV3: &Vec2d,
        );
        #[doc = "<me> is set to the following linear form : theA1 * theV1 + theA2 * theV2"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formreal_2(
            self: Pin<&mut Vec2d>,
            theA1: f64,
            theV1: &Vec2d,
            theA2: f64,
            theV2: &Vec2d,
        );
        #[doc = "<me> is set to the following linear form : theA1 * theV1 + theV2"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formreal_3(self: Pin<&mut Vec2d>, theA1: f64, theV1: &Vec2d, theV2: &Vec2d);
        #[doc = "<me> is set to the following linear form : theV1 + theV2"]
        #[cxx_name = "SetLinearForm"]
        fn set_linear_formvec2d_4(self: Pin<&mut Vec2d>, theV1: &Vec2d, theV2: &Vec2d);
        #[doc = "Performs the symmetrical transformation of a vector with respect to the vector theV which is the center of the  symmetry."]
        #[cxx_name = "Mirror"]
        fn mirrorvec2d(self: Pin<&mut Vec2d>, theV: &Vec2d);
        #[doc = "Performs the symmetrical transformation of a vector with respect to an axis placement which is the axis of the symmetry."]
        #[cxx_name = "Mirror"]
        fn mirrorax2d_2(self: Pin<&mut Vec2d>, theA1: &Ax2d);
        #[cxx_name = "Rotate"]
        fn rotate(self: Pin<&mut Vec2d>, theAng: f64);
        #[cxx_name = "Scale"]
        fn scale(self: Pin<&mut Vec2d>, theS: f64);
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Vec2d>, theT: &Trsf2d);
        #[doc = "Adds two vectors"]
        #[cxx_name = "gp_Vec2d_Added"]
        fn Vec2d_added(self_: &Vec2d, theOther: &Vec2d) -> UniquePtr<Vec2d>;
        #[doc = "divides a vector by a scalar"]
        #[cxx_name = "gp_Vec2d_Divided"]
        fn Vec2d_divided(self_: &Vec2d, theScalar: f64) -> UniquePtr<Vec2d>;
        #[cxx_name = "gp_Vec2d_GetNormal"]
        fn Vec2d_get_normal(self_: &Vec2d) -> UniquePtr<Vec2d>;
        #[doc = "Normalizes a vector Raises an exception if the magnitude of the vector is lower or equal to Resolution from package gp."]
        #[cxx_name = "gp_Vec2d_Multiplied"]
        fn Vec2d_multiplied(self_: &Vec2d, theScalar: f64) -> UniquePtr<Vec2d>;
        #[doc = "Normalizes a vector Raises an exception if the magnitude of the vector is lower or equal to Resolution from package gp. Reverses the direction of a vector"]
        #[cxx_name = "gp_Vec2d_Normalized"]
        fn Vec2d_normalized(self_: &Vec2d) -> UniquePtr<Vec2d>;
        #[doc = "Reverses the direction of a vector"]
        #[cxx_name = "gp_Vec2d_Reversed"]
        fn Vec2d_reversed(self_: &Vec2d) -> UniquePtr<Vec2d>;
        #[doc = "Subtracts two vectors"]
        #[cxx_name = "gp_Vec2d_Subtracted"]
        fn Vec2d_subtracted(self_: &Vec2d, theRight: &Vec2d) -> UniquePtr<Vec2d>;
        #[doc = "Performs the symmetrical transformation of a vector with respect to the vector theV which is the center of the  symmetry."]
        #[cxx_name = "gp_Vec2d_Mirrored"]
        fn Vec2d_mirroredvec2d(self_: &Vec2d, theV: &Vec2d) -> UniquePtr<Vec2d>;
        #[doc = "Performs the symmetrical transformation of a vector with respect to an axis placement which is the axis of the symmetry."]
        #[cxx_name = "gp_Vec2d_Mirrored"]
        fn Vec2d_mirroredax2d_2(self_: &Vec2d, theA1: &Ax2d) -> UniquePtr<Vec2d>;
        #[doc = "Rotates a vector. theAng is the angular value of the rotation in radians."]
        #[cxx_name = "gp_Vec2d_Rotated"]
        fn Vec2d_rotated(self_: &Vec2d, theAng: f64) -> UniquePtr<Vec2d>;
        #[doc = "Scales a vector. theS is the scaling value."]
        #[cxx_name = "gp_Vec2d_Scaled"]
        fn Vec2d_scaled(self_: &Vec2d, theS: f64) -> UniquePtr<Vec2d>;
        #[doc = "Transforms a vector with a Trsf from gp."]
        #[cxx_name = "gp_Vec2d_Transformed"]
        fn Vec2d_transformed(self_: &Vec2d, theT: &Trsf2d) -> UniquePtr<Vec2d>;
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
        fn mirrorax1_2(self: Pin<&mut Dir>, theA1: &Ax1);
        #[cxx_name = "Mirror"]
        fn mirrorax2_3(self: Pin<&mut Dir>, theA2: &Ax2);
        #[cxx_name = "Rotate"]
        fn rotate(self: Pin<&mut Dir>, theA1: &Ax1, theAng: f64);
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Dir>, theT: &Trsf);
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
        fn Dir_mirroredax1_2(self_: &Dir, theA1: &Ax1) -> UniquePtr<Dir>;
        #[doc = "Performs the symmetrical transformation of a direction with respect to a plane. The axis placement theA2 locates the plane of the symmetry : (Location, XDirection, YDirection)."]
        #[cxx_name = "gp_Dir_Mirrored"]
        fn Dir_mirroredax2_3(self_: &Dir, theA2: &Ax2) -> UniquePtr<Dir>;
        #[doc = "Rotates a direction. theA1 is the axis of the rotation. theAng is the angular value of the rotation in radians."]
        #[cxx_name = "gp_Dir_Rotated"]
        fn Dir_rotated(self_: &Dir, theA1: &Ax1, theAng: f64) -> UniquePtr<Dir>;
        #[doc = "Transforms a direction with a \"Trsf\" from gp. Warnings : If the scale factor of the \"Trsf\" theT is negative then the direction <me> is reversed."]
        #[cxx_name = "gp_Dir_Transformed"]
        fn Dir_transformed(self_: &Dir, theT: &Trsf) -> UniquePtr<Dir>;
        #[doc = " ======================== gp_Dir2d ========================"]
        #[doc = "Describes a unit vector in the plane (2D space). This unit vector is also called \"Direction\". See Also gce_MakeDir2d which provides functions for more complex unit vector constructions Geom2d_Direction which provides additional functions for constructing unit vectors and works, in particular, with the parametric equations of unit vectors"]
        #[cxx_name = "gp_Dir2d"]
        type Dir2d;
        #[doc = "Creates a direction corresponding to X axis."]
        #[cxx_name = "gp_Dir2d_ctor"]
        fn Dir2d_ctor() -> UniquePtr<Dir2d>;
        #[doc = "Normalizes the vector theV and creates a Direction. Raises ConstructionError if theV.Magnitude() <= Resolution from gp."]
        #[cxx_name = "gp_Dir2d_ctor_vec2d"]
        fn Dir2d_ctor_vec2d(theV: &Vec2d) -> UniquePtr<Dir2d>;
        #[doc = "Creates a Direction from a doublet of coordinates. Raises ConstructionError if theCoord.Modulus() <= Resolution from gp."]
        #[cxx_name = "gp_Dir2d_ctor_xy"]
        fn Dir2d_ctor_xy(theCoord: &gp_XY) -> UniquePtr<Dir2d>;
        #[doc = "Creates a Direction with its 2 cartesian coordinates. Raises ConstructionError if Sqrt(theXv*theXv + theYv*theYv) <= Resolution from gp."]
        #[cxx_name = "gp_Dir2d_ctor_real2"]
        fn Dir2d_ctor_real2(theXv: f64, theYv: f64) -> UniquePtr<Dir2d>;
        #[doc = "For this unit vector, assigns: the value theXi to: -   the X coordinate if theIndex is 1, or -   the Y coordinate if theIndex is 2, and then normalizes it. Warning Remember that all the coordinates of a unit vector are implicitly modified when any single one is changed directly. Exceptions Standard_OutOfRange if theIndex is not 1 or 2. Standard_ConstructionError if either of the following is less than or equal to gp::Resolution(): -   Sqrt(theXv*theXv + theYv*theYv), or -   the modulus of the number pair formed by the new value theXi and the other coordinate of this vector that was not directly modified. Raises OutOfRange if theIndex != {1, 2}."]
        #[cxx_name = "SetCoord"]
        fn set_coordint(self: Pin<&mut Dir2d>, theIndex: i32, theXi: f64);
        #[doc = "For this unit vector, assigns: -   the values theXv and theYv to its two coordinates, Warning Remember that all the coordinates of a unit vector are implicitly modified when any single one is changed directly. Exceptions Standard_OutOfRange if theIndex is not 1 or 2. Standard_ConstructionError if either of the following is less than or equal to gp::Resolution(): -   Sqrt(theXv*theXv + theYv*theYv), or -   the modulus of the number pair formed by the new value Xi and the other coordinate of this vector that was not directly modified. Raises OutOfRange if theIndex != {1, 2}."]
        #[cxx_name = "SetCoord"]
        fn set_coordreal_2(self: Pin<&mut Dir2d>, theXv: f64, theYv: f64);
        #[doc = "Assigns the given value to the X coordinate of this unit   vector, and then normalizes it. Warning Remember that all the coordinates of a unit vector are implicitly modified when any single one is changed directly. Exceptions Standard_ConstructionError if either of the following is less than or equal to gp::Resolution(): -   the modulus of Coord, or -   the modulus of the number pair formed from the new X or Y coordinate and the other coordinate of this vector that was not directly modified."]
        #[cxx_name = "SetX"]
        fn set_x(self: Pin<&mut Dir2d>, theX: f64);
        #[doc = "Assigns  the given value to the Y coordinate of this unit   vector, and then normalizes it. Warning Remember that all the coordinates of a unit vector are implicitly modified when any single one is changed directly. Exceptions Standard_ConstructionError if either of the following is less than or equal to gp::Resolution(): -   the modulus of Coord, or -   the modulus of the number pair formed from the new X or Y coordinate and the other coordinate of this vector that was not directly modified."]
        #[cxx_name = "SetY"]
        fn set_y(self: Pin<&mut Dir2d>, theY: f64);
        #[doc = "Assigns: -   the two coordinates of theCoord to this unit vector, and then normalizes it. Warning Remember that all the coordinates of a unit vector are implicitly modified when any single one is changed directly. Exceptions Standard_ConstructionError if either of the following is less than or equal to gp::Resolution(): -   the modulus of theCoord, or -   the modulus of the number pair formed from the new X or Y coordinate and the other coordinate of this vector that was not directly modified."]
        #[cxx_name = "SetXY"]
        fn set_xy(self: Pin<&mut Dir2d>, theCoord: &gp_XY);
        #[doc = "For this unit vector returns the coordinate of range theIndex : theIndex = 1 => X is returned theIndex = 2 => Y is returned Raises OutOfRange if theIndex != {1, 2}."]
        #[cxx_name = "Coord"]
        fn coordint(self: &Dir2d, theIndex: i32) -> f64;
        #[doc = "For this unit vector returns its two coordinates theXv and theYv. Raises OutOfRange if theIndex != {1, 2}."]
        #[cxx_name = "Coord"]
        fn coordreal_2(self: &Dir2d, theXv: &mut f64, theYv: &mut f64);
        #[doc = "For this unit vector, returns its X coordinate."]
        #[cxx_name = "X"]
        fn x(self: &Dir2d) -> f64;
        #[doc = "For this unit vector, returns its Y coordinate."]
        #[cxx_name = "Y"]
        fn y(self: &Dir2d) -> f64;
        #[doc = "For this unit vector, returns its two coordinates as a number pair. Comparison between Directions The precision value is an input data."]
        #[cxx_name = "XY"]
        fn xy(self: &Dir2d) -> &gp_XY;
        #[doc = "Returns True if the two vectors have the same direction i.e. the angle between this unit vector and the unit vector theOther is less than or equal to theAngularTolerance."]
        #[cxx_name = "IsEqual"]
        fn is_equal(self: &Dir2d, theOther: &Dir2d, theAngularTolerance: f64) -> bool;
        #[doc = "Returns True if the angle between this unit vector and the unit vector theOther is equal to Pi/2 or -Pi/2 (normal) i.e. Abs(Abs(<me>.Angle(theOther)) - PI/2.) <= theAngularTolerance"]
        #[cxx_name = "IsNormal"]
        fn is_normal(self: &Dir2d, theOther: &Dir2d, theAngularTolerance: f64) -> bool;
        #[doc = "Returns True if the angle between this unit vector and the unit vector theOther is equal to Pi or -Pi (opposite). i.e.  PI - Abs(<me>.Angle(theOther)) <= theAngularTolerance"]
        #[cxx_name = "IsOpposite"]
        fn is_opposite(self: &Dir2d, theOther: &Dir2d, theAngularTolerance: f64) -> bool;
        #[doc = "returns true if the angle between this unit vector and unit vector theOther is equal to 0, Pi or -Pi. i.e.  Abs(Angle(<me>, theOther)) <= theAngularTolerance or PI - Abs(Angle(<me>, theOther)) <= theAngularTolerance"]
        #[cxx_name = "IsParallel"]
        fn is_parallel(self: &Dir2d, theOther: &Dir2d, theAngularTolerance: f64) -> bool;
        #[doc = "Computes the angular value in radians between <me> and <theOther>. Returns the angle in the range [-PI, PI]."]
        #[cxx_name = "Angle"]
        fn angle(self: &Dir2d, theOther: &Dir2d) -> f64;
        #[doc = "Computes the cross product between two directions."]
        #[cxx_name = "Crossed"]
        fn crossed(self: &Dir2d, theRight: &Dir2d) -> f64;
        #[doc = "Computes the scalar product"]
        #[cxx_name = "Dot"]
        fn dot(self: &Dir2d, theOther: &Dir2d) -> f64;
        #[cxx_name = "Reverse"]
        fn reverse(self: Pin<&mut Dir2d>);
        #[cxx_name = "Mirror"]
        fn mirrordir2d(self: Pin<&mut Dir2d>, theV: &Dir2d);
        #[cxx_name = "Mirror"]
        fn mirrorax2d_2(self: Pin<&mut Dir2d>, theA: &Ax2d);
        #[cxx_name = "Rotate"]
        fn rotate(self: Pin<&mut Dir2d>, Ang: f64);
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Dir2d>, theT: &Trsf2d);
        #[doc = "Reverses the orientation of a direction"]
        #[cxx_name = "gp_Dir2d_Reversed"]
        fn Dir2d_reversed(self_: &Dir2d) -> UniquePtr<Dir2d>;
        #[doc = "Performs the symmetrical transformation of a direction with respect to the direction theV which is the center of the  symmetry."]
        #[cxx_name = "gp_Dir2d_Mirrored"]
        fn Dir2d_mirroreddir2d(self_: &Dir2d, theV: &Dir2d) -> UniquePtr<Dir2d>;
        #[doc = "Performs the symmetrical transformation of a direction with respect to an axis placement which is the axis of the symmetry."]
        #[cxx_name = "gp_Dir2d_Mirrored"]
        fn Dir2d_mirroredax2d_2(self_: &Dir2d, theA: &Ax2d) -> UniquePtr<Dir2d>;
        #[doc = "Rotates a direction.  theAng is the angular value of the rotation in radians."]
        #[cxx_name = "gp_Dir2d_Rotated"]
        fn Dir2d_rotated(self_: &Dir2d, theAng: f64) -> UniquePtr<Dir2d>;
        #[doc = "Transforms a direction with the \"Trsf\" theT. Warnings : If the scale factor of the \"Trsf\" theT is negative then the direction <me> is reversed."]
        #[cxx_name = "gp_Dir2d_Transformed"]
        fn Dir2d_transformed(self_: &Dir2d, theT: &Trsf2d) -> UniquePtr<Dir2d>;
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
        #[doc = " ======================== gp_Ax1 ========================"]
        #[doc = "Describes an axis in 3D space. An axis is defined by: -   its origin (also referred to as its \"Location point\"), and -   its unit vector (referred to as its \"Direction\" or \"main   Direction\"). An axis is used: -   to describe 3D geometric entities (for example, the axis of a revolution entity). It serves the same purpose as the STEP function \"axis placement one axis\", or -   to define geometric transformations (axis of symmetry, axis of rotation, and so on). For example, this entity can be used to locate a geometric entity or to define a symmetry axis."]
        #[cxx_name = "gp_Ax1"]
        type Ax1;
        #[doc = "Creates an axis object representing Z axis of the reference coordinate system."]
        #[cxx_name = "gp_Ax1_ctor"]
        fn Ax1_ctor() -> UniquePtr<Ax1>;
        #[doc = "P is the location point and V is the direction of <me>."]
        #[cxx_name = "gp_Ax1_ctor_pnt_dir"]
        fn Ax1_ctor_pnt_dir(theP: &Pnt, theV: &Dir) -> UniquePtr<Ax1>;
        #[doc = "Assigns V as the \"Direction\"  of this axis."]
        #[cxx_name = "SetDirection"]
        fn set_direction(self: Pin<&mut Ax1>, theV: &Dir);
        #[doc = "Assigns  P as the origin of this axis."]
        #[cxx_name = "SetLocation"]
        fn set_location(self: Pin<&mut Ax1>, theP: &Pnt);
        #[doc = "Returns the direction of <me>."]
        #[cxx_name = "Direction"]
        fn direction(self: &Ax1) -> &Dir;
        #[doc = "Returns the location point of <me>."]
        #[cxx_name = "Location"]
        fn location(self: &Ax1) -> &Pnt;
        #[doc = "Returns True if  : . the angle between <me> and <Other> is lower or equal to <AngularTolerance> and . the distance between <me>.Location() and <Other> is lower or equal to <LinearTolerance> and . the distance between <Other>.Location() and <me> is lower or equal to LinearTolerance."]
        #[cxx_name = "IsCoaxial"]
        fn is_coaxial(self: &Ax1, Other: &Ax1, AngularTolerance: f64, LinearTolerance: f64)
            -> bool;
        #[doc = "Returns True if the direction of this and another axis are normal to each other. That is, if the angle between the two axes is equal to Pi/2. Note: the tolerance criterion is given by theAngularTolerance."]
        #[cxx_name = "IsNormal"]
        fn is_normal(self: &Ax1, theOther: &Ax1, theAngularTolerance: f64) -> bool;
        #[doc = "Returns True if the direction of this and another axis are parallel with opposite orientation. That is, if the angle between the two axes is equal to Pi. Note: the tolerance criterion is given by theAngularTolerance."]
        #[cxx_name = "IsOpposite"]
        fn is_opposite(self: &Ax1, theOther: &Ax1, theAngularTolerance: f64) -> bool;
        #[doc = "Returns True if the direction of this and another axis are parallel with same orientation or opposite orientation. That is, if the angle between the two axes is equal to 0 or Pi. Note: the tolerance criterion is given by theAngularTolerance."]
        #[cxx_name = "IsParallel"]
        fn is_parallel(self: &Ax1, theOther: &Ax1, theAngularTolerance: f64) -> bool;
        #[doc = "Computes the angular value, in radians, between this.Direction() and theOther.Direction(). Returns the angle between 0 and 2*PI radians."]
        #[cxx_name = "Angle"]
        fn angle(self: &Ax1, theOther: &Ax1) -> f64;
        #[doc = "Reverses the unit vector of this axis and assigns the result to this axis."]
        #[cxx_name = "Reverse"]
        fn reverse(self: Pin<&mut Ax1>);
        #[doc = "Performs the symmetrical transformation of an axis placement with respect to the point P which is the center of the symmetry and assigns the result to this axis."]
        #[cxx_name = "Mirror"]
        fn mirrorpnt(self: Pin<&mut Ax1>, P: &Pnt);
        #[doc = "Performs the symmetrical transformation of an axis placement with respect to an axis placement which is the axis of the symmetry and assigns the result to this axis."]
        #[cxx_name = "Mirror"]
        fn mirrorax1_2(self: Pin<&mut Ax1>, A1: &Ax1);
        #[doc = "Performs the symmetrical transformation of an axis placement with respect to a plane. The axis placement <A2> locates the plane of the symmetry : (Location, XDirection, YDirection) and assigns the result to this axis."]
        #[cxx_name = "Mirror"]
        fn mirrorax2_3(self: Pin<&mut Ax1>, A2: &Ax2);
        #[doc = "Rotates this axis at an angle theAngRad (in radians) about the axis theA1 and assigns the result to this axis."]
        #[cxx_name = "Rotate"]
        fn rotate(self: Pin<&mut Ax1>, theA1: &Ax1, theAngRad: f64);
        #[doc = "Applies a scaling transformation to this axis with: - scale factor theS, and - center theP and assigns the result to this axis."]
        #[cxx_name = "Scale"]
        fn scale(self: Pin<&mut Ax1>, theP: &Pnt, theS: f64);
        #[doc = "Applies the transformation theT to this axis and assigns the result to this axis."]
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Ax1>, theT: &Trsf);
        #[doc = "Translates this axis by the vector theV, and assigns the result to this axis."]
        #[cxx_name = "Translate"]
        fn translatevec(self: Pin<&mut Ax1>, theV: &Vec_);
        #[doc = "Translates this axis by: the vector (theP1, theP2) defined from point theP1 to point theP2. and assigns the result to this axis."]
        #[cxx_name = "Translate"]
        fn translatepnt_2(self: Pin<&mut Ax1>, theP1: &Pnt, theP2: &Pnt);
        #[doc = "Reverses the unit vector of this axis and creates a new one."]
        #[cxx_name = "gp_Ax1_Reversed"]
        fn Ax1_reversed(self_: &Ax1) -> UniquePtr<Ax1>;
        #[doc = "Performs the symmetrical transformation of an axis placement with respect to the point P which is the center of the symmetry and creates a new axis."]
        #[cxx_name = "gp_Ax1_Mirrored"]
        fn Ax1_mirroredpnt(self_: &Ax1, P: &Pnt) -> UniquePtr<Ax1>;
        #[doc = "Performs the symmetrical transformation of an axis placement with respect to an axis placement which is the axis of the symmetry and creates a new axis."]
        #[cxx_name = "gp_Ax1_Mirrored"]
        fn Ax1_mirroredax1_2(self_: &Ax1, A1: &Ax1) -> UniquePtr<Ax1>;
        #[doc = "Performs the symmetrical transformation of an axis placement with respect to a plane. The axis placement <A2> locates the plane of the symmetry : (Location, XDirection, YDirection) and creates a new axis."]
        #[cxx_name = "gp_Ax1_Mirrored"]
        fn Ax1_mirroredax2_3(self_: &Ax1, A2: &Ax2) -> UniquePtr<Ax1>;
        #[doc = "Rotates this axis at an angle theAngRad (in radians) about the axis theA1 and creates a new one."]
        #[cxx_name = "gp_Ax1_Rotated"]
        fn Ax1_rotated(self_: &Ax1, theA1: &Ax1, theAngRad: f64) -> UniquePtr<Ax1>;
        #[doc = "Applies a scaling transformation to this axis with: - scale factor theS, and - center theP and creates a new axis."]
        #[cxx_name = "gp_Ax1_Scaled"]
        fn Ax1_scaled(self_: &Ax1, theP: &Pnt, theS: f64) -> UniquePtr<Ax1>;
        #[doc = "Applies the transformation theT to this axis and creates a new one. Translates an axis plaxement in the direction of the vector <V>. The magnitude of the translation is the vector's magnitude."]
        #[cxx_name = "gp_Ax1_Transformed"]
        fn Ax1_transformed(self_: &Ax1, theT: &Trsf) -> UniquePtr<Ax1>;
        #[doc = "Translates this axis by the vector theV, and creates a new one."]
        #[cxx_name = "gp_Ax1_Translated"]
        fn Ax1_translatedvec(self_: &Ax1, theV: &Vec_) -> UniquePtr<Ax1>;
        #[doc = "Translates this axis by: the vector (theP1, theP2) defined from point theP1 to point theP2. and creates a new one."]
        #[cxx_name = "gp_Ax1_Translated"]
        fn Ax1_translatedpnt_2(self_: &Ax1, theP1: &Pnt, theP2: &Pnt) -> UniquePtr<Ax1>;
        #[doc = " ======================== gp_Ax2 ========================"]
        #[doc = "Describes a right-handed coordinate system in 3D space. A coordinate system is defined by: -   its origin (also referred to as its \"Location point\"), and -   three orthogonal unit vectors, termed respectively the \"X Direction\", the \"Y Direction\" and the \"Direction\" (also referred to as the \"main Direction\"). The \"Direction\" of the coordinate system is called its \"main Direction\" because whenever this unit vector is modified, the \"X Direction\" and the \"Y Direction\" are recomputed. However, when we modify either the \"X Direction\" or the \"Y Direction\", \"Direction\" is not modified. The \"main Direction\" is also the \"Z Direction\". Since an Ax2 coordinate system is right-handed, its \"main Direction\" is always equal to the cross product of its \"X Direction\" and \"Y Direction\". (To define a left-handed coordinate system, use gp_Ax3.) A coordinate system is used: -   to describe geometric entities, in particular to position them. The local coordinate system of a geometric entity serves the same purpose as the STEP function \"axis placement two axes\", or -   to define geometric transformations. Note: we refer to the \"X Axis\", \"Y Axis\" and \"Z Axis\", respectively, as to axes having: - the origin of the coordinate system as their origin, and -   the unit vectors \"X Direction\", \"Y Direction\" and \"main Direction\", respectively, as their unit vectors. The \"Z Axis\" is also the \"main Axis\"."]
        #[cxx_name = "gp_Ax2"]
        type Ax2;
        #[doc = "Creates an object corresponding to the reference coordinate system (OXYZ)."]
        #[cxx_name = "gp_Ax2_ctor"]
        fn Ax2_ctor() -> UniquePtr<Ax2>;
        #[doc = "Creates an axis placement with an origin P such that: -   N is the Direction, and -   the \"X Direction\" is normal to N, in the plane defined by the vectors (N, Vx): \"X Direction\" = (N ^ Vx) ^ N, Exception: raises ConstructionError if N and Vx are parallel (same or opposite orientation)."]
        #[cxx_name = "gp_Ax2_ctor_pnt_dir2"]
        fn Ax2_ctor_pnt_dir2(P: &Pnt, N: &Dir, Vx: &Dir) -> UniquePtr<Ax2>;
        #[doc = "Creates -   a coordinate system with an origin P, where V gives the \"main Direction\" (here, \"X Direction\" and \"Y Direction\" are defined automatically)."]
        #[cxx_name = "gp_Ax2_ctor_pnt_dir"]
        fn Ax2_ctor_pnt_dir(P: &Pnt, V: &Dir) -> UniquePtr<Ax2>;
        #[doc = "Assigns the origin and \"main Direction\" of the axis A1 to this coordinate system, then recomputes its \"X Direction\" and \"Y Direction\". Note: The new \"X Direction\" is computed as follows: new \"X Direction\" = V1 ^(previous \"X Direction\" ^ V) where V is the \"Direction\" of A1. Exceptions Standard_ConstructionError if A1 is parallel to the \"X Direction\" of this coordinate system."]
        #[cxx_name = "SetAxis"]
        fn set_axis(self: Pin<&mut Ax2>, A1: &Ax1);
        #[doc = "Changes the \"main Direction\" of this coordinate system, then recomputes its \"X Direction\" and \"Y Direction\". Note: the new \"X Direction\" is computed as follows: new \"X Direction\" = V ^ (previous \"X Direction\" ^ V) Exceptions Standard_ConstructionError if V is parallel to the \"X Direction\" of this coordinate system."]
        #[cxx_name = "SetDirection"]
        fn set_direction(self: Pin<&mut Ax2>, V: &Dir);
        #[doc = "Changes the \"Location\" point (origin) of <me>."]
        #[cxx_name = "SetLocation"]
        fn set_location(self: Pin<&mut Ax2>, theP: &Pnt);
        #[doc = "Changes the \"Xdirection\" of <me>. The main direction \"Direction\" is not modified, the \"Ydirection\" is modified. If <Vx> is not normal to the main direction then <XDirection> is computed as follows XDirection = Direction ^ (Vx ^ Direction). Exceptions Standard_ConstructionError if Vx or Vy is parallel to the \"main Direction\" of this coordinate system."]
        #[cxx_name = "SetXDirection"]
        fn set_x_direction(self: Pin<&mut Ax2>, theVx: &Dir);
        #[doc = "Changes the \"Ydirection\" of <me>. The main direction is not modified but the \"Xdirection\" is changed. If <Vy> is not normal to the main direction then \"YDirection\" is computed as  follows YDirection = Direction ^ (<Vy> ^ Direction). Exceptions Standard_ConstructionError if Vx or Vy is parallel to the \"main Direction\" of this coordinate system."]
        #[cxx_name = "SetYDirection"]
        fn set_y_direction(self: Pin<&mut Ax2>, theVy: &Dir);
        #[doc = "Computes the angular value, in radians, between the main direction of <me> and the main direction of <theOther>. Returns the angle between 0 and PI in radians."]
        #[cxx_name = "Angle"]
        fn angle(self: &Ax2, theOther: &Ax2) -> f64;
        #[doc = "Returns the main axis of <me>. It is the \"Location\" point and the main \"Direction\"."]
        #[cxx_name = "Axis"]
        fn axis(self: &Ax2) -> &Ax1;
        #[doc = "Returns the main direction of <me>."]
        #[cxx_name = "Direction"]
        fn direction(self: &Ax2) -> &Dir;
        #[doc = "Returns the \"Location\" point (origin) of <me>."]
        #[cxx_name = "Location"]
        fn location(self: &Ax2) -> &Pnt;
        #[doc = "Returns the \"XDirection\" of <me>."]
        #[cxx_name = "XDirection"]
        fn x_direction(self: &Ax2) -> &Dir;
        #[doc = "Returns the \"YDirection\" of <me>."]
        #[cxx_name = "YDirection"]
        fn y_direction(self: &Ax2) -> &Dir;
        #[cxx_name = "IsCoplanar"]
        fn is_coplanarax2(
            self: &Ax2,
            Other: &Ax2,
            LinearTolerance: f64,
            AngularTolerance: f64,
        ) -> bool;
        #[doc = "Returns True if . the distance between <me> and the \"Location\" point of A1 is lower of equal to LinearTolerance and . the main direction of <me> and the direction of A1 are normal. Note: the tolerance criterion for angular equality is given by AngularTolerance."]
        #[cxx_name = "IsCoplanar"]
        fn is_coplanarax1_2(
            self: &Ax2,
            A1: &Ax1,
            LinearTolerance: f64,
            AngularTolerance: f64,
        ) -> bool;
        #[doc = "Performs a symmetrical transformation of this coordinate system with respect to: -   the point P, and assigns the result to this coordinate system. Warning This transformation is always performed on the origin. In case of a reflection with respect to a point: - the main direction of the coordinate system is not changed, and - the \"X Direction\" and the \"Y Direction\" are simply reversed In case of a reflection with respect to an axis or a plane: -   the transformation is applied to the \"X Direction\" and the \"Y Direction\", then -   the \"main Direction\" is recomputed as the cross product \"X Direction\" ^ \"Y   Direction\". This maintains the right-handed property of the coordinate system."]
        #[cxx_name = "Mirror"]
        fn mirrorpnt(self: Pin<&mut Ax2>, P: &Pnt);
        #[doc = "Performs a symmetrical transformation of this coordinate system with respect to: -   the axis A1, and assigns the result to this coordinate systeme. Warning This transformation is always performed on the origin. In case of a reflection with respect to a point: - the main direction of the coordinate system is not changed, and - the \"X Direction\" and the \"Y Direction\" are simply reversed In case of a reflection with respect to an axis or a plane: -   the transformation is applied to the \"X Direction\" and the \"Y Direction\", then -   the \"main Direction\" is recomputed as the cross product \"X Direction\" ^ \"Y   Direction\". This maintains the right-handed property of the coordinate system."]
        #[cxx_name = "Mirror"]
        fn mirrorax1_2(self: Pin<&mut Ax2>, A1: &Ax1);
        #[doc = "Performs a symmetrical transformation of this coordinate system with respect to: -   the plane defined by the origin, \"X Direction\" and \"Y Direction\" of coordinate system A2 and  assigns the result to this coordinate systeme. Warning This transformation is always performed on the origin. In case of a reflection with respect to a point: - the main direction of the coordinate system is not changed, and - the \"X Direction\" and the \"Y Direction\" are simply reversed In case of a reflection with respect to an axis or a plane: -   the transformation is applied to the \"X Direction\" and the \"Y Direction\", then -   the \"main Direction\" is recomputed as the cross product \"X Direction\" ^ \"Y   Direction\". This maintains the right-handed property of the coordinate system."]
        #[cxx_name = "Mirror"]
        fn mirrorax2_3(self: Pin<&mut Ax2>, A2: &Ax2);
        #[cxx_name = "Rotate"]
        fn rotate(self: Pin<&mut Ax2>, theA1: &Ax1, theAng: f64);
        #[cxx_name = "Scale"]
        fn scale(self: Pin<&mut Ax2>, theP: &Pnt, theS: f64);
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Ax2>, theT: &Trsf);
        #[cxx_name = "Translate"]
        fn translatevec(self: Pin<&mut Ax2>, theV: &Vec_);
        #[cxx_name = "Translate"]
        fn translatepnt_2(self: Pin<&mut Ax2>, theP1: &Pnt, theP2: &Pnt);
        #[doc = "Performs a symmetrical transformation of this coordinate system with respect to: -   the point P, and creates a new one. Warning This transformation is always performed on the origin. In case of a reflection with respect to a point: - the main direction of the coordinate system is not changed, and - the \"X Direction\" and the \"Y Direction\" are simply reversed In case of a reflection with respect to an axis or a plane: -   the transformation is applied to the \"X Direction\" and the \"Y Direction\", then -   the \"main Direction\" is recomputed as the cross product \"X Direction\" ^ \"Y   Direction\". This maintains the right-handed property of the coordinate system."]
        #[cxx_name = "gp_Ax2_Mirrored"]
        fn Ax2_mirroredpnt(self_: &Ax2, P: &Pnt) -> UniquePtr<Ax2>;
        #[doc = "Performs a symmetrical transformation of this coordinate system with respect to: -   the axis A1, and  creates a new one. Warning This transformation is always performed on the origin. In case of a reflection with respect to a point: - the main direction of the coordinate system is not changed, and - the \"X Direction\" and the \"Y Direction\" are simply reversed In case of a reflection with respect to an axis or a plane: -   the transformation is applied to the \"X Direction\" and the \"Y Direction\", then -   the \"main Direction\" is recomputed as the cross product \"X Direction\" ^ \"Y   Direction\". This maintains the right-handed property of the coordinate system."]
        #[cxx_name = "gp_Ax2_Mirrored"]
        fn Ax2_mirroredax1_2(self_: &Ax2, A1: &Ax1) -> UniquePtr<Ax2>;
        #[doc = "Performs a symmetrical transformation of this coordinate system with respect to: -   the plane defined by the origin, \"X Direction\" and \"Y Direction\" of coordinate system A2 and creates a new one. Warning This transformation is always performed on the origin. In case of a reflection with respect to a point: - the main direction of the coordinate system is not changed, and - the \"X Direction\" and the \"Y Direction\" are simply reversed In case of a reflection with respect to an axis or a plane: -   the transformation is applied to the \"X Direction\" and the \"Y Direction\", then -   the \"main Direction\" is recomputed as the cross product \"X Direction\" ^ \"Y   Direction\". This maintains the right-handed property of the coordinate system."]
        #[cxx_name = "gp_Ax2_Mirrored"]
        fn Ax2_mirroredax2_3(self_: &Ax2, A2: &Ax2) -> UniquePtr<Ax2>;
        #[doc = "Rotates an axis placement. <theA1> is the axis of the rotation. theAng is the angular value of the rotation in radians."]
        #[cxx_name = "gp_Ax2_Rotated"]
        fn Ax2_rotated(self_: &Ax2, theA1: &Ax1, theAng: f64) -> UniquePtr<Ax2>;
        #[doc = "Applies a scaling transformation on the axis placement. The \"Location\" point of the axisplacement is modified. Warnings : If the scale <S> is negative : . the main direction of the axis placement is not changed. . The \"XDirection\" and the \"YDirection\" are reversed. So the axis placement stay right handed."]
        #[cxx_name = "gp_Ax2_Scaled"]
        fn Ax2_scaled(self_: &Ax2, theP: &Pnt, theS: f64) -> UniquePtr<Ax2>;
        #[doc = "Transforms an axis placement with a Trsf. The \"Location\" point, the \"XDirection\" and the \"YDirection\" are transformed with theT. The resulting main \"Direction\" of <me> is the cross product between the \"XDirection\" and the \"YDirection\" after transformation."]
        #[cxx_name = "gp_Ax2_Transformed"]
        fn Ax2_transformed(self_: &Ax2, theT: &Trsf) -> UniquePtr<Ax2>;
        #[doc = "Translates an axis plaxement in the direction of the vector <theV>. The magnitude of the translation is the vector's magnitude."]
        #[cxx_name = "gp_Ax2_Translated"]
        fn Ax2_translatedvec(self_: &Ax2, theV: &Vec_) -> UniquePtr<Ax2>;
        #[doc = "Translates an axis placement from the point <theP1> to the point <theP2>."]
        #[cxx_name = "gp_Ax2_Translated"]
        fn Ax2_translatedpnt_2(self_: &Ax2, theP1: &Pnt, theP2: &Pnt) -> UniquePtr<Ax2>;
        #[doc = " ======================== gp_Ax2d ========================"]
        #[doc = "Describes an axis in the plane (2D space). An axis is defined by: -   its origin (also referred to as its \"Location point\"),   and -   its unit vector (referred to as its \"Direction\"). An axis implicitly defines a direct, right-handed coordinate system in 2D space by: -   its origin, - its \"Direction\" (giving the \"X Direction\" of the coordinate system), and -   the unit vector normal to \"Direction\" (positive angle measured in the trigonometric sense). An axis is used: -   to describe 2D geometric entities (for example, the axis which defines angular coordinates on a circle). It serves for the same purpose as the STEP function \"axis placement one axis\", or -   to define geometric transformations (axis of symmetry, axis of rotation, and so on). Note: to define a left-handed 2D coordinate system, use gp_Ax22d."]
        #[cxx_name = "gp_Ax2d"]
        type Ax2d;
        #[doc = "Creates an axis object representing X axis of the reference co-ordinate system."]
        #[cxx_name = "gp_Ax2d_ctor"]
        fn Ax2d_ctor() -> UniquePtr<Ax2d>;
        #[doc = "Creates an Ax2d. <theP> is the \"Location\" point of the axis placement and theV is the \"Direction\" of the axis placement."]
        #[cxx_name = "gp_Ax2d_ctor_pnt2d_dir2d"]
        fn Ax2d_ctor_pnt2d_dir2d(theP: &Pnt2d, theV: &Dir2d) -> UniquePtr<Ax2d>;
        #[doc = "Changes the \"Location\" point (origin) of <me>."]
        #[cxx_name = "SetLocation"]
        fn set_location(self: Pin<&mut Ax2d>, theP: &Pnt2d);
        #[doc = "Changes the direction of <me>."]
        #[cxx_name = "SetDirection"]
        fn set_direction(self: Pin<&mut Ax2d>, theV: &Dir2d);
        #[doc = "Returns the origin of <me>."]
        #[cxx_name = "Location"]
        fn location(self: &Ax2d) -> &Pnt2d;
        #[doc = "Returns the direction of <me>."]
        #[cxx_name = "Direction"]
        fn direction(self: &Ax2d) -> &Dir2d;
        #[doc = "Returns True if  : . the angle between <me> and <Other> is lower or equal to <AngularTolerance> and . the distance between <me>.Location() and <Other> is lower or equal to <LinearTolerance> and . the distance between <Other>.Location() and <me> is lower or equal to LinearTolerance."]
        #[cxx_name = "IsCoaxial"]
        fn is_coaxial(
            self: &Ax2d,
            Other: &Ax2d,
            AngularTolerance: f64,
            LinearTolerance: f64,
        ) -> bool;
        #[doc = "Returns true if this axis and the axis theOther are normal to each other. That is, if the angle between the two axes is equal to Pi/2 or -Pi/2. Note: the tolerance criterion is given by theAngularTolerance."]
        #[cxx_name = "IsNormal"]
        fn is_normal(self: &Ax2d, theOther: &Ax2d, theAngularTolerance: f64) -> bool;
        #[doc = "Returns true if this axis and the axis theOther are parallel, and have opposite orientations. That is, if the angle between the two axes is equal to Pi or -Pi. Note: the tolerance criterion is given by theAngularTolerance."]
        #[cxx_name = "IsOpposite"]
        fn is_opposite(self: &Ax2d, theOther: &Ax2d, theAngularTolerance: f64) -> bool;
        #[doc = "Returns true if this axis and the axis theOther are parallel, and have either the same or opposite orientations. That is, if the angle between the two axes is equal to 0, Pi or -Pi. Note: the tolerance criterion is given by theAngularTolerance."]
        #[cxx_name = "IsParallel"]
        fn is_parallel(self: &Ax2d, theOther: &Ax2d, theAngularTolerance: f64) -> bool;
        #[doc = "Computes the angle, in radians, between this axis and the axis theOther. The value of the angle is between -Pi and Pi."]
        #[cxx_name = "Angle"]
        fn angle(self: &Ax2d, theOther: &Ax2d) -> f64;
        #[doc = "Reverses the direction of <me> and assigns the result to this axis."]
        #[cxx_name = "Reverse"]
        fn reverse(self: Pin<&mut Ax2d>);
        #[cxx_name = "Mirror"]
        fn mirrorpnt2d(self: Pin<&mut Ax2d>, P: &Pnt2d);
        #[cxx_name = "Mirror"]
        fn mirrorax2d_2(self: Pin<&mut Ax2d>, A: &Ax2d);
        #[cxx_name = "Rotate"]
        fn rotate(self: Pin<&mut Ax2d>, theP: &Pnt2d, theAng: f64);
        #[cxx_name = "Scale"]
        fn scale(self: Pin<&mut Ax2d>, P: &Pnt2d, S: f64);
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Ax2d>, theT: &Trsf2d);
        #[cxx_name = "Translate"]
        fn translatevec2d(self: Pin<&mut Ax2d>, theV: &Vec2d);
        #[cxx_name = "Translate"]
        fn translatepnt2d_2(self: Pin<&mut Ax2d>, theP1: &Pnt2d, theP2: &Pnt2d);
        #[doc = "Computes a new axis placement with a direction opposite to the direction of <me>."]
        #[cxx_name = "gp_Ax2d_Reversed"]
        fn Ax2d_reversed(self_: &Ax2d) -> UniquePtr<Ax2d>;
        #[doc = "Performs the symmetrical transformation of an axis placement with respect to the point P which is the center of the symmetry."]
        #[cxx_name = "gp_Ax2d_Mirrored"]
        fn Ax2d_mirroredpnt2d(self_: &Ax2d, P: &Pnt2d) -> UniquePtr<Ax2d>;
        #[doc = "Performs the symmetrical transformation of an axis placement with respect to an axis placement which is the axis of the symmetry."]
        #[cxx_name = "gp_Ax2d_Mirrored"]
        fn Ax2d_mirroredax2d_2(self_: &Ax2d, A: &Ax2d) -> UniquePtr<Ax2d>;
        #[doc = "Rotates an axis placement. <theP> is the center of the rotation. theAng is the angular value of the rotation in radians."]
        #[cxx_name = "gp_Ax2d_Rotated"]
        fn Ax2d_rotated(self_: &Ax2d, theP: &Pnt2d, theAng: f64) -> UniquePtr<Ax2d>;
        #[doc = "Applies a scaling transformation on the axis placement. The \"Location\" point of the axisplacement is modified. The \"Direction\" is reversed if the scale is negative."]
        #[cxx_name = "gp_Ax2d_Scaled"]
        fn Ax2d_scaled(self_: &Ax2d, theP: &Pnt2d, theS: f64) -> UniquePtr<Ax2d>;
        #[doc = "Transforms an axis placement with a Trsf."]
        #[cxx_name = "gp_Ax2d_Transformed"]
        fn Ax2d_transformed(self_: &Ax2d, theT: &Trsf2d) -> UniquePtr<Ax2d>;
        #[doc = "Translates an axis placement in the direction of the vector theV. The magnitude of the translation is the vector's magnitude."]
        #[cxx_name = "gp_Ax2d_Translated"]
        fn Ax2d_translatedvec2d(self_: &Ax2d, theV: &Vec2d) -> UniquePtr<Ax2d>;
        #[doc = "Translates an axis placement from the point theP1 to the point theP2."]
        #[cxx_name = "gp_Ax2d_Translated"]
        fn Ax2d_translatedpnt2d_2(self_: &Ax2d, theP1: &Pnt2d, theP2: &Pnt2d) -> UniquePtr<Ax2d>;
        #[doc = " ======================== gp_Ax3 ========================"]
        #[doc = "Describes a coordinate system in 3D space. Unlike a gp_Ax2 coordinate system, a gp_Ax3 can be right-handed (\"direct sense\") or left-handed (\"indirect sense\"). A coordinate system is defined by: -   its origin (also referred to as its \"Location point\"), and -   three orthogonal unit vectors, termed the \"X Direction\", the \"Y Direction\" and the \"Direction\" (also referred to as the \"main Direction\"). The \"Direction\" of the coordinate system is called its \"main Direction\" because whenever this unit vector is modified, the \"X Direction\" and the \"Y Direction\" are recomputed. However, when we modify either the \"X Direction\" or the \"Y Direction\", \"Direction\" is not modified. \"Direction\" is also the \"Z Direction\". The \"main Direction\" is always parallel to the cross product of its \"X Direction\" and \"Y Direction\". If the coordinate system is right-handed, it satisfies the equation: \"main Direction\" = \"X Direction\" ^ \"Y Direction\" and if it is left-handed, it satisfies the equation: \"main Direction\" = -\"X Direction\" ^ \"Y Direction\" A coordinate system is used: -   to describe geometric entities, in particular to position them. The local coordinate system of a geometric entity serves the same purpose as the STEP function \"axis placement three axes\", or -   to define geometric transformations. Note: -   We refer to the \"X Axis\", \"Y Axis\" and \"Z Axis\", respectively, as the axes having: -   the origin of the coordinate system as their origin, and -   the unit vectors \"X Direction\", \"Y Direction\" and \"main Direction\", respectively, as their unit vectors. -   The \"Z Axis\" is also the \"main Axis\". -   gp_Ax2 is used to define a coordinate system that must be always right-handed."]
        #[cxx_name = "gp_Ax3"]
        type Ax3;
        #[doc = "Creates an object corresponding to the reference coordinate system (OXYZ)."]
        #[cxx_name = "gp_Ax3_ctor"]
        fn Ax3_ctor() -> UniquePtr<Ax3>;
        #[doc = "Creates  a  coordinate  system from a right-handed coordinate system."]
        #[cxx_name = "gp_Ax3_ctor_ax2"]
        fn Ax3_ctor_ax2(theA: &Ax2) -> UniquePtr<Ax3>;
        #[doc = "Creates a  right handed axis placement with the \"Location\" point theP and  two directions, theN gives the \"Direction\" and theVx gives the \"XDirection\". Raises ConstructionError if theN and theVx are parallel (same or opposite orientation)."]
        #[cxx_name = "gp_Ax3_ctor_pnt_dir2"]
        fn Ax3_ctor_pnt_dir2(theP: &Pnt, theN: &Dir, theVx: &Dir) -> UniquePtr<Ax3>;
        #[doc = "Creates an axis placement with the \"Location\" point <theP> and the normal direction <theV>."]
        #[cxx_name = "gp_Ax3_ctor_pnt_dir"]
        fn Ax3_ctor_pnt_dir(theP: &Pnt, theV: &Dir) -> UniquePtr<Ax3>;
        #[doc = "Reverses the X direction of <me>."]
        #[cxx_name = "XReverse"]
        fn x_reverse(self: Pin<&mut Ax3>);
        #[doc = "Reverses the Y direction of <me>."]
        #[cxx_name = "YReverse"]
        fn y_reverse(self: Pin<&mut Ax3>);
        #[doc = "Reverses the Z direction of <me>."]
        #[cxx_name = "ZReverse"]
        fn z_reverse(self: Pin<&mut Ax3>);
        #[doc = "Assigns the origin and \"main Direction\" of the axis theA1 to this coordinate system, then recomputes its \"X Direction\" and \"Y Direction\". Note: -   The new \"X Direction\" is computed as follows: new \"X Direction\" = V1 ^(previous \"X Direction\" ^ V) where V is the \"Direction\" of theA1. -   The orientation of this coordinate system (right-handed or left-handed) is not modified. Raises ConstructionError  if the \"Direction\" of <theA1> and the \"XDirection\" of <me> are parallel (same or opposite orientation) because it is impossible to calculate the new \"XDirection\" and the new \"YDirection\"."]
        #[cxx_name = "SetAxis"]
        fn set_axis(self: Pin<&mut Ax3>, theA1: &Ax1);
        #[doc = "Changes the main direction of this coordinate system, then recomputes its \"X Direction\" and \"Y Direction\". Note: -   The new \"X Direction\" is computed as follows: new \"X Direction\" = theV ^ (previous \"X Direction\" ^ theV). -   The orientation of this coordinate system (left- or right-handed) is not modified. Raises ConstructionError if <theV> and the previous \"XDirection\" are parallel because it is impossible to calculate the new \"XDirection\" and the new \"YDirection\"."]
        #[cxx_name = "SetDirection"]
        fn set_direction(self: Pin<&mut Ax3>, theV: &Dir);
        #[doc = "Changes the \"Location\" point (origin) of <me>."]
        #[cxx_name = "SetLocation"]
        fn set_location(self: Pin<&mut Ax3>, theP: &Pnt);
        #[doc = "Changes the \"Xdirection\" of <me>. The main direction \"Direction\" is not modified, the \"Ydirection\" is modified. If <theVx> is not normal to the main direction then <XDirection> is computed as follows XDirection = Direction ^ (theVx ^ Direction). Raises ConstructionError if <theVx> is parallel (same or opposite orientation) to the main direction of <me>"]
        #[cxx_name = "SetXDirection"]
        fn set_x_direction(self: Pin<&mut Ax3>, theVx: &Dir);
        #[doc = "Changes the \"Ydirection\" of <me>. The main direction is not modified but the \"Xdirection\" is changed. If <theVy> is not normal to the main direction then \"YDirection\" is computed as  follows YDirection = Direction ^ (<theVy> ^ Direction). Raises ConstructionError if <theVy> is parallel to the main direction of <me>"]
        #[cxx_name = "SetYDirection"]
        fn set_y_direction(self: Pin<&mut Ax3>, theVy: &Dir);
        #[doc = "Computes the angular value between the main direction of <me> and the main direction of <theOther>. Returns the angle between 0 and PI in radians."]
        #[cxx_name = "Angle"]
        fn angle(self: &Ax3, theOther: &Ax3) -> f64;
        #[doc = "Returns the main axis of <me>. It is the \"Location\" point and the main \"Direction\"."]
        #[cxx_name = "Axis"]
        fn axis(self: &Ax3) -> &Ax1;
        #[doc = "Returns the main direction of <me>."]
        #[cxx_name = "Direction"]
        fn direction(self: &Ax3) -> &Dir;
        #[doc = "Returns the \"Location\" point (origin) of <me>."]
        #[cxx_name = "Location"]
        fn location(self: &Ax3) -> &Pnt;
        #[doc = "Returns the \"XDirection\" of <me>."]
        #[cxx_name = "XDirection"]
        fn x_direction(self: &Ax3) -> &Dir;
        #[doc = "Returns the \"YDirection\" of <me>."]
        #[cxx_name = "YDirection"]
        fn y_direction(self: &Ax3) -> &Dir;
        #[doc = "Returns  True if  the  coordinate  system is right-handed. i.e. XDirection().Crossed(YDirection()).Dot(Direction()) > 0"]
        #[cxx_name = "Direct"]
        fn direct(self: &Ax3) -> bool;
        #[doc = "Returns True if . the distance between the \"Location\" point of <me> and <theOther> is lower or equal to theLinearTolerance and . the distance between the \"Location\" point of <theOther> and <me> is lower or equal to theLinearTolerance and . the main direction of <me> and the main direction of <theOther> are parallel (same or opposite orientation)."]
        #[cxx_name = "IsCoplanar"]
        fn is_coplanarax3(
            self: &Ax3,
            theOther: &Ax3,
            theLinearTolerance: f64,
            theAngularTolerance: f64,
        ) -> bool;
        #[doc = "Returns True if . the distance between <me> and the \"Location\" point of theA1 is lower of equal to theLinearTolerance and . the distance between theA1 and the \"Location\" point of <me> is lower or equal to theLinearTolerance and . the main direction of <me> and the direction of theA1 are normal."]
        #[cxx_name = "IsCoplanar"]
        fn is_coplanarax1_2(
            self: &Ax3,
            theA1: &Ax1,
            theLinearTolerance: f64,
            theAngularTolerance: f64,
        ) -> bool;
        #[cxx_name = "Mirror"]
        fn mirrorpnt(self: Pin<&mut Ax3>, theP: &Pnt);
        #[cxx_name = "Mirror"]
        fn mirrorax1_2(self: Pin<&mut Ax3>, theA1: &Ax1);
        #[cxx_name = "Mirror"]
        fn mirrorax2_3(self: Pin<&mut Ax3>, theA2: &Ax2);
        #[cxx_name = "Rotate"]
        fn rotate(self: Pin<&mut Ax3>, theA1: &Ax1, theAng: f64);
        #[cxx_name = "Scale"]
        fn scale(self: Pin<&mut Ax3>, theP: &Pnt, theS: f64);
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Ax3>, theT: &Trsf);
        #[cxx_name = "Translate"]
        fn translatevec(self: Pin<&mut Ax3>, theV: &Vec_);
        #[cxx_name = "Translate"]
        fn translatepnt_2(self: Pin<&mut Ax3>, theP1: &Pnt, theP2: &Pnt);
        #[doc = "Computes a right-handed coordinate system with the same \"X Direction\" and \"Y Direction\" as those of this coordinate system, then recomputes the \"main Direction\". If this coordinate system is right-handed, the result returned is the same coordinate system. If this coordinate system is left-handed, the result is reversed."]
        #[cxx_name = "gp_Ax3_Ax2"]
        fn Ax3_ax2(self_: &Ax3) -> UniquePtr<Ax2>;
        #[doc = "Performs the symmetrical transformation of an axis placement with respect to the point theP which is the center of the symmetry. Warnings : The main direction of the axis placement is not changed. The \"XDirection\" and the \"YDirection\" are reversed. So the axis placement stay right handed."]
        #[cxx_name = "gp_Ax3_Mirrored"]
        fn Ax3_mirroredpnt(self_: &Ax3, theP: &Pnt) -> UniquePtr<Ax3>;
        #[doc = "Performs the symmetrical transformation of an axis placement with respect to an axis placement which is the axis of the symmetry. The transformation is performed on the \"Location\" point, on the \"XDirection\" and \"YDirection\". The resulting main \"Direction\" is the cross product between the \"XDirection\" and the \"YDirection\" after transformation."]
        #[cxx_name = "gp_Ax3_Mirrored"]
        fn Ax3_mirroredax1_2(self_: &Ax3, theA1: &Ax1) -> UniquePtr<Ax3>;
        #[doc = "Performs the symmetrical transformation of an axis placement with respect to a plane. The axis placement  <theA2> locates the plane of the symmetry : (Location, XDirection, YDirection). The transformation is performed on the \"Location\" point, on the \"XDirection\" and \"YDirection\". The resulting main \"Direction\" is the cross product between the \"XDirection\" and the \"YDirection\" after transformation."]
        #[cxx_name = "gp_Ax3_Mirrored"]
        fn Ax3_mirroredax2_3(self_: &Ax3, theA2: &Ax2) -> UniquePtr<Ax3>;
        #[doc = "Rotates an axis placement. <theA1> is the axis of the rotation . theAng is the angular value of the rotation in radians."]
        #[cxx_name = "gp_Ax3_Rotated"]
        fn Ax3_rotated(self_: &Ax3, theA1: &Ax1, theAng: f64) -> UniquePtr<Ax3>;
        #[doc = "Applies a scaling transformation on the axis placement. The \"Location\" point of the axisplacement is modified. Warnings : If the scale <theS> is negative : . the main direction of the axis placement is not changed. . The \"XDirection\" and the \"YDirection\" are reversed. So the axis placement stay right handed."]
        #[cxx_name = "gp_Ax3_Scaled"]
        fn Ax3_scaled(self_: &Ax3, theP: &Pnt, theS: f64) -> UniquePtr<Ax3>;
        #[doc = "Transforms an axis placement with a Trsf. The \"Location\" point, the \"XDirection\" and the \"YDirection\" are transformed with theT.  The resulting main \"Direction\" of <me> is the cross product between the \"XDirection\" and the \"YDirection\" after transformation."]
        #[cxx_name = "gp_Ax3_Transformed"]
        fn Ax3_transformed(self_: &Ax3, theT: &Trsf) -> UniquePtr<Ax3>;
        #[doc = "Translates an axis plaxement in the direction of the vector <theV>. The magnitude of the translation is the vector's magnitude."]
        #[cxx_name = "gp_Ax3_Translated"]
        fn Ax3_translatedvec(self_: &Ax3, theV: &Vec_) -> UniquePtr<Ax3>;
        #[doc = "Translates an axis placement from the point <theP1> to the point <theP2>."]
        #[cxx_name = "gp_Ax3_Translated"]
        fn Ax3_translatedpnt_2(self_: &Ax3, theP1: &Pnt, theP2: &Pnt) -> UniquePtr<Ax3>;
        #[doc = " ======================== gp_Trsf ========================"]
        #[doc = "Defines a non-persistent transformation in 3D space. The following transformations are implemented : . Translation, Rotation, Scale . Symmetry with respect to a point, a line, a plane. Complex transformations can be obtained by combining the previous elementary transformations using the method Multiply. The transformations can be represented as follow : @code V1   V2   V3    T       XYZ        XYZ | a11  a12  a13   a14 |   | x |      | x'| | a21  a22  a23   a24 |   | y |      | y'| | a31  a32  a33   a34 |   | z |   =  | z'| |  0    0    0     1  |   | 1 |      | 1 | @endcode where {V1, V2, V3} defines the vectorial part of the transformation and T defines the translation part of the transformation. This transformation never change the nature of the objects."]
        #[cxx_name = "gp_Trsf"]
        type Trsf;
        #[doc = "Returns the identity transformation."]
        #[cxx_name = "gp_Trsf_ctor"]
        fn Trsf_ctor() -> UniquePtr<Trsf>;
        #[doc = "Creates  a 3D transformation from the 2D transformation theT. The resulting transformation has a homogeneous vectorial part, V3, and a translation part, T3, built from theT: a11    a12 0             a13 V3 =    a21    a22    0       T3 =   a23 0    0    1. 0 It also has the same scale factor as theT. This guarantees (by projection) that the transformation which would be performed by theT in a plane (2D space) is performed by the resulting transformation in the xOy plane of the 3D space, (i.e. in the plane defined by the origin (0., 0., 0.) and the vectors DX (1., 0., 0.), and DY (0., 1., 0.)). The scale factor is applied to the entire space."]
        #[cxx_name = "gp_Trsf_ctor_trsf2d"]
        fn Trsf_ctor_trsf2d(theT: &Trsf2d) -> UniquePtr<Trsf>;
        #[doc = "Makes the transformation into a symmetrical transformation. theP is the center of the symmetry."]
        #[cxx_name = "SetMirror"]
        fn set_mirrorpnt(self: Pin<&mut Trsf>, theP: &Pnt);
        #[doc = "Makes the transformation into a symmetrical transformation. theA1 is the center of the axial symmetry."]
        #[cxx_name = "SetMirror"]
        fn set_mirrorax1_2(self: Pin<&mut Trsf>, theA1: &Ax1);
        #[doc = "Makes the transformation into a symmetrical transformation. theA2 is the center of the planar symmetry and defines the plane of symmetry by its origin, \"X Direction\" and \"Y Direction\"."]
        #[cxx_name = "SetMirror"]
        fn set_mirrorax2_3(self: Pin<&mut Trsf>, theA2: &Ax2);
        #[doc = "Changes the transformation into a rotation. theA1 is the rotation axis and theAng is the angular value of the rotation in radians."]
        #[cxx_name = "SetRotation"]
        fn set_rotationax1(self: Pin<&mut Trsf>, theA1: &Ax1, theAng: f64);
        #[doc = "Changes the transformation into a rotation defined by quaternion. Note that rotation is performed around origin, i.e. no translation is involved."]
        #[cxx_name = "SetRotation"]
        fn set_rotationquaternion_2(self: Pin<&mut Trsf>, theR: &gp_Quaternion);
        #[doc = "Replaces the rotation part with specified quaternion."]
        #[cxx_name = "SetRotationPart"]
        fn set_rotation_part(self: Pin<&mut Trsf>, theR: &gp_Quaternion);
        #[doc = "Changes the transformation into a scale. theP is the center of the scale and theS is the scaling value. Raises ConstructionError  If <theS> is null."]
        #[cxx_name = "SetScale"]
        fn set_scale(self: Pin<&mut Trsf>, theP: &Pnt, theS: f64);
        #[doc = "Modifies this transformation so that it transforms the coordinate system defined by theFromSystem1 into the one defined by theToSystem2. After this modification, this transformation transforms: -   the origin of theFromSystem1 into the origin of theToSystem2, -   the \"X Direction\" of theFromSystem1 into the \"X Direction\" of theToSystem2, -   the \"Y Direction\" of theFromSystem1 into the \"Y Direction\" of theToSystem2, and -   the \"main Direction\" of theFromSystem1 into the \"main Direction\" of theToSystem2. Warning When you know the coordinates of a point in one coordinate system and you want to express these coordinates in another one, do not use the transformation resulting from this function. Use the transformation that results from SetTransformation instead. SetDisplacement and SetTransformation create related transformations: the vectorial part of one is the inverse of the vectorial part of the other."]
        #[cxx_name = "SetDisplacement"]
        fn set_displacement(self: Pin<&mut Trsf>, theFromSystem1: &Ax3, theToSystem2: &Ax3);
        #[doc = "Modifies this transformation so that it transforms the coordinates of any point, (x, y, z), relative to a source coordinate system into the coordinates (x', y', z') which are relative to a target coordinate system, but which represent the same point The transformation is from the coordinate system \"theFromSystem1\" to the coordinate system \"theToSystem2\". Example : @code gp_Ax3 theFromSystem1, theToSystem2; double x1, y1, z1;  // are the coordinates of a point in the local system theFromSystem1 double x2, y2, z2;  // are the coordinates of a point in the local system theToSystem2 gp_Pnt P1 (x1, y1, z1) gp_Trsf T; T.SetTransformation (theFromSystem1, theToSystem2); gp_Pnt P2 = P1.Transformed (T); P2.Coord (x2, y2, z2); @endcode"]
        #[cxx_name = "SetTransformation"]
        fn set_transformationax3(self: Pin<&mut Trsf>, theFromSystem1: &Ax3, theToSystem2: &Ax3);
        #[doc = "Modifies this transformation so that it transforms the coordinates of any point, (x, y, z), relative to a source coordinate system into the coordinates (x', y', z') which are relative to a target coordinate system, but which represent the same point The transformation is from the default coordinate system @code {P(0.,0.,0.), VX (1.,0.,0.), VY (0.,1.,0.), VZ (0., 0. ,1.) } @endcode to the local coordinate system defined with the Ax3 theToSystem. Use in the same way  as the previous method. FromSystem1 is defaulted to the absolute coordinate system."]
        #[cxx_name = "SetTransformation"]
        fn set_transformationax3_2(self: Pin<&mut Trsf>, theToSystem: &Ax3);
        #[doc = "Sets transformation by directly specified rotation and translation."]
        #[cxx_name = "SetTransformation"]
        fn set_transformationquaternion_3(self: Pin<&mut Trsf>, R: &gp_Quaternion, theT: &Vec_);
        #[doc = "Changes the transformation into a translation. theV is the vector of the translation."]
        #[cxx_name = "SetTranslation"]
        fn set_translationvec(self: Pin<&mut Trsf>, theV: &Vec_);
        #[doc = "Makes the transformation into a translation where the translation vector is the vector (theP1, theP2) defined from point theP1 to point theP2."]
        #[cxx_name = "SetTranslation"]
        fn set_translationpnt_2(self: Pin<&mut Trsf>, theP1: &Pnt, theP2: &Pnt);
        #[doc = "Replaces the translation vector with the vector theV."]
        #[cxx_name = "SetTranslationPart"]
        fn set_translation_part(self: Pin<&mut Trsf>, theV: &Vec_);
        #[doc = "Modifies the scale factor. Raises ConstructionError  If theS is null."]
        #[cxx_name = "SetScaleFactor"]
        fn set_scale_factor(self: Pin<&mut Trsf>, theS: f64);
        #[doc = "Sets the coefficients  of the transformation.  The transformation  of the  point  x,y,z is  the point x',y',z' with : @code x' = a11 x + a12 y + a13 z + a14 y' = a21 x + a22 y + a23 z + a24 z' = a31 x + a32 y + a33 z + a34 @endcode The method Value(i,j) will return aij. Raises ConstructionError if the determinant of  the aij is null. The matrix is orthogonalized before future using."]
        #[cxx_name = "SetValues"]
        fn set_values(
            self: Pin<&mut Trsf>,
            a11: f64,
            a12: f64,
            a13: f64,
            a14: f64,
            a21: f64,
            a22: f64,
            a23: f64,
            a24: f64,
            a31: f64,
            a32: f64,
            a33: f64,
            a34: f64,
        );
        #[doc = "Returns true if the determinant of the vectorial part of this transformation is negative."]
        #[cxx_name = "IsNegative"]
        fn is_negative(self: &Trsf) -> bool;
        #[doc = "Returns the scale factor."]
        #[cxx_name = "ScaleFactor"]
        fn scale_factor(self: &Trsf) -> f64;
        #[doc = "Returns the translation part of the transformation's matrix"]
        #[cxx_name = "TranslationPart"]
        fn translation_part(self: &Trsf) -> &XYZ;
        #[doc = "Returns the boolean True if there is non-zero rotation. In the presence of rotation, the output parameters store the axis and the angle of rotation. The method always returns positive value \"theAngle\", i.e., 0. < theAngle <= PI. Note that this rotation is defined only by the vectorial part of the transformation; generally you would need to check also the translational part to obtain the axis (gp_Ax1) of rotation."]
        #[cxx_name = "GetRotation"]
        fn get_rotationxyz(self: &Trsf, theAxis: Pin<&mut XYZ>, theAngle: &mut f64) -> bool;
        #[doc = "Computes the homogeneous vectorial part of the transformation. It is a 3*3 matrix which doesn't include the scale factor. In other words, the vectorial part of this transformation is equal to its homogeneous vectorial part, multiplied by the scale factor. The coefficients of this matrix must be multiplied by the scale factor to obtain the coefficients of the transformation."]
        #[cxx_name = "HVectorialPart"]
        fn h_vectorial_part(self: &Trsf) -> &gp_Mat;
        #[doc = "Returns the coefficients of the transformation's matrix. It is a 3 rows * 4 columns matrix. This coefficient includes the scale factor. Raises OutOfRanged if theRow < 1 or theRow > 3 or theCol < 1 or theCol > 4"]
        #[cxx_name = "Value"]
        fn value(self: &Trsf, theRow: i32, theCol: i32) -> f64;
        #[cxx_name = "Invert"]
        fn invert(self: Pin<&mut Trsf>);
        #[doc = "Computes the transformation composed with <me> and theT. <me> = <me> * theT"]
        #[cxx_name = "Multiply"]
        fn multiply(self: Pin<&mut Trsf>, theT: &Trsf);
        #[doc = "Computes the transformation composed with <me> and T. <me> = theT * <me>"]
        #[cxx_name = "PreMultiply"]
        fn pre_multiply(self: Pin<&mut Trsf>, theT: &Trsf);
        #[cxx_name = "Power"]
        fn power(self: Pin<&mut Trsf>, theN: i32);
        #[cxx_name = "Transforms"]
        fn transformsreal(self: &Trsf, theX: &mut f64, theY: &mut f64, theZ: &mut f64);
        #[doc = "Transformation of a triplet XYZ with a Trsf"]
        #[cxx_name = "Transforms"]
        fn transformsxyz_2(self: &Trsf, theCoord: Pin<&mut XYZ>);
        #[doc = "Returns the nature of the transformation. It can be: an identity transformation, a rotation, a translation, a mirror transformation (relative to a point, an axis or a plane), a scaling transformation, or a compound transformation."]
        #[cxx_name = "gp_Trsf_Form"]
        fn Trsf_form(self_: &Trsf) -> UniquePtr<gp_TrsfForm>;
        #[doc = "Returns quaternion representing rotational part of the transformation."]
        #[cxx_name = "gp_Trsf_GetRotation"]
        fn Trsf_get_rotation(self_: &Trsf) -> UniquePtr<gp_Quaternion>;
        #[doc = "Returns the vectorial part of the transformation. It is a 3*3 matrix which includes the scale factor."]
        #[cxx_name = "gp_Trsf_VectorialPart"]
        fn Trsf_vectorial_part(self_: &Trsf) -> UniquePtr<gp_Mat>;
        #[doc = "Computes the reverse transformation Raises an exception if the matrix of the transformation is not inversible, it means that the scale factor is lower or equal to Resolution from package gp. Computes the transformation composed with T and  <me>. In a C++ implementation you can also write Tcomposed = <me> * T. Example : @code gp_Trsf T1, T2, Tcomp; ............... Tcomp = T2.Multiplied(T1);         // or   (Tcomp = T2 * T1) gp_Pnt P1(10.,3.,4.); gp_Pnt P2 = P1.Transformed(Tcomp); // using Tcomp gp_Pnt P3 = P1.Transformed(T1);    // using T1 then T2 P3.Transform(T2);                  // P3 = P2 !!! @endcode"]
        #[cxx_name = "gp_Trsf_Inverted"]
        fn Trsf_inverted(self_: &Trsf) -> UniquePtr<Trsf>;
        #[cxx_name = "gp_Trsf_Multiplied"]
        fn Trsf_multiplied(self_: &Trsf, theT: &Trsf) -> UniquePtr<Trsf>;
        #[doc = "Computes the following composition of transformations <me> * <me> * .......* <me>, theN time. if theN = 0 <me> = Identity if theN < 0 <me> = <me>.Inverse() *...........* <me>.Inverse(). Raises if theN < 0 and if the matrix of the transformation not inversible."]
        #[cxx_name = "gp_Trsf_Powered"]
        fn Trsf_powered(self_: &Trsf, theN: i32) -> UniquePtr<Trsf>;
        #[doc = " ======================== gp_Trsf2d ========================"]
        #[doc = "Defines a non-persistent transformation in 2D space. The following transformations are implemented : - Translation, Rotation, Scale - Symmetry with respect to a point and a line. Complex transformations can be obtained by combining the previous elementary transformations using the method Multiply. The transformations can be represented as follow : @code V1   V2   T       XY        XY | a11  a12  a13 |   | x |     | x'| | a21  a22  a23 |   | y |     | y'| |  0    0    1  |   | 1 |     | 1 | @endcode where {V1, V2} defines the vectorial part of the transformation and T defines the translation part of the transformation. This transformation never change the nature of the objects."]
        #[cxx_name = "gp_Trsf2d"]
        type Trsf2d;
        #[doc = "Returns identity transformation."]
        #[cxx_name = "gp_Trsf2d_ctor"]
        fn Trsf2d_ctor() -> UniquePtr<Trsf2d>;
        #[doc = "Creates a 2d transformation in the XY plane from a 3d transformation ."]
        #[cxx_name = "gp_Trsf2d_ctor_trsf"]
        fn Trsf2d_ctor_trsf(theT: &Trsf) -> UniquePtr<Trsf2d>;
        #[doc = "Changes the transformation into a symmetrical transformation. theP is the center of the symmetry."]
        #[cxx_name = "SetMirror"]
        fn set_mirrorpnt2d(self: Pin<&mut Trsf2d>, theP: &Pnt2d);
        #[doc = "Changes the transformation into a symmetrical transformation. theA is the center of the axial symmetry."]
        #[cxx_name = "SetMirror"]
        fn set_mirrorax2d_2(self: Pin<&mut Trsf2d>, theA: &Ax2d);
        #[doc = "Changes the transformation into a rotation. theP is the rotation's center and theAng is the angular value of the rotation in radian."]
        #[cxx_name = "SetRotation"]
        fn set_rotation(self: Pin<&mut Trsf2d>, theP: &Pnt2d, theAng: f64);
        #[doc = "Changes the transformation into a scale. theP is the center of the scale and theS is the scaling value."]
        #[cxx_name = "SetScale"]
        fn set_scale(self: Pin<&mut Trsf2d>, theP: &Pnt2d, theS: f64);
        #[doc = "Changes a transformation allowing passage from the coordinate system \"theFromSystem1\" to the coordinate system \"theToSystem2\"."]
        #[cxx_name = "SetTransformation"]
        fn set_transformationax2d(
            self: Pin<&mut Trsf2d>,
            theFromSystem1: &Ax2d,
            theToSystem2: &Ax2d,
        );
        #[doc = "Changes the transformation allowing passage from the basic coordinate system {P(0.,0.,0.), VX (1.,0.,0.), VY (0.,1.,0.)} to the local coordinate system defined with the Ax2d theToSystem."]
        #[cxx_name = "SetTransformation"]
        fn set_transformationax2d_2(self: Pin<&mut Trsf2d>, theToSystem: &Ax2d);
        #[doc = "Changes the transformation into a translation. theV is the vector of the translation."]
        #[cxx_name = "SetTranslation"]
        fn set_translationvec2d(self: Pin<&mut Trsf2d>, theV: &Vec2d);
        #[doc = "Makes the transformation into a translation from the point theP1 to the point theP2."]
        #[cxx_name = "SetTranslation"]
        fn set_translationpnt2d_2(self: Pin<&mut Trsf2d>, theP1: &Pnt2d, theP2: &Pnt2d);
        #[doc = "Replaces the translation vector with theV."]
        #[cxx_name = "SetTranslationPart"]
        fn set_translation_part(self: Pin<&mut Trsf2d>, theV: &Vec2d);
        #[doc = "Modifies the scale factor."]
        #[cxx_name = "SetScaleFactor"]
        fn set_scale_factor(self: Pin<&mut Trsf2d>, theS: f64);
        #[doc = "Returns true if the determinant of the vectorial part of this transformation is negative.."]
        #[cxx_name = "IsNegative"]
        fn is_negative(self: &Trsf2d) -> bool;
        #[doc = "Returns the scale factor."]
        #[cxx_name = "ScaleFactor"]
        fn scale_factor(self: &Trsf2d) -> f64;
        #[doc = "Returns the translation part of the transformation's matrix"]
        #[cxx_name = "TranslationPart"]
        fn translation_part(self: &Trsf2d) -> &gp_XY;
        #[doc = "Returns the homogeneous vectorial part of the transformation. It is a 2*2 matrix which doesn't include the scale factor. The coefficients of this matrix must be multiplied by the scale factor to obtain the coefficients of the transformation."]
        #[cxx_name = "HVectorialPart"]
        fn h_vectorial_part(self: &Trsf2d) -> &gp_Mat2d;
        #[doc = "Returns the angle corresponding to the rotational component of the transformation matrix (operation opposite to SetRotation())."]
        #[cxx_name = "RotationPart"]
        fn rotation_part(self: &Trsf2d) -> f64;
        #[doc = "Returns the coefficients of the transformation's matrix. It is a 2 rows * 3 columns matrix. Raises OutOfRange if theRow < 1 or theRow > 2 or theCol < 1 or theCol > 3"]
        #[cxx_name = "Value"]
        fn value(self: &Trsf2d, theRow: i32, theCol: i32) -> f64;
        #[cxx_name = "Invert"]
        fn invert(self: Pin<&mut Trsf2d>);
        #[doc = "Computes the transformation composed from <me> and theT. <me> = <me> * theT"]
        #[cxx_name = "Multiply"]
        fn multiply(self: Pin<&mut Trsf2d>, theT: &Trsf2d);
        #[doc = "Computes the transformation composed from <me> and theT. <me> = theT * <me>"]
        #[cxx_name = "PreMultiply"]
        fn pre_multiply(self: Pin<&mut Trsf2d>, theT: &Trsf2d);
        #[cxx_name = "Power"]
        fn power(self: Pin<&mut Trsf2d>, theN: i32);
        #[cxx_name = "Transforms"]
        fn transformsreal(self: &Trsf2d, theX: &mut f64, theY: &mut f64);
        #[doc = "Transforms  a doublet XY with a Trsf2d"]
        #[cxx_name = "Transforms"]
        fn transformsxy_2(self: &Trsf2d, theCoord: Pin<&mut gp_XY>);
        #[doc = "Sets the coefficients  of the transformation. The transformation  of the  point  x,y is  the point x',y' with : @code x' = a11 x + a12 y + a13 y' = a21 x + a22 y + a23 @endcode The method Value(i,j) will return aij. Raises ConstructionError if the determinant of the aij is null. If the matrix as not a uniform scale it will be orthogonalized before future using."]
        #[cxx_name = "SetValues"]
        fn set_values(
            self: Pin<&mut Trsf2d>,
            a11: f64,
            a12: f64,
            a13: f64,
            a21: f64,
            a22: f64,
            a23: f64,
        );
        #[doc = "Returns the nature of the transformation. It can be  an identity transformation, a rotation, a translation, a mirror (relative to a point or an axis), a scaling transformation, or a compound transformation."]
        #[cxx_name = "gp_Trsf2d_Form"]
        fn Trsf2d_form(self_: &Trsf2d) -> UniquePtr<gp_TrsfForm>;
        #[doc = "Returns the vectorial part of the transformation. It is a 2*2 matrix which includes the scale factor."]
        #[cxx_name = "gp_Trsf2d_VectorialPart"]
        fn Trsf2d_vectorial_part(self_: &Trsf2d) -> UniquePtr<gp_Mat2d>;
        #[doc = "Computes the reverse transformation. Raises an exception if the matrix of the transformation is not inversible, it means that the scale factor is lower or equal to Resolution from package gp."]
        #[cxx_name = "gp_Trsf2d_Inverted"]
        fn Trsf2d_inverted(self_: &Trsf2d) -> UniquePtr<Trsf2d>;
        #[cxx_name = "gp_Trsf2d_Multiplied"]
        fn Trsf2d_multiplied(self_: &Trsf2d, theT: &Trsf2d) -> UniquePtr<Trsf2d>;
        #[doc = "Computes the following composition of transformations <me> * <me> * .......* <me>,  theN time. if theN = 0 <me> = Identity if theN < 0 <me> = <me>.Inverse() *...........* <me>.Inverse(). Raises if theN < 0 and if the matrix of the transformation not inversible."]
        #[cxx_name = "gp_Trsf2d_Powered"]
        fn Trsf2d_powered(self_: Pin<&mut Trsf2d>, theN: i32) -> UniquePtr<Trsf2d>;
        #[doc = " ======================== gp_GTrsf ========================"]
        #[doc = "Defines a non-persistent transformation in 3D space. This transformation is a general transformation. It can be a gp_Trsf, an affinity, or you can define your own transformation giving the matrix of transformation. With a gp_GTrsf you can transform only a triplet of coordinates gp_XYZ. It is not possible to transform other geometric objects because these transformations can change the nature of non-elementary geometric objects. The transformation gp_GTrsf can be represented as follow: @code V1   V2   V3    T       XYZ        XYZ | a11  a12  a13   a14 |   | x |      | x'| | a21  a22  a23   a24 |   | y |      | y'| | a31  a32  a33   a34 |   | z |   =  | z'| |  0    0    0     1  |   | 1 |      | 1 | @endcode where {V1, V2, V3} define the vectorial part of the transformation and T defines the translation part of the transformation. Warning A gp_GTrsf transformation is only applicable to coordinates. Be careful if you apply such a transformation to all points of a geometric object, as this can change the nature of the object and thus render it incoherent! Typically, a circle is transformed into an ellipse by an affinity transformation. To avoid modifying the nature of an object, use a gp_Trsf transformation instead, as objects of this class respect the nature of geometric objects."]
        #[cxx_name = "gp_GTrsf"]
        type GTrsf;
        #[doc = "Returns the Identity transformation."]
        #[cxx_name = "gp_GTrsf_ctor"]
        fn GTrsf_ctor() -> UniquePtr<GTrsf>;
        #[doc = "Converts the gp_Trsf transformation theT into a general transformation, i.e. Returns a GTrsf with the same matrix of coefficients as the Trsf theT."]
        #[cxx_name = "gp_GTrsf_ctor_trsf"]
        fn GTrsf_ctor_trsf(theT: &Trsf) -> UniquePtr<GTrsf>;
        #[doc = "Creates a transformation based on the matrix theM and the vector theV where theM defines the vectorial part of the transformation, and V the translation part, or"]
        #[cxx_name = "gp_GTrsf_ctor_mat_xyz"]
        fn GTrsf_ctor_mat_xyz(theM: &gp_Mat, theV: &XYZ) -> UniquePtr<GTrsf>;
        #[doc = "Changes this transformation into an affinity of ratio theRatio with respect to the axis theA1. Note: an affinity is a point-by-point transformation that transforms any point P into a point P' such that if H is the orthogonal projection of P on the axis theA1 or the plane A2, the vectors HP and HP' satisfy: HP' = theRatio * HP."]
        #[cxx_name = "SetAffinity"]
        fn set_affinityax1(self: Pin<&mut GTrsf>, theA1: &Ax1, theRatio: f64);
        #[doc = "Changes this transformation into an affinity of ratio theRatio with respect to  the plane defined by the origin, the \"X Direction\" and the \"Y Direction\" of coordinate system theA2. Note: an affinity is a point-by-point transformation that transforms any point P into a point P' such that if H is the orthogonal projection of P on the axis A1 or the plane theA2, the vectors HP and HP' satisfy: HP' = theRatio * HP."]
        #[cxx_name = "SetAffinity"]
        fn set_affinityax2_2(self: Pin<&mut GTrsf>, theA2: &Ax2, theRatio: f64);
        #[doc = "Replaces  the coefficient (theRow, theCol) of the matrix representing this transformation by theValue.  Raises OutOfRange if  theRow < 1 or theRow > 3 or theCol < 1 or theCol > 4"]
        #[cxx_name = "SetValue"]
        fn set_value(self: Pin<&mut GTrsf>, theRow: i32, theCol: i32, theValue: f64);
        #[doc = "Replaces the vectorial part of this transformation by theMatrix."]
        #[cxx_name = "SetVectorialPart"]
        fn set_vectorial_part(self: Pin<&mut GTrsf>, theMatrix: &gp_Mat);
        #[doc = "Replaces the translation part of this transformation by the coordinates of the number triple theCoord."]
        #[cxx_name = "SetTranslationPart"]
        fn set_translation_part(self: Pin<&mut GTrsf>, theCoord: &XYZ);
        #[doc = "Assigns the vectorial and translation parts of theT to this transformation."]
        #[cxx_name = "SetTrsf"]
        fn set_trsf(self: Pin<&mut GTrsf>, theT: &Trsf);
        #[doc = "Returns true if the determinant of the vectorial part of this transformation is negative."]
        #[cxx_name = "IsNegative"]
        fn is_negative(self: &GTrsf) -> bool;
        #[doc = "Returns true if this transformation is singular (and therefore, cannot be inverted). Note: The Gauss LU decomposition is used to invert the transformation matrix. Consequently, the transformation is considered as singular if the largest pivot found is less than or equal to gp::Resolution(). Warning If this transformation is singular, it cannot be inverted."]
        #[cxx_name = "IsSingular"]
        fn is_singular(self: &GTrsf) -> bool;
        #[doc = "verify and set the shape of the GTrsf Other or CompoundTrsf Ex : @code myGTrsf.SetValue(row1,col1,val1); myGTrsf.SetValue(row2,col2,val2); ... myGTrsf.SetForm(); @endcode"]
        #[cxx_name = "SetForm"]
        fn set_form(self: Pin<&mut GTrsf>);
        #[doc = "Returns the translation part of the GTrsf."]
        #[cxx_name = "TranslationPart"]
        fn translation_part(self: &GTrsf) -> &XYZ;
        #[doc = "Computes the vectorial part of the GTrsf. The returned Matrix is a  3*3 matrix."]
        #[cxx_name = "VectorialPart"]
        fn vectorial_part(self: &GTrsf) -> &gp_Mat;
        #[doc = "Returns the coefficients of the global matrix of transformation. Raises OutOfRange if theRow < 1 or theRow > 3 or theCol < 1 or theCol > 4"]
        #[cxx_name = "Value"]
        fn value(self: &GTrsf, theRow: i32, theCol: i32) -> f64;
        #[cxx_name = "Invert"]
        fn invert(self: Pin<&mut GTrsf>);
        #[doc = "Computes the transformation composed with <me> and theT. <me> = <me> * theT"]
        #[cxx_name = "Multiply"]
        fn multiply(self: Pin<&mut GTrsf>, theT: &GTrsf);
        #[doc = "Computes the product of the transformation theT and this transformation and assigns the result to this transformation. this = theT * this"]
        #[cxx_name = "PreMultiply"]
        fn pre_multiply(self: Pin<&mut GTrsf>, theT: &GTrsf);
        #[cxx_name = "Power"]
        fn power(self: Pin<&mut GTrsf>, theN: i32);
        #[cxx_name = "Transforms"]
        fn transformsxyz(self: &GTrsf, theCoord: Pin<&mut XYZ>);
        #[doc = "Transforms a triplet XYZ with a GTrsf."]
        #[cxx_name = "Transforms"]
        fn transformsreal_2(self: &GTrsf, theX: &mut f64, theY: &mut f64, theZ: &mut f64);
        #[doc = "Returns the nature of the transformation.  It can be an identity transformation, a rotation, a translation, a mirror transformation (relative to a point, an axis or a plane), a scaling transformation, a compound transformation or some other type of transformation."]
        #[cxx_name = "gp_GTrsf_Form"]
        fn GTrsf_form(self_: &GTrsf) -> UniquePtr<gp_TrsfForm>;
        #[doc = "Computes the reverse transformation. Raises an exception if the matrix of the transformation is not inversible."]
        #[cxx_name = "gp_GTrsf_Inverted"]
        fn GTrsf_inverted(self_: &GTrsf) -> UniquePtr<GTrsf>;
        #[doc = "Computes the transformation composed from theT and <me>. In a C++ implementation you can also write Tcomposed = <me> * theT. Example : @code gp_GTrsf T1, T2, Tcomp; ............... //composition : Tcomp = T2.Multiplied(T1);         // or   (Tcomp = T2 * T1) // transformation of a point gp_XYZ P(10.,3.,4.); gp_XYZ P1(P); Tcomp.Transforms(P1);               //using Tcomp gp_XYZ P2(P); T1.Transforms(P2);                  //using T1 then T2 T2.Transforms(P2);                  // P1 = P2 !!! @endcode"]
        #[cxx_name = "gp_GTrsf_Multiplied"]
        fn GTrsf_multiplied(self_: &GTrsf, theT: &GTrsf) -> UniquePtr<GTrsf>;
        #[doc = "Computes: -   the product of this transformation multiplied by itself theN times, if theN is positive, or -   the product of the inverse of this transformation multiplied by itself |theN| times, if theN is negative. If theN equals zero, the result is equal to the Identity transformation. I.e.:  <me> * <me> * .......* <me>, theN time. if theN =0 <me> = Identity if theN < 0 <me> = <me>.Inverse() *...........* <me>.Inverse(). Raises an exception if N < 0 and if the matrix of the transformation not inversible."]
        #[cxx_name = "gp_GTrsf_Powered"]
        fn GTrsf_powered(self_: &GTrsf, theN: i32) -> UniquePtr<GTrsf>;
        #[cxx_name = "gp_GTrsf_Trsf"]
        fn GTrsf_trsf(self_: &GTrsf) -> UniquePtr<Trsf>;
        #[doc = " ======================== gp_GTrsf2d ========================"]
        #[doc = "Defines a non persistent transformation in 2D space. This transformation is a general transformation. It can be a gp_Trsf2d, an affinity, or you can define your own transformation giving the corresponding matrix of transformation. With a gp_GTrsf2d you can transform only a doublet of coordinates gp_XY. It is not possible to transform other geometric objects because these transformations can change the nature of non-elementary geometric objects. A gp_GTrsf2d is represented with a 2 rows * 3 columns matrix: @code V1   V2   T        XY         XY | a11  a12  a14 |   | x |      | x'| | a21  a22  a24 |   | y |   =  | y'| |  0    0    1  |   | 1 |      | 1 | @endcode where {V1, V2} defines the vectorial part of the transformation and T defines the translation part of the transformation. Warning A gp_GTrsf2d transformation is only applicable on coordinates. Be careful if you apply such a transformation to all the points of a geometric object, as this can change the nature of the object and thus render it incoherent! Typically, a circle is transformed into an ellipse by an affinity transformation. To avoid modifying the nature of an object, use a gp_Trsf2d transformation instead, as objects of this class respect the nature of geometric objects."]
        #[cxx_name = "gp_GTrsf2d"]
        type GTrsf2d;
        #[doc = "returns identity transformation."]
        #[cxx_name = "gp_GTrsf2d_ctor"]
        fn GTrsf2d_ctor() -> UniquePtr<GTrsf2d>;
        #[doc = "Converts the gp_Trsf2d transformation theT into a general transformation."]
        #[cxx_name = "gp_GTrsf2d_ctor_trsf2d"]
        fn GTrsf2d_ctor_trsf2d(theT: &Trsf2d) -> UniquePtr<GTrsf2d>;
        #[doc = "Creates   a transformation based on the matrix theM and the vector theV where theM defines the vectorial part of the transformation, and theV the translation part."]
        #[cxx_name = "gp_GTrsf2d_ctor_mat2d_xy"]
        fn GTrsf2d_ctor_mat2d_xy(theM: &gp_Mat2d, theV: &gp_XY) -> UniquePtr<GTrsf2d>;
        #[doc = "Changes this transformation into an affinity of ratio theRatio with respect to the axis theA. Note: An affinity is a point-by-point transformation that transforms any point P into a point P' such that if H is the orthogonal projection of P on the axis theA, the vectors HP and HP' satisfy: HP' = theRatio * HP."]
        #[cxx_name = "SetAffinity"]
        fn set_affinity(self: Pin<&mut GTrsf2d>, theA: &Ax2d, theRatio: f64);
        #[doc = "Replaces   the coefficient (theRow, theCol) of the matrix representing this transformation by theValue, Raises OutOfRange if theRow < 1 or theRow > 2 or theCol < 1 or theCol > 3"]
        #[cxx_name = "SetValue"]
        fn set_value(self: Pin<&mut GTrsf2d>, theRow: i32, theCol: i32, theValue: f64);
        #[doc = "Replaces the translation part of this transformation by the coordinates of the number pair theCoord."]
        #[cxx_name = "SetTranslationPart"]
        fn set_translation_part(self: Pin<&mut GTrsf2d>, theCoord: &gp_XY);
        #[doc = "Assigns the vectorial and translation parts of theT to this transformation."]
        #[cxx_name = "SetTrsf2d"]
        fn set_trsf2d(self: Pin<&mut GTrsf2d>, theT: &Trsf2d);
        #[doc = "Replaces the vectorial part of this transformation by theMatrix."]
        #[cxx_name = "SetVectorialPart"]
        fn set_vectorial_part(self: Pin<&mut GTrsf2d>, theMatrix: &gp_Mat2d);
        #[doc = "Returns true if the determinant of the vectorial part of this transformation is negative."]
        #[cxx_name = "IsNegative"]
        fn is_negative(self: &GTrsf2d) -> bool;
        #[doc = "Returns true if this transformation is singular (and therefore, cannot be inverted). Note: The Gauss LU decomposition is used to invert the transformation matrix. Consequently, the transformation is considered as singular if the largest pivot found is less than or equal to gp::Resolution(). Warning If this transformation is singular, it cannot be inverted."]
        #[cxx_name = "IsSingular"]
        fn is_singular(self: &GTrsf2d) -> bool;
        #[doc = "Returns the translation part of the GTrsf2d."]
        #[cxx_name = "TranslationPart"]
        fn translation_part(self: &GTrsf2d) -> &gp_XY;
        #[doc = "Computes the vectorial part of the GTrsf2d. The returned Matrix is a 2*2 matrix."]
        #[cxx_name = "VectorialPart"]
        fn vectorial_part(self: &GTrsf2d) -> &gp_Mat2d;
        #[doc = "Returns the coefficients of the global matrix of transformation. Raised OutOfRange if theRow < 1 or theRow > 2 or theCol < 1 or theCol > 3"]
        #[cxx_name = "Value"]
        fn value(self: &GTrsf2d, theRow: i32, theCol: i32) -> f64;
        #[cxx_name = "Invert"]
        fn invert(self: Pin<&mut GTrsf2d>);
        #[cxx_name = "Multiply"]
        fn multiply(self: Pin<&mut GTrsf2d>, theT: &GTrsf2d);
        #[doc = "Computes the product of the transformation theT and this transformation, and assigns the result to this transformation: this = theT * this"]
        #[cxx_name = "PreMultiply"]
        fn pre_multiply(self: Pin<&mut GTrsf2d>, theT: &GTrsf2d);
        #[cxx_name = "Power"]
        fn power(self: Pin<&mut GTrsf2d>, theN: i32);
        #[cxx_name = "Transforms"]
        fn transformsxy(self: &GTrsf2d, theCoord: Pin<&mut gp_XY>);
        #[doc = "Applies this transformation to the coordinates: -   of the number pair Coord, or -   X and Y. Note: -   Transforms modifies theX, theY, or the coordinate pair Coord, while -   Transformed creates a new coordinate pair."]
        #[cxx_name = "Transforms"]
        fn transformsreal_2(self: &GTrsf2d, theX: &mut f64, theY: &mut f64);
        #[doc = "Returns the nature of the transformation.  It can be an identity transformation, a rotation, a translation, a mirror transformation (relative to a point or axis), a scaling transformation, a compound transformation or some other type of transformation."]
        #[cxx_name = "gp_GTrsf2d_Form"]
        fn GTrsf2d_form(self_: &GTrsf2d) -> UniquePtr<gp_TrsfForm>;
        #[doc = "Computes the reverse transformation. Raised an exception if the matrix of the transformation is not inversible."]
        #[cxx_name = "gp_GTrsf2d_Inverted"]
        fn GTrsf2d_inverted(self_: &GTrsf2d) -> UniquePtr<GTrsf2d>;
        #[doc = "Computes the transformation composed with theT and <me>. In a C++ implementation you can also write Tcomposed = <me> * theT. Example : @code gp_GTrsf2d T1, T2, Tcomp; ............... //composition : Tcomp = T2.Multiplied(T1);         // or   (Tcomp = T2 * T1) // transformation of a point gp_XY P(10.,3.); gp_XY P1(P); Tcomp.Transforms(P1);               //using Tcomp gp_XY P2(P); T1.Transforms(P2);                  //using T1 then T2 T2.Transforms(P2);                  // P1 = P2 !!! @endcode"]
        #[cxx_name = "gp_GTrsf2d_Multiplied"]
        fn GTrsf2d_multiplied(self_: &GTrsf2d, theT: &GTrsf2d) -> UniquePtr<GTrsf2d>;
        #[doc = "Computes the following composition of transformations <me> * <me> * .......* <me>, theN time. if theN = 0 <me> = Identity if theN < 0 <me> = <me>.Inverse() *...........* <me>.Inverse(). Raises an exception if theN < 0 and if the matrix of the transformation is not inversible."]
        #[cxx_name = "gp_GTrsf2d_Powered"]
        fn GTrsf2d_powered(self_: &GTrsf2d, theN: i32) -> UniquePtr<GTrsf2d>;
        #[cxx_name = "gp_GTrsf2d_Transformed"]
        fn GTrsf2d_transformed(self_: &GTrsf2d, theCoord: &gp_XY) -> UniquePtr<gp_XY>;
        #[doc = "Converts this transformation into a gp_Trsf2d transformation. Exceptions Standard_ConstructionError if this transformation cannot be converted, i.e. if its form is gp_Other."]
        #[cxx_name = "gp_GTrsf2d_Trsf2d"]
        fn GTrsf2d_trsf2d(self_: &GTrsf2d) -> UniquePtr<Trsf2d>;
        #[doc = " ======================== gp_Lin ========================"]
        #[doc = "Describes a line in 3D space. A line is positioned in space with an axis (a gp_Ax1 object) which gives it an origin and a unit vector. A line and an axis are similar objects, thus, we can convert one into the other. A line provides direct access to the majority of the edit and query functions available on its positioning axis. In addition, however, a line has specific functions for computing distances and positions. See Also gce_MakeLin which provides functions for more complex line constructions Geom_Line which provides additional functions for constructing lines and works, in particular, with the parametric equations of lines"]
        #[cxx_name = "gp_Lin"]
        type Lin;
        #[doc = "Creates a Line corresponding to Z axis of the reference coordinate system."]
        #[cxx_name = "gp_Lin_ctor"]
        fn Lin_ctor() -> UniquePtr<Lin>;
        #[doc = "Creates a line defined by axis theA1."]
        #[cxx_name = "gp_Lin_ctor_ax1"]
        fn Lin_ctor_ax1(theA1: &Ax1) -> UniquePtr<Lin>;
        #[doc = "Creates a line passing through point theP and parallel to vector theV (theP and theV are, respectively, the origin and the unit vector of the positioning axis of the line)."]
        #[cxx_name = "gp_Lin_ctor_pnt_dir"]
        fn Lin_ctor_pnt_dir(theP: &Pnt, theV: &Dir) -> UniquePtr<Lin>;
        #[cxx_name = "Reverse"]
        fn reverse(self: Pin<&mut Lin>);
        #[doc = "Changes the direction of the line."]
        #[cxx_name = "SetDirection"]
        fn set_direction(self: Pin<&mut Lin>, theV: &Dir);
        #[doc = "Changes the location point (origin) of the line."]
        #[cxx_name = "SetLocation"]
        fn set_location(self: Pin<&mut Lin>, theP: &Pnt);
        #[doc = "Complete redefinition of the line. The \"Location\" point of <theA1> is the origin of the line. The \"Direction\" of <theA1> is  the direction of the line."]
        #[cxx_name = "SetPosition"]
        fn set_position(self: Pin<&mut Lin>, theA1: &Ax1);
        #[doc = "Returns the direction of the line."]
        #[cxx_name = "Direction"]
        fn direction(self: &Lin) -> &Dir;
        #[doc = "Returns the location point (origin) of the line."]
        #[cxx_name = "Location"]
        fn location(self: &Lin) -> &Pnt;
        #[doc = "Returns the axis placement one axis with the same location and direction as <me>."]
        #[cxx_name = "Position"]
        fn position(self: &Lin) -> &Ax1;
        #[doc = "Computes the angle between two lines in radians."]
        #[cxx_name = "Angle"]
        fn angle(self: &Lin, theOther: &Lin) -> f64;
        #[doc = "Returns true if this line contains the point theP, that is, if the distance between point theP and this line is less than or equal to theLinearTolerance.."]
        #[cxx_name = "Contains"]
        fn contains(self: &Lin, theP: &Pnt, theLinearTolerance: f64) -> bool;
        #[doc = "Computes the distance between <me> and the point theP."]
        #[cxx_name = "Distance"]
        fn distancepnt(self: &Lin, theP: &Pnt) -> f64;
        #[doc = "Computes the distance between two lines."]
        #[cxx_name = "Distance"]
        fn distancelin_2(self: &Lin, theOther: &Lin) -> f64;
        #[doc = "Computes the square distance between <me> and the point theP."]
        #[cxx_name = "SquareDistance"]
        fn square_distancepnt(self: &Lin, theP: &Pnt) -> f64;
        #[doc = "Computes the square distance between two lines."]
        #[cxx_name = "SquareDistance"]
        fn square_distancelin_2(self: &Lin, theOther: &Lin) -> f64;
        #[cxx_name = "Mirror"]
        fn mirrorpnt(self: Pin<&mut Lin>, theP: &Pnt);
        #[cxx_name = "Mirror"]
        fn mirrorax1_2(self: Pin<&mut Lin>, theA1: &Ax1);
        #[cxx_name = "Mirror"]
        fn mirrorax2_3(self: Pin<&mut Lin>, theA2: &Ax2);
        #[cxx_name = "Rotate"]
        fn rotate(self: Pin<&mut Lin>, theA1: &Ax1, theAng: f64);
        #[cxx_name = "Scale"]
        fn scale(self: Pin<&mut Lin>, theP: &Pnt, theS: f64);
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Lin>, theT: &Trsf);
        #[cxx_name = "Translate"]
        fn translatevec(self: Pin<&mut Lin>, theV: &Vec_);
        #[cxx_name = "Translate"]
        fn translatepnt_2(self: Pin<&mut Lin>, theP1: &Pnt, theP2: &Pnt);
        #[doc = "Reverses the direction of the line. Note: -   Reverse assigns the result to this line, while -   Reversed creates a new one."]
        #[cxx_name = "gp_Lin_Reversed"]
        fn Lin_reversed(self_: &Lin) -> UniquePtr<Lin>;
        #[doc = "Computes the line normal to the direction of <me>, passing through the point theP.  Raises ConstructionError if the distance between <me> and the point theP is lower or equal to Resolution from gp because there is an infinity of solutions in 3D space."]
        #[cxx_name = "gp_Lin_Normal"]
        fn Lin_normal(self_: &Lin, theP: &Pnt) -> UniquePtr<Lin>;
        #[doc = "Performs the symmetrical transformation of a line with respect to the point theP which is the center of the symmetry."]
        #[cxx_name = "gp_Lin_Mirrored"]
        fn Lin_mirroredpnt(self_: &Lin, theP: &Pnt) -> UniquePtr<Lin>;
        #[doc = "Performs the symmetrical transformation of a line with respect to an axis placement which is the axis of the symmetry."]
        #[cxx_name = "gp_Lin_Mirrored"]
        fn Lin_mirroredax1_2(self_: &Lin, theA1: &Ax1) -> UniquePtr<Lin>;
        #[doc = "Performs the symmetrical transformation of a line with respect to a plane. The axis placement  <theA2> locates the plane of the symmetry : (Location, XDirection, YDirection)."]
        #[cxx_name = "gp_Lin_Mirrored"]
        fn Lin_mirroredax2_3(self_: &Lin, theA2: &Ax2) -> UniquePtr<Lin>;
        #[doc = "Rotates a line. A1 is the axis of the rotation. Ang is the angular value of the rotation in radians."]
        #[cxx_name = "gp_Lin_Rotated"]
        fn Lin_rotated(self_: &Lin, theA1: &Ax1, theAng: f64) -> UniquePtr<Lin>;
        #[doc = "Scales a line. theS is the scaling value. The \"Location\" point (origin) of the line is modified. The \"Direction\" is reversed if the scale is negative."]
        #[cxx_name = "gp_Lin_Scaled"]
        fn Lin_scaled(self_: &Lin, theP: &Pnt, theS: f64) -> UniquePtr<Lin>;
        #[doc = "Transforms a line with the transformation theT from class Trsf."]
        #[cxx_name = "gp_Lin_Transformed"]
        fn Lin_transformed(self_: &Lin, theT: &Trsf) -> UniquePtr<Lin>;
        #[doc = "Translates a line in the direction of the vector theV. The magnitude of the translation is the vector's magnitude."]
        #[cxx_name = "gp_Lin_Translated"]
        fn Lin_translatedvec(self_: &Lin, theV: &Vec_) -> UniquePtr<Lin>;
        #[doc = "Translates a line from the point theP1 to the point theP2."]
        #[cxx_name = "gp_Lin_Translated"]
        fn Lin_translatedpnt_2(self_: &Lin, theP1: &Pnt, theP2: &Pnt) -> UniquePtr<Lin>;
        #[doc = " ======================== gp_Circ ========================"]
        #[doc = "Describes a circle in 3D space. A circle is defined by its radius and positioned in space with a coordinate system (a gp_Ax2 object) as follows: -   the origin of the coordinate system is the center of the circle, and -   the origin, \"X Direction\" and \"Y Direction\" of the coordinate system define the plane of the circle. This positioning coordinate system is the \"local coordinate system\" of the circle. Its \"main Direction\" gives the normal vector to the plane of the circle. The \"main Axis\" of the coordinate system is referred to as the \"Axis\" of the circle. Note: when a gp_Circ circle is converted into a Geom_Circle circle, some implicit properties of the circle are used explicitly: -   the \"main Direction\" of the local coordinate system gives an implicit orientation to the circle (and defines its trigonometric sense), -   this orientation corresponds to the direction in which parameter values increase, -   the starting point for parameterization is that of the \"X Axis\" of the local coordinate system (i.e. the \"X Axis\" of the circle). See Also gce_MakeCirc which provides functions for more complex circle constructions Geom_Circle which provides additional functions for constructing circles and works, in particular, with the parametric equations of circles"]
        #[cxx_name = "gp_Circ"]
        type Circ;
        #[doc = "Creates an indefinite circle."]
        #[cxx_name = "gp_Circ_ctor"]
        fn Circ_ctor() -> UniquePtr<Circ>;
        #[doc = "A2 locates the circle and gives its orientation in 3D space. Warnings : It is not forbidden to create a circle with theRadius = 0.0  Raises ConstructionError if theRadius < 0.0"]
        #[cxx_name = "gp_Circ_ctor_ax2_real"]
        fn Circ_ctor_ax2_real(theA2: &Ax2, theRadius: f64) -> UniquePtr<Circ>;
        #[doc = "Changes the main axis of the circle. It is the axis perpendicular to the plane of the circle. Raises ConstructionError if the direction of theA1 is parallel to the \"XAxis\" of the circle."]
        #[cxx_name = "SetAxis"]
        fn set_axis(self: Pin<&mut Circ>, theA1: &Ax1);
        #[doc = "Changes the \"Location\" point (center) of the circle."]
        #[cxx_name = "SetLocation"]
        fn set_location(self: Pin<&mut Circ>, theP: &Pnt);
        #[doc = "Changes the position of the circle."]
        #[cxx_name = "SetPosition"]
        fn set_position(self: Pin<&mut Circ>, theA2: &Ax2);
        #[doc = "Modifies the radius of this circle. Warning. This class does not prevent the creation of a circle where theRadius is null. Exceptions Standard_ConstructionError if theRadius is negative."]
        #[cxx_name = "SetRadius"]
        fn set_radius(self: Pin<&mut Circ>, theRadius: f64);
        #[doc = "Computes the area of the circle."]
        #[cxx_name = "Area"]
        fn area(self: &Circ) -> f64;
        #[doc = "Returns the main axis of the circle. It is the axis perpendicular to the plane of the circle, passing through the \"Location\" point (center) of the circle."]
        #[cxx_name = "Axis"]
        fn axis(self: &Circ) -> &Ax1;
        #[doc = "Computes the circumference of the circle."]
        #[cxx_name = "Length"]
        fn length(self: &Circ) -> f64;
        #[doc = "Returns the center of the circle. It is the \"Location\" point of the local coordinate system of the circle"]
        #[cxx_name = "Location"]
        fn location(self: &Circ) -> &Pnt;
        #[doc = "Returns the position of the circle. It is the local coordinate system of the circle."]
        #[cxx_name = "Position"]
        fn position(self: &Circ) -> &Ax2;
        #[doc = "Returns the radius of this circle."]
        #[cxx_name = "Radius"]
        fn radius(self: &Circ) -> f64;
        #[doc = "Computes the minimum of distance between the point theP and any point on the circumference of the circle."]
        #[cxx_name = "Distance"]
        fn distance(self: &Circ, theP: &Pnt) -> f64;
        #[doc = "Computes the square distance between <me> and the point theP."]
        #[cxx_name = "SquareDistance"]
        fn square_distance(self: &Circ, theP: &Pnt) -> f64;
        #[doc = "Returns True if the point theP is on the circumference. The distance between <me> and <theP> must be lower or equal to theLinearTolerance."]
        #[cxx_name = "Contains"]
        fn contains(self: &Circ, theP: &Pnt, theLinearTolerance: f64) -> bool;
        #[cxx_name = "Mirror"]
        fn mirrorpnt(self: Pin<&mut Circ>, theP: &Pnt);
        #[cxx_name = "Mirror"]
        fn mirrorax1_2(self: Pin<&mut Circ>, theA1: &Ax1);
        #[cxx_name = "Mirror"]
        fn mirrorax2_3(self: Pin<&mut Circ>, theA2: &Ax2);
        #[cxx_name = "Rotate"]
        fn rotate(self: Pin<&mut Circ>, theA1: &Ax1, theAng: f64);
        #[cxx_name = "Scale"]
        fn scale(self: Pin<&mut Circ>, theP: &Pnt, theS: f64);
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Circ>, theT: &Trsf);
        #[cxx_name = "Translate"]
        fn translatevec(self: Pin<&mut Circ>, theV: &Vec_);
        #[cxx_name = "Translate"]
        fn translatepnt_2(self: Pin<&mut Circ>, theP1: &Pnt, theP2: &Pnt);
        #[doc = "Returns the \"XAxis\" of the circle. This axis is perpendicular to the axis of the conic. This axis and the \"Yaxis\" define the plane of the conic."]
        #[cxx_name = "gp_Circ_XAxis"]
        fn Circ_x_axis(self_: &Circ) -> UniquePtr<Ax1>;
        #[doc = "Returns the \"YAxis\" of the circle. This axis and the \"Xaxis\" define the plane of the conic. The \"YAxis\" is perpendicular to the \"Xaxis\"."]
        #[cxx_name = "gp_Circ_YAxis"]
        fn Circ_y_axis(self_: &Circ) -> UniquePtr<Ax1>;
        #[doc = "Performs the symmetrical transformation of a circle with respect to the point theP which is the center of the symmetry."]
        #[cxx_name = "gp_Circ_Mirrored"]
        fn Circ_mirroredpnt(self_: &Circ, theP: &Pnt) -> UniquePtr<Circ>;
        #[doc = "Performs the symmetrical transformation of a circle with respect to an axis placement which is the axis of the symmetry."]
        #[cxx_name = "gp_Circ_Mirrored"]
        fn Circ_mirroredax1_2(self_: &Circ, theA1: &Ax1) -> UniquePtr<Circ>;
        #[doc = "Performs the symmetrical transformation of a circle with respect to a plane. The axis placement theA2 locates the plane of the of the symmetry : (Location, XDirection, YDirection)."]
        #[cxx_name = "gp_Circ_Mirrored"]
        fn Circ_mirroredax2_3(self_: &Circ, theA2: &Ax2) -> UniquePtr<Circ>;
        #[doc = "Rotates a circle. theA1 is the axis of the rotation. theAng is the angular value of the rotation in radians."]
        #[cxx_name = "gp_Circ_Rotated"]
        fn Circ_rotated(self_: &Circ, theA1: &Ax1, theAng: f64) -> UniquePtr<Circ>;
        #[doc = "Scales a circle. theS is the scaling value. Warnings : If theS is negative the radius stay positive but the \"XAxis\" and the \"YAxis\" are  reversed as for an ellipse."]
        #[cxx_name = "gp_Circ_Scaled"]
        fn Circ_scaled(self_: &Circ, theP: &Pnt, theS: f64) -> UniquePtr<Circ>;
        #[doc = "Transforms a circle with the transformation theT from class Trsf."]
        #[cxx_name = "gp_Circ_Transformed"]
        fn Circ_transformed(self_: &Circ, theT: &Trsf) -> UniquePtr<Circ>;
        #[doc = "Translates a circle in the direction of the vector theV. The magnitude of the translation is the vector's magnitude."]
        #[cxx_name = "gp_Circ_Translated"]
        fn Circ_translatedvec(self_: &Circ, theV: &Vec_) -> UniquePtr<Circ>;
        #[doc = "Translates a circle from the point theP1 to the point theP2."]
        #[cxx_name = "gp_Circ_Translated"]
        fn Circ_translatedpnt_2(self_: &Circ, theP1: &Pnt, theP2: &Pnt) -> UniquePtr<Circ>;
        #[doc = " ======================== gp_Pln ========================"]
        #[doc = "Describes a plane. A plane is positioned in space with a coordinate system (a gp_Ax3 object), such that the plane is defined by the origin, \"X Direction\" and \"Y Direction\" of this coordinate system, which is the \"local coordinate system\" of the plane. The \"main Direction\" of the coordinate system is a vector normal to the plane. It gives the plane an implicit orientation such that the plane is said to be \"direct\", if the coordinate system is right-handed, or \"indirect\" in the other case. Note: when a gp_Pln plane is converted into a Geom_Plane plane, some implicit properties of its local coordinate system are used explicitly: -   its origin defines the origin of the two parameters of the planar surface, -   its implicit orientation is also that of the Geom_Plane. See Also gce_MakePln which provides functions for more complex plane constructions Geom_Plane which provides additional functions for constructing planes and works, in particular, with the parametric equations of planes"]
        #[cxx_name = "gp_Pln"]
        type Pln;
        #[doc = "Creates a plane coincident with OXY plane of the reference coordinate system."]
        #[cxx_name = "gp_Pln_ctor"]
        fn Pln_ctor() -> UniquePtr<Pln>;
        #[doc = "The coordinate system of the plane is defined with the axis placement theA3. The \"Direction\" of theA3 defines the normal to the plane. The \"Location\" of theA3 defines the location (origin) of the plane. The \"XDirection\" and \"YDirection\" of theA3 define the \"XAxis\" and the \"YAxis\" of the plane used to parametrize the plane."]
        #[cxx_name = "gp_Pln_ctor_ax3"]
        fn Pln_ctor_ax3(theA3: &Ax3) -> UniquePtr<Pln>;
        #[doc = "Creates a plane with the  \"Location\" point <theP> and the normal direction <theV>."]
        #[cxx_name = "gp_Pln_ctor_pnt_dir"]
        fn Pln_ctor_pnt_dir(theP: &Pnt, theV: &Dir) -> UniquePtr<Pln>;
        #[doc = "Creates a plane from its cartesian equation : @code theA * X + theB * Y + theC * Z + theD = 0.0 @endcode Raises ConstructionError if Sqrt (theA*theA + theB*theB + theC*theC) <= Resolution from gp."]
        #[cxx_name = "gp_Pln_ctor_real4"]
        fn Pln_ctor_real4(theA: f64, theB: f64, theC: f64, theD: f64) -> UniquePtr<Pln>;
        #[doc = "Returns the coefficients of the plane's cartesian equation : @code theA * X + theB * Y + theC * Z + theD = 0. @endcode"]
        #[cxx_name = "Coefficients"]
        fn coefficients(self: &Pln, theA: &mut f64, theB: &mut f64, theC: &mut f64, theD: &mut f64);
        #[doc = "Modifies this plane, by redefining its local coordinate system so that -   its origin and \"main Direction\" become those of the axis theA1 (the \"X Direction\" and \"Y Direction\" are then recomputed). Raises ConstructionError if the theA1 is parallel to the \"XAxis\" of the plane."]
        #[cxx_name = "SetAxis"]
        fn set_axis(self: Pin<&mut Pln>, theA1: &Ax1);
        #[doc = "Changes the origin of the plane."]
        #[cxx_name = "SetLocation"]
        fn set_location(self: Pin<&mut Pln>, theLoc: &Pnt);
        #[doc = "Changes the local coordinate system of the plane."]
        #[cxx_name = "SetPosition"]
        fn set_position(self: Pin<&mut Pln>, theA3: &Ax3);
        #[doc = "Reverses the   U   parametrization of   the  plane reversing the XAxis."]
        #[cxx_name = "UReverse"]
        fn u_reverse(self: Pin<&mut Pln>);
        #[doc = "Reverses the   V   parametrization of   the  plane reversing the YAxis."]
        #[cxx_name = "VReverse"]
        fn v_reverse(self: Pin<&mut Pln>);
        #[doc = "returns true if the Ax3 is right handed."]
        #[cxx_name = "Direct"]
        fn direct(self: &Pln) -> bool;
        #[doc = "Returns the plane's normal Axis."]
        #[cxx_name = "Axis"]
        fn axis(self: &Pln) -> &Ax1;
        #[doc = "Returns the plane's location (origin)."]
        #[cxx_name = "Location"]
        fn location(self: &Pln) -> &Pnt;
        #[doc = "Returns the local coordinate system of the plane ."]
        #[cxx_name = "Position"]
        fn position(self: &Pln) -> &Ax3;
        #[doc = "Computes the distance between <me> and the point <theP>."]
        #[cxx_name = "Distance"]
        fn distancepnt(self: &Pln, theP: &Pnt) -> f64;
        #[doc = "Computes the distance between <me> and the line <theL>."]
        #[cxx_name = "Distance"]
        fn distancelin_2(self: &Pln, theL: &Lin) -> f64;
        #[doc = "Computes the distance between two planes."]
        #[cxx_name = "Distance"]
        fn distancepln_3(self: &Pln, theOther: &Pln) -> f64;
        #[doc = "Computes the square distance between <me> and the point <theP>."]
        #[cxx_name = "SquareDistance"]
        fn square_distancepnt(self: &Pln, theP: &Pnt) -> f64;
        #[doc = "Computes the square distance between <me> and the line <theL>."]
        #[cxx_name = "SquareDistance"]
        fn square_distancelin_2(self: &Pln, theL: &Lin) -> f64;
        #[doc = "Computes the square distance between two planes."]
        #[cxx_name = "SquareDistance"]
        fn square_distancepln_3(self: &Pln, theOther: &Pln) -> f64;
        #[doc = "Returns true if this plane contains the point theP. This means that -   the distance between point theP and this plane is less than or equal to theLinearTolerance, or -   line L is normal to the \"main Axis\" of the local coordinate system of this plane, within the tolerance AngularTolerance, and the distance between the origin of line L and this plane is less than or equal to theLinearTolerance."]
        #[cxx_name = "Contains"]
        fn containspnt(self: &Pln, theP: &Pnt, theLinearTolerance: f64) -> bool;
        #[doc = "Returns true if this plane contains the line theL. This means that -   the distance between point P and this plane is less than or equal to LinearTolerance, or -   line theL is normal to the \"main Axis\" of the local coordinate system of this plane, within the tolerance theAngularTolerance, and the distance between the origin of line theL and this plane is less than or equal to theLinearTolerance."]
        #[cxx_name = "Contains"]
        fn containslin_2(
            self: &Pln,
            theL: &Lin,
            theLinearTolerance: f64,
            theAngularTolerance: f64,
        ) -> bool;
        #[cxx_name = "Mirror"]
        fn mirrorpnt(self: Pin<&mut Pln>, theP: &Pnt);
        #[cxx_name = "Mirror"]
        fn mirrorax1_2(self: Pin<&mut Pln>, theA1: &Ax1);
        #[cxx_name = "Mirror"]
        fn mirrorax2_3(self: Pin<&mut Pln>, theA2: &Ax2);
        #[cxx_name = "Rotate"]
        fn rotate(self: Pin<&mut Pln>, theA1: &Ax1, theAng: f64);
        #[cxx_name = "Scale"]
        fn scale(self: Pin<&mut Pln>, theP: &Pnt, theS: f64);
        #[cxx_name = "Transform"]
        fn transform(self: Pin<&mut Pln>, theT: &Trsf);
        #[cxx_name = "Translate"]
        fn translatevec(self: Pin<&mut Pln>, theV: &Vec_);
        #[cxx_name = "Translate"]
        fn translatepnt_2(self: Pin<&mut Pln>, theP1: &Pnt, theP2: &Pnt);
        #[doc = "Returns the X axis of the plane."]
        #[cxx_name = "gp_Pln_XAxis"]
        fn Pln_x_axis(self_: &Pln) -> UniquePtr<Ax1>;
        #[doc = "Returns the Y axis  of the plane."]
        #[cxx_name = "gp_Pln_YAxis"]
        fn Pln_y_axis(self_: &Pln) -> UniquePtr<Ax1>;
        #[doc = "Performs the symmetrical transformation of a plane with respect to the point <theP> which is the center of the symmetry Warnings : The normal direction to the plane is not changed. The \"XAxis\" and the \"YAxis\" are reversed."]
        #[cxx_name = "gp_Pln_Mirrored"]
        fn Pln_mirroredpnt(self_: &Pln, theP: &Pnt) -> UniquePtr<Pln>;
        #[doc = "Performs   the symmetrical transformation  of a plane with respect to an axis placement  which is the axis of  the symmetry.  The  transformation is performed on the \"Location\" point, on  the \"XAxis\"  and the \"YAxis\".    The resulting normal  direction  is  the cross product between the \"XDirection\" and the \"YDirection\" after transformation if  the  initial plane was right  handed,  else  it is the opposite."]
        #[cxx_name = "gp_Pln_Mirrored"]
        fn Pln_mirroredax1_2(self_: &Pln, theA1: &Ax1) -> UniquePtr<Pln>;
        #[doc = "Performs the  symmetrical transformation  of  a plane    with respect to    an axis  placement.   The axis placement  <A2> locates the plane  of  the symmetry.   The transformation is performed  on  the  \"Location\" point, on the  \"XAxis\" and  the    \"YAxis\".  The resulting    normal direction is the cross  product between   the \"XDirection\" and the \"YDirection\"  after  transformation if the initial plane was right handed, else it is the opposite."]
        #[cxx_name = "gp_Pln_Mirrored"]
        fn Pln_mirroredax2_3(self_: &Pln, theA2: &Ax2) -> UniquePtr<Pln>;
        #[doc = "rotates a plane. theA1 is the axis of the rotation. theAng is the angular value of the rotation in radians."]
        #[cxx_name = "gp_Pln_Rotated"]
        fn Pln_rotated(self_: &Pln, theA1: &Ax1, theAng: f64) -> UniquePtr<Pln>;
        #[doc = "Scales a plane. theS is the scaling value."]
        #[cxx_name = "gp_Pln_Scaled"]
        fn Pln_scaled(self_: &Pln, theP: &Pnt, theS: f64) -> UniquePtr<Pln>;
        #[doc = "Transforms a plane with the transformation theT from class Trsf. The transformation is performed on the \"Location\" point, on the \"XAxis\" and the \"YAxis\". The resulting normal direction is the cross product between the \"XDirection\" and the \"YDirection\" after transformation."]
        #[cxx_name = "gp_Pln_Transformed"]
        fn Pln_transformed(self_: &Pln, theT: &Trsf) -> UniquePtr<Pln>;
        #[doc = "Translates a plane in the direction of the vector theV. The magnitude of the translation is the vector's magnitude."]
        #[cxx_name = "gp_Pln_Translated"]
        fn Pln_translatedvec(self_: &Pln, theV: &Vec_) -> UniquePtr<Pln>;
        #[doc = "Translates a plane from the point theP1 to the point theP2."]
        #[cxx_name = "gp_Pln_Translated"]
        fn Pln_translatedpnt_2(self_: &Pln, theP1: &Pnt, theP2: &Pnt) -> UniquePtr<Pln>;
    }
    impl UniquePtr<Pnt> {}
    impl UniquePtr<Pnt2d> {}
    impl UniquePtr<Vec_> {}
    impl UniquePtr<Vec2d> {}
    impl UniquePtr<Dir> {}
    impl UniquePtr<Dir2d> {}
    impl UniquePtr<XYZ> {}
    impl UniquePtr<Ax1> {}
    impl UniquePtr<Ax2> {}
    impl UniquePtr<Ax2d> {}
    impl UniquePtr<Ax3> {}
    impl UniquePtr<Trsf> {}
    impl UniquePtr<Trsf2d> {}
    impl UniquePtr<GTrsf> {}
    impl UniquePtr<GTrsf2d> {}
    impl UniquePtr<Lin> {}
    impl UniquePtr<Circ> {}
    impl UniquePtr<Pln> {}
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
pub use ffi::Pnt2d;
impl Pnt2d {
    #[doc = "Creates a point with zero coordinates."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Pnt2d_ctor()
    }

    #[doc = "Creates a point with a doublet of coordinates."]
    pub fn new_xy(theCoord: &ffi::gp_XY) -> cxx::UniquePtr<Self> {
        ffi::Pnt2d_ctor_xy(theCoord)
    }

    #[doc = "Creates a  point with its 2 cartesian's coordinates : theXp, theYp."]
    pub fn new_real2(theXp: f64, theYp: f64) -> cxx::UniquePtr<Self> {
        ffi::Pnt2d_ctor_real2(theXp, theYp)
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
pub use ffi::Vec2d;
impl Vec2d {
    #[doc = "Creates a zero vector."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Vec2d_ctor()
    }

    #[doc = "Creates a unitary vector from a direction theV."]
    pub fn new_dir2d(theV: &ffi::Dir2d) -> cxx::UniquePtr<Self> {
        ffi::Vec2d_ctor_dir2d(theV)
    }

    #[doc = "Creates a vector with a doublet of coordinates."]
    pub fn new_xy(theCoord: &ffi::gp_XY) -> cxx::UniquePtr<Self> {
        ffi::Vec2d_ctor_xy(theCoord)
    }

    #[doc = "Creates a point with its two Cartesian coordinates."]
    pub fn new_real2(theXv: f64, theYv: f64) -> cxx::UniquePtr<Self> {
        ffi::Vec2d_ctor_real2(theXv, theYv)
    }

    #[doc = "Creates a vector from two points. The length of the vector is the distance between theP1 and theP2"]
    pub fn new_pnt2d2(theP1: &ffi::Pnt2d, theP2: &ffi::Pnt2d) -> cxx::UniquePtr<Self> {
        ffi::Vec2d_ctor_pnt2d2(theP1, theP2)
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
pub use ffi::Dir2d;
impl Dir2d {
    #[doc = "Creates a direction corresponding to X axis."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Dir2d_ctor()
    }

    #[doc = "Normalizes the vector theV and creates a Direction. Raises ConstructionError if theV.Magnitude() <= Resolution from gp."]
    pub fn new_vec2d(theV: &ffi::Vec2d) -> cxx::UniquePtr<Self> {
        ffi::Dir2d_ctor_vec2d(theV)
    }

    #[doc = "Creates a Direction from a doublet of coordinates. Raises ConstructionError if theCoord.Modulus() <= Resolution from gp."]
    pub fn new_xy(theCoord: &ffi::gp_XY) -> cxx::UniquePtr<Self> {
        ffi::Dir2d_ctor_xy(theCoord)
    }

    #[doc = "Creates a Direction with its 2 cartesian coordinates. Raises ConstructionError if Sqrt(theXv*theXv + theYv*theYv) <= Resolution from gp."]
    pub fn new_real2(theXv: f64, theYv: f64) -> cxx::UniquePtr<Self> {
        ffi::Dir2d_ctor_real2(theXv, theYv)
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
pub use ffi::Ax1;
impl Ax1 {
    #[doc = "Creates an axis object representing Z axis of the reference coordinate system."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Ax1_ctor()
    }

    #[doc = "P is the location point and V is the direction of <me>."]
    pub fn new_pnt_dir(theP: &ffi::Pnt, theV: &ffi::Dir) -> cxx::UniquePtr<Self> {
        ffi::Ax1_ctor_pnt_dir(theP, theV)
    }
}
pub use ffi::Ax2;
impl Ax2 {
    #[doc = "Creates an object corresponding to the reference coordinate system (OXYZ)."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Ax2_ctor()
    }

    #[doc = "Creates an axis placement with an origin P such that: -   N is the Direction, and -   the \"X Direction\" is normal to N, in the plane defined by the vectors (N, Vx): \"X Direction\" = (N ^ Vx) ^ N, Exception: raises ConstructionError if N and Vx are parallel (same or opposite orientation)."]
    pub fn new_pnt_dir2(P: &ffi::Pnt, N: &ffi::Dir, Vx: &ffi::Dir) -> cxx::UniquePtr<Self> {
        ffi::Ax2_ctor_pnt_dir2(P, N, Vx)
    }

    #[doc = "Creates -   a coordinate system with an origin P, where V gives the \"main Direction\" (here, \"X Direction\" and \"Y Direction\" are defined automatically)."]
    pub fn new_pnt_dir(P: &ffi::Pnt, V: &ffi::Dir) -> cxx::UniquePtr<Self> {
        ffi::Ax2_ctor_pnt_dir(P, V)
    }
}
pub use ffi::Ax2d;
impl Ax2d {
    #[doc = "Creates an axis object representing X axis of the reference co-ordinate system."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Ax2d_ctor()
    }

    #[doc = "Creates an Ax2d. <theP> is the \"Location\" point of the axis placement and theV is the \"Direction\" of the axis placement."]
    pub fn new_pnt2d_dir2d(theP: &ffi::Pnt2d, theV: &ffi::Dir2d) -> cxx::UniquePtr<Self> {
        ffi::Ax2d_ctor_pnt2d_dir2d(theP, theV)
    }
}
pub use ffi::Ax3;
impl Ax3 {
    #[doc = "Creates an object corresponding to the reference coordinate system (OXYZ)."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Ax3_ctor()
    }

    #[doc = "Creates  a  coordinate  system from a right-handed coordinate system."]
    pub fn new_ax2(theA: &ffi::Ax2) -> cxx::UniquePtr<Self> {
        ffi::Ax3_ctor_ax2(theA)
    }

    #[doc = "Creates a  right handed axis placement with the \"Location\" point theP and  two directions, theN gives the \"Direction\" and theVx gives the \"XDirection\". Raises ConstructionError if theN and theVx are parallel (same or opposite orientation)."]
    pub fn new_pnt_dir2(
        theP: &ffi::Pnt,
        theN: &ffi::Dir,
        theVx: &ffi::Dir,
    ) -> cxx::UniquePtr<Self> {
        ffi::Ax3_ctor_pnt_dir2(theP, theN, theVx)
    }

    #[doc = "Creates an axis placement with the \"Location\" point <theP> and the normal direction <theV>."]
    pub fn new_pnt_dir(theP: &ffi::Pnt, theV: &ffi::Dir) -> cxx::UniquePtr<Self> {
        ffi::Ax3_ctor_pnt_dir(theP, theV)
    }
}
pub use ffi::Trsf;
impl Trsf {
    #[doc = "Returns the identity transformation."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Trsf_ctor()
    }

    #[doc = "Creates  a 3D transformation from the 2D transformation theT. The resulting transformation has a homogeneous vectorial part, V3, and a translation part, T3, built from theT: a11    a12 0             a13 V3 =    a21    a22    0       T3 =   a23 0    0    1. 0 It also has the same scale factor as theT. This guarantees (by projection) that the transformation which would be performed by theT in a plane (2D space) is performed by the resulting transformation in the xOy plane of the 3D space, (i.e. in the plane defined by the origin (0., 0., 0.) and the vectors DX (1., 0., 0.), and DY (0., 1., 0.)). The scale factor is applied to the entire space."]
    pub fn new_trsf2d(theT: &ffi::Trsf2d) -> cxx::UniquePtr<Self> {
        ffi::Trsf_ctor_trsf2d(theT)
    }
}
pub use ffi::Trsf2d;
impl Trsf2d {
    #[doc = "Returns identity transformation."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Trsf2d_ctor()
    }

    #[doc = "Creates a 2d transformation in the XY plane from a 3d transformation ."]
    pub fn new_trsf(theT: &ffi::Trsf) -> cxx::UniquePtr<Self> {
        ffi::Trsf2d_ctor_trsf(theT)
    }
}
pub use ffi::GTrsf;
impl GTrsf {
    #[doc = "Returns the Identity transformation."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::GTrsf_ctor()
    }

    #[doc = "Converts the gp_Trsf transformation theT into a general transformation, i.e. Returns a GTrsf with the same matrix of coefficients as the Trsf theT."]
    pub fn new_trsf(theT: &ffi::Trsf) -> cxx::UniquePtr<Self> {
        ffi::GTrsf_ctor_trsf(theT)
    }

    #[doc = "Creates a transformation based on the matrix theM and the vector theV where theM defines the vectorial part of the transformation, and V the translation part, or"]
    pub fn new_mat_xyz(theM: &ffi::gp_Mat, theV: &ffi::XYZ) -> cxx::UniquePtr<Self> {
        ffi::GTrsf_ctor_mat_xyz(theM, theV)
    }
}
pub use ffi::GTrsf2d;
impl GTrsf2d {
    #[doc = "returns identity transformation."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::GTrsf2d_ctor()
    }

    #[doc = "Converts the gp_Trsf2d transformation theT into a general transformation."]
    pub fn new_trsf2d(theT: &ffi::Trsf2d) -> cxx::UniquePtr<Self> {
        ffi::GTrsf2d_ctor_trsf2d(theT)
    }

    #[doc = "Creates   a transformation based on the matrix theM and the vector theV where theM defines the vectorial part of the transformation, and theV the translation part."]
    pub fn new_mat2d_xy(theM: &ffi::gp_Mat2d, theV: &ffi::gp_XY) -> cxx::UniquePtr<Self> {
        ffi::GTrsf2d_ctor_mat2d_xy(theM, theV)
    }
}
pub use ffi::Lin;
impl Lin {
    #[doc = "Creates a Line corresponding to Z axis of the reference coordinate system."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Lin_ctor()
    }

    #[doc = "Creates a line defined by axis theA1."]
    pub fn new_ax1(theA1: &ffi::Ax1) -> cxx::UniquePtr<Self> {
        ffi::Lin_ctor_ax1(theA1)
    }

    #[doc = "Creates a line passing through point theP and parallel to vector theV (theP and theV are, respectively, the origin and the unit vector of the positioning axis of the line)."]
    pub fn new_pnt_dir(theP: &ffi::Pnt, theV: &ffi::Dir) -> cxx::UniquePtr<Self> {
        ffi::Lin_ctor_pnt_dir(theP, theV)
    }
}
pub use ffi::Circ;
impl Circ {
    #[doc = "Creates an indefinite circle."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Circ_ctor()
    }

    #[doc = "A2 locates the circle and gives its orientation in 3D space. Warnings : It is not forbidden to create a circle with theRadius = 0.0  Raises ConstructionError if theRadius < 0.0"]
    pub fn new_ax2_real(theA2: &ffi::Ax2, theRadius: f64) -> cxx::UniquePtr<Self> {
        ffi::Circ_ctor_ax2_real(theA2, theRadius)
    }
}
pub use ffi::Pln;
impl Pln {
    #[doc = "Creates a plane coincident with OXY plane of the reference coordinate system."]
    pub fn new() -> cxx::UniquePtr<Self> {
        ffi::Pln_ctor()
    }

    #[doc = "The coordinate system of the plane is defined with the axis placement theA3. The \"Direction\" of theA3 defines the normal to the plane. The \"Location\" of theA3 defines the location (origin) of the plane. The \"XDirection\" and \"YDirection\" of theA3 define the \"XAxis\" and the \"YAxis\" of the plane used to parametrize the plane."]
    pub fn new_ax3(theA3: &ffi::Ax3) -> cxx::UniquePtr<Self> {
        ffi::Pln_ctor_ax3(theA3)
    }

    #[doc = "Creates a plane with the  \"Location\" point <theP> and the normal direction <theV>."]
    pub fn new_pnt_dir(theP: &ffi::Pnt, theV: &ffi::Dir) -> cxx::UniquePtr<Self> {
        ffi::Pln_ctor_pnt_dir(theP, theV)
    }

    #[doc = "Creates a plane from its cartesian equation : @code theA * X + theB * Y + theC * Z + theD = 0.0 @endcode Raises ConstructionError if Sqrt (theA*theA + theB*theB + theC*theC) <= Resolution from gp."]
    pub fn new_real4(theA: f64, theB: f64, theC: f64, theD: f64) -> cxx::UniquePtr<Self> {
        ffi::Pln_ctor_real4(theA, theB, theC, theD)
    }
}
