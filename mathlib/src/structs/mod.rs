/// (x,y,z,0) - vector
/// (x,y,z,1) - point
#[derive(Debug)]
pub struct Tupl{
    x: f32,
    y: f32,
    z: f32,
    w: isize,
}
impl Tupl {
    pub fn new(x: f32,y: f32, z:f32, w: isize) -> Self {
        Tupl { x, y, z, w }
    }

    pub fn new_vector(x: f32, y: f32, z:f32) -> Self {
        Tupl::new(x, y, z, 0)
    }

    pub fn new_point(x: f32, y: f32, z:f32) -> Self {
        Tupl::new(x, y, z, 1)
    }
}
