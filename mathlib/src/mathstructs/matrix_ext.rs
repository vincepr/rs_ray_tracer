use super::matrix::Matrix;

impl Matrix {
    // these are the Matrix'es created
    // that after multiplying apply the following transformations:

    // translation (x,y,z)=     [1 0 0 x]
    //                          [0 1 0 y]
    //                          [0 0 1 z]
    //                          [0 0 0 1]
    /// translation moves a Point by x-y-z. But doesnt affect a Vector
    pub fn translation_new(x: f32, y: f32, z: f32) -> Self {
        Self::new([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    pub fn itranslation_new(x: isize, y: isize, z: isize) -> Self {
        Self::translation_new(x as f32, y as f32, z as f32)
    }

    // scaling (x,y,z)=         [x 0 0 0]
    //                          [0 y 0 0]
    //                          [0 0 z 0]
    //                          [0 0 0 1]
    /// scales by the origin. Effectively making it larger or smaller
    pub fn scaling_new(x: f32, y: f32, z: f32) -> Self {
        Self::new([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    pub fn iscaling_new(x: isize, y: isize, z: isize) -> Self {
        Self::scaling_new(x as f32, y as f32, z as f32)
    }

    // rotationX (x,y,z)=   [1      0     0    0]
    //                      [0  cos r  -sin r  0]
    //                      [0  sin r   cos r  0]
    //                      [0      0     0    1]
    /// the rotation will appear to be clockwise arround the corresponding axis
    /// when viewed along that axis. towards the negative end.
    pub fn rotation_x_new(rad: f32) -> Self {
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, rad.cos(), -rad.sin(), 0.0],
            [0.0, rad.sin(), rad.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_y_new(rad: f32) -> Self {
        Self::new([
            [rad.cos(), 0.0, rad.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-rad.sin(), 0.0, rad.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn rotation_z_new(rad: f32) -> Self {
        Self::new([
            [rad.cos(), -rad.sin(), 0.0, 0.0],
            [rad.sin(), rad.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    // shearing
    // X in proportion to y
    // X in proportion to z
    // Y in proportion to x
    // Y in proportion to z
    // Z in proportion to x
    // Z in proportion to y
    /// making straight lines slanted. (the further away the more it shears off)
    #[allow(non_snake_case)]
    pub fn shearing_new(Xy: f32, Xz: f32, Yx: f32, Yz: f32, Zx: f32, Zy: f32) -> Self {
        Self::new([
            [1.0, Xy, Xz, 0.0],
            [Yx, 1.0, Yz, 0.0],
            [Zx, Zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    #[allow(non_snake_case)]
    pub fn shearingi_new(Xy: usize, Xz: usize, Yx: usize, Yz: usize, Zx: usize, Zy: usize) -> Self {
        Self::shearing_new(
            Xy as f32, Xz as f32, Yx as f32, Yz as f32, Zx as f32, Zy as f32,
        )
    }
}

// implement fluent-API for the transformations
impl Matrix {
    pub fn translate(&self, x: f32, y: f32, z: f32) -> Self {
        Self::translation_new(x, y, z) * *self
    }
    pub fn scale(&self, x: f32, y: f32, z: f32) -> Self {
        Self::scaling_new(x, y, z) * *self
    }
    pub fn rotate_x(&self, rad: f32) -> Self {
        Self::rotation_x_new(rad) * *self
    }
    pub fn rotate_y(&self, rad: f32) -> Self {
        Self::rotation_y_new(rad) * *self
    }
    pub fn rotate_z(&self, rad: f32) -> Self {
        Self::rotation_z_new(rad) * *self
    }
    #[allow(non_snake_case)]
    pub fn shear(&self, Xy: f32, Xz: f32, Yx: f32, Yz: f32, Zx: f32, Zy: f32) -> Self {
        Self::shearing_new(Xy, Xz, Yx, Yz, Zx, Zy) * *self
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use crate::mathstructs::{point::Point, vector::Vector};

    use super::*;

    // translation
    #[test]
    fn multiplying_by_a_translation_matrix() {
        let t = Matrix::translation_new(5.0, -3.0, 2.0);
        let p = Point::inew(-3, 4, 5);
        let res = t * p;
        assert_eq!(res, Point::inew(2, 1, 7));
    }

    #[test]
    fn multiplying_by_inverse_of_a_translation_matrix() {
        let t = Matrix::translation_new(5.0, -3.0, 2.0);
        let inv = t.inverse().unwrap();
        let p = Point::inew(-3, 4, 5);
        let res = inv * p;
        assert_eq!(res, Point::inew(-8, 7, 3));
    }

    #[test]
    fn transformation_does_not_affect_vectors() {
        let t = Matrix::translation_new(5.0, -3.0, 2.0);
        let v = Vector::inew(-3, 4, 5);
        assert_eq!(t * v, Vector::inew(-3, 4, 5));
    }

    // scaling
    #[test]
    fn scaling_matrix_applied_to_point() {
        let t = Matrix::iscaling_new(2, 3, 4);
        let p = Point::inew(-4, 6, 8);
        assert_eq!(t * p, Point::inew(-8, 18, 32));
    }

    #[test]
    fn scaling_matrix_applied_to_vector() {
        let t = Matrix::iscaling_new(2, 3, 4);
        let p = Vector::inew(-4, 6, 8);
        assert_eq!(t * p, Vector::inew(-8, 18, 32));
    }

    #[test]
    fn scaling_matrix_multiplying_by_inverse_of() {
        let t = Matrix::iscaling_new(2, 3, 4);
        let inv = t.inverse().unwrap();
        let v = Vector::inew(-4, 6, 8);
        assert_eq!(inv * v, Vector::inew(-2, 2, 2));
    }

    #[test]
    fn reflection_is_scaling_by_negative_value() {
        let t = Matrix::iscaling_new(-1, 1, 1);
        let p = Point::inew(2, 3, 4);
        assert_eq!(t * p, Point::inew(-2, 3, 4));
    }

    // rotation
    #[test]
    fn rotation_point_arround_x_axis() {
        let p = Point::inew(0, 1, 0);
        let half_quarter = Matrix::rotation_x_new(PI / 4.0);
        let sqrt = 2.0_f32.sqrt() / 2.0;
        assert_eq!(half_quarter * p, Point::new(0.0, sqrt, sqrt));
        let full_quarter = Matrix::rotation_x_new(PI / 2.0);
        assert_eq!(full_quarter * p, Point::inew(0, 0, 1));
    }

    #[test]
    fn rotation_point_arround_y_axis() {
        let p = Point::inew(0, 0, 1);
        let half_quarter = Matrix::rotation_y_new(PI / 4.0);
        let sqrt = 2.0_f32.sqrt() / 2.0;
        assert_eq!(half_quarter * p, Point::new(sqrt, 0.0, sqrt));
        let full_quarter = Matrix::rotation_y_new(PI / 2.0);
        assert_eq!(full_quarter * p, Point::inew(1, 0, 0));
    }

    #[test]
    fn rotation_point_arround_z_axis() {
        let p = Point::inew(0, 1, 0);
        let half_quarter = Matrix::rotation_z_new(PI / 4.0);
        let sqrt = 2.0_f32.sqrt() / 2.0;
        assert_eq!(half_quarter * p, Point::new(-sqrt, sqrt, 0.0));
        let full_quarter = Matrix::rotation_z_new(PI / 2.0);
        assert_eq!(full_quarter * p, Point::inew(-1, 0, 0));
    }

    // shearing
    #[test]
    fn shearing_all_directions() {
        shearing_transform_moves_x_in_proportion_to_y();
        shearing_transform_moves_x_in_proportion_to_z();
        shearing_transform_moves_y_in_proportion_to_x();
        shearing_transform_moves_y_in_proportion_to_z();
        shearing_transform_moves_z_in_proportion_to_x();
        shearing_transform_moves_z_in_proportion_to_y();
    }

    fn shearing_transform_moves_x_in_proportion_to_y() {
        let t = Matrix::shearingi_new(1, 0, 0, 0, 0, 0);
        let p = Point::inew(2, 3, 4);
        assert_eq!(t * p, Point::inew(5, 3, 4));
    }
    fn shearing_transform_moves_x_in_proportion_to_z() {
        let t = Matrix::shearingi_new(0, 1, 0, 0, 0, 0);
        let p = Point::inew(2, 3, 4);
        assert_eq!(t * p, Point::inew(6, 3, 4));
    }
    fn shearing_transform_moves_y_in_proportion_to_x() {
        let t = Matrix::shearingi_new(0, 0, 1, 0, 0, 0);
        let p = Point::inew(2, 3, 4);
        assert_eq!(t * p, Point::inew(2, 5, 4));
    }
    fn shearing_transform_moves_y_in_proportion_to_z() {
        let t = Matrix::shearingi_new(0, 0, 0, 1, 0, 0);
        let p = Point::inew(2, 3, 4);
        assert_eq!(t * p, Point::inew(2, 7, 4));
    }
    fn shearing_transform_moves_z_in_proportion_to_x() {
        let t = Matrix::shearingi_new(0, 0, 0, 0, 1, 0);
        let p = Point::inew(2, 3, 4);
        assert_eq!(t * p, Point::inew(2, 3, 6));
    }
    fn shearing_transform_moves_z_in_proportion_to_y() {
        let t = Matrix::shearingi_new(0, 0, 0, 0, 0, 1);
        let p = Point::inew(2, 3, 4);
        assert_eq!(t * p, Point::inew(2, 3, 7));
    }

    #[test]
    fn transformations_applied_in_sequence() {
        let p1 = Point::inew(1, 0, 1);
        let a = Matrix::rotation_x_new(PI / 2.0);
        let b = Matrix::iscaling_new(5, 5, 5);
        let c = Matrix::itranslation_new(10, 5, 7);
        // apply one after the other
        let p2 = a * p1;
        assert_eq!(p2, Point::inew(1, -1, 0));
        let p3 = b * p2;
        assert_eq!(p3, Point::inew(5, -5, 0));
        let p4 = c * p3;
        assert_eq!(p4, Point::inew(15, 0, 7));
        // matrix multiplicatoins are associative (though not commutative (so order important!))
        let ass = c * b * a;
        let ass_res = ass * p1;
        assert_eq!(ass_res, p4);
    }

    #[test]
    fn transformations_fluent_api_the_same() {
        let p1 = Point::inew(1, 0, 1);
        let a = Matrix::rotation_x_new(PI / 2.0);
        let b = Matrix::iscaling_new(5, 5, 5);
        let c = Matrix::itranslation_new(10, 5, 7);
        // apply one after the other
        let p2 = a * p1;
        assert_eq!(p2, Point::inew(1, -1, 0));
        let p3 = b * p2;
        assert_eq!(p3, Point::inew(5, -5, 0));
        let p4 = c * p3;
        assert_eq!(p4, Point::inew(15, 0, 7));
        // apply with fluent
        let chain = Matrix::new_identity() // we know identity does nothing
            .rotate_x(PI / 2.0)
            .scale(5.0, 5.0, 5.0)
            .translate(10.0, 5.0, 7.0);
        let res_chain = chain * p1;
        assert_eq!(res_chain, p4);
    }
}
