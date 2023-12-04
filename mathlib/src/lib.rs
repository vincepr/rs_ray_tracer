pub mod cmp;
pub mod io {
    pub mod png;
    pub mod ppm;
}
#[allow(dead_code)]
pub mod mathstructs {
    pub mod matrix;
    pub mod matrix_ext;
    pub mod point;
    pub mod vector;
}
pub mod objects {
    pub mod material;
    pub mod object;
    pub mod sphere;
}
#[allow(dead_code)]
pub mod ray;
pub mod visual {
    pub mod canvas;
    pub mod color;
    pub mod light;
}
