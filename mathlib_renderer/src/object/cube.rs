use std::f64::INFINITY;

use crate::{
    cmp::EPSILON,
    mathstructs::{point::Point, vector::Vector},
    ray::{intersects::IntersectsRay, Ray},
};

use super::{Object, Shape};

pub struct Cube {}

impl IntersectsRay for Cube {
    fn intersect_raw(&self, ray: &Ray) -> Option<(f64, f64)> {
        let (x_tmin, x_tmax) = Self::check_axis(ray.origin.x, ray.direction.x);
        let (y_tmin, y_tmax) = Self::check_axis(ray.origin.y, ray.direction.y);
        let (z_tmin, z_tmax) = Self::check_axis(ray.origin.z, ray.direction.z);
        let tmin = f64::max(f64::max(x_tmin, y_tmin), z_tmin);
        let tmax = f64::min(f64::min(x_tmax, y_tmax), z_tmax);

        match tmin > tmax {
            true => None,
            false => Some((tmin, tmax)),
        }
    }

    // (1.0,0.3,0.6) -> the value with 1.0 is direction of our normal.
    // BUT because f64 we better check absolute for max here. 1.0 might rounding-error
    fn normal_at(point: Point) -> Vector {
        let maxc = f64::max(f64::max(point.x.abs(), point.y.abs()), point.z.abs());
        match maxc {
            n if n == point.x.abs() => Vector::new(point.x, 0., 0.),
            n if n == point.y.abs() => Vector::new(0., point.y, 0.),
            _ => Vector::new(0., 0., point.z),
        }
    }
}

impl Cube {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Object {
        Object::new(Shape::Cube)
    }

    /// returns ( smallest minimun, largest maximum ) on that axis
    fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1. - origin;
        let tmax_numerator = 1. - origin;

        let (mut tmin, mut tmax) = if direction.abs() >= EPSILON {
            (tmin_numerator / direction, tmax_numerator / direction)
        } else {
            // cant divide by 0 -> +/- infinity will handle it on f64::max min
            (tmin_numerator * INFINITY, tmax_numerator * INFINITY)
        };

        if tmin > tmax {
            (tmin, tmax) = (tmax, tmin);
        }
        (tmin, tmax)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mathstructs::{point::Point, vector::Vector},
        ray::Ray,
    };

    use super::*;

    fn expect_intersects(origin: Point, direction: Vector, t1: f64, t2: f64) {
        let c = Cube::new();
        let ray = Ray::new(origin, direction);
        let xs = c.intersect_raw(&ray).unwrap();
        assert_eq!(xs, (t1, t2));
    }

    #[test]
    fn ray_intersects_cube() {
        expect_intersects(Point::new(5.0, 0.5, 0.0), Vector::inew(-1, 0, 0), 4., 6.);
        expect_intersects(Point::new(-5.0, 0.5, 0.0), Vector::inew(1, 0, 0), 4., 6.);
        expect_intersects(Point::new(0.5, 5.0, 0.0), Vector::inew(0, -1, 0), 4., 6.);
        expect_intersects(Point::new(0.5, -5.0, 0.0), Vector::inew(0, 1, 0), 4., 6.);
        expect_intersects(Point::new(0.5, 0.0, 5.0), Vector::inew(0, 0, -1), 4., 6.);
        expect_intersects(Point::new(0.5, 0.0, -5.0), Vector::inew(0, 0, 1), 4., 6.);
        expect_intersects(Point::new(0.0, 0.5, 0.0), Vector::inew(0, 0, 1), -1., 1.);
    }

    fn expect_no_intersects(origin: Point, direction: Vector) {
        let c = Cube::new();
        let ray = Ray::new(origin, direction);
        let xs = c.intersect_raw(&ray);
        assert_eq!(xs, None);
    }

    #[test]
    fn ray_misses_a_cube() {
        expect_no_intersects(Point::inew(-2, 0, 0), Vector::new(0.2673, 0.5345, 0.8018));
        expect_no_intersects(Point::inew(0, -2, 0), Vector::new(0.8018, 0.2673, 0.5345));
        expect_no_intersects(Point::inew(0, 0, -2), Vector::new(0.5345, 0.8018, 0.2673));
        expect_no_intersects(Point::inew(2, 0, 2), Vector::new(0., 0., -1.));
        expect_no_intersects(Point::inew(0, 2, 2), Vector::new(0., -1., 0.));
        expect_no_intersects(Point::inew(2, 2, 0), Vector::new(-1., 0., 0.));
    }

    fn expect_normal(point: Point, vector: Vector) {
        let c = Cube::new();
        let normal = c.normal_at(&point);
        assert_eq!(normal, vector);
    }

    #[test]
    fn normal_on_the_surface_of_a_cube() {
        expect_normal(Point::new(1., 0.5, -0.8), Vector::inew(1, 0, 0));
        expect_normal(Point::new(-1., -0.2, 0.9), Vector::inew(-1, 0, 0));
        expect_normal(Point::new(-0.4, 1., -0.1), Vector::inew(0, 1, 0));
        expect_normal(Point::new(0.3, -1., -0.7), Vector::inew(0, -1, 0));
        expect_normal(Point::new(-0.6, 0.3, 1.), Vector::inew(0, 0, 1));
        expect_normal(Point::new(0.4, 0.4, -1.), Vector::inew(0, 0, -1));
        expect_normal(Point::new(1., 1., 1.), Vector::inew(1, 0, 0));
        expect_normal(Point::new(-1., -1., -1.), Vector::inew(-1, 0, 0));
    }
}
