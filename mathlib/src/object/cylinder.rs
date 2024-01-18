use std::f64::{NEG_INFINITY, INFINITY};

use crate::{ray::{intersects::IntersectsRay, Ray}, mathstructs::{point::Point, vector::Vector}, cmp::ApproxEq};

use super::{Object, Shape};


#[derive(Debug, PartialEq, Clone)]
pub struct Cylinder {
    /// lower bounds of the otherwise inifinite cylinder. Not inclusive this point.
    min: f64,
    /// upper bounds of the otherwise inifinite cylinder. Not inclusive this point.
    max: f64,
}

impl IntersectsRay for Cylinder {
    fn intersect_raw(&self, ray: &Ray) -> Option<(f64, f64)> {
        let a = ray.direction.x * ray.direction.x + ray.direction.z * ray.direction.z;

        // ray is parallel to the y axis
        if a.apx_eq(&0.0) { return None };

        let b = 2. * ray.origin.x * ray.direction.x + 2. *ray.origin.z * ray.direction.z;
        let c = ray.origin.x * ray.origin.x + ray.origin.z * ray.origin.z - 1.;
        let disc = b * b - 4. * a * c;
        
        if disc < 0. {
            return None;    // ray does not intersect the cylinder:
        }
        let mut t0 = (-b - disc.sqrt()) / (2. * a);
        let mut t1 = (-b + disc.sqrt()) / (2. * a);
        if t0 > t1 {
            (t0, t1) = (t1, t0);
        }
        
        // calculate the y for each intersection, if were in solid space (between min and max)
        // -> its a valid interseciton
        let y0 = ray.origin.y + t0 * ray.direction.y;
        let y1 = ray.origin.y + t1 * ray.direction.y;

        let mut hit0 : Option<f64> = None;
        let mut hit1 : Option<f64> = None;
        if self.min < y0 && y0 < self.max {
            // t0 is real
            hit0 = Some(t0);
        }

        if self.min < y1 && y1 < self.max {
            // t1 is real
            hit1 = Some(t1);
        }
        match (hit0, hit1) {
            (Some(hit0),_) => Some((t0, t1)),
            _ => None,
        }

        // Some((t0, t1))
    }

    fn normal_at(point: Point) -> Vector {
        return Vector::new(point.x, 0., point.z);
    }
}

impl Cylinder {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Object {
        Object::new(Shape::CylinderShape(Cylinder::default()))
    }

    pub fn new_with_bounds(min: Option<f64>, max: Option<f64>) -> Object {
        let min = min.unwrap_or(NEG_INFINITY);
        let max = max.unwrap_or(INFINITY);
        Object::new(Shape::CylinderShape(Cylinder { min, max }))
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Self { min: NEG_INFINITY, max: INFINITY }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::{INFINITY, NEG_INFINITY};

    use super::*;

    fn expect_miss(origin: Point, direction: Vector) {
        let shape = Cylinder::new();
        let dir = direction.normalize();
        let ray = Ray::new(origin, dir);
        let xs = shape.intersect_raw(&ray);
        assert_eq!(xs, None);
    }

    #[test]
    fn ray_misses_a_cylinder() {
        expect_miss(Point::inew(1,0,0), Vector::inew(0,1,0));
        expect_miss(Point::inew(0,0,0), Vector::inew(0,1,0));
        expect_miss(Point::inew(0,0,-5), Vector::inew(1,1,1));
    }

    fn expect_hit(origin: Point, direction: Vector, result: (f64, f64)) {
        let shape = Cylinder::new();
        let dir = direction.normalize();
        let ray = Ray::new(origin, dir);
        let xs = shape.intersect_raw(&ray).unwrap();
        assert!(xs.0.apx_eq(&result.0));
        assert!(xs.1.apx_eq(&result.1));
    }

    #[test]
    fn ray_hits_a_cylinder() {
        expect_hit(Point::inew(1,0,-5), Vector::inew(0,0,1), (5., 5.));
        expect_hit(Point::inew(0,0,-5), Vector::inew(0,0,1), (4., 6.));
        expect_hit(Point::new(0.5,0.,-5.), Vector::new(0.1,1.,1.), (6.80798, 7.08872));
    }

    fn expect_normal(point: Point, normal: Vector) {
        let shape = Cylinder::new();
        let res = shape.normal_at(&point);
        assert_eq!(res, normal);
    }

    #[test]
    fn calculate_normals_on_a_cylinder() {
        expect_normal(Point::inew(1,0,0), Vector::inew(1,0,0));
        expect_normal(Point::inew(0,5,-1), Vector::inew(0,0,-1));
        expect_normal(Point::inew(0,-2,1), Vector::inew(0,0,1));
        expect_normal(Point::inew(-1,1,0), Vector::inew(-1,0,0));
    }

    #[test]
    fn default_min_max_for_a_cylinder() {
        let obj = Cylinder::new();
        if let Shape::CylinderShape(shape) = obj.shape {
            assert_eq!(shape.min, -INFINITY);
            assert_eq!(shape.max, INFINITY);
        } else {
            unreachable!();
        }
    }

    fn expect_miss_on_constrained(origin: Point, direction: Vector) {
        let shape = Cylinder::new_with_bounds(Some(1.), Some(2.));
        let dir = direction.normalize();
        let ray = Ray::new(origin, dir);
        let xs = shape.intersect_raw(&ray);
        assert_eq!(xs, None);
    }

    fn expect_hit_on_constrained(origin: Point, direction: Vector, result: (f64, f64)) {
        let shape = Cylinder::new();
        let dir = direction.normalize();
        let ray = Ray::new(origin, dir);
        let xs = shape.intersect_raw(&ray).unwrap();
        assert!(xs.0.apx_eq(&result.0));
        assert!(xs.1.apx_eq(&result.1));
    }

    #[test]
    fn intersecting_a_constrained_cylinder() {
        expect_miss_on_constrained(Point::new(0.,1.5,1.), Vector::new(0.1,1.,0.));
        expect_miss_on_constrained(Point::inew(0,3,-5), Vector::inew(0,0,1));
        expect_miss_on_constrained(Point::inew(0,0,-5), Vector::inew(0,0,1));
        expect_miss_on_constrained(Point::inew(0,2,-5), Vector::inew(0,0,1));
        expect_miss_on_constrained(Point::inew(0,1,-5), Vector::inew(0,0,1));
        expect_hit_on_constrained(Point::new(0.,1.5,-2.), Vector::inew(0,0,1), (1.,1.));
    }
}