pub mod computations;
pub mod intersects;

use crate::mathstructs::{matrix::Matrix, point::Point, vector::Vector};

// article on returning iterators/impl Iterator Trait:
// https://depth-first.com/articles/2020/06/22/returning-rust-iterators/
// and lifetimes for iterators:
// https://blog.katona.me/2019/12/29/Rust-Lifetimes-and-Iterators/

#[derive(Debug)]
pub struct Ray {
    /// the Origin on the ray
    pub origin: Point,
    /// the Direction of the ray
    pub direction: Vector,
}
impl Ray {
    pub fn new(ori: Point, dir: Vector) -> Ray {
        Ray {
            origin: ori,
            direction: dir,
        }
    }

    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    /// creates a copy of the ray translated by the matrix given.
    /// - this is used to translate from worldspace <-> objectspace
    pub fn transform(&self, m: &Matrix) -> Self {
        Self {
            origin: *m * self.origin,
            direction: *m * self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_query_a_ray() {
        let origin = Point::inew(1, 2, 3);
        let direction = Vector::inew(4, 5, 6);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(Point::inew(2, 3, 4), Vector::inew(1, 0, 0));
        assert_eq!(r.position(0.0), Point::inew(2, 3, 4));
        assert_eq!(r.position(1.0), Point::inew(3, 3, 4));
        assert_eq!(r.position(-1.0), Point::inew(1, 3, 4));
        assert_eq!(r.position(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn translating_a_ray() {
        let r1 = Ray::new(Point::inew(1, 2, 3), Vector::inew(0, 1, 0));
        let m = Matrix::itranslation_new(3, 4, 5);
        let r2 = r1.transform(&m);
        assert_eq!(r2.origin, Point::inew(4, 6, 8));
        assert_eq!(r2.direction, Vector::inew(0, 1, 0));
        assert_eq!(r1.origin, Point::inew(1, 2, 3));
        assert_eq!(r1.direction, Vector::inew(0, 1, 0));
    }

    #[test]
    fn scaling_a_ray() {
        let r1 = Ray::new(Point::inew(1, 2, 3), Vector::inew(0, 1, 0));
        let m = Matrix::iscaling_new(2, 3, 4);
        let r2 = r1.transform(&m);
        assert_eq!(r2.origin, Point::inew(2, 6, 12));
        assert_eq!(r2.direction, Vector::inew(0, 3, 0));
    }
}
