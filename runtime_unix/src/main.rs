pub mod parallel;
pub mod png_io;
pub mod examples {
    pub mod chapter10;
    pub mod chapter11;
    pub mod chapter6;
    pub mod chapter7;
    pub mod chapter8;
    pub mod appendix1;
}

fn main() {
    examples::appendix1::parse_yaml();
}
