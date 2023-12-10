pub mod parallel;
pub mod png_io;
pub mod examples {
    pub mod chapter6;
    pub mod chapter7;
    pub mod chapter8;
}

fn main() {
    examples::chapter8::build_example();
}
