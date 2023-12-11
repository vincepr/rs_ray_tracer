use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

use crate::{cmp::ApproxEq, io::ppm::COLOR_MAXVAL};

/// represents a RGB-Color
///  - final colors should be between 0 - 1.
/// But intermediate ones used for further calculating might exceede those borders in both directions
#[derive(Debug, Clone, Copy)]
pub struct Col {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Col {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

impl Display for Col {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}  ",
            base_255(self.r),
            base_255(self.g),
            base_255(self.b)
        )
    }
}

// usess COLOR_MAXVAL to translate 0-1 range into percentage of that value
fn base_255(f: f64) -> u8 {
    match f {
        n if n < 0.0 => 0,
        n if n > COLOR_MAXVAL as f64 => 255,
        n => (n * ((COLOR_MAXVAL + 1) as f64)).floor() as u8,
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

impl Mul<f64> for Col {
    type Output = Col;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}
impl Mul<Col> for f64 {
    type Output = Col;
    fn mul(self, rhs: Col) -> Self::Output {
        Self::Output {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}

pub const BLACK: Col = Col {
    r: 0.0,
    g: 0.0,
    b: 0.0,
};
pub const WHITE: Col = Col {
    r: 1.0,
    g: 1.0,
    b: 1.0,
};

pub const RED: Col = Col {
    r: 1.0,
    g: 0.0,
    b: 0.0,
};
pub const GREEN: Col = Col {
    r: 0.0,
    g: 1.0,
    b: 0.0,
};
pub const BLUE: Col = Col {
    r: 0.0,
    g: 0.0,
    b: 1.0,
};

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
    fn mul_col_f64() {
        let l = Col::new(0.2, 0.3, 0.4);
        let e = Col::new(0.4, 0.6, 0.8);
        assert_eq!(l * 2.0, e);
        assert_eq!(0.5 * e, l);
    }
}
