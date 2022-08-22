pub fn rev_str(input: &str) -> String {
    let mut string = String::new();
    for word in input.split_whitespace().rev() {
        for c in word.chars().rev() {
            string.push(c);       
           
        } string.push(' '); 
     
    }
    let trim1 = string.trim();
    trim1.to_string()
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[allow(dead_code)]
    fn test_reverse(input: &str, expected: &str) {
        assert_eq!(&rev_str(input), expected);
    }
    #[test]
    // testing just one word
    fn test_simple_word() {
        test_reverse("robot", "tobor");
        test_reverse("Ramen", "nemaR");
        test_reverse("I'm hungry!", "!yrgnuh m'I");
        test_reverse("racecar", "racecar");
        test_reverse("drawer", "reward");
        test_reverse("子猫", "猫子");
        test_reverse("", "");
    }
    #[test]
    // testing two or more words
    fn test_more_than_one() {
        test_reverse("Hello, world!", "!dlrow ,olleH");
        test_reverse("Hello, my name is Roman", "namoR si eman ym ,olleH");
        test_reverse("I have a nice car!", "!rac ecin a evah I");
        test_reverse("How old are You", "uoY era dlo woH");
        test_reverse("ex: this is an example água", "augá elpmaxe na si siht :xe");
    }
}
