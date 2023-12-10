pub trait ApproxEq {
    /// compares for approximate-float equality with EPSILON of 0.00001
    fn apx_eq(&self, other: &Self) -> bool;
}

pub const EPSILON: f64 = 0.00001;

impl ApproxEq for f64 {
    fn apx_eq(&self, other: &f64) -> bool {
        *self == *other || (*self - *other).abs() < EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f64_aprox_eq() {
        let a: f64 = 1.1;
        let b: f64 = 1.1;
        let c: f64 = 1.100001;
        let d: f64 = 1.10001;
        assert!(a.apx_eq(&b));
        assert!(a.apx_eq(&c));
        assert!(!a.apx_eq(&d));
    }
}
