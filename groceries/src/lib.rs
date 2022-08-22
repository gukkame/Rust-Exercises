pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val)
}

pub fn at_index(vec: &Vec<String>, index: usize) -> String {
    let el: Option<&String> = vec.get(index);
    match el {
        Some(el) => el.to_string(),
        None => "".to_string(),
    }
}