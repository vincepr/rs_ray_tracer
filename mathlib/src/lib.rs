pub mod cmp;
pub mod io{
    pub mod ppm;
    pub mod png;
}
pub mod mathstructs {
    pub mod point;
    pub mod vector;
    #[allow(dead_code)] // TODO: remove this after every function gets used (and make others private)
    pub mod matrix;
    pub mod matrix_ext;
}
pub mod objects {
    pub mod object;
    pub mod sphere;
}
pub mod ray;
pub mod visual {
    pub mod canvas;
    pub mod color;
}