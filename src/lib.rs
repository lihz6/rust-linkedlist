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
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|Node::Node(boxed)| &boxed.0)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|Node::Node(boxed)| &mut boxed.0)
    }
}

pub struct ListIntoIter<T>(List<T>);

impl<T> Iterator for ListIntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ListIntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        ListIntoIter(self)
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

    #[test]
    fn peek_peek_mut() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
    }
    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);

        let mut list = List::new();
        list.push(2);
        list.push(1);
        list.push(0);
        for i in list {
            assert!(i >= 0 && i < 3);
        }
    }
}
