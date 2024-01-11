pub mod parallel;
pub mod png_io;
pub mod examples {
    pub mod chapter10;
    pub mod chapter11;
    pub mod chapter6;
    pub mod chapter7;
    pub mod chapter8;
}

fn main() {
    examples::chapter11::build_example();
}
