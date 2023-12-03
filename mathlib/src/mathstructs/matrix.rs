use super::{point::Point, vector::Vector};
use crate::cmp::ApproxEq;
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct Matrix2([[f32; 2]; 2]);

#[derive(Debug, Clone, Copy)]
pub struct Matrix3([[f32; 3]; 3]);

/// Matrix4 - 4 rows, 4 columns
#[derive(Debug, Clone, Copy)]
pub struct Matrix([[f32; 4]; 4]);

/// impl Intexing into for all 3 matrix-types
macro_rules! impl_IndexAndIndexMut {
    ($name:ty) => {
        impl core::ops::Index<usize> for $name {
            type Output = [f32];
            fn index(&self, index: usize) -> &Self::Output {
                &self.0[index]
            }
        }
        impl core::ops::IndexMut<usize> for $name {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                &mut self.0[index]
            }
        }
    };
}
impl_IndexAndIndexMut!(Matrix2);
impl_IndexAndIndexMut!(Matrix3);
impl_IndexAndIndexMut!(Matrix);

//          [y][x]      0.0 is top left of "screen"
// impl Intexing into for all 3 matrix-types
macro_rules! impl_PartialEq_WithRounding {
    ($name:ty) => {
        impl core::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                for (y, row) in other.0.iter().enumerate() {
                    for (x, nr) in row.iter().enumerate() {
                        if !self[y][x].apx_eq(nr) {
                            return false;
                        }
                    }
                }
                return true;
            }
        }
    };
}
impl_PartialEq_WithRounding!(Matrix2);
impl_PartialEq_WithRounding!(Matrix3);
impl_PartialEq_WithRounding!(Matrix);

impl Matrix2 {
    pub fn new(matrix: [[f32; 2]; 2]) -> Self {
        Self(matrix)
    }

    /// determinant == the 1/x equivalent in matrix land
    pub fn determinant(&self) -> f32 {
        self[0][0] * self[1][1] - self[0][1] * self[1][0]
    }
}

impl Matrix3 {
    pub fn new(matrix: [[f32; 3]; 3]) -> Self {
        Self(matrix)
    }

    /// deletes a row & colum to make the size smaller (3->2)
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2 {
        let mut vals: Vec<f32> = Vec::with_capacity(4);
        for y in 0..3 {
            if y == row {
                continue;
            }
            for x in 0..3 {
                if x == col {
                    continue;
                }
                vals.push(self[y][x]);
            }
        }
        Matrix2::new([[vals[0], vals[1]], [vals[2], vals[3]]])
    }

    /// submatrix first then determinant of that 2x2
    pub fn minor(&self, row: usize, col: usize) -> f32 {
        let mut vals: Vec<f32> = Vec::with_capacity(4);
        for y in 0..3 {
            if y == row {
                continue;
            }
            for x in 0..3 {
                if x == col {
                    continue;
                }
                vals.push(self[y][x]);
            }
        }
        vals[0] * vals[3] - vals[1] * vals[2]
    }

    /// minors that change their sign if row+col is odd
    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let res = self.minor(row, col);
        if (row + col) % 2 == 0 {
            return res;
        }
        -res
    }

    /// determinant == the 1/x equivalent in matrix land
    pub fn determinant(&self) -> f32 {
        self[0][0] * self.cofactor(0, 0)
            + self[0][1] * self.cofactor(0, 1)
            + self[0][2] * self.cofactor(0, 2)
    }
}

impl Matrix {
    pub fn new(matrix: [[f32; 4]; 4]) -> Self {
        Self(matrix)
    }

    /// identity-matrix any M*identity = M
    pub fn new_identity() -> Self {
        Self([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    /// turns rows to cols and vice verse. Used for translating normal vectors between obj-space <-> world-space
    pub fn transpose(&self) -> Self {
        Self([
            [self[0][0], self[1][0], self[2][0], self[3][0]],
            [self[0][1], self[1][1], self[2][1], self[3][1]],
            [self[0][2], self[1][2], self[2][2], self[3][2]],
            [self[0][3], self[1][3], self[2][3], self[3][3]],
        ])
    }

    // deletes a row & colum to make the size smaller (4->3)
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix3 {
        let mut vals: Vec<f32> = Vec::with_capacity(9);
        for y in 0..4 {
            if y == row {
                continue;
            }
            for x in 0..4 {
                if x == col {
                    continue;
                }
                vals.push(self[y][x]);
            }
        }
        Matrix3::new([
            [vals[0], vals[1], vals[2]],
            [vals[3], vals[4], vals[5]],
            [vals[6], vals[7], vals[8]],
        ])
    }

    /// submatrix first then determinant of 3x3
    pub fn minor(&self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }

    /// minors that change their sign if row+col is odd
    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let res = self.minor(row, col);
        if (row + col) % 2 == 0 {
            return res;
        }
        -res
    }

    /// determinant == the 1/x equivalent in matrix land
    pub fn determinant(&self) -> f32 {
        (0..4).fold(0.0, |acc, x| acc + self[0][x] * self.cofactor(0, x))
    }

    /// inverts the effect or reversing multiplication of a matrix
    pub fn inverse(&self) -> Self {
        if self.determinant().apx_eq(&0.0) {
            panic!("Could not find inverse of {self:?}");
            // since we never handle this case anyway this should be more conveniant.
        }
        let determinant = self.determinant();
        let mut result = Matrix([[0.0; 4]; 4]);
        for row in 0..4 {
            for col in 0..4 {
                let c = self.cofactor(row, col);
                result[col][row] = c / determinant;
            }
        }
        result
    }
}

impl Default for Matrix {
    fn default() -> Self {
        Self::new_identity()
    }
}

// multiplication (only needed for x4 * x4)
impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result_matrix = [[0.0; 4]; 4];
        for y in 0..4 {
            for x in 0..4 {
                result_matrix[y][x] = self[y][0] * rhs[0][x]
                    + self[y][1] * rhs[1][x]
                    + self[y][2] * rhs[2][x]
                    + self[y][3] * rhs[3][x];
            }
        }
        Matrix::new(result_matrix)
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::new(
            self[0][0] * rhs.x
                + self[0][1] * rhs.y
                + self[0][2] * rhs.z
                + self[0][3] * (rhs.w() as f32),
            self[1][0] * rhs.x
                + self[1][1] * rhs.y
                + self[1][2] * rhs.z
                + self[1][3] * (rhs.w() as f32),
            self[2][0] * rhs.x
                + self[2][1] * rhs.y
                + self[2][2] * rhs.z
                + self[2][3] * (rhs.w() as f32),
        )
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(
            self[0][0] * rhs.x
                + self[0][1] * rhs.y
                + self[0][2] * rhs.z
                + self[0][3] * (rhs.w() as f32),
            self[1][0] * rhs.x
                + self[1][1] * rhs.y
                + self[1][2] * rhs.z
                + self[1][3] * (rhs.w() as f32),
            self[2][0] * rhs.x
                + self[2][1] * rhs.y
                + self[2][2] * rhs.z
                + self[2][3] * (rhs.w() as f32),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_matrix_index_into() {
        let mut m = Matrix2::new([[1.1, 2.2], [3.3, 4.4]]);
        assert_eq!(m[0][0], 1.1);
        assert_eq!(m[1][0], 3.3);
        assert_eq!(m[0][1], 2.2);
        assert_eq!(m[1][1], 4.4);
        m.0[1][1] = 5.5;
        assert_eq!(m[1][1], 5.5);
    }

    #[test]
    fn create_matrix4_4() {
        let m = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, -15.5, 16.5],
        ]);
        assert_eq!(m[0][0], 1.0);
        assert_eq!(m[0][3], 4.0);
        assert_eq!(m[1][0], 5.5);
        assert_eq!(m[1][2], 7.5);
        assert_eq!(m[2][2], 11.0);
        assert_eq!(m[3][0], 13.5);
        assert_eq!(m[3][2], -15.5);
    }

    #[test]
    fn matrix_comparison() {
        let m1 = Matrix2::new([[1.1, 2.2], [3.3, 4.4]]);
        let mut m2 = Matrix2::new([[1.1, 2.2], [3.3, 4.4]]);
        assert_eq!(m1, m2);
        m2[0][0] = 1.10001;
        assert_ne!(m1, m2);
        m2[0][0] = 1.100001;
        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix_multiplication_4_4() {
        let l = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let r = Matrix::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let exp = Matrix::new([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        assert_eq!(l * r, exp);
    }

    #[test]
    fn matrix_multiplication_4_point() {
        let l = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 0.1],
        ]);
        let r = Point::new(1.0, 2.0, 3.0);
        let exp = Point::new(18.0, 24.0, 33.0);
        assert_eq!(l * r, exp);
    }

    #[test]
    fn matrix_multiplication_4_vector() {
        let l = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 0.1],
        ]);
        let r = Vector::new(1.0, 2.0, 3.0);
        let exp = Vector::new(14.0, 22.0, 32.0);
        assert_eq!(l * r, exp);
    }

    #[test]
    fn matrix_multiplication_with_idenity_does_nothing() {
        let l = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        let r = Matrix::new_identity();
        assert_eq!(l * r, l);
    }

    #[test]
    fn matrix_multiplication_with_idenity_does_nothing_vec() {
        let l = Matrix::new_identity();
        let r = Point::new(4.0, 3.3, 5.5);
        let exp = Point::new(4.0, 3.3, 5.5);
        assert_eq!(l * r, exp);
    }

    #[test]
    fn matrix_transponse() {
        let l = Matrix::new([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let exp = Matrix::new([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);
        assert_eq!(l.transpose(), exp);
        assert_eq!(Matrix::new_identity().transpose(), Matrix::new_identity());
    }

    #[test]
    fn matrix_determinant_2_2() {
        let l = Matrix2::new([[1.0, 5.0], [-3.0, 2.0]]);
        assert_eq!(l.determinant(), 17.0);
    }

    // submatrix calcs:
    #[test]
    fn matrix_submatrix_3_3() {
        let l = Matrix3::new([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        assert_eq!(l.submatrix(0, 2), Matrix2::new([[-3.0, 2.0], [0.0, 6.0]]));
        assert_eq!(l.submatrix(2, 2), Matrix2::new([[1.0, 5.0], [-3.0, 2.0]]));
        assert_eq!(l.submatrix(1, 1), Matrix2::new([[1.0, 0.0], [0.0, -3.0]]));
    }

    #[test]
    fn matrix_submatrix_4_4() {
        let l = Matrix::new([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        let exp = Matrix3::new([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);
        assert_eq!(l.submatrix(2, 1), exp);
    }

    #[test]
    fn matrix_minor_3_3() {
        let a = Matrix3::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        let b = a.submatrix(1, 0);
        assert_eq!(a.submatrix(1, 0), Matrix2::new([[5.0, 0.0], [-1.0, 5.0]]));
        assert_eq!(b.determinant(), 25.0);
        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn matrix_cofactor_3_3() {
        let l = Matrix3::new([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(l.minor(0, 0), -12.0);
        assert_eq!(l.cofactor(0, 0), -12.0); // no sign change
        assert_eq!(l.minor(1, 0), 25.0);
        assert_eq!(l.cofactor(1, 0), -25.0); // here sign changes
    }

    #[test]
    fn matrix_determinant_3_3() {
        let l = Matrix3::new([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert_eq!(l.cofactor(0, 0), 56.0);
        assert_eq!(l.cofactor(0, 1), 12.0);
        assert_eq!(l.cofactor(0, 2), -46.0);
        assert_eq!(l.determinant(), -196.0);
    }

    #[test]
    fn matrix_determinant_4_4() {
        let l = Matrix::new([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(l.cofactor(0, 0), 690.0);
        assert_eq!(l.cofactor(0, 1), 447.0);
        assert_eq!(l.cofactor(0, 2), 210.0);
        assert_eq!(l.cofactor(0, 3), 51.0);
        assert_eq!(l.determinant(), -4071.0);
    }

    #[test]
    #[should_panic]
    fn matrix_inverse_fails_on_zero_determinant() {
        let not_invertible = Matrix::new([
            [-2.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(not_invertible.determinant(), 0.0);
        not_invertible.inverse(); // -> Panics
    }

    #[test]
    fn matrix_inverse_calculating1() {
        let invertible = Matrix::new([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        assert_eq!(invertible.determinant(), 532.0);
        assert_eq!(invertible.cofactor(2, 3), -160.0);
        assert_eq!(invertible.cofactor(3, 2), 105.0);

        let result = invertible.inverse();
        assert_eq!(result[3][2], -160.0 / 532.0);
        assert_eq!(result[2][3], 105.0 / 532.0);

        let exp = Matrix::new([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);
        assert_eq!(result, exp);
    }

    #[test]
    fn matrix_inverse_calculating2() {
        let invertible = Matrix::new([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        let result = invertible.inverse();
        let exp = Matrix::new([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        assert_eq!(result, exp);
    }

    #[test]
    fn matrix_inverse_calculating3() {
        let invertible = Matrix::new([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        let result = invertible.inverse();
        let exp = Matrix::new([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);
        assert_eq!(result, exp);
    }

    #[test]
    fn multiplying_product_by_inverse() {
        let a = Matrix::new([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let b = Matrix::new([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);
        let c = a * b;
        assert_eq!(c * b.inverse(), a);
    }
}
