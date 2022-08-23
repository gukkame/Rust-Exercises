use std::str::FromStr;
pub fn delete_and_backspace(s: &mut String) {
    let mut vec = Vec::new();
    let mut sum = 0;
    for ch in s.chars() {
        vec.push(ch);
        if sum > 0 && ch != '+' {
            vec.remove(vec.len() - 1);
            sum -= 1;
            continue;
        }
        if ch == '-' {
            vec.remove(vec.len() - 2);
            vec.remove(vec.len() - 1);
        }
        if ch == '+' {
            vec.remove(vec.len() - 1);
            sum += 1;
        }
    }
    let string: String = vec.iter().collect::<String>();
    *s = string;
}

pub fn is_correct(v: &mut Vec<&str>) -> usize {
    let vec = v.clone();
    let mut sum = 0;
    for eq in vec {
        let strings: Vec<&str> = eq.split("=").collect();
        let res: i32 = FromStr::from_str(strings[1]).unwrap();
        let num: Vec<&str> = strings[0].split("+").collect();

        if num.len() == 1 {
            let number: Vec<&str> = strings[0].split("-").collect();
            let a: i32 = FromStr::from_str(number[0]).unwrap();
            let b: i32 = FromStr::from_str(number[1]).unwrap();
            let c = a - b;
            if c == res {
                sum += 25;
                v.push("✔");
            } else {
                v.push("✘");
            }
        } else {
            let a: i32 = FromStr::from_str(num[0]).unwrap();
            let b: i32 = FromStr::from_str(num[1]).unwrap();
            let c = a + b;
            if c == res {
                sum += 25;
                v.push("✔");
            } else {
                v.push("✘");
            }
        }
    }
    v.remove(0);
    v.remove(0);
    v.remove(0);
    v.remove(0);
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_string() {
        let mut a_1 = String::from("bpp--o+er+++sskroi-++lcw");
        let mut a_2 = String::from("hs-+deasdasd------l+++dsdp");
        let mut a_3 = String::from("pad-rtic+eulqw--+rar");
        delete_and_backspace(&mut a_1);
        delete_and_backspace(&mut a_2);
        delete_and_backspace(&mut a_3);
        assert_eq!(a_1, "borrow".to_string());
        assert_eq!(a_2, "help".to_string());
        assert_eq!(a_3, "particular".to_string());
    }

    #[test]
    fn reference_vec() {
        let mut b_1: Vec<&str> = vec!["2+2=4", "3+2=5", "10-3=3", "5+5=10"];
        let mut b_2: Vec<&str> = vec!["1+2=4", "0+2=5", "10-3=3", "41+5=10"];
        let mut b_3: Vec<&str> = vec!["2+2=4", "3+2=5", "10-3=7", "5+5=10"];
        let result_1 = is_correct(&mut b_1);
        let result_2 = is_correct(&mut b_2);
        let result_3 = is_correct(&mut b_3);
        assert_eq!(result_1, 75);
        assert_eq!(result_2, 0);
        assert_eq!(result_3, 100);
        assert_eq!(b_1, vec!["✔", "✔", "✘", "✔"]);
        assert_eq!(b_2, vec!["✘", "✘", "✘", "✘"]);
        assert_eq!(b_3, vec!["✔", "✔", "✔", "✔"]);
    }
}
