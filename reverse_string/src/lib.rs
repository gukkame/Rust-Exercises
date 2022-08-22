pub fn rev_str(input: &str) -> String {
    let mut string = String::new();
    for word in input.split_whitespace().rev() {
        for c in word.chars().rev() {
            string.push(c);       
        }
      string.push(' ');
    }
    string
}
