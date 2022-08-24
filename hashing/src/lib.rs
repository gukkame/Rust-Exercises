use std::collections::HashMap;
pub fn mean(list: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for val in list.iter() {
        sum += val;
    }
    sum as f64 / list.len() as f64
}

pub fn median(list: &Vec<i32>) -> i32 {
    let mut val = 0;
    let mut n = 0;
    let mut list2 = list.clone();
    list2.sort();
    if list2.len() % 2 == 1 { 
        val = list2.len() / 2; 
        n = list2[val];
    }else{
        val = list2.len() / 2;
        n = list2[val];
        n += list2[val+1];
    }
    n
}

pub fn mode(list: &Vec<i32>) -> i32 {
    let numbers = list.clone() ;
    let mut times = HashMap::new();
    let mut mode: i32 = 0;

    // count
    for x in &numbers {
        let cnt = times.entry(*x as usize).or_insert(0);
        *cnt += 1;
    }

    let mut best: (i32, i32) = (*times.iter().nth(0).expect("Fatal.").0 as i32, *times.iter().nth(0).expect("Fatal.").1 as i32);

    for x in times.iter() {
        if *x.1 > best.1 {
            best = (*x.0 as i32, *x.1);
        }
    }
    mode = best.0;
    mode

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;
    #[allow(dead_code)]
    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < f64::EPSILON
    }
    #[test]
    fn test_mean() {
        let v = vec![4, 7, 5, 2, 5, 1, 3];
        assert!(approx_eq(mean(&v), 3.857142857142857));
    }
    #[test]
    fn test_median() {
        let v = vec![4, 7, 5, 2, 5, 1, 3];
        assert_eq!(median(&v), 4);
    }
    #[test]
    fn test_mode() {
        let v = vec![4, 7, 5, 2, 5, 1, 3];
        assert_eq!(mode(&v), 5);
    }
}
