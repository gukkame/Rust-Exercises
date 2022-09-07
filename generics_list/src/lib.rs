
#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) where T: Clone {
        match &self.head {
            None => {
                self.head = Some(Node {
                    value,
                    next: None
                });
            },
            Some(_) => {
                self.head = Some(Node {
                    value,
                    next: Some(Box::new(self.head.as_ref().unwrap().clone()))
                });
            }
        }
    }

    pub fn pop(&mut self) where T: Clone {
        if let Some(node) = self.head.take() {
            match node.next {
                None => {
                    self.head = None
                },
                _ => {
                    let unboxed = *(node.next.unwrap());
                    self.head = Some(unboxed)
                }
            }
        }
    }

    pub fn len(&self) ->  usize where T: Clone  {
        match self.head {
            None => {return 0},
            _ => {}
        }
        let mut count: usize = 1;
        let mut current = self.head.as_ref().unwrap().clone();
        loop {
            match current.next {
                Some(x) => {
                    count += 1;
                    current = *x;
                },
                None => {break}
            }
        }
        return count
    
    }
}
