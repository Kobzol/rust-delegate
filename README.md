Method delegation with less boilerplate
=======================================

[![Build Status](https://travis-ci.com/chancancode/rust-delegate.svg?branch=master)](https://travis-ci.com/chancancode/rust-delegate)
[![Crates.io](https://img.shields.io/crates/v/delegate.svg)](https://crates.io/crates/delegate)

This crate helps remove some boilerplate for structs that simply delegates
most of its methods to one or more fields.

For example, let's say we want to implement a stack in Rust. In case you aren't
familiar with the idea, a stack is a data structure in which items are inserted
and accessed in a LIFO (Last-In, First-Out) manner. Typically, a stack supports
the following basic operations:

* **push**: insert an item to the top of the stack.
* **pop**: remove an item from the top of the stack (if stack is not empty).

In addition, a stack may support secondary operations such as querying if the
stack is empty, the current size of the stack, a **peek** operation that gives
access to the top item without removing it, a method to clear the stack and so
on.

One way to implement such a data structure in Rust would be to use a `Vec`:

```rust
#[derive(Clone, Debug)]
struct Stack<T> {
    inner: Vec<T>,
}

impl<T> Stack<T> {
    /// Allocate an empty stack
    pub fn new() -> Stack<T> {
        Stack { inner: vec![] }
    }

    /// The number of items in the stack
    pub fn size(&self) -> usize {
        self.inner.len()
    }

    /// Whether the stack is empty
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Push an item to the top of the stack
    pub fn push(&mut self, value: T) {
        self.inner.push(value)
    }

    /// Remove an item from the top of the stack
    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop()
    }

    /// Accessing the top item without removing it from the stack
    pub fn peek(&self) -> Option<&T> {
        self.inner.last()
    }

    /// Remove all items from the stack
    pub fn clear(&mut self) {
        self.inner.clear()
    }
}
```

As you can see, `Vec` already supports most of the operations we needed, so in
most cases our implementation is simply delegating to the underlying `Vec`,
except ocassionally re-mapping the method to a different name (e.g. `peek()` is
called `last()` in `Vec`).

The fact that these implemenations are boring (simply delegating to another
struct) is probably notable and worth calling out. If the reader of the code is
already familiar with the behavior with the other struct, they can safely gloss
over these methods and focus on the more interesting ones. Further more, if we
can trust that the struct we are delegating to has a solid implementation and
is well tested, we can probably just write a simple smoke test and not worry
about re-testing the edge cases.

Unfortunately, this detail could easily get lost, especially when these methods
are burried within other non-delegating methods. The only way to be sure is to
carefully read the implementation to confirm that they aren't doing anything
more, which somewhat defeats the purpose.

The `delegate!` macro in this crate helps solve this problem by making your
delegating methods more declarative:

```rust
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
        }
    }
}
```

This macro invocation would generate exactly the same code as we had written by
hand in the example above (with one minor difference, see below). Not only did
you save a few lines of typing, you are making your intent more clear to your
readers as well.

The macro support all the usual syntatic elements that are valid around method
declarations, such as (doc) comments, attributes, `pub` modifiers, generics,
lifetimes, return type and where clauses. The only difference is that instead
of providing a block for the method body, you simply end it with a `;` after
the method signature. The macro will automatically generate a suitable body.

The macro will also automatically insert the `#[inline]` hint for the compiler,
which is often desirable for delegating methods. You can override this by
inserting an  explicit `#[inline]` attribute (such as `#[inline(always)]` or
`#[inline(never)]`).

As seen in the example above, if the name of the method does not match, you can
override the inferred name (same name as your struct method) with the custom
`#[target_method(...)]` attribute. (This attribute is removed by the macro
during expansion, so it does not rely on the experimental "custom_attribute"
feature.)

You may also delegate different methods to different fields inside the same
`delegate!` block. For example:

```rust
#[macro_use]
extern crate delegate;

#[derive(Clone, Debug)]
struct MultiStack<T> {
    left: Vec<T>,
    right: Vec<T>,
}

impl<T> MultiStack<T> {
    pub fn new() -> MultiStack<T> {
        MultiStack { left: vec![], right: vec![] }
    }

    delegate! {
        target self.left {
            /// Push an item to the top of the left stack
            #[target_method(push)]
            pub fn push_left(&mut self, value: T);

            /// Remove an item from the top of the left stack
            #[target_method(pop)]
            pub fn pop_left(&mut self, value: T);
        }

        target self.right {
            /// Push an item to the top of the right stack
            #[target_method(push)]
            pub fn push_right(&mut self, value: T);

            /// Remove an item from the top of the right stack
            #[target_method(pop)]
            pub fn pop_right(&mut self, value: T);
        }
    }
}
```

This macro is implemented completely using the `macro_rules` system, therefore,
this crate does not have a dependency on the nightly compiler or any unstable
features. However, since the macro does recurse pretty deeply, you may need to
add the `#![recursion_limit="..."]` attribute. The compiler will let you know
if/when it is necessary.

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
