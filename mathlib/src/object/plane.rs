use crate::{ray::intersects::IntersectsRay, mathstructs::{vector::Vector, point::Point}, cmp::EPSILON};

use super::{Object, Shape};

/// Like Sphere plane doesnt hold any state itself and is only there for consistency/structure
/// - the unit plane is infinite in both x and z.
pub struct Plane {}

impl Plane {
    pub fn new() -> Object {
        Object::new(Shape::Plane)
    }
}

impl IntersectsRay for Plane {
    // 
    fn intersect_raw(&self, ray: &crate::ray::Ray) -> Option<(f64, f64)> {
        // ray is parallel to the plance
        if ray.direction.y.abs() < EPSILON {
            return None;
        }
        let t = -ray.origin.y / ray.direction.y;
        Some((t, t))    // Note: book returns only one here. length 1. might have to rewrite later
    }

    // since a plane has no curvature it's always the same:
    fn normal_at(_point: Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {


    use crate::{mathstructs::vector::Vector, ray::{Ray, intersects::VecIntersections}};

    use super::*;
    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::new();
        let n1 = p.normal_at(&Point::inew(0, 0, 0));
        let n2 = p.normal_at(&Point::inew(10, 0, -10));
        let n3 = p.normal_at(&Point::inew(-5, 0, 150));
        assert_eq!(n1, Vector::inew(0, 1, 0));
        assert_eq!(n2, Vector::inew(0, 1, 0));
        assert_eq!(n3, Vector::inew(0, 1, 0));
    }

    #[test]
    fn intersect_with_ray_parallel_to_the_plane() {
        let ray = Ray::new(Point::inew(0, 10, 0), Vector::inew(0,0,1));
        let xs = &Plane{}.intersect_raw(&ray);
        assert_eq!(xs, &None);
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let ray = Ray::new(Point::inew(0, 0, 0), Vector::inew(0,0,1));
        let xs = &Plane{}.intersect_raw(&ray);
        assert_eq!(xs, &None);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let ray = Ray::new(Point::inew(0, 1, 0), Vector::inew(0,-1, 0));
        let xs = &Plane{}.intersect_raw(&ray);
        assert_eq!(*xs, Some((1.0, 1.0)));
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let ray = Ray::new(Point::inew(0, -1, 0), Vector::inew(0,1, 0));
        let xs = &Plane{}.intersect_raw(&ray);
        assert_eq!(*xs, Some((1.0, 1.0)));

        // comparing the reference reference
        let obj = Plane::new();
        let mut inters = VecIntersections::new();
        inters.intersect_add(&ray, &obj);
        let hit = inters.hit().unwrap().object;
        assert_eq!(&obj as *const _, hit as *const _); // again comparing pointer adresses
    }
}