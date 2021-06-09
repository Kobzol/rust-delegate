Method delegation with less boilerplate
=======================================

[![Build Status](https://github.com/chancancode/rust-delegate/workflows/Tests/badge.svg)](https://github.com/chancancode/rust-delegate/actions)
[![Crates.io](https://img.shields.io/crates/v/delegate.svg)](https://crates.io/crates/delegate)

This crate removes some boilerplate for structs that simply delegate
some of their methods to one or more of their fields.

It gives you the `delegate!` macro, which delegates method calls to selected expressions (usually inner fields).

## Example:
A Stack data structure implemented using an inner Vec via delegation.
```rust
use delegate::delegate;

#[derive(Clone, Debug)]
struct Stack<T> {
    inner: Vec<T>,
}
impl<T> Stack<T> {
    pub fn new() -> Self<T> {
        Self { inner: vec![] }
    }

    delegate! {
        to self.inner {
            pub fn is_empty(&self) -> bool;
            pub fn push(&mut self, value: T);
            pub fn pop(&mut self) -> Option<T>;
            pub fn clear(&mut self);

            #[call(len)]
            pub fn size(&self) -> usize;

            #[call(last)]
            pub fn peek(&self) -> Option<&T>;

        }
    }
}
```

## Features:
- Delegate to a method with a different name
```rust
struct Stack { inner: Vec<u32> }
impl Stack {
    delegate! {
        to self.inner {
            #[call(push)]
            pub fn add(&mut self, value: u32);
        }
    }
}
```
- Use an arbitrary inner field expression
```rust
struct Wrapper { inner: Rc<RefCell<Vec<u32>>> }
impl Wrapper {
    delegate! {
        to self.inner.deref().borrow_mut() {
            pub fn push(&mut self, val: u32);
        }
    }
}
```
- Change the return type of the delegated method using a `From` impl or omit it altogether
```rust
struct Inner;
impl Inner {
    pub fn method(&self, num: u32) -> u32 { num }
}
struct Wrapper { inner: Inner }
impl Wrapper {
    delegate! {
        to self.inner {
            // calls method, converts result to u64
            #[into]
            pub fn method(&self, num: u32) -> u64;

            // calls method, returns ()
            #[call(method)]
            pub fn method_noreturn(&self, num: u32);
        }
    }
}
```
- Delegate to multiple fields
```rust
struct MultiStack {
    left: Vec<u32>,
    right: Vec<u32>,
}
impl MultiStack {
    delegate! {
        to self.left {
            /// Push an item to the top of the left stack
            #[call(push)]
            pub fn push_left(&mut self, value: u32);
        }
        to self.right {
            /// Push an item to the top of the right stack
            #[call(push)]
            pub fn push_right(&mut self, value: u32);
        }
    }
}
```
- Delegation of generic methods
- Inserts `#[inline(always)]` automatically (unless you specify `#[inline]` manually on the method)
- Delegate with additional arguments appended to the argument list
```rust
use delegate::delegate;
struct Inner;
impl Inner {
    pub fn method(&self, num: u32, factor: u32, offset: u32) -> u32 { factor * num + offset }
}
struct Wrapper { inner: Inner, default_offset: u32 }
impl Wrapper {
    delegate! {
        to self.inner {
            // Calls `method` so that `2` is passed in as the `factor`
            // argument and `self.default_offset` is passed in as the
            // `offset` argument
            #[extra_args(2, self.default_offset)]
            pub fn method(&self, num: u32) -> u32;
        }
    }
}
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

## Conduct

Please follow the [Rust Code of Conduct]. For escalation or moderation issues
please contact the crate author(s) listed in [`Cargo.toml`](./Cargo.toml).

[Rust Code of Conduct]: https://www.rust-lang.org/conduct.html
