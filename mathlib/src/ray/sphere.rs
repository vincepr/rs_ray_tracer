use crate::mathstructs::point::Point;

use super::{intersects::IntersectsRay, Ray};

#[derive(Debug, PartialEq)]
pub struct Sphere {}

/// sphere at origin of radius 1
pub fn sphere() -> Sphere {
    Sphere {}
}

impl Sphere {
    pub fn new() -> Self {
        Self {}
    }
}
impl Default for Sphere {
    fn default() -> Self {
        Self::new()
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
    use crate::mathstructs::{point::Point, vector::Vector};

    use super::*;
    #[test]
    fn ray_intersects_a_sphere_at_two_points() {
        let ray = Ray::new(Point::newi(0, 0, -5), Vector::newi(0, 0, 1));
        let sphere = sphere();
        let intersects = sphere.intersect(&ray).unwrap();
        assert_eq!(intersects, (4.0, 6.0));
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let ray = Ray::new(Point::newi(0, 1, -5), Vector::newi(0, 0, 1));
        let sphere = sphere();
        let intersects = sphere.intersect(&ray).unwrap();
        assert_eq!(intersects, (5.0, 5.0));
    }

    #[test]
    fn ray_misses_a_sphere() {
        let ray = Ray::new(Point::newi(0, 2, -5), Vector::newi(0, 0, 1));
        let sphere = sphere();
        let intersects = sphere.intersect(&ray);
        assert_eq!(intersects, None);
    }

    #[test]
    fn ray_originates_inside_a_sphere() {
        let ray = Ray::new(Point::newi(0, 0, 0), Vector::newi(0, 0, 1));
        let sphere = sphere();
        let intersects = sphere.intersect(&ray).unwrap();
        assert_eq!(intersects, (-1.0, 1.0));
    }

    #[test]
    fn ray_starts_after_sphere() {
        let ray = Ray::new(Point::newi(0, 0, 5), Vector::newi(0, 0, 1));
        let sphere = sphere();
        let intersects = sphere.intersect(&ray).unwrap();
        assert_eq!(intersects.0, -6.0);
        assert_eq!(intersects.1, -4.0);
        assert_eq!(intersects, (-6.0, -4.0));
    }
}
