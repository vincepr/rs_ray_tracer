use std::{ops::{Add, Mul, Sub}, fmt::Display};

use crate::cmp::ApproxEq;

/// represents a RGB-Color
///  - final colors should be between 0 - 1.
/// But intermediate ones used for further calculating might exceede those borders in both directions
#[derive(Debug, Clone, Copy)]
pub struct Col {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}


impl Col {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    /// Col { 0, 0, 0}
    pub fn new_black() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    /// Col { 1, 1, 1}
    pub fn new_white() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }
}

impl Display for Col {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", base_255(self.r), base_255(self.g), base_255(self.b))
    }
}

fn base_255(f: f32) -> u8 {
    match f {
        n if n < 0.0 => 0,
        n if n > 255.0 => 255,
        n => (n*256.0).floor() as u8,
    }
} 

impl PartialEq for Col {
    fn eq(&self, other: &Self) -> bool {
        self.r.apx_eq(&other.r) && self.g.apx_eq(&other.g) && self.b.apx_eq(&other.b)
    }
}

impl Add for Col {
    type Output = Col;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Sub for Col {
    type Output = Col;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Mul for Col {
    type Output = Col;
    // used to blend colors together.
    fn mul(self, rhs: Self) -> Self::Output {
        Self::Output {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Mul<f32> for Col {
    type Output = Col;
    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}
impl Mul<Col> for f32 {
    type Output = Col;
    fn mul(self, rhs: Col) -> Self::Output {
        Self::Output {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_col() {
        let c1 = Col::new(1.0, 2.0, 3.0);
        let c2 = Col {
            r: 1.0,
            g: 2.0,
            b: 3.0,
        };
        assert_eq!(c1, c2);
        assert!((1.0, 2.0, 3.0) == (c1.r, c2.g, c1.b));
    }

    #[test]
    fn add_col_col() {
        let l = Col::new(0.9, 0.6, 0.75);
        let r = Col::new(0.7, 0.1, 0.25);
        let e = Col::new(1.6, 0.7, 1.0);
        assert_eq!(l + r, e);
    }

    #[test]
    fn sub_col_col() {
        let l = Col::new(0.9, 0.6, 0.75);
        let r = Col::new(0.7, 0.1, 0.25);
        let e = Col::new(0.2, 0.5, 0.5);
        assert_eq!(l - r, e);
    }

    #[test]
    fn mul_col_col() {
        let l = Col::new(1.0, 0.2, 0.4);
        let r = Col::new(0.9, 1.0, 0.1);
        let e = Col::new(0.9, 0.2, 0.04);
        assert_eq!(l * r, e);
    }

    #[test]
    fn mul_col_f32() {
        let l = Col::new(0.2, 0.3, 0.4);
        let e = Col::new(0.4, 0.6, 0.8);
        assert_eq!(l * 2.0, e);
        assert_eq!(0.5 * e, l);
    }
}
