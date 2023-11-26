pub mod point;
#[derive(Debug)]
pub struct Point{
    x: f32,
    y: f32,
    z: f32,
}
impl Point {
    pub fn new(x: f32,y: f32, z:f32) -> Self {
        Self { x, y, z }
    }

    pub fn w() -> u8 {
        1
    }
}