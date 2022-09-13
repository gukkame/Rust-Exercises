pub use lalgebra_scalar::Scalar;
mod ops;
mod mult;
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
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
        return Matrix(matrix);
    }
    pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix: Vec<Vec<T>> = Vec::new();
        for i in 0..n {
            let mut vec: Vec<T> = Vec::new();
            for j in 0..n {
                match i as i8 - j as i8 {
                    0 => vec.push(T::one()),
                    _ => vec.push(T::zero()),
                }
            }
            matrix.push(vec);
        }
        return Matrix(matrix);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_row() {
        let matrix: Matrix<u32> = Matrix(vec![vec![3, 6], vec![8, 0]]);
        assert_eq!(vec![3u32, 6], matrix.row(0));
        assert_eq!(vec![8u32, 0], matrix.row(1));
    }
    #[test]
    fn get_col() {
        let matrix: Matrix<u32> = Matrix(vec![vec![3, 6], vec![8, 0]]);
        assert_eq!(matrix.col(0), vec![3u32, 8]);
        assert_eq!(vec![6u32, 0], matrix.col(1));
    }
    #[test]
    fn matrix_multiplication() {
        let matrix_1: Matrix<u32> = Matrix(vec![vec![0, 1], vec![0, 0]]);
        let matrix_2: Matrix<u32> = Matrix(vec![vec![0, 0], vec![1, 0]]);
        let expected: Matrix<u32> = Matrix(vec![vec![1, 0], vec![0, 0]]);
        assert_eq!(matrix_1 * matrix_2, Some(expected));
/* 
0 1   0 0  1 0 
0 0   1 0  0 0
 


*/



        let matrix_1: Matrix<u32> = Matrix(vec![vec![0, 1], vec![0, 0]]);
        let matrix_2: Matrix<u32> = Matrix(vec![vec![0, 0, 0], vec![1, 0, 0], vec![1, 1, 1]]);
        assert_eq!(matrix_1 * matrix_2, None);
    }
}
