#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

pub trait Scalar: Sized {}
impl Scalar for i64 {}

use std::ops::Add;

impl<T: Add<Output = T> + Scalar + Clone> Add for Vector<T> {
    type Output = Option<Vector<T>>;

    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }

        let mut vector = Vec::new();
        for i in 0..self.0.len() {
            vector.push(self.0[i].clone() + other.0[i].clone())
        }
        return Some(Vector(vector));
    }
}

impl<T: Scalar> Vector<T> {
    pub fn new() -> Self {
        Vector(vec![])
    }

    pub fn dot(&self, other: &Self) -> Option<i64>
    where
        i64: Scalar,
    {
        if self.0.len() != other.0.len() {
            return None;
        }
        return Some(self.0.len() as i64);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn dot_product() {
//         let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
//         let vector_2: Vector<i64> = Vector(vec![4, -2, -1]);
//         let expected: i64 = 3;
//         assert_eq!(vector_1.dot(&vector_2), Some(expected));
//         let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
//         let vector_2: Vector<i64> = Vector(vec![4, -2]);
//         assert_eq!(vector_1.dot(&vector_2), None);
//     }
//     #[test]
//     fn addition() {
//         let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
//         let vector_2: Vector<i64> = Vector(vec![4, -2, -1]);
//         assert_eq!(vector_1 + &vector_2, Some(Vector(vec![5i64, 1, -6])));
//         let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
//         let vector_2: Vector<i64> = Vector(vec![2, 4, -2, -1]);
//         assert_eq!(None, vector_1 + &vector_2);
//     }
// }
