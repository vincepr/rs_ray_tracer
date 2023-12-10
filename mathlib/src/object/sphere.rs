use crate::{
    mathstructs::{point::Point, vector::Vector},
    ray::{intersects::IntersectsRay, Ray},
};

use super::{Object, Shape};

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
    /// the t value of the position on the ray where the intersections happen. 0, 1, 2 possible.
    fn intersect_raw(&self, ray: &Ray) -> Option<(f64, f64)> {
        let sphere_to_ray = ray.origin - Point::new(0.0, 0.0, 0.0);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
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

    /// points perpendicular to the surface of the sphere
    fn normal_at(point: Point) -> Vector {
        point - Point::new_origin() // .normalize() not neccessary as long as we assume unit-sphere
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::mathstructs::matrix::Matrix;

    use super::*;
    #[test]
    fn ray_intersects_a_sphere_at_two_points() {
        let ray = Ray::new(Point::inew(0, 0, -5), Vector::inew(0, 0, 1));
        let sphere = Sphere::new();
        let intersects = sphere.intersect_raw(&ray).unwrap();
        assert_eq!(intersects, (4.0, 6.0));
    }

    #[test]
    fn ray_intersects_sphere_at_a_tangent() {
        let ray = Ray::new(Point::inew(0, 1, -5), Vector::inew(0, 0, 1));
        let sphere = Sphere::new();
        let intersects = sphere.intersect_raw(&ray).unwrap();
        assert_eq!(intersects, (5.0, 5.0));
    }

    #[test]
    fn ray_misses_a_sphere() {
        let ray = Ray::new(Point::inew(0, 2, -5), Vector::inew(0, 0, 1));
        let sphere = Sphere::new();
        let intersects = sphere.intersect_raw(&ray);
        assert_eq!(intersects, None);
    }

    #[test]
    fn ray_originates_inside_a_sphere() {
        let ray = Ray::new(Point::inew(0, 0, 0), Vector::inew(0, 0, 1));
        let sphere = Sphere::new();
        let intersects = sphere.intersect_raw(&ray).unwrap();
        assert_eq!(intersects, (-1.0, 1.0));
    }

    #[test]
    fn ray_starts_after_sphere() {
        let ray = Ray::new(Point::inew(0, 0, 5), Vector::inew(0, 0, 1));
        let sphere = Sphere::new();
        let intersects = sphere.intersect_raw(&ray).unwrap();
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
        let xs = s.intersect_raw(&r);
        assert!(xs.is_some());
        assert_eq!(xs, Some((3.0, 7.0)));
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Point::inew(0, 0, -5), Vector::inew(0, 0, 1));
        let mut s = Sphere::new();
        s.set_transform(Matrix::translation_new(5.0, 0.0, 0.0));
        let xs = s.intersect_raw(&r);
        dbg!(xs);
        assert!(!xs.is_some())
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        let res = s.normal_at(&Point::inew(1, 0, 0));
        assert_eq!(res, Vector::inew(1, 0, 0));
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();
        let res = s.normal_at(&Point::inew(0, 1, 0));
        assert_eq!(res, Vector::inew(0, 1, 0));
    }

    #[test]
    fn normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();
        let res = s.normal_at(&Point::inew(0, 0, 1));
        assert_eq!(res, Vector::inew(0, 0, 1));
    }

    #[test]
    fn normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::new();
        let sq = 3.0_f64.sqrt() / 3.0;
        let res = s.normal_at(&Point::new(sq, sq, sq));
        assert_eq!(res, Vector::new(sq, sq, sq));
    }

    #[test]
    fn normal_on_a_sphere_is_a_normalized_vector() {
        let s = Sphere::new();
        let sq = 3.0_f64.sqrt() / 3.0;
        let res = s.normal_at(&Point::new(sq, sq, sq));
        let norm_res = res.clone().normalize();
        assert_eq!(res, norm_res);
    }

    #[test]
    fn computing_normal_on_a_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(Matrix::translation_new(0.0, 1.0, 0.0));
        let res = s.normal_at(&Point::new(0.0, 1.70711, -0.70711));
        assert_eq!(res, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_normal_on_a_transformed_sphere() {
        let mut s = Sphere::new();
        s.set_transform(Matrix::scaling_new(1.0, 0.5, 1.0) * Matrix::rotation_z_new(PI / 5.0));
        let sq = 2.0_f64.sqrt() / 2.0;
        let res = s.normal_at(&Point::new(0.0, sq, sq));
        assert_eq!(res, Vector::new(0.0, 0.97014, 0.24254));
    }
}
