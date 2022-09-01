pub fn number_logic(num: u32) -> bool {
    let len = num.to_string().len();
    let mut n = 0;
    let mut res = 0;
    println!("{}", len);
    for ch in num.to_string().chars() {
        println!("ch {}", ch);
        n = ch.to_digit(10).unwrap();
        res += n.pow(len as u32);
        println!("{}", n);
    }
    if res == num {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_zero() {
        assert!(number_logic(0))
    }
    #[test]
    fn test_single_digit_numbers() {
        assert!(number_logic(5));
        assert!(number_logic(9))
    }
    #[test]
    fn test_two_digit_numbers() {
        assert!(!number_logic(10))
    }
    #[test]
    fn test_three_or_more_digit_number() {
        assert!(number_logic(153));
        assert!(!number_logic(100));
        assert!(number_logic(9474));
        assert!(!number_logic(9475));
        assert!(number_logic(9_926_315));
        assert!(!number_logic(9_926_316))
    }
}
