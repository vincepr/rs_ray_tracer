

#[cfg(test)]
mod tests {
    use crate::visual::color::{Col, COL_BLACK, COL_WHITE};

    // setting up world/defaults
    #[test]
    fn static_colors() {
        assert_eq!(COL_BLACK, Col::new(0.0, 0.0, 0.0));
        assert_eq!(COL_WHITE, Col::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn creating_a_stripe_patter() {
        // let pattern = Pattern::stripe()
    }
}