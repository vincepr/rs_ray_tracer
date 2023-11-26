use std::ops::{Add, Sub};

use crate::cmp::ApproxEq;


#[derive(Debug, PartialEq)]
pub enum TuplType {
    Point,
    Vector,
}

/// (x,y,z,0) - vector
/// (x,y,z,1) - point
#[derive(Debug)]
pub struct Tupl{
    pub x: f32,
    pub y: f32,
    pub z: f32,
    w: TuplType,
}
impl Tupl {
    pub fn new_point(x: f32, y: f32, z: f32) -> Self {
        Tupl { x, y, z, w: TuplType::Point }
    }

    pub fn new_vector(x: f32, y: f32, z: f32) -> Self {
        Tupl { x, y, z, w: TuplType::Vector }
    }
    
    pub fn w(&self ) -> i8 {
        match self.w {
            TuplType::Vector => 0,
            TuplType::Point => 1,
        }
    }

    /// because we overwrite eq we provide this in case we ever need to check for exact values
    pub fn exact_eq(&self, other: Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}

impl PartialEq for Tupl {
    /// overwritten with approximate equality for our float based values. So we can use == and !=
    fn eq(&self, other: &Self) -> bool {
        self.apx_eq(&other)
    }
}

impl ApproxEq for Tupl {
    fn apx_eq(&self, other: &Self) -> bool {
        self.x.apx_eq(&other.x) 
        && self.y.apx_eq(&other.y)
        && self.z.apx_eq(&other.z)
        && self.w == other.w
    }
}

impl Add for Tupl {
    type Output = Tupl;
    fn add(self, rhs: Self) -> Self::Output {
        match (&self.w, &rhs.w){
            // adding point + point should NOT be allowed
            (TuplType::Point, TuplType::Point) => unreachable!(),   // should probably do this properly and return Option?
            (_, _) => Self::Output{
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
                w: self.w,
            }
        }
    }
}

impl Sub for Tupl {
    type Output = Tupl;
    fn sub(self, rhs: Self) -> Self::Output {
        match (&self.w, &rhs.w){
            // subtracting vector - point should NOT be allowed
            (TuplType::Vector, TuplType::Point) => unreachable!(),   // should probably do this properly and return Option?
            (_, _) => Self{
                x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z,
                w: self.w,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn w1_is_point() {
        let (x,y,z) = (1.1, 2.2, -3.3);
        let p = Tupl::new_point(x, y, z);
        assert!(p.w == TuplType::Point);
        assert!(p.w() == 1);
        assert_eq!((1.1, 2.2, -3.3), (p.x, p.y, p.z));
    }

    #[test]
    fn w0_is_vector() {
        let (x,y,z) = (1.1, 2.2, -3.3);
        let p = Tupl::new_vector(x, y, z);
        assert!(p.w == TuplType::Vector);
        assert!(p.w() == 0);
        assert_eq!((1.1, 2.2, -3.3), (p.x, p.y, p.z));
    }

    #[test]
    fn partial_eq_tests() {
        let p1 = Tupl::new_point(1.1, 1.1, 1.1);
        let p2 = Tupl::new_point(1.1, 1.1, 1.1);
        let p3 = Tupl::new_point(-1.1, 1.1, 1.1);
        let v1 = Tupl::new_vector(1.1, 1.1, 1.1);
        let v2 = Tupl::new_vector(1.100001, 1.1, 1.1);
        assert!(p1 == p2);
        assert!(p1 != p3);
        assert!(p1 != v1);
        assert!(v1 == v2);
    }

    #[test]
    fn approx_eq_tests() {
        let v1 = Tupl::new_vector(1.1, 1.1, 1.1);
        let v2 = Tupl::new_vector(1.10001, 1.1, 1.1);   // last unequal digit
        let v3 = Tupl::new_vector(1.100001, 1.1, 1.1);
        let v4 = Tupl::new_vector(1.1, 1.1, 1.1);
        assert!(!v1.apx_eq(&v2));
        assert!(v1.apx_eq(&v3));
        assert!(v1.apx_eq(&v4));
    }

    #[test]
    fn add_tupls() {
        let v1 = Tupl::new_vector(1.1, 1.1, 1.1);
        let p2 = Tupl::new_point(1.1, 2.2, -1.1);
        let e = Tupl::new_vector(2.2, 3.3, 0.0);
        assert!(v1+p2 == e);
    }

    #[test]
    fn subtracting_tupls() {
        let v = Tupl::new_vector(1.1, 1.1, 1.1);
        let p = Tupl::new_point(1.1, 2.2, -1.1);
        let e = Tupl::new_vector(0.0,1.1, 0.0);
        assert!(p-v == e)
    }
}
