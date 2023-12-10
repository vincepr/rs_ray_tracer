use std::ops::{Add, Sub};

use crate::cmp::ApproxEq;

use super::vector::Vector;

/// (x, y, z, w=1) - Point
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Point {
    pub fn inew(x: isize, y: isize, z: isize) -> Self {
        Self::new(x as f64, y as f64, z as f64)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    pub fn w(&self) -> i8 {
        1
    }

    /// Point(0, 0, 0)
    pub fn new_origin() -> Self {
        Point {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    /// because we overwrite eq we provide this in case we ever need to check for exact values
    pub fn exact_eq(&self, other: Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl PartialEq for Point {
    /// overwritten with approximate equality for our float based values. So we can use == and !=
    fn eq(&self, other: &Self) -> bool {
        self.apx_eq(other)
    }
}

impl ApproxEq for Point {
    fn apx_eq(&self, other: &Self) -> bool {
        self.x.apx_eq(&other.x) && self.y.apx_eq(&other.y) && self.z.apx_eq(&other.z)
    }
}

impl Add<Vector> for Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Point;
    fn sub(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_point_point() {
        let l = Point::new(1.1, 2.2, 3.3);
        let r = Point::new(1.1, 2.2, -3.3);
        let e = Vector::new(0.0, 0.0, 6.6);
        assert!(l - r == e);
        assert!(r - l == -e);
    }

    #[test]
    fn sub_pnt_vec() {
        let l = Point::new(1.1, 1.1, 1.1);
        let r = Vector::new(1.1, 2.2, -1.2);
        let e = Point::new(0.0, -1.1, 2.3);
        assert!(l - r == e);
    }
}
