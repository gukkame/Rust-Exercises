#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    validation: bool,
    expected: String,
}
impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation: validation,
            expected: expected,
        }
    }
}
pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    if original == "" || ciphered == "" {
        return None;
    }
    let mut string = "".to_string();
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let t = alphabet.chars().rev().collect::<String>();

    for ch in original.chars() {
        if ch.is_ascii_digit() || ch == ' ' || ch.is_ascii_punctuation() {
            string.push(ch);
        } else if ch.is_ascii_alphabetic() {
            let mut num = 0;
            for (i, n) in alphabet.chars().enumerate() {
                if n == ch || n == ch.to_ascii_lowercase() {
                    num = i;
                }
            }
            if ch.is_uppercase() {
                string.push(t.chars().nth(num).unwrap().to_ascii_uppercase())
            } else {
                string.push(t.chars().nth(num).unwrap())
            }
        }
    }

    if string.to_owned() == ciphered {
        return Some(Ok(true));
    } else {
        Some(Err(CipherError {
            validation: false,
            expected: string,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cipher() {
        assert_eq!(cipher("1Hello 2world!", "1Svool 2dliow!"), Some(Ok(true)));
        assert_eq!(cipher("", "1Svool 2dliow!"), None);
        assert_eq!(cipher("1Hello 2world!", ""), None);
        assert_eq!(
            cipher("1Hello 2world!", "1svool 2dliow!"),
            Some(Err(CipherError {
                validation: false,
                expected: String::from("1Svool 2dliow!")
            }))
        );
        assert_eq!(cipher("asdasd", "zhwzhw"), Some(Ok(true)));
        assert_eq!(
            cipher("asdasd", "lkdas"),
            Some(Err(CipherError {
                validation: false,
                expected: String::from("zhwzhw")
            }))
        );
        assert_eq!(cipher("3(/&%fsd 32das", "3(/&%uhw 32wzh"), Some(Ok(true)));
        assert_eq!(
            cipher("3(/&%sd 32das", "3(/&%uhw 32wzh"),
            Some(Err(CipherError {
                validation: false,
                expected: String::from("3(/&%hw 32wzh")
            }))
        );
    }
}
