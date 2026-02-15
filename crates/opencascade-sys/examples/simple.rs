pub fn main() {
    let point = opencascade_sys::gp::Pnt::new_real3(10.0, 7.0, 23.5);
    let y = point.y();
    println!("The point's Y value is {y}");
}
