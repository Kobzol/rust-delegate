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
- Delegate with additional arguments (either inlined or appended to the end of the argument list)
```rust
use delegate::delegate;
struct Inner;
impl Inner {
    pub fn polynomial(&self, a: i32, x: i32, b: i32, y: i32, c: i32) -> i32 { 
        a + x * x + b * y + c 
    }
}
struct Wrapper { inner: Inner, a: i32, b: i32, c: i32 }
impl Wrapper {
    delegate! {
        to self.inner {
            // Calls `polynomial` on `inner` with `self.a`, `self.b` and 
            // `self.c` passed as arguments `a`, `b`, and `c`, effectively 
            // executing `self.a + x * x + self.b * y + self.c` 
            pub fn polynomial(&self, [ self.a ], x: i32, [ self.b ], y: i32, [ self.c ]) -> i32 ;
            // Calls `polynomial` on `inner` with `0`s, passed for arguments
            // `x`, `b`, `y`,  and `c`, effectively executing 
            // `a + 0 * 0 + 0 * 0 + 0` 
            #[call(polynomial)] 
            #[append_args(0, 0, 0, 0)]
            pub fn constant(&self, a: i32) -> i32;
            // Calls `polynomial` on `inner` with `0`s passed for arguments 
            // `a` and `x`, and `self.b` and `self.c` for `b` and `c`, 
            // effectively executing `0 + 0 * 0 + self.b * y + self.c`.
            #[call(polynomial)]
            pub fn linear(&self, [ 0 ], [ 0 ], [ self.b ], y: i32, [ self.c ]) -> i32 ;
            // Calls `polynomial` on `inner` with `self.a`s passed for `a`, 
            // `0`s for `b` and 'y', and `self.c` for `c` effectively executing
            // `self.a + x * x + 0 * 0 + self.c`.
            #[call(polynomial)]
            #[append_args(0, 0, self.c)]
            pub fn univariate_quadratic(&self, [ self.a ], x: i32) -> i32 ; 
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
