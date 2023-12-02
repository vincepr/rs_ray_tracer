pub mod intersects;
pub mod sphere;

use crate::mathstructs::{point::Point, vector::Vector};

// article on returning iterators/impl Iterator Trait:
// https://depth-first.com/articles/2020/06/22/returning-rust-iterators/
// and lifetimes for iterators:
// https://blog.katona.me/2019/12/29/Rust-Lifetimes-and-Iterators/

#[derive(Debug)]
pub struct Ray {
    /// the Origin on the ray
    pub ori: Point,
    /// the Direction of the ray
    pub dir: Vector,
}
impl Ray {
    pub fn new(ori: Point, dir: Vector) -> Ray {
        Ray { ori, dir }
    }
}

impl Ray {
    pub fn position(&self, t: f32) -> Point {
        self.ori + self.dir * t
    }
}

#[cfg(test)]
mod tests {
    use crate::mathstructs::{point::Point, vector::Vector};

    use super::*;

    #[test]
    fn create_and_query_a_ray() {
        let origin = Point::newi(1, 2, 3);
        let direction = Vector::newi(4, 5, 6);
        let ray = Ray::new(origin, direction);
        assert_eq!(ray.ori, origin);
        assert_eq!(ray.dir, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(Point::newi(2, 3, 4), Vector::newi(1, 0, 0));
        assert_eq!(r.position(0.0), Point::newi(2, 3, 4));
        assert_eq!(r.position(1.0), Point::newi(3, 3, 4));
        assert_eq!(r.position(-1.0), Point::newi(1, 3, 4));
        assert_eq!(r.position(2.5), Point::new(4.5, 3.0, 4.0));
    }
}
