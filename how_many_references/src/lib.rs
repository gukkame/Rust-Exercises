pub use std::rc::Rc;

pub struct Node {
    pub value: Vec<Rc<String>>,
}

impl Node {
    pub fn new(value: Vec<Rc<String>>) -> Node {
        Node { value: value }
    }
    pub fn add_ele(&mut self, v: Rc<String>) {
        self.value.push(v);
    }
    pub fn rm_all_ref(&mut self, v: Rc<String>) {
        self.value.retain(|elem| !Rc::ptr_eq(elem, &v));
    }
}

pub fn how_many_references(value: &Rc<String>) -> usize {
    Rc::strong_count(&value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_ele() {
        let a = Rc::new(String::from("a"));
        let b = Rc::new(String::from("b"));
        let c = Rc::new(String::from("c"));
        let mut new_node = Node::new(vec![a.clone()]);
        new_node.add_ele(a.clone());
        new_node.add_ele(b.clone());
        new_node.add_ele(c.clone());
        assert_eq!(new_node.value, vec![a.clone(), a, b, c]);
    }
    #[test]
    fn test_how_many_references() {
        let a = Rc::new(String::from("a"));
        let b = Rc::new(String::from("b"));
        let c = Rc::new(String::from("c"));
        let d = Rc::new(String::from("d"));
        let mut new_node = Node::new(vec![]);
        new_node.add_ele(b.clone());
        new_node.add_ele(a.clone());
        new_node.add_ele(c.clone());
        new_node.add_ele(a.clone());
        assert_eq!(how_many_references(&d), 1);
        assert_eq!(how_many_references(&a), 3);
        assert_eq!(how_many_references(&b), 2);
        assert_eq!(how_many_references(&c), 2);
    }
    #[test]
    fn test_rm_all_ref() {
        let a = Rc::new(String::from("a"));
        let b = Rc::new(String::from("b"));
        let c = Rc::new(String::from("c"));
        let d = Rc::new(String::from("d"));
        let a1 = Rc::new(String::from("a"));
        let b1 = Rc::new(String::from("b"));
        let c1 = Rc::new(String::from("c"));
        let d1 = Rc::new(String::from("d"));
        let mut new_node = Node::new(vec![
            d.clone(),
            d.clone(),
            b.clone(),
            a.clone(),
            c.clone(),
            a.clone(),
            d.clone(),
        ]);
        new_node.rm_all_ref(a1.clone());
        assert_eq!(how_many_references(&a), 3);
        new_node.rm_all_ref(a.clone());
        assert_eq!(how_many_references(&a), 1);
        new_node.rm_all_ref(b1.clone());
        assert_eq!(how_many_references(&b), 2);
        new_node.rm_all_ref(b.clone());
        assert_eq!(how_many_references(&b), 1);
        new_node.rm_all_ref(c1.clone());
        assert_eq!(how_many_references(&c), 2);
        new_node.rm_all_ref(c.clone());
        assert_eq!(how_many_references(&c), 1);
        new_node.rm_all_ref(d1.clone());
        assert_eq!(how_many_references(&d), 4);
        new_node.rm_all_ref(d.clone());
        assert_eq!(how_many_references(&d), 1);
    }
}
