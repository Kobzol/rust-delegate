#![recursion_limit="128"]

#[macro_use]
extern crate delegate;

#[derive(Clone, Debug)]
struct Stack<T> {
    inner: Vec<T>,
}

impl<T> Stack<T> {
    /// Allocate an empty stack
    pub fn new() -> Stack<T> {
        Stack { inner: vec![] }
    }

    delegate! {
        target self.inner {
            /// The number of items in the stack
            #[target_method(len)]
            pub fn size(&self) -> usize;

            /// Whether the stack is empty
            pub fn is_empty(&self) -> bool;

            /// Push an item to the top of the stack
            pub fn push(&mut self, value: T);

            /// Remove an item from the top of the stack
            pub fn pop(&mut self) -> Option<T>;

            /// Accessing the top item without removing it from the stack
            #[target_method(last)]
            pub fn peek(&self) -> Option<&T>;

            /// Remove all items from the stack
            pub fn clear(&mut self);

            #[doc(hidden)]
            pub fn into_boxed_slice(self) -> Box<[T]>;
        }
    }
}

#[test]
fn test_stack() {
    let mut stack = Stack::new();

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
