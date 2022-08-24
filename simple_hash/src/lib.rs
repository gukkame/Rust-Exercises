use std::ops::RangeBounds;

use std::collections::HashMap;
pub fn contain(h: &HashMap<&str, i32>, s: &str) -> bool {
    h.contains_key(s)
}

pub fn remove(h: &mut HashMap<&str, i32>, s: &str) {
    h.remove(s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
