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
pub mod yaml_parsing {
    pub mod yaml;
}

fn main() {
    examples::appendix1::parse_yaml();
}
