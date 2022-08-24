pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let vec = s1.as_bytes();
    let vec2 = s2.as_bytes();
    let sum: u8 = vec.iter().sum();
    let sum2: u8 = vec2.iter().sum();
    if sum == sum2 {
        true
    } else {
        false
    }
}

