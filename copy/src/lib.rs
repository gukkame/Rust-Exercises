pub fn str_function(a: String) -> (String, String) {
    let mut string = String::from("");
    for ch in a.split_whitespace() {
        let n = ch.to_string().parse::<i32>().unwrap();
        let res = (n as f64).exp();
        for num in res.to_string().chars() {
            string.push(num)
        }
        string.push(' ')
    }
    let trim = string.trim();
    (a.to_string(), trim.to_string())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut vec = Vec::new();
    let mut orig = Vec::new();
    for num in b {
        let log = (num as f64).abs().ln();
        vec.push(log);
        orig.push(num);
    }
    (orig, vec)
}

pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let result = (c as f64).exp();
    return (c, result, (c as f64).abs().ln());
}
