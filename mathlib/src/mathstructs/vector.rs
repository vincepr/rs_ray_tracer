use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::cmp::ApproxEq;

use super::point::Point;

/// (x, y, z, w=0) - vector
#[derive(Debug, Clone, Copy)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vector {
    /// quality of live, just casts int -> float
    pub fn inew(x: i32, y: i32, z: i32) -> Self {
        Self::new(x as f64, y as f64, z as f64)
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }

    pub fn w(&self) -> i8 {
        0
    }

    /// because we overwrite eq we provide this in case we ever need to check for exact values
    pub fn exact_eq(&self, other: Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    /// distance represented by a vector. Calculated by classic pythagoras
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// convert vector to unit vector (with "length" = magniture = 1)
    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    /// dot product, aka scalar product. takes 2 vectors returns a scalar value
    pub fn dot(&self, other: &Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// get a new vector perpendicular to the two imput ones. used for transformations
    pub fn cross(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// relecting should happen at input- = output-angle
    pub fn reflect(input: &Vector, normal: &Vector) -> Self {
        *input - *normal * 2.0 * input.dot(normal)
    }
}

impl PartialEq for Vector {
    /// overwritten with approximate equality for our float based values. So we can use == and !=
    fn eq(&self, other: &Self) -> bool {
        self.apx_eq(other)
    }
}

impl ApproxEq for Vector {
    fn apx_eq(&self, other: &Self) -> bool {
        self.x.apx_eq(&other.x) && self.y.apx_eq(&other.y) && self.z.apx_eq(&other.z)
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Add<Point> for Vector {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: rhs * self.x,
            y: rhs * self.y,
            z: rhs * self.z,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Vector;
    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn w1_is_point() {
        let (x, y, z) = (1.1, 2.2, -3.3);
        let v = Vector::new(x, y, z);
        assert!(v.w() == 0);
        assert_eq!((1.1, 2.2, -3.3), (v.x, v.y, v.z));
    }

    #[test]
    fn partial_eq_tests() {
        let v1 = Vector::new(1.1, 1.1, 1.1);
        let v2 = Vector::new(1.1, 1.1, 1.1);
        let v3 = Vector::new(1.100001, 1.1, 1.1);
        let v4 = Vector::new(1.10001, 1.1, 1.1);
        let v5 = Vector::new(1.1001, 1.1, 1.1);
        assert!(v1 == v2);
        assert!(v1 == v3);
        assert!(v1 != v4);
        assert!(v1 != v5);
    }

    #[test]
    fn add_vec_vec() {
        let l = Vector::new(1.1, 1.1, 1.1);
        let r = Vector::new(1.1, 2.2, -1.2);
        let e = Vector::new(2.2, 3.3, -0.1);
        assert!(l + r == e);
    }

    #[test]
    fn add_vec_pnt() {
        let l = Vector::new(1.1, 1.1, 1.1);
        let r = Point::new(1.1, 2.2, -1.2);
        let e = Point::new(2.2, 3.3, -0.1);
        assert!(l + r == e);
        assert!(r + l == e);
    }

    #[test]
    fn sub_vec_vec() {
        let l = Vector::new(1.1, 1.1, 1.1);
        let r = Vector::new(1.1, 2.2, -1.2);
        let e = Vector::new(0.0, -1.1, 2.3);
        let e2 = Vector::new(0.0, 1.1, -2.3);
        assert!(l - r == e);
        assert!(r - l == e2);
        assert!(-e == e2);
    }

    #[test]
    fn neg_vec() {
        let v = Vector::new(1.1, 0.0, -4.4);
        let e = Vector::new(-1.1, 0.0, 4.4);
        assert!(-v == e);
    }

    #[test]
    fn mul_vec() {
        let v = Vector::new(1.1, 0.0, -3.3);
        let e = Vector::new(2.2, 0.0, -6.6);
        let e2 = Vector::new(0.55, 0.0, -1.65);
        assert!(2.0 * v == e);
        assert!(-2.0 * v == -e);
        assert!(0.5 * v == e2);
    }

    #[test]
    fn div_vec() {
        let v = Vector::new(1.1, 0.0, -3.3);
        let e = Vector::new(2.2, 0.0, -6.6);
        let e2 = Vector::new(0.55, 0.0, -1.65);
        assert!(v / 1.0 == v);
        assert!(v / 2.0 == e2);
        assert!(v / 0.5 == e);
    }

    #[test]
    fn magnitude_vec() {
        assert!(Vector::inew(1, 0, 0).magnitude().apx_eq(&1.0));
        assert!(Vector::inew(0, 1, 0).magnitude().apx_eq(&1.0));
        assert!(Vector::inew(0, 0, 1).magnitude().apx_eq(&1.0));
        let sqrt14 = (14.0 as f64).sqrt();
        assert!(Vector::inew(1, 2, 3).magnitude().apx_eq(&sqrt14));
        assert!(Vector::inew(-1, -2, -3).magnitude().apx_eq(&sqrt14));
    }

    #[test]
    fn normalize_vec() {
        assert_eq!(Vector::inew(4, 0, 0).normalize(), Vector::inew(1, 0, 0));

        let res = Vector::inew(1, 2, 3).normalize();
        let exp = Vector::new(0.26726, 0.53452, 0.80178);
        assert_eq!(res, exp);
        assert!(res.magnitude().apx_eq(&1.0));
    }

    #[test]
    fn dotproduct_vec() {
        let a = Vector::inew(1, 2, 3);
        let b = Vector::inew(2, 3, 4);
        let res = a.dot(&b);
        assert_eq!(res, 20.0);
    }

    #[test]
    fn crossproduct_vec() {
        let a = Vector::inew(1, 2, 3);
        let b = Vector::inew(2, 3, 4);
        let exp1 = Vector::inew(-1, 2, -1);
        let exp2 = Vector::inew(1, -2, 1);
        assert_eq!(a.cross(&b), exp1);
        assert_eq!(b.cross(&a), exp2);
    }

    #[test]
    fn reflecting_vector_approaching_at_45_deg() {
        let v = Vector::inew(1, -1, 0);
        let n = Vector::inew(0, 1, 0);
        let res = Vector::reflect(&v, &n);
        assert_eq!(res, Vector::inew(1, 1, 0));
    }

    #[test]
    fn reflecting_vector_slanted_angle() {
        let v = Vector::inew(0, -1, 0);
        let sq = 2.0_f64.sqrt() / 2.0;
        let n = Vector::new(sq, sq, 0.0);
        let res = Vector::reflect(&v, &n);
        assert_eq!(res, Vector::inew(1, 0, 0));
    }
}
