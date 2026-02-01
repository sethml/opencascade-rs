//! Rust port of OCCT sample: GeometrySamples::PointInfo3dSample
//!
//! Original C++ source:
//! crates/occt-sys/OCCT/samples/OCCTOverview/code/GeometrySamples.cxx
//! Lines 1079-1122
//!
//! This sample demonstrates basic gp_Pnt operations:
//! - Creating points with default and coordinate constructors
//! - Accessing coordinates via X(), Y(), Z()
//! - Comparing points with IsEqual()
//! - Computing distances with Distance() and SquareDistance()

use opencascade_sys::gp::Pnt;

fn main() {
    // gp_Pnt aPnt1;
    let pnt1 = Pnt::new();

    // gp_Pnt aPnt2(10.0, 10.0, 10.0);
    let pnt2 = Pnt::new_real3(10.0, 10.0, 10.0);

    // gp_Pnt aPnt3(10.0, -10.0, 0.0);
    let pnt3 = Pnt::new_real3(10.0, -10.0, 0.0);

    // gp_Pnt aPnt4(10.0, 10.0, 10.0);
    let pnt4 = Pnt::new_real3(10.0, 10.0, 10.0);

    // Standard_Boolean anIsEqual2_3 = aPnt2.IsEqual(aPnt3, 1E-6);
    let is_equal_2_3 = pnt2.is_equal(&pnt3, 1e-6);

    // Standard_Boolean anIsEqual2_4 = aPnt2.IsEqual(aPnt4, 1E-6);
    let is_equal_2_4 = pnt2.is_equal(&pnt4, 1e-6);

    // Standard_Real aDistance1_2 = aPnt1.Distance(aPnt2);
    let distance_1_2 = pnt1.distance(&pnt2);

    // Standard_Real aDistance2_4 = aPnt2.Distance(aPnt4);
    let distance_2_4 = pnt2.distance(&pnt4);

    // Standard_Real aSquareDistance1_2 = aPnt1.SquareDistance(aPnt2);
    let square_distance_1_2 = pnt1.square_distance(&pnt2);

    // Standard_Real aSquareDistance2_4 = aPnt2.SquareDistance(aPnt4);
    let square_distance_2_4 = pnt2.square_distance(&pnt4);

    // myResult << "A coordinate of a point 1: X: " << aPnt1.X() << " Y: " << aPnt1.Y() << " Z: " << aPnt1.Z() << std::endl;
    println!(
        "A coordinate of a point 1: X: {} Y: {} Z: {}",
        pnt1.x(),
        pnt1.y(),
        pnt1.z()
    );

    // myResult << "A coordinate of a point 2: X: " << aPnt2.X() << " Y: " << aPnt2.Y() << " Z: " << aPnt2.Z() << std::endl;
    println!(
        "A coordinate of a point 2: X: {} Y: {} Z: {}",
        pnt2.x(),
        pnt2.y(),
        pnt2.z()
    );

    // myResult << "A coordinate of a point 3: X: " << aPnt3.X() << " Y: " << aPnt3.Y() << " Z: " << aPnt3.Z() << std::endl;
    println!(
        "A coordinate of a point 3: X: {} Y: {} Z: {}",
        pnt3.x(),
        pnt3.y(),
        pnt3.z()
    );

    // myResult << "A coordinate of a point 4: X: " << aPnt4.X() << " Y: " << aPnt4.Y() << " Z: " << aPnt4.Z() << std::endl;
    println!(
        "A coordinate of a point 4: X: {} Y: {} Z: {}",
        pnt4.x(),
        pnt4.y(),
        pnt4.z()
    );

    // if (anIsEqual2_3) { myResult << "A point 2 is equal to a point 3" << std::endl; }
    // else { myResult << "A point 2 is different from a point 3" << std::endl; }
    if is_equal_2_3 {
        println!("A point 2 is equal to a point 3");
    } else {
        println!("A point 2 is different from a point 3");
    }

    // if (anIsEqual2_4) { myResult << "A point 2 is equal to a point 4" << std::endl; }
    // else { myResult << "A point 2 is different from a point 4" << std::endl; }
    if is_equal_2_4 {
        println!("A point 2 is equal to a point 4");
    } else {
        println!("A point 2 is different from a point 4");
    }

    // myResult << "A distance from a point 1 to a point 2 is: " << aDistance1_2 << std::endl;
    println!("A distance from a point 1 to a point 2 is: {}", distance_1_2);

    // myResult << "A distance from a point 2 to a point 4 is: " << aDistance2_4 << std::endl;
    println!("A distance from a point 2 to a point 4 is: {}", distance_2_4);

    // myResult << "A square distance from a point 1 to a point 2 is: " << aSquareDistance1_2 << std::endl;
    println!(
        "A square distance from a point 1 to a point 2 is: {}",
        square_distance_1_2
    );

    // myResult << "A square distance from a point 2 to a point 4 is: " << aSquareDistance2_4 << std::endl;
    println!(
        "A square distance from a point 2 to a point 4 is: {}",
        square_distance_2_4
    );
}
