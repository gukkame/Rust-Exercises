pub fn first_fifty_even_square() -> Vec<i32> {
    let mut vec = Vec::new();
    for x in 2..=100 {
        if x % 2 == 0 {
            vec.push(x * x);
        }
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let v1 = vec![
            4, 16, 36, 64, 100, 144, 196, 256, 324, 400, 484, 576, 676, 784, 900, 1024, 1156, 1296,
            1444, 1600, 1764, 1936, 2116, 2304, 2500, 2704, 2916, 3136, 3364, 3600, 3844, 4096,
            4356, 4624, 4900, 5184, 5476, 5776, 6084, 6400, 6724, 7056, 7396, 7744, 8100, 8464,
            8836, 9216, 9604, 10000,
        ];
        assert_eq!(v1, first_fifty_even_square());
    }
}
