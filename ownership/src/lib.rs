pub fn first_subword(mut s: String) -> String {
    let mut first_word = String::from("");
    for (i, c) in s.chars().enumerate() { 
      if c == '_' || (c.is_uppercase() && i != 0) {
          break
      }
      first_word.push(c);
    }
    first_word
}

