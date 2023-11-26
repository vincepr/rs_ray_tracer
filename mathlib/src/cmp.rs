pub trait ApproxEq {
    /// compares for approximate-float equality with EPSILON of 0.00001
    fn apx_eq(&self, other: &Self) -> bool;
}
impl  ApproxEq for f32 {
   fn apx_eq(&self, other: &f32) -> bool {
        const EPSILON: f32 = 0.00001;
        *self==*other || (*self-*other).abs() < EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f32_aprox_eq() {
        let a: f32 = 1.1;
        let b: f32 = 1.1;
        let c: f32 = 1.100001;
        let d: f32 = 1.10001;
        assert!(a.apx_eq(&b));
        assert!(a.apx_eq(&c));
        assert!(!a.apx_eq(&d));
    }
}
