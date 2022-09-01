pub fn num_to_ordinal(x: u32) -> String {
    let mut string = x.to_string();
    if string == "11" || string == "12" || string == "13" {
        string.push_str("th");
        return string;
    }
    let last_nb = x.clone().to_string().chars().last().unwrap();
    let str = match last_nb {
        '1' => "st",
        '2' => "nd",
        '3' => "rd",
        _ => "th",
    };
    string.push_str(str);
    return string;
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_to_ordinal() {
        assert_eq!(num_to_ordinal(1), "1st");
        assert_eq!(num_to_ordinal(12), "12th");
        assert_eq!(num_to_ordinal(0), "0th");
        assert_eq!(num_to_ordinal(22), "22nd");
        assert_eq!(num_to_ordinal(43), "43rd");
        assert_eq!(num_to_ordinal(67), "67th");
        assert_eq!(num_to_ordinal(1901), "1901st");
    }
}
