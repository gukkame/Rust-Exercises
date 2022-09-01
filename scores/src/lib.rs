pub fn score(s: &str) -> u64 {
    let mut score = 0;

    for ch in s.to_ascii_uppercase().chars() {
         score += match ch {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
            'D' | 'G' => 2,
            'B' | 'C' | 'M' | 'P' => 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => 4,
            'K' => 5,
            'J' | 'X' => 8,
            'Q' | 'Z' => 10,
            _ => 0,
        };
    }
   score
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple() {
        assert_eq!(score("a"), 1);
        assert_eq!(score("A"), 1);
        assert_eq!(score("h"), 4);
        assert_eq!(score("at"), 2);
        assert_eq!(score("Yes"), 6);
        assert_eq!(score("cellphones"), 17);
    }
    #[test]
    fn test_empty() {
        assert_eq!(score(""), 0);
        assert_eq!(score(" "), 0);
    }
    #[test]
    fn test_special() {
        assert_eq!(score("in Portugal NÃO stands for: no"), 30);
        assert_eq!(score("This is a test, comparação, têm Água?"), 36);
    }
    #[test]
    fn test_long() {
        assert_eq!(score("ThiS is A Test"), 14);
        assert_eq!(score("long sentences are working"), 34);
        assert_eq!(score("abcdefghijklmnopqrstuvwxyz"), 87);
    }
}
