use crate::{
    mathstructs::{matrix::Matrix, point::Point, vector::Vector},
    ray::{
        intersects::{IntersectsRay},
        Ray,
    },
};

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

impl Object {
    pub fn intersect_raw(&self, ray: &Ray) -> Option<(f32, f32)> {
        // to translate from worldspace to objectspace - aka swap choordinate-system
        // we transform the ray itself by the inverse of the .transformation Matrix
        let ray = ray.transform(&self.transformation.inverse());

        match &self.shape {
            Shape::Sphere => Sphere {}.intersect_raw(&ray),
        }
    }

    #[allow(clippy::let_and_return)]
    /// gets point perpendicular to surface.
    pub fn normal_at(&self, world_point: &Point) -> Vector {
        // transform to object's choordinate system
        let object_point = self.world_to_obj(*world_point);
        // do the shape's normal_at implementation
        let object_normal = match &self.shape {
            Shape::Sphere => Sphere::normal_at(object_point),
        };
        // transform back to world choordinates:
        let world_normal = self.obj_to_world(object_normal).normalize();
        world_normal
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

    /// translates between choordinate systems. World_point to object_point
    fn world_to_obj(&self, world_point: Point) -> Point {
        self.transformation.inverse() * world_point
    }

    /// translates between choordinate systems. Object_vector to world_vector
    fn obj_to_world(&self, object_normal: Vector) -> Vector {
        (self.transformation.inverse()).transpose() * object_normal
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


#[cfg(test)]
mod tests {
}