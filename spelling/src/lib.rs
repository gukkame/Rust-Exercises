pub fn spell(n: u64) -> String {
    // let rev_nbr = n.to_string().chars().rev().collect::<String>();
    let mut str = String::new();

    if n < 100 {
        str = two_dig_num(n);
    } else if n > 99 && n < 1000 {
        str = three_dig_num(n);
    } else if n > 999 && n < 1000000 {
        str = thousands(n);
    }else if n == 1000000{
        str = "one million".to_string();
    }
    println!("String {}, {}", n, str);
    str
}
pub fn thousands(n: u64) -> String {
    let mut str = String::new();
    let num_len = n.to_string().len();
    println!("Number : {}", n);
    if num_len == 4 {
        let st = n.to_string().chars().nth(0).unwrap().to_digit(10).unwrap();
        str.push_str(ones(st).as_str());
    } else if num_len == 5 {
        let len = n / 1000;
        println!("5 dig number to first 2 dig: {}", len);
        str.push_str(two_dig_num(len).as_str());
    } else if num_len == 6 {
        let len = n / 1000;
        println!("6 dig number to first 3 dig: {}", len);
        str.push_str(three_dig_num(len).as_str());
    }
    str.push_str(" thousand");
    let len = n % 1000;
    if n != 1000 {
        if len.to_string().len() == 2 {
            str.push(' ');
            str.push_str(two_dig_num(len).as_str());
        }
        if len.to_string().len() == 3 {
            str.push(' ');
            str.push_str(three_dig_num(len).as_str());
        }
    }
    return str;
}
pub fn three_dig_num(n: u64) -> String {
    let mut str = String::new();
    let st = n.to_string().chars().nth(0).unwrap().to_digit(10).unwrap();
    let s = ones(st);
    str.push_str(s.as_str());
    str.push_str(" hundred ");
    let len = n % 100;
    str.push_str(two_dig_num(len).as_str());

    return str;
}
pub fn two_dig_num(n: u64) -> String {
    let mut str = String::new();
    if n < 10 {
        let s = ones(n.to_string().chars().nth(0).unwrap().to_digit(10).unwrap());
        str.push_str(s.as_str());
    } else if n < 20 && n > 12 {
        let s = ones(n.to_string().chars().nth(1).unwrap().to_digit(10).unwrap());
        str.push_str(s.as_str());
        str.push_str("teen");
    } else if n == 11{
        str.push_str("eleven");
    } else if n == 12{
        str.push_str("twelve");
    }else {
        let s = tens(n.to_string().chars().nth(0).unwrap().to_digit(10).unwrap());
        str.push_str(s.as_str());
        if n.to_string().chars().nth(1).unwrap().to_digit(10).unwrap() != 0 {
            let s2 = ones(n.to_string().chars().nth(1).unwrap().to_digit(10).unwrap());
            str.push('-');
            str.push_str(s2.as_str());
        }
    }

    return str;
}

pub fn tens(n: u32) -> String {
    return match n {
        0 => "hundred",
        1 => "ten",
        2 => "twenty",
        3 => "thirty",
        4 => "fourty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninty",
        _ => "",
    }
    .to_string();
}

pub fn ones(n: u32) -> String {
    return match n {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        _ => "",
    }
    .to_string();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_one() {
        assert_eq!(spell(0), String::from("zero"));
        assert_eq!(spell(1), String::from("one"));
        assert_eq!(spell(14), String::from("fourteen"));
        assert_eq!(spell(20), String::from("twenty"));
        assert_eq!(spell(22), String::from("twenty-two"));
        assert_eq!(spell(101), String::from("one hundred one"));
        assert_eq!(spell(120), String::from("one hundred twenty"));
        assert_eq!(spell(123), String::from("one hundred twenty-three"));
        assert_eq!(spell(1000), String::from("one thousand"));
        assert_eq!(spell(1055), String::from("one thousand fifty-five"));
        assert_eq!(
            spell(1234),
            String::from("one thousand two hundred thirty-four")
        );
        assert_eq!(
            spell(10123),
            String::from("ten thousand one hundred twenty-three")
        );
        assert_eq!(
            spell(910112),
            String::from("nine hundred ten thousand one hundred twelve")
        );
        assert_eq!(
            spell(651123),
            String::from("six hundred fifty-one thousand one hundred twenty-three")
        );
        assert_eq!(spell(810000), String::from("eight hundred ten thousand"));
        assert_eq!(spell(1000000), String::from("one million"));
    }
}
