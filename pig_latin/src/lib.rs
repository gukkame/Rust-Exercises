pub fn pig_latin(text: &str) -> String {
    let mut str = String::new();
    let first = text.chars().nth(0).unwrap();

    let mut input = text.to_string().clone();
    if first == 'a' || first == 'i' || first == 'e' || first == 'u' || first == 'o' {
        str.push_str(text);
    } else {
        let mut temp = String::new();
        for (i, ch) in text.chars().enumerate() {
            if ch == 'a' || ch == 'i' || ch == 'e' || ch == 'o' || ch == 'u' {
                println!("CH2 {:?}", ch);
                str.push_str(&input);
                str.push_str(temp.as_str());
                break;
            } else {
                if text.chars().nth(i).unwrap() == 'q' && text.chars().nth(i + 1).unwrap() == 'u' {
                    let mut edit = text.to_string().clone();
                    edit.remove(0);
                    edit.remove(0);
                    if i == 1 {
                        edit.remove(0);
                        str.push_str(edit.as_str());
                        str.push(text.chars().nth(0).unwrap())
                    } else {
                        str.push_str(edit.as_str());
                    }
                    str.push(ch);
                    str.push('u');
                    str.push_str("ay");
                    return str;
                } else {
                    input.remove(0);
                    temp.push(ch);
                }
            }
        }
    }
    str.push_str("ay");
    str
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_word_beginning_with_consonant() {
        assert_eq!(pig_latin(&String::from("queen")), "eenquay");
        assert_eq!(pig_latin(&String::from("square")), "aresquay");
        assert_eq!(pig_latin(&String::from("equal")), "equalay");
        assert_eq!(pig_latin(&String::from("pig")), "igpay");
        assert_eq!(pig_latin(&String::from("koala")), "oalakay");
        assert_eq!(pig_latin(&String::from("yellow")), "ellowyay");
        assert_eq!(pig_latin(&String::from("xenon")), "enonxay");
        assert_eq!(pig_latin(&String::from("qat")), "atqay");
        assert_eq!(pig_latin(&String::from("chair")), "airchay");
        assert_eq!(pig_latin(&String::from("therapy")), "erapythay");
        assert_eq!(pig_latin(&String::from("thrush")), "ushthray");
        assert_eq!(pig_latin(&String::from("school")), "oolschay");
    }
    #[test]
    fn test_word_beginning_with_vowel() {
        assert_eq!(pig_latin(&String::from("apple")), "appleay");
        assert_eq!(pig_latin(&String::from("ear")), "earay");
        assert_eq!(pig_latin(&String::from("igloo")), "iglooay");
        assert_eq!(pig_latin(&String::from("object")), "objectay");
        assert_eq!(pig_latin(&String::from("under")), "underay");
    }
}
