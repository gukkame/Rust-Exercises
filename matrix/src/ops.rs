use std::ops::{Add, Sub};

use lalgebra_scalar::Scalar;

use crate::Matrix;


impl <T: Scalar<Item = T> + std::ops::Add<Output = T> + Clone> Add for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn add(self, other: Matrix<T>) -> Self::Output {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }
        let mut matrix: Vec<Vec<T>> = Vec::new();
        for i in 0..self.0.len() {
            let mut vec: Vec<T> = Vec::new();
            for j in 0..other.0[i].len()  {
                vec.push(self.0[i][j].clone() + other.0[j][i].clone());
            }
            matrix.push(vec);
        }
        return Some(Matrix(matrix));
    }
}

impl <T: Scalar<Item = T> + std::ops::Sub<Output = T> + Clone> Sub for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn sub(self, other: Matrix<T>) -> Self::Output {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }
        let mut matrix: Vec<Vec<T>> = Vec::new();
        for i in 0..self.0.len() {
            let mut vec: Vec<T> = Vec::new();
            for j in 0..other.0[i].len()  {
                vec.push(self.0[i][j].clone() - other.0[j][i].clone());
            }
            matrix.push(vec);
        }
        return Some(Matrix(matrix));
    }
}
