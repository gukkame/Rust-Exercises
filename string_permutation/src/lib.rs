pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let vec = s1.bytes();
    let vec2 = s2.bytes();
    let mut sum = 0;
    let mut sum2 = 0;
    for val in vec {
        sum += val as i32;
    }
    for val in vec2 {
        sum2 += val as i32;
    }
    if sum == sum2 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation_ascii() {
        assert!(is_permutation("abcde", "edbca"));
        assert!(!is_permutation("avcde", "edbca"));
        assert!(!is_permutation("cde", "edbca"));
        assert!(is_permutation("code", "deco"));
        assert!(!is_permutation("code", "deeco"));
        assert!(!is_permutation("codde", "deeco"));
    }
    #[test]
    fn permutation_unicode() {
        assert!(is_permutation("hello♥", "♥oelhl"));
        assert!(!is_permutation("♥", "♥♥"));
    }
}
