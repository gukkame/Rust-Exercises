#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    // expected public fields
}
impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {

    }
}
pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
