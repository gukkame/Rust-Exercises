pub fn arrange_phrase(phrase: &str) -> String {
    let vec: Vec<&str> = phrase.split_whitespace().collect();
    let mut vec2: Vec<&str> = phrase.split_whitespace().collect();

    for word in vec {
        for ch in word.chars() {
            if ch.is_numeric() {
                let i: u32 = ch.to_digit(10).unwrap();
                vec2[(i as usize) - 1] = word;
            }
        }
    }
    let mut string = String::new();
    for word in vec2 {
        for ch in word.chars() {
            if !ch.is_numeric()  {
                string.push(ch);
            }
        }
        string.push(' ');
    }

    string.pop();
    string
}

#[cfg(test)]
mod tests {
    use super::*;
    #[allow(dead_code)]
    fn arrange_phrase_sol(phrase: &str) -> String {
        let nbrs: Vec<&str> = phrase.matches(char::is_numeric).collect();
        let a = &phrase.replace(char::is_numeric, "");
        let mut m: Vec<&str> = a.split_whitespace().collect();
        for (i, ele) in nbrs.iter().enumerate() {
            let strs: Vec<&str> = a.split_whitespace().collect();
            m[ele.parse::<usize>().unwrap() - 1] = strs[i];
        }
        m.join(" ")
    }
    #[test]
    fn test_function() {
        let cases = vec![
            "4of Fo1r pe6ople g3ood th5e the2",
            "is2 Thi1s T4est 3a",
            "w7ork t3he a4rt o5f Per1formance is2 a6voiding",
        ];
        for v in cases {
            assert_eq!(arrange_phrase(v), arrange_phrase_sol(v));
        }
    }
}
