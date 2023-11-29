use std::ops::Mul;

use crate::cmp::ApproxEq;

use super::{point::Point, vector::Vector};

#[derive(Debug, Clone)]
struct Matrix2([[f32; 2]; 2]);

#[derive(Debug, Clone)]
struct Matrix3([[f32; 3]; 3]);

#[derive(Debug, Clone)]
struct Matrix4([[f32; 4]; 4]);

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
impl_IndexAndIndexMut!(Matrix4);

impl Matrix2 {
    pub fn new(matrix: [[f32; 2]; 2]) -> Self {
        Self { 0: matrix }
    }
    
    /// determinand == the 1/x equivalent in matrix land
    pub fn determinant(& self) -> f32 {
        self[0][0]*self[1][1] - self[0][1]*self[1][0]
    }
}

impl Matrix3 {
    pub fn new(matrix: [[f32; 3]; 3]) -> Self {
        Self { 0: matrix }
    }

    // deletes a row & colum to make the size smaller (3->2)
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2 {
        let mut vals: Vec<f32> = Vec::with_capacity(4);
        for y in 0..3{
            if y == row {continue;}
            for x in 0..3 {
                if x == col {continue;}
                vals.push(self[y][x]);
            }
        } 
        Matrix2::new([[vals[0], vals[1]], [vals[2], vals[3]]])
    }
}

impl Matrix4 {
    pub fn new(matrix: [[f32; 4]; 4]) -> Self {
        Self { 0: matrix }
    }

    /// identity-matrix any M*identity = M
    pub fn new_identity() -> Self {
        Self { 0: [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]] }
    }

    /// turns rows to cols and vice verse. Used for translating normal vectors between obj-space <-> world-space
    pub fn transponse(&self) -> Self {
        Self { 0: [
            [self[0][0], self[1][0], self[2][0], self[3][0]], 
            [self[0][1], self[1][1], self[2][1], self[3][1]], 
            [self[0][2], self[1][2], self[2][2], self[3][2]], 
            [self[0][3], self[1][3], self[2][3], self[3][3]], 
        ]}
    }
}

//          [y][x]      0.0 is top left of "screen"
// impl Intexing into for all 3 matrix-types
macro_rules! impl_PartialEq_WithRounding {
    ($name:ty) => {
        impl core::cmp::PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                for (y, row) in other.0.iter().enumerate() {
                    for (x, nr) in row.iter().enumerate() {
                        if !self[y][x].apx_eq(nr) { return false; }
                    }
                }
                return true;
            }
        }
    };
}
impl_PartialEq_WithRounding!(Matrix2);
impl_PartialEq_WithRounding!(Matrix3);
impl_PartialEq_WithRounding!(Matrix4);

// multiplication (only needed for x4 * x4)
impl Mul for Matrix4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result_matrix = [[0.0; 4];4];
        for y in 0..4 {
            for x in 0..4 {
                result_matrix[y][x] = 
                    self[y][0] * rhs[0][x] +
                    self[y][1] * rhs[1][x] +
                    self[y][2] * rhs[2][x] +
                    self[y][3] * rhs[3][x];
            }
        }
        Matrix4::new(result_matrix)
    }
}

impl Mul<Point> for Matrix4 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point::new(
            self[0][0]*rhs.x + self[0][1]*rhs.y + self[0][2]*rhs.z + self[0][3]*(rhs.w() as f32),
            self[1][0]*rhs.x + self[1][1]*rhs.y + self[1][2]*rhs.z + self[1][3]*(rhs.w() as f32),
            self[2][0]*rhs.x + self[2][1]*rhs.y + self[2][2]*rhs.z + self[2][3]*(rhs.w() as f32),
        )
    }
}

impl Mul<Vector> for Matrix4 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(
            self[0][0]*rhs.x + self[0][1]*rhs.y + self[0][2]*rhs.z + self[0][3]*(rhs.w() as f32),
            self[1][0]*rhs.x + self[1][1]*rhs.y + self[1][2]*rhs.z + self[1][3]*(rhs.w() as f32),
            self[2][0]*rhs.x + self[2][1]*rhs.y + self[2][2]*rhs.z + self[2][3]*(rhs.w() as f32),
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
        let m = Matrix4::new([
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
        let l = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]); 
        let r = Matrix4::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]); 
        let exp = Matrix4::new([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]); 

        assert_eq!(l*r, exp);
    }

    #[test]
    fn matrix_multiplication_4_point() {
        let l = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 0.1],
        ]); 
        let r = Point::new(1.0,2.0,3.0);
        let exp = Point::new(18.0, 24.0, 33.0);
        assert_eq!(l*r, exp);
    }

    #[test]
    fn matrix_multiplication_4_vector() {
        let l = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 0.1],
        ]); 
        let r = Vector::new(1.0,2.0,3.0);
        let exp = Vector::new(14.0, 22.0, 32.0);
        assert_eq!(l*r, exp);
    }

    #[test]
    fn matrix_multiplication_with_idenity_does_nothing() {
        let l = Matrix4::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [4.0, 8.0, 16.0, 32.0],
        ]); 
        let r = Matrix4::new_identity();
        assert_eq!(l.clone()*r, l);
    }

    #[test]
    fn matrix_multiplication_with_idenity_does_nothing_vec() {
        let l = Matrix4::new_identity();
        let r = Point::new(4.0, 3.3, 5.5);
        let exp = Point::new(4.0, 3.3, 5.5);
        assert_eq!(l*r, exp);
    }

    #[test]
    fn matrix_transponse() {
        let l = Matrix4::new([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]); 
        let exp= Matrix4::new([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]); 
        assert_eq!(l.transponse(), exp);
        assert_eq!(Matrix4::new_identity().transponse(), Matrix4::new_identity());
    }

    #[test]
    fn matrix_determinant() {
        let l = Matrix2::new([[1.0, 5.0], [-3.0, 2.0]]);
        assert_eq!(l.determinant(), 17.0);
    }

    // submatrix calcs:
    #[test]
    fn matrix_submatrix_3_3() {
        let l = Matrix3::new([
            [1.0, 5.0, 0.0], 
            [-3.0, 2.0, 7.0], 
            [0.0, 6.0, -3.0]]); 
        assert_eq!(l.submatrix(0, 2), Matrix2::new([[-3.0, 2.0], [0.0, 6.0]]));
        assert_eq!(l.submatrix(2, 2), Matrix2::new([[1.0, 5.0], [-3.0, 2.0]]));
        assert_eq!(l.submatrix(1, 1), Matrix2::new([[1.0, 0.0], [0.0, -3.0]]));
    }

    // #[test]
    // fn matrix_submatrix_4_4() {
    //     let l = Matrix4::new([
    //         [-6.0, 1.0, 1.0, 6.0],
    //         [-8.0, 5.0, 8.0, 6.0],
    //         [-1.0, 0.0, 8.0, 2.0],
    //         [-7.0, 1.0, -1.0, 1.0],
    //     ]);
    //     let epx = Matrix3::new([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]); 
    //     assert_eq!(l.submatrix(row: 2, col:1), 17.0);
    // }

}
