pub use lalgebra_scalar::{Scalar}; 
mod ops;
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
    fn addition() {
        let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let expected = Matrix(vec![vec![2, 2], vec![2, 2]]);
        assert_eq!(matrix + matrix_2, Some(expected));
        let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
        assert_eq!(matrix + matrix_2, None);
    }
    #[test]
    fn subtraction() {
        let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let expected = Matrix(vec![vec![0, 0], vec![0, 0]]);
        assert_eq!(matrix - matrix_2, Some(expected));
        let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
        assert_eq!(matrix - matrix_2, None);
    }

}
