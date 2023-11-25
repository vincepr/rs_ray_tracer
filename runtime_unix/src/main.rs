use mathlib::structs::Tupl;


fn main() {
    println!("Hello, world!");
    let t = Tupl::new_point(1.0, 2.0, 3.0);
    println!("t is currently: {t:?}");
}