pub fn sum(arr: &[i32]) -> i32 {
    let sum: i32 = arr.iter().sum();
    sum
}

pub fn thirtytwo_tens() -> [i32; 32] {
    let arr: [i32; 32] = [10; 32];
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thirtytwo_tens() {
        assert_eq!(thirtytwo_tens(), [10; 32]);
    }
    #[test]
    fn test_sum() {
        let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(sum(&a), a.iter().sum());
    }
}
