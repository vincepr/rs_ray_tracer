pub mod cmp;
pub mod io {
    pub mod ppm;
}
#[allow(dead_code)]
pub mod mathstructs {
    pub mod matrix;
    pub mod matrix_ext;
    pub mod point;
    pub mod vector;
}

#[allow(dead_code)]
pub mod ray;

pub mod object;

pub mod visual {
    pub mod camera;
    pub mod canvas;
    pub mod color;
    pub mod light;
    pub mod material;
    pub mod patterns;
    pub mod world;
}
