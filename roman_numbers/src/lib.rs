use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla,
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
    fn from(_: u32) -> Self {
        RomanDigit::X
    }
}

impl From<u32> for RomanNumber {
    fn from(n: u32) -> Self {
        let mut number = n.clone();
        println!("NUMBER: {}", number);
        let mut vec: Vec<RomanDigit> = Vec::new();
        let conversions = vec![
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];
        for (i, str) in conversions {
            println!("I {}", i);
            println!("STR: {}", str);
            while number >= i {
                for ch in str.chars() {
                    //  vec.push(str);
                    let el = match ch {
                        'M' => RomanDigit::M,
                        'D' => RomanDigit::D,
                        'C' => RomanDigit::C,
                        'L' => RomanDigit::L,
                        'X' => RomanDigit::X,
                        'V' => RomanDigit::V,
                        'I' => RomanDigit::I,
                        _ => RomanDigit::Nulla,
                    };
                    vec.push(el);
                }
                number -= i;
            }
        }

        RomanNumber { 0: vec }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(RomanNumber::from(3).0, [I, I, I]);
        assert_eq!(RomanNumber::from(6).0, [V, I]);
        assert_eq!(RomanNumber::from(15).0, [X, V]);
        assert_eq!(RomanNumber::from(30).0, [X, X, X]);
        assert_eq!(RomanNumber::from(150).0, [C, L]);
        assert_eq!(RomanNumber::from(200).0, [C, C]);
        assert_eq!(RomanNumber::from(600).0, [D, C]);
        assert_eq!(RomanNumber::from(1500).0, [M, D]);
    }
    #[test]
    fn substractive_notation() {
        assert_eq!(RomanNumber::from(4).0, [I, V]);
        assert_eq!(RomanNumber::from(44).0, [X, L, I, V]);
        assert_eq!(RomanNumber::from(3446).0, [M, M, M, C, D, X, L, V, I]);
        assert_eq!(RomanNumber::from(9).0, [I, X]);
        assert_eq!(RomanNumber::from(94).0, [X, C, I, V]);
    }
}
