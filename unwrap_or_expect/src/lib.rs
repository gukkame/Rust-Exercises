pub fn odd_to_even(data: Vec<u32>) -> Result<Vec<u32>, (String, Vec<u32>)> {
    let mut a = Vec::new();
    a.extend(data.iter().filter(|&value| value % 2 == 0));
    if a.len() != 0 {
        return Err(("There is a even value in the vector!".to_string(), a));
    }
    a.extend(data.iter().map(|&value| value + 1));
    Ok(a)
}
pub fn expect(v: Vec<u32>) -> Vec<u32> {
    let result = odd_to_even(v);
    let res = match result {
        Ok(v) => v,
        Err(error) => panic!("ERROR : {:?})", error),
    };
    res
}
pub fn unwrap_or(v: Vec<u32>) -> Vec<u32> {
    let result = odd_to_even(v);
    let empty = Vec::new();
    let res = match result {
        Ok(v) => v,
        Err(_) => empty,
    };
    res
}
pub fn unwrap_err(v: Vec<u32>) -> (String, Vec<u32>) {
    let result = odd_to_even(v);
    let res = match result {
        Ok(error) => panic!("{:?}", error),
        Err(error) => (error),
    };
    res
}
pub fn unwrap(v: Vec<u32>) -> Vec<u32> {
    let result = odd_to_even(v);
    let result2 = result.clone();
    let res = match result {
        Ok(v) => v,
        Err(_error) => (result2.unwrap()),
    };
    res
}
pub fn unwrap_or_else(v: Vec<u32>) -> Vec<u32> {
    let result = odd_to_even(v);
    let res = match result {
        Ok(v) => v,
        Err(error) => (error.1),
    };
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic(expected = "ERROR : (\"There is a even value in the vector!\", [2])")]
    fn test_expect() {
        expect(vec![1, 3, 2, 5]);
    }
    #[test]
    #[should_panic(
        expected = "called `Result::unwrap()` on an `Err` value: (\"There is a even value in the vector!\", [2])"
    )]
    fn test_unwrap() {
        unwrap(vec![1, 3, 2, 5]);
    }
    #[test]
    #[should_panic]
    fn test_unwrap_err() {
        unwrap_err(vec![1, 3, 5]);
    }
    #[test]
    fn test_unwrap_or() {
        assert_eq!(unwrap_or(vec![1, 3, 2, 5]), vec![]);
    }
    #[test]
    fn test_unwrap_or_else() {
        assert_eq!(unwrap_or_else(vec![1, 3, 5]), vec![2, 4, 6]);
        assert_eq!(unwrap_or_else(vec![3, 2, 6, 5]), vec![2, 6]);
    }
    // #[test]
    // fn test_ok() {
    //     assert_eq!(expect(vec![1, 3, 5]), vec![2, 4, 6]);
    //     assert_eq!(unwrap_or(vec![1, 3, 5]), vec![2, 4, 6]);
    //     assert_eq!(unwrap_or_else(vec![1, 3, 5]), vec![2, 4, 6]);
    //     assert_eq!(unwrap(vec![1, 3, 5]), vec![2, 4, 6]);
    //     assert_eq!(unwrap_err(vec![1, 2, 3, 4, 5]).0, "There is a even value in the vector!");
    //     assert_eq!(unwrap_err(vec![1, 2, 3, 4, 5]).1, vec![2, 4]);
    // }
}
