extern crate delegate;
use delegate::delegate;

#[derive(Clone, Debug)]
struct Stack<T> {
    inner: Vec<T>,
}

impl<T> Stack<T> {
    /// Allocate an empty stack
    pub fn new() -> Stack<T> {
        Stack { inner: vec![] }
    }

    delegate! { self.inner
        fn size(&self) -> usize; use len

        /// doc comment
        fn is_empty(&self) -> bool;

        #[inline(never)]
        fn push(&mut self, v: T);
        fn pop(&mut self) -> Option<T>;
        fn peek(&self) -> Option<&T>; use last
        fn clear(&mut self);
        fn into_boxed_slice(self) -> Box<[T]>;
    }
}

#[test]
fn test_stack() {
    let mut stack: Stack<u32> = Stack::new();

    assert_eq!(stack.size(), 0);
    assert_eq!(stack.is_empty(), true);
    assert_eq!(stack.peek(), None);

    stack.clear();

    assert_eq!(stack.size(), 0);
    assert_eq!(stack.is_empty(), true);
    assert_eq!(stack.peek(), None);

    assert_eq!(stack.pop(), None);

    assert_eq!(stack.size(), 0);
    assert_eq!(stack.is_empty(), true);
    assert_eq!(stack.peek(), None);

    stack.push(1);

    assert_eq!(stack.size(), 1);
    assert_eq!(stack.is_empty(), false);
    assert_eq!(stack.peek(), Some(&1));

    assert_eq!(stack.pop(), Some(1));

    assert_eq!(stack.size(), 0);
    assert_eq!(stack.is_empty(), true);
    assert_eq!(stack.peek(), None);

    stack.push(1);
    stack.push(2);
    stack.push(3);

    assert_eq!(stack.size(), 3);
    assert_eq!(stack.is_empty(), false);
    assert_eq!(stack.peek(), Some(&3));

    assert_eq!(stack.clone().into_boxed_slice().into_vec(), stack.inner);

    assert_eq!(stack.pop(), Some(3));

    assert_eq!(stack.size(), 2);
    assert_eq!(stack.is_empty(), false);
    assert_eq!(stack.peek(), Some(&2));

    stack.clear();

    assert_eq!(stack.size(), 0);
    assert_eq!(stack.is_empty(), true);
    assert_eq!(stack.peek(), None);
}
