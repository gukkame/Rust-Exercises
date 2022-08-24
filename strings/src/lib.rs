pub fn char_length(s: &str) -> usize {
    s.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = char_length("hello");
        let result1 =  char_length("形聲字");
        let result2 = char_length("😍");
        assert_eq!(result, 5);
        assert_eq!(result1, 3);
        assert_eq!(result2, 1);
    }
}
