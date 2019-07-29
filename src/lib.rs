enum Node {
    Node(Box<(i32, Option<Node>)>),
}

pub struct List {
    head: Option<Node>,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, item: i32) {
        self.head = Some(Node::Node(Box::new((item, self.head.take()))));
    }
    pub fn pop(&mut self) -> Option<i32> {
        match self.head.take() {
            None => None,
            Some(Node::Node(boxed)) => {
                let (item, node) = *boxed;
                self.head = node;
                Some(item)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_push_pop() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
