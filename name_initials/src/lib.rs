pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut vec = Vec::new();
    for string in names {
        let mut str: String = "".to_owned();
        let mut i = 0;
        for ch in string.chars() {
            if ch.is_uppercase() {
                i += 1;
                str.push_str((ch.to_string() + ".").as_str());
                if i == 1 {
                    str.push_str(" ");
                }
            }
        }
        vec.push(str);
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initials_test() {
        let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
        let result = initials(names);
        assert_eq!(result, ["H. P.", "S. E.", "J. L.", "B. O."]);
    }
}
