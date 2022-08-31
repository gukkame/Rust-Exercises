extern crate case;
pub fn expected_variable(orig: &str, exp: &str) -> Option<String> {
    let mut cap = false;
    let mut line = false;
    if orig.is_empty() {
        // println!("Empty");
      return None
    }
    if exp.is_empty() {
        // println!("2Empty");
      return None
    }
    for ch in orig.chars() {
        if ch == '_' {
            line = true;
            break;
        }
        if ch.is_ascii_uppercase() {
            cap = true;
            break;
        }
    }
    // println!("{:?}, {:?}", cap, line);
    if cap == true || line == true {
        let size = edit_distance(orig.to_lowercase().as_str(), exp.to_lowercase().as_str());
        let res: f64 = 100.0-(size as f64 * 100.0 / exp.len() as f64);
        let mut str_res =(res.round() as i16).to_string();
        str_res.push_str("%");
       return Some(str_res)
    } else {
       return None
    }
}
pub fn edit_distance(a: &str, b: &str) -> usize {
    let mut result = 0;

    /* Shortcut optimizations / degenerate cases. */
    if a == b {
        return result;
    }

    let length_a = a.chars().count();
    let length_b = b.chars().count();

    if length_a == 0 {
        return length_b;
    }

    if length_b == 0 {
        return length_a;
    }

    let mut cache: Vec<usize> = (1..).take(length_a).collect();
    let mut distance_a;
    let mut distance_b;

    for (index_b, code_b) in b.chars().enumerate() {
        result = index_b;
        distance_a = index_b;

        for (index_a, code_a) in a.chars().enumerate() {
            distance_b = if code_a == code_b {
                distance_a
            } else {
                distance_a + 1
            };

            distance_a = cache[index_a];

            result = if distance_a > result {
                if distance_b > result {
                    result + 1
                } else {
                    distance_b
                }
            } else if distance_b > distance_a {
                distance_a + 1
            } else {
                distance_b
            };

            cache[index_a] = result;
        }
    }

    result
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn expected_variable_test() {
        assert_eq!(
            None,
            expected_variable("On_Point", "on_point")
        );
    }
    #[test]
    fn expected_variable_test2() {
        assert_eq!(
            Some("88% close to it".to_string()),
            expected_variable("soClose", "So_Close")
        );
    }
    #[test]
    fn expected_variable_test3() {
        assert_eq!(
            None,
            expected_variable("something", "something_completely_different")
        );
    }
    #[test]
    fn expected_variable_test4() {
        assert_eq!(
            Some("67% close to it".to_string()),
            expected_variable("BenedictCumberbatch", "BeneficialCucumbersnatch")
        );
    }
}