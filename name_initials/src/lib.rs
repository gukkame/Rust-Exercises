pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut vec = Vec::new();
    for string in names {
        let mut str = String::new();
        for ch in string.chars() {
            if ch.is_uppercase() {
                str.push(ch);
                str.push('.'); 
                str.push(' '); 
            }
        }
        str.pop();
        vec.push(str);
    }
    println!("AAA{:?}", vec);
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
