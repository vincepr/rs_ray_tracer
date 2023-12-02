use crate::{
    mathstructs::point::Point,
    ray::{intersects::IntersectsRay, Ray},
};

use super::object::{Object, Shape};

/// always of radius 1 and at (0.0.0) - has no real state so not really a struct tbh
#[derive(Debug, PartialEq)]
pub struct Sphere {}

impl Sphere {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Object {
        Object::new(Shape::Sphere)
    }
}

impl IntersectsRay for Sphere {
    // the t value of the position on the ray where the intersections happen. 0, 1, 2 possible.
    fn intersect(&self, ray: &Ray) -> Option<(f32, f32)> {
        let sphere_to_ray = ray.ori - Point::new(0.0, 0.0, 0.0);
        let a = ray.dir.dot(&ray.dir);
        let b = 2.0 * ray.dir.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let disc = b * b - 4.0 * a * c;
        // if disc < 0 => ray not hitting the sphere
        if disc < 0.0 {
            return None;
        }
        // else we hit it twice (or "twice on the same spot")
        let t1 = (-b - disc.sqrt()) / (2.0 * a);
        let t2 = (-b + disc.sqrt()) / (2.0 * a);
        Some((t1, t2))
    }
}

// impl Sphere {
//     /// calculates intersection then adds to collection
//     pub fn intersect_add(&self, ray: &Ray, coll: &VecIntersections) {
//         coll.intersections(self.intersect(ray), &self);
//     }
// }

#[cfg(test)]
mod tests {
    use crate::mathstructs::{vector::Vector, matrix::Matrix};

    use super::*;
    #[test]
    fn ray_intersects_a_sphere_at_two_points() {
        let ray = Ray::new(Point::inew(0, 0, -5), Vector::inew(0, 0, 1));
        let sphere = Sphere::new();
        let intersects = sphere.intersect(&ray).unwrap();
        assert_eq!(intersects, (4.0, 6.0));
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let ray = Ray::new(Point::inew(0, 1, -5), Vector::inew(0, 0, 1));
        let sphere = Sphere::new();
        let intersects = sphere.intersect(&ray).unwrap();
        assert_eq!(intersects, (5.0, 5.0));
    }

    #[test]
    fn ray_misses_a_sphere() {
        let ray = Ray::new(Point::inew(0, 2, -5), Vector::inew(0, 0, 1));
        let sphere = Sphere::new();
        let intersects = sphere.intersect(&ray);
        assert_eq!(intersects, None);
    }

    #[test]
    fn ray_originates_inside_a_sphere() {
        let ray = Ray::new(Point::inew(0, 0, 0), Vector::inew(0, 0, 1));
        let sphere = Sphere::new();
        let intersects = sphere.intersect(&ray).unwrap();
        assert_eq!(intersects, (-1.0, 1.0));
    }

    #[test]
    fn ray_starts_after_sphere() {
        let ray = Ray::new(Point::inew(0, 0, 5), Vector::inew(0, 0, 1));
        let sphere = Sphere::new();
        let intersects = sphere.intersect(&ray).unwrap();
        assert_eq!(intersects.0, -6.0);
        assert_eq!(intersects.1, -4.0);
        assert_eq!(intersects, (-6.0, -4.0));
    }

    #[test]
    fn sphere_s_default_transformation_is_identity() {
        let s = Sphere::new();
        assert_eq!(s.transformation, Matrix::new_identity());
    }

    #[test]
    fn changing_a_sphere_s_transformation() {
        let mut s = Sphere::new();
        let t = Matrix::translation_new(2.0, 3.0, 4.0);
        s.set_transform(t);
        assert_eq!(s.transformation, t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Point::inew(0, 0, -5), Vector::inew(0, 0, 1));
        let mut s = Sphere::new();
        s.set_transform(Matrix::scaling_new(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert!(xs.is_some());
        assert_eq!(xs, Some((3.0, 7.0)));
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Point::inew(0, 0, -5), Vector::inew(0, 0, 1));
        let mut s = Sphere::new();
        s.set_transform(Matrix::translation_new(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        dbg!(xs);
        assert!(!xs.is_some())
    }
}
