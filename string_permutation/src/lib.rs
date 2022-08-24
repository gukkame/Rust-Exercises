pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let vec = s1.as_bytes();
    let vec2 = s2.as_bytes();
    if vec == vec2 {
        true
    } else {
        false
    }
}

