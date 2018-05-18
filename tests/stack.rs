#[macro_use]
extern crate delegate;

#[derive(Clone, Debug)]
struct Stack<T> {
    inner: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { inner: vec![] }
    }

    delegate! {
        target self.inner {
            pub fn len(&self) -> usize;
            pub fn push(&mut self, value: T);
            pub fn pop(&mut self) -> Option<T>;
            pub fn clear(&mut self);

            #[doc(hidden)]
            pub fn into_boxed_slice(self) -> Box<[T]>;
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.inner.last()
    }
}

#[test]
fn test_stack() {
    let mut stack = Stack::new();

    assert_eq!(stack.len(), 0);
    assert_eq!(stack.peek(), None);

    stack.clear();

    assert_eq!(stack.len(), 0);
    assert_eq!(stack.peek(), None);

    assert_eq!(stack.pop(), None);

    assert_eq!(stack.len(), 0);
    assert_eq!(stack.peek(), None);

    stack.push(1);

    assert_eq!(stack.len(), 1);
    assert_eq!(stack.peek(), Some(&1));

    assert_eq!(stack.pop(), Some(1));

    assert_eq!(stack.len(), 0);
    assert_eq!(stack.peek(), None);

    stack.push(1);
    stack.push(2);
    stack.push(3);

    assert_eq!(stack.len(), 3);
    assert_eq!(stack.peek(), Some(&3));

    assert_eq!(stack.clone().into_boxed_slice().into_vec(), stack.inner);

    assert_eq!(stack.pop(), Some(3));

    assert_eq!(stack.len(), 2);
    assert_eq!(stack.peek(), Some(&2));

    stack.clear();

    assert_eq!(stack.len(), 0);
    assert_eq!(stack.peek(), None);
}
