enum Node<T> {
    Node(Box<(T, Option<Node<T>>)>),
}

pub struct List<T> {
    head: Option<Node<T>>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, item: T) {
        self.head = Some(Node::Node(Box::new((item, self.head.take()))));
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|Node::Node(boxed)| {
            let (item, node) = *boxed;
            self.head = node;
            item
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut node = self.head.take();
        while let Some(Node::Node(mut boxed)) = node {
            node = boxed.1.take();
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
