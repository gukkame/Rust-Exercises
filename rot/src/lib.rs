pub fn rotate(input: &str, key: i8) -> String {
    let alph_low = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz";
    let alph_up = "abcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyz".to_ascii_uppercase();
    println!("Given input: {:?}, {:?}", input, key);
    let mut string = String::new();
    for (i, ch) in input.chars().enumerate() {
        if ch.is_ascii_lowercase() {
            let mut once = false;
            if key.is_negative() {
                let rev_alph = alph_low.chars().rev().collect::<String>();
                println!("Given alh: {:?}", rev_alph);
                for (j, letter) in rev_alph.chars().enumerate() {
                    if letter == ch && once == false {
                        println!("NNth: {:?}", j);
                        println!("NNth2: {:?}", (key * -1) );

                        string.push(
                            rev_alph
                                .chars()
                                .nth((j as i32 + (key as i32) * -1 ).try_into().unwrap())
                                .unwrap(),
                        );
                        once = true;
                    }
                }
            }

            for (j, letter) in alph_low.chars().enumerate() {
                if letter == ch && once == false {
                    println!("Nth: {:?}", j);
                    println!("Nth2: {:?}", key);

                    string.push(
                        alph_low
                            .chars()
                            .nth((j as i32 + key as i32).try_into().unwrap())
                            .unwrap(),
                    );
                    once = true;
                }
            }
        }
        if ch.is_ascii_uppercase() {
            let mut once = false;
            if key.is_negative() {
                let rev_alph = alph_up.chars().rev().collect::<String>();
                println!("Given alph: {:?}", rev_alph);
                for (j, letter) in rev_alph.chars().enumerate() {
                    if letter == ch && once == false {
                        println!("NNth: {:?}", j);
                        println!("NNth2: {:?}", (key * -1) - 1);

                        string.push(
                            rev_alph
                                .chars()
                                .nth((j as i32 + (key as i32) * -1 ).try_into().unwrap())
                                .unwrap(),
                        );
                        once = true;
                    }
                }
            }
            for (j, letter) in alph_up.chars().enumerate() {
                if letter == ch && once == false {
                    println!("Nth: {:?}", j);
                    println!("Nth2: {:?}", key);

                    string.push(
                        alph_up
                            .chars()
                            .nth((j as i32 + key as i32).try_into().unwrap())
                            .unwrap(),
                    );
                    once = true;
                }
            }
        }
        if ch.is_ascii_punctuation() || ch == ' ' || ch.is_ascii_digit() {
            string.push(ch);
        }
    }
    println!("String: {:?}", string);

    string
}

// abcde ---- I get b and -2 it would be e
//edcba I get  el 1

// abcdefgh --- I get e and -7 it would be f
//hgfedcba el 6

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_simple() {
        assert_eq!("z", rotate("m", 13));
        assert_eq!("n", rotate("m", 1));
        assert_eq!("a", rotate("a", 26));
        assert_eq!("z", rotate("a", 25));
        assert_eq!("b", rotate("l", 16));
        assert_eq!("j", rotate("j", 0));
        assert_eq!("RNXX", rotate("MISS", 5));
        assert_eq!("M J Q Q T", rotate("H E L L O", 5));
    }
    #[test]
    fn test_all_letters() {
        assert_eq!(
            "Gur svir obkvat jvmneqf whzc dhvpxyl.",
            rotate("The five boxing wizards jump quickly.", 13)
        );
    }
    #[test]
    fn test_numbers_punctuation() {
        assert_eq!(
            "Xiwxmrk amxl ryqfivw 1 2 3",
            rotate("Testing with numbers 1 2 3", 4)
        );
        assert_eq!("Gzo\'n zvo, Bmviyhv!", rotate("Let\'s eat, Grandma!", 21));
    }
    #[test]
    fn test_negative() {
        assert_eq!("z", rotate("a", -1));
        assert_eq!("W", rotate("A", -4));
        assert_eq!("Fqefuzs", rotate("Testing", -14));
    }
}

// abcde ---- I get b and -2 it would be e
//edcba I get  el 1

// abcdefgh --- I get e and -7 it would be f
//hgfedcba el 6

// abcdefgh --- I get b and -4 it would be f
//hgfedcba el 3
