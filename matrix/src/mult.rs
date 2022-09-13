use std::ops::Mul;

use lalgebra_scalar::Scalar;

use crate::Matrix;

impl<T: Clone> Matrix<T> {
    pub fn number_of_cols(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_rows(&self) -> usize {
        self.0[0].len()
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        let mut vec = Vec::new();
        for col in 0..self.0.len() {
            for row in 0..self.0[col].len() {
                if row == n {
                    vec.push(self.0[col][row].clone())
                }
            }
        }
        vec
    }
}

impl <T: Scalar<Item = T> + std::ops::AddAssign + std::ops::Mul<Output = T> + Clone> Mul for Matrix<T> {
    type Output = Option<Matrix<T>>;
    fn mul(self, other: Matrix<T>) -> Self::Output {
        if self.0.len() != other.0.len() || self.0[0].len() != other.0[0].len() {
            return None;
        }
        let mut matrix: Matrix<T> = Matrix::zero(self.0.len(), other.0.len());
        for i in 0..matrix.number_of_cols() {
            for j in 0..matrix.number_of_rows() {
                let row = self.row(i);
                let col = other.col(j);
                let mut sum = T::zero();
                for index in 0..col.len() {
                    sum += row[index].clone()*col[index].clone();
                }
                matrix.0[i][j] = sum;
            }
        }
        return Some(matrix)
    }
}
