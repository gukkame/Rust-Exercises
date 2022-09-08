#[derive(Debug, Copy, Clone, PartialEq)]
pub struct ThreeDVector<T> {
	pub i: T,
	pub j: T,
	pub k: T,
}

use std::ops::{Add, Sub};

impl<T: Add<Output = T>> Add for ThreeDVector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            i: self.i + other.i,
            j: self.j + other.j,
            k: self.k + other.k,
        }
    }
}

impl<T: Sub<Output = T>> Sub for ThreeDVector<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            i: self.i - other.i,
            j: self.j - other.j,
            k: self.k - other.k,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    fn main() {
        let a = ThreeDVector { i: 3, j: 5, k: 2 };
        let b = ThreeDVector { i: 2, j: 7, k: 4 };
        println!("{:?}", a + b);
    }
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    #[test]
    fn test_addition() {
        let a = ThreeDVector { i: 3, j: 5, k: 2 };
        let b = ThreeDVector { i: 2, j: 7, k: 4 };
        let a_plus_b = ThreeDVector { i: 5, j: 12, k: 6 };
        assert_eq!(a + b, a_plus_b);
    }
    #[test]
    fn test_subtraction() {
        let a = ThreeDVector { i: 3, j: 5, k: 2 };
        let b = ThreeDVector { i: 2, j: 7, k: 4 };
        let a_minus_b = ThreeDVector { i: 1, j: -2, k: -2 };
        assert_eq!(a - b, a_minus_b);
    }
}
