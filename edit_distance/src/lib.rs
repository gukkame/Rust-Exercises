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



// pub fn edit_distance(source: &str, target: &str) -> usize {
//     let mut string = String::from("");
//     let mut original = source.to_string();
//     let mut x = 0;

//     'outer: while x < 18 {
//         println!("String: {}, {}", original, target);
//         // println!("String: {}, {}", original.len(), target.len());
//         println!("String3: {}", string);

       
//         let mut k = 0;
//         'inner: for (i, ch) in original.chars().enumerate() {

//             k += 1;
//             if ch == target.chars().nth(k).unwrap() {
//                 //if same add
//                 string.push(ch);
//                 println!("First: {}", string);
//                 continue;
//             } else if original.len() > target.len()  {//delete
//                 //if len of original is longer then remove this ch from original string
//                 original.remove(i);
//                 k-=1;
//                 println!("Remove: {}", original);
//                 break 'inner;
//             } else if original.len() < target.len() {//add
//                 string.push(target.chars().nth(k - 1).unwrap());
//                 string.push(target.chars().nth(k ).unwrap());
//                 println!("Add: {}", string);
//                 original.push(target.chars().nth(k).unwrap());
//                 // k+=1;
//                 string = "".to_string();
//                 break 'inner;
//             } else { //replace
//                 string.push(target.chars().nth(k - 1).unwrap());
//                 println!("Elsee: {}", string);
//                 // original.remove(i);
//             }
//             if string == target.to_string() {
//                 println!("BREAKKK!");
//                 break 'outer;
//             }
//         }

//         println!("String3333333: {}", string);
//         if original == target {
//             break 'outer;
//         }
//         x += 1;
//     }
//     println!("String222: {}", string);
//     2
// }
//Insert, Remove, Subtitute
// Infinit loop => add ch by ch
//go ch by ch checks if not ==, => not include
//If lenght of target < source => delete ch from string => quit loop => string has been changed and start new loop.
//If lenght == then Subtitute with character from target
//if lenght target > source then add ch now plus next one from original
//AND START LOOP AGAIN





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        // assert_eq!(edit_distance("alignment", "assignment"), 2);
        assert_eq!(edit_distance("gumbo", "gambol"), 2);
        assert_eq!(edit_distance("kitten", "sitting"), 3);
        assert_eq!(edit_distance("rosettacode", "raisethysword"), 8);
    }
}
