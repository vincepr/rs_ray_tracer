use crate::{mathstructs::matrix::Matrix, ray::intersects::IntersectsRay};

use super::sphere::Sphere;

#[derive(Debug, PartialEq)]
pub enum Shape {
    Sphere, // Sphere has no state so i guess no need to actually wrap the shape-struct here
}

/// wrapper that represents a shape like a Sphere and applied transformations etc.
#[derive(Debug, PartialEq)]
pub struct Object {
    shape: Shape,
    pub transformation: Matrix,
}

impl IntersectsRay for Object {
    fn intersect(&self, ray: &crate::ray::Ray) -> Option<(f32, f32)> {
        match &self.shape {
            Shape::Sphere => Sphere {}.intersect(ray),
        }
    }
}

impl Object {
    pub fn new(shape: Shape) -> Self {
        Self {
            shape,
            ..Default::default()
        }
    }

    pub fn set_transform(&mut self, m: Matrix) {
        self.transformation = m;
    }
}

impl Default for Object {
    fn default() -> Self {
        Self {
            shape: Shape::Sphere,
            transformation: Matrix::new_identity(),
        }
    }
}
