pub fn is_empty(v: &str) -> bool {
    v.is_empty()
    // if &v.len().to_string() == "0" {
    //     true
    // } else {
    //     false
    // }
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
    // let mut flag = true;
    // for ch in v.chars() {
    //     let ascii = ch as char as u8;
    //     if ascii > 31 && ascii < 128 {
    //         flag = true;
    //     } else {
    //         flag = false;
    //         break;
    //     }
    // }
    // flag
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
    // let vec: Vec<&str> = v.split(pat).collect();
    // if vec.len() == 1 {
    //     false
    // } else {
    //     true
    // }
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {
   v.find(pat).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functions() {
        assert!(is_empty(""));
        assert!(!is_empty("something"));
        assert!(is_ascii("rust"));
        assert!(!is_ascii("rust¬"));
        assert!(contains("rust", "ru"));
        assert!(!contains("something", "mer"));
        assert_eq!(split_at("rust", 2), ("ru", "st"));
        assert_eq!(find("ru-st-e", '-'), 2);
        assert_eq!(find("ru-st-e", 'e'), 6);
    }
}
