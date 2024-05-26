use std::{array::IntoIter, f64::{NEG_INFINITY, INFINITY}};

use crate::{ray::{intersects::IntersectsRay, Ray}, mathstructs::{point::Point, vector::Vector}, cmp::{ApproxEq, EPSILON}};

use super::{Object, Shape};


#[derive(Debug, PartialEq, Clone)]
pub struct Cylinder {
    /// lower bounds of the otherwise inifinite cylinder. Not inclusive this point.
    min: f64,
    /// upper bounds of the otherwise inifinite cylinder. Not inclusive this point.
    max: f64,
    /// indicates that the cylinders should be capped. Default is hollow cylinder with capped false.
    capped: bool,
}


// -------------- --------------
// TODO: refactor this away? or maybe return -> Result<(f64, f64), Vec<f64>> instead
/// represents our intersections-collection of 0-4 hits:
pub struct XS {
    pub count: u8,
    pub slice: [f64;4]
}
impl XS {
    pub fn new() -> Self {
        Self { count: 0, slice: [0.,0.,0.,0.] }
    }

    pub fn push(&mut self, nr: f64) {
        self.count += self.count;
        if self.count > 3 { unreachable!("wrong use, should never overflow"); }
        self.slice[self.count as usize] = nr;
    }
}

pub struct XSIterator {
    xs: XS,
    idx: usize,
}
impl Iterator for XSIterator {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        const MAX: usize = 3;
        let result = if self.idx > self.xs.count as usize || self.idx > MAX {
            None        
        } else {
            Some(self.xs.slice[self.idx])
        };
        self.idx += self.idx;
        result
    }
}

impl<'a> IntoIterator for XS {
    type Item = f64;

    type IntoIter = XSIterator;

    fn into_iter(self) -> Self::IntoIter {
        XSIterator{
            xs:self,
            idx: 0,
        }
    }
}
// -------------- --------------

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
            (Some(_hit0),_) => Some((t0, t1)),
            _ => None,
        }

        // Some((t0, t1))
    }

    // unused, using actual_normal_at currently, should refactor this out (or add self to interface)
    fn normal_at(_point: Point) -> Vector {
        unreachable!();
        // return Vector::new(_point.x, 0., _point.z);
    }
}

/// helper fn, to check if is inside radius of 1. To reduce duplication
fn check_cap(ray: &Ray, t: f64) -> bool {
    let x = ray.origin.x + t * ray.direction.x;
    let z = ray.origin.z + t * ray.direction.x;
    (x * x + z * z) <= 1.0
}

impl Cylinder {
    /// helper fn, check if given ray intersects the end caps of the cylinder:
    fn intersect_caps(&self, ray: &Ray, mut xs: XS) -> XS {
        // caps only matter if the cylinder is closed -> might intersect the ray
        if self.capped || ray.direction.y.apx_eq(&0.0) {
            return xs;
        }
        // check for intersection with the lower end cap (plane at y=self.min)
        let t = (self.min - ray.origin.y) / ray.direction.y;
        if check_cap(ray, t) {
            xs.push(t);
        }
        // check for intersection with the upper end cap (plane at y=self.max)
        let t = (self.max - ray.origin.y) / ray.direction.y;
        if check_cap(ray, t) {
            xs.push(t);
        }
        xs
    }
}

impl Cylinder {
    // i should probably refactor normal_at out of the IntersectsRayInterface as is now.
    pub fn actual_normal_at(&self, point: Point) -> Vector {
        // compute the square of the distance from the y axis
        let dist = point.x.sqrt() + point.z.sqrt();
        if dist < 1. && point.y >= self.max - EPSILON {
            return Vector::new(0., 1., 0.);
        }
        if dist < 1. && point.y <= self.min + EPSILON {
            return Vector::new(0., -1., 0.);
        }
        Vector::new(point.x, 0., point.z)
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
        Object::new(Shape::CylinderShape(Cylinder { min, max, capped: false }))
    }

    /// instead of hollow default use a filled cylinder.
    pub fn new_with_bounds_capped(min: Option<f64>, max: Option<f64>) -> Object {
        let min = min.unwrap_or(NEG_INFINITY);
        let max = max.unwrap_or(INFINITY);
        Object::new(Shape::CylinderShape(Cylinder { min, max, capped: true }))
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Self { min: NEG_INFINITY, max: INFINITY, capped: false }
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
        dbg!(&point, &normal);
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
        expect_miss_on_constrained(Point::new(0.,1.5,0.), Vector::new(0.1,1.,0.));
        expect_miss_on_constrained(Point::inew(0,3,-5), Vector::inew(0,0,1));
        expect_miss_on_constrained(Point::inew(0,0,-5), Vector::inew(0,0,1));
        expect_miss_on_constrained(Point::inew(0,2,-5), Vector::inew(0,0,1));
        expect_miss_on_constrained(Point::inew(0,1,-5), Vector::inew(0,0,1));
        // book expects xs.len() == 2 here
        expect_hit_on_constrained(Point::new(0.,1.5,-2.), Vector::inew(0,0,1), (1.,3.));
    }

    fn expect_two_hits(point: Point, direction: Vector) {
        let shape = Cylinder::new_with_bounds_capped(Some(1.), Some(2.));
        let direction = direction.normalize();
        let ray = Ray::new(point, direction);
        let xs = shape.intersect_raw(&ray).unwrap();
        // book expects xs.len() == 2 here
        assert!(!xs.0.apx_eq(&xs.1));
    }

    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder() {
        expect_two_hits(Point::inew(0,3,0), Vector::inew(0,-1,0));
        expect_two_hits(Point::inew(0,3,-2), Vector::inew(0,-1,2));
        expect_two_hits(Point::inew(0,4,-2), Vector::inew(0,-1,1)); // corner case
        expect_two_hits(Point::inew(0,0,-2), Vector::inew(0,1,2));
        expect_two_hits(Point::inew(0,-1,-2), Vector::inew(0,1,1)); // corner case
    }

    fn expect_normal_capped(point: Point, normal: Vector) {
        let shape = Cylinder::new_with_bounds_capped(Some(1.), Some(2.));
        let res = shape.normal_at(&point);
        assert_eq!(res, normal);
    }

    #[test]
    fn normal_vector_on_a_cylinders_end_caps() {
        expect_normal_capped(Point::inew(0,1,0), Vector::inew(0,-1,0));
        expect_normal_capped(Point::new(0.5,1.,0.), Vector::inew(0,-1,0));
        expect_normal_capped(Point::new(0.,1.,0.5), Vector::inew(0,-1,0));
        expect_normal_capped(Point::inew(0,2,0), Vector::inew(0,1,0));
        expect_normal_capped(Point::new(0.5,2.,0.), Vector::inew(0,1,0));
        expect_normal_capped(Point::new(0.,2.,0.5), Vector::inew(0,1,0));
    }
}