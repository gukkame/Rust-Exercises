pub fn get_diamond(c: char) -> Vec<String> {
    let alph = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let len = alph.find(c).unwrap() * 2 + 1;
    let (first, _) = alph.split_at((len / 2) + 1);
    let mut vec: Vec<String> = Vec::new();
    for (i, ch) in first.chars().enumerate() {
        let mut temp = String::new();
        if i == 0 {
            let mut j = 0;
            while j < len {
                if len / 2 == j {
                    temp.push(ch);
                } else {
                    temp.push(' ');
                }
                j += 1;
            }
            vec.push(temp);
        } else if i != first.len()-1 {
            let mut j = 1;
            while j <= len {
                let pos = len/2-i+1;
                if pos == j  {//left side
                    temp.push(ch);
                } else if (len  - (pos) + 1) == j  { //right side
                    temp.push(ch);
                } else {
                    temp.push(' ');
                }
                j += 1;
            }
            vec.push(temp);
        }
    }
    if len == 1 {
        return vec
    }
    for (i, ch) in first.chars().rev().enumerate() {
        let mut temp = String::new();
        // if i != 0 {
            if i == first.len() - 1 {
                // println!("LEN3333: {}, {}, {}", first.len(), i, ch);
                let mut j = 0;
                while j < len {
                    if len / 2 == j {
                        temp.push(ch);
                    } else {
                        temp.push(' ');
                    }
                    j += 1;
                }
                // println!("Temp3333: {}", temp);
                vec.push(temp);
            } else {
                // println!("LEN44444: {}, {}, {}, {}", len, first.len(), i, ch);
                let mut j = 1;
                while j <= len {
                    if i + 1 == j {
                        temp.push(ch);
                    } else if (len - i) == j {
                        // println!("LEN_3: {}, {}", j, (len / (i + 1)));
                        temp.push(ch);
                    } else {
                        temp.push(' ');
                    }
                    j += 1;
                }
                // println!("Temp555555: {}", temp);
                vec.push(temp);
            }
        // }
    }

    vec
}

//"..B.B.." LEN = 7 | i=2 7/2=3  ||| 5 || 7-3+1||| i=2
//".C...C." LEN = 7 | i=3 7/3=2  ||| 6 || 7-2+1||| i=1

//"    B B    " LEN = 11 | i=2  5 ||| 7 | 11-5+1
//"   C   C   " LEN = 11 | i=3  4 ||| 8 |
//"  D     D  " LEN = 11 | i=4  3 ||| 9 | 11-3+1

//".C...C." LEN = 7 i=1 ||
//"..B.B.." LEN = 7 i=2 ||
//
//"...B." LEN = 5 i=2   5/2=2.5 || 4 || 5-2+1
//"....C" LEN = 5 i=3   5/3=1    || 5 if len/2=2.5 +1 == i ||This way i will make middle line that is only once

#[cfg(test)]
mod test {
    use super::*;
    // #[test]
    // fn test_a() {
    //     assert_eq!(get_diamond('A'), vec!["A"]);
    // }
    // #[test]
    // fn test_b() {
    //     assert_eq!(get_diamond('B'), vec![" A ", "B B", " A "]);
    // }
    // #[test]
    // fn test_c() {
    //     assert_eq!(
    //         get_diamond('C'),
    //         vec!["  A  ", " B B ", "C   C", " B B ", "  A  "]
    //     );
    // }
    #[test]
    fn test_d() {
        assert_eq!(
            get_diamond('F'),
            vec!["     A     ", "    B B    ", "   C   C   ", "  D     D  ", " E       E ", "F         F", " E       E ", "  D     D  ", "   C   C   ", "    B B    ", "     A     ",]
        );
    }
    #[test]
    fn test_z() {
        assert_eq!(
            get_diamond('Z'),
            vec![
                "                         A                         ",
                "                        B B                        ",
                "                       C   C                       ",
                "                      D     D                      ",
                "                     E       E                     ",
                "                    F         F                    ",
                "                   G           G                   ",
                "                  H             H                  ",
                "                 I               I                 ",
                "                J                 J                ",
                "               K                   K               ",
                "              L                     L              ",
                "             M                       M             ",
                "            N                         N            ",
                "           O                           O           ",
                "          P                             P          ",
                "         Q                               Q         ",
                "        R                                 R        ",
                "       S                                   S       ",
                "      T                                     T      ",
                "     U                                       U     ",
                "    V                                         V    ",
                "   W                                           W   ",
                "  X                                             X  ",
                " Y                                               Y ",
                "Z                                                 Z",
                " Y                                               Y ",
                "  X                                             X  ",
                "   W                                           W   ",
                "    V                                         V    ",
                "     U                                       U     ",
                "      T                                     T      ",
                "       S                                   S       ",
                "        R                                 R        ",
                "         Q                               Q         ",
                "          P                             P          ",
                "           O                           O           ",
                "            N                         N            ",
                "             M                       M             ",
                "              L                     L              ",
                "               K                   K               ",
                "                J                 J                ",
                "                 I               I                 ",
                "                  H             H                  ",
                "                   G           G                   ",
                "                    F         F                    ",
                "                     E       E                     ",
                "                      D     D                      ",
                "                       C   C                       ",
                "                        B B                        ",
                "                         A                         ",
            ]
        );
    }
}
