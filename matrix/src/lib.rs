pub use lalgebra_scalar::{Scalar}; 
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl <T: Scalar<Item = T>> Matrix<T> {
	pub fn new() -> Matrix<T> {
        Matrix(Vec::new())
	}

	pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut matrix: Vec<Vec<T>> = Vec::new();
        for _ in 0..row {
            let mut vec: Vec<T> = Vec::new();
            for _ in 0..col {
                vec.push(T::zero())
            }
            matrix.push(vec);
        }
        return Matrix(matrix)
	}
	pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix: Vec<Vec<T>> = Vec::new();
        for i in 0..n {
            let mut vec: Vec<T> = Vec::new();
            for j in 0..n {
                match i as i8 - j as i8 {
                    0 => vec.push(T::one()),
                    _ => vec.push(T::zero())
                }
            }
            matrix.push(vec);
        }
        return Matrix(matrix)
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_property() {
        let matrix: Matrix<u32> = Matrix::zero(3, 4);
        let expected: Matrix<u32> = Matrix(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
        assert_eq!(matrix, expected);
        let matrix: Matrix<u32> = Matrix::zero(2, 2);
        let expected: Matrix<u32> = Matrix(vec![vec![0, 0], vec![0, 0]]);
        assert_eq!(matrix, expected);
    }
    #[test]
    fn identy_matrix() {
        let matrix: Matrix<u32> = Matrix::identity(2);
        let expected: Matrix<u32> = Matrix(vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(matrix, expected);
        let matrix: Matrix<u32> = Matrix::identity(3);
        let expected: Matrix<u32> = Matrix(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);
        assert_eq!(matrix, expected);
    }
}
