pub struct Node<T>(Box<(T, Option<Node<T>>)>);

pub struct List<T>(Option<Node<T>>);

impl<T> List<T> {
    pub fn new() -> Self {
        List(None)
    }
    pub fn push(&mut self, item: T) {
        self.0 = Some(Node(Box::new((item, self.0.take()))));
    }
    pub fn pop(&mut self) -> Option<T> {
        self.0.take().map(|Node(node)| {
            let (item, next) = *node;
            self.0 = next;
            item
        })
    }
    pub fn peek(&self) -> Option<&T> {
        self.0.as_ref().map(|Node(node)| &node.0)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.0.as_mut().map(|Node(node)| &mut node.0)
    }
    pub fn iter(&self) -> impl Iterator<Item = &'_ T> {
        self.into_iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &'_ mut T> {
        self.into_iter()
    }
}

pub struct IntoIter<T>(T);

// for i in &list
impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = IntoIter<Option<&'a Node<T>>>;
    fn into_iter(self) -> Self::IntoIter {
        impl<'a, T> Iterator for IntoIter<Option<&'a Node<T>>> {
            type Item = &'a T;
            fn next(&mut self) -> Option<Self::Item> {
                self.0.take().map(|Node(node)| {
                    let (item, next) = node.as_ref();
                    self.0 = next.as_ref();
                    item
                })
            }
        }
        IntoIter(self.0.as_ref())
    }
}

// for i in &mut list
impl<'a, T> IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;
    type IntoIter = IntoIter<Option<&'a mut Node<T>>>;
    fn into_iter(self) -> Self::IntoIter {
        impl<'a, T> Iterator for IntoIter<Option<&'a mut Node<T>>> {
            type Item = &'a mut T;
            fn next(&mut self) -> Option<Self::Item> {
                self.0.take().map(|Node(node)| {
                    let (item, next) = node.as_mut();
                    self.0 = next.as_mut();
                    item
                })
            }
        }
        IntoIter(self.0.as_mut())
    }
}

// for i in list
impl<T> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = IntoIter<List<T>>;
    fn into_iter(self) -> Self::IntoIter {
        impl<T> Iterator for IntoIter<List<T>> {
            type Item = T;
            fn next(&mut self) -> Option<Self::Item> {
                self.0.pop()
            }
        }
        IntoIter(self)
    }
}

// drop
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut next = self.0.take();
        while let Some(Node(mut node)) = next {
            next = node.1.take();
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
        for &i in &list {
            assert!(i >= 0 && i < 3);
        }
        for &mut i in &mut list {
            assert!(i >= 0 && i < 3);
        }
        for i in list {
            assert!(i >= 0 && i < 3);
        }
    }
    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }
    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
