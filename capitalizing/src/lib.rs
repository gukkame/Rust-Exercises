pub fn capitalize_first(input: &str) -> String {
    // let mut string = String::from;
    let mut string = String::from("");
    for (i, ch) in input.chars().enumerate() {
        if i == 0 {
            let u = ch.to_ascii_uppercase();
            string.push(u);
        } else {
            string.push(ch)
        }
    }
    string
}

pub fn title_case(input: &str) -> String {
    let mut string = String::from("");
    let mut space = false;
    for (i, ch) in input.chars().enumerate() {
        if i == 0 || space == true {
            let u = ch.to_ascii_uppercase();
            string.push(u);
            space = false;
        } else if ch == ' '{
            space = true;
            string.push(ch);
        }else{
            string.push(ch);
        }
    }
    string
}

pub fn change_case(input: &str) -> String {
    let mut string = String::from("");
    for  ch in input.chars() {
        if ch.is_uppercase() {
            let u = ch.to_ascii_lowercase();
            string.push(u);
            
        } else{
            let u = ch.to_ascii_uppercase();
            string.push(u);
        }
    }
    string
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(capitalize_first("joe is missing"), "Joe is missing");
        assert_eq!(title_case("jill is leaving A"), "Jill Is Leaving A");
        assert_eq!(change_case("heLLo THere"), "HEllO thERE");
    }
}
