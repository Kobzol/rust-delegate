# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.13.5](https://github.com/Kobzol/rust-delegate/compare/v0.13.4...v0.13.5) - 2025-11-17

### Other

- Enable changelog
- Merge pull request #95 from Kobzol/update-changelog
- Update changelog
- Install cargo-expand in publish workflow
- Merge pull request #88 from JRRudy1/fields
- Added `#[field]` example and Development section to README.md
- Removed support for `#[field(ref ...)]` syntax in favor of `#[field(& ...)]`
- Added `#[field]` example to the module docs
- Added standard and macro expansion tests for the `#[field]` attribute
- Added `cargo-expand` installation to CI
- Implemented `#[field]` attribute
# Dev

- Allow delegating to fields (implemented by @JRRudy1 in https://github.com/Kobzol/rust-delegate/pull/88).

# 0.13.4 (14. 7. 2025)

- Do not explicitly forward lifetime arguments when calling delegated functions (https://github.com/Kobzol/rust-delegate/issues/85).

# 0.13.3 (25. 3. 2025)

- Add `#[const(path::to::Trait::CONST)]` attribute to delegate associated constants via a getter (implemented by @vic1707).
- Add `#[expr(<$ template>)]` attribute to modify delegated call using custom code (implemented by @vic1707).

# 0.13.2 (14. 1. 2025)

- Correctly parse attributes with segmented paths (e.g. `#[a::b::c]`) (https://github.com/Kobzol/rust-delegate/issues/77).

# 0.13.1 (9. 10. 2024)

- Correctly pass generic method type and lifetime arguments to the delegated method.

# 0.13.0 (2. 9. 2024)

- Generalize match arms handling. You can now combine a match expression target with annotations like `#[into]` and
  others:

```rust
struct A;

impl A {
    pub fn callable(self) -> Self {
        self
    }
}

struct B;

impl B {
    pub fn callable(self) -> Self {
        self
    }
}

enum Common {
    A(A),
    B(B),
}

impl From<A> for Common {
    fn from(inner: A) -> Self {
        Self::A(inner)
    }
}

impl From<B> for Common {
    fn from(inner: B) -> Self {
        Self::B(inner)
    }
}

impl Common {
    delegate! {
        to match self {
        // ---------- `match` arms have incompatible types
            Common::A(inner) => inner;
            Common::B(inner) => inner;
        } {
            #[into]
            pub fn callable(self) -> Self;
        }
    }

    // Generates
    // pub fn callable(self) -> Self {
    //     match self {
    //         Common::A(inner) => inner.callable().into(),
    //         Common::B(inner) => inner.callable().into(),
    //     }
    // }
}
```

- The crate should be `#[no_std]` compatible again (https://github.com/Kobzol/rust-delegate/pull/74).

# 0.12.0 (22. 12. 2023)

- Add new `#[newtype]` function parameter modifier ([#64](https://github.com/Kobzol/rust-delegate/pull/64)).
  Implemented by [Techassi](https://github.com/Techassi)

- Allow passing arbitrary attributes to delegation segments:

```rust
impl Foo {
    delegate! {
    #[inline(always)]
    to self.0 { ... }
  }
}
```

- Change the default inlining mode from `#[inline(always)]`
  to `#[inline]` (https://github.com/Kobzol/rust-delegate/issues/61).

# 0.11.0 (4. 12. 2023)

- Allow delegating an associated function (not just a method).

```rust
struct A {}

impl A {
    fn foo(a: u32) -> u32 {
        a + 1
    }
}

struct B;

impl B {
    delegate! {
        to A {
            fn foo(a: u32) -> u32;
        }
    }
}
```

# 0.10.0 (29. 6. 2023)

- Allow specifying certain attributes (e.g. `#[into]` or `#[unwrap]`) on delegated segments.
  The attribute will then be applied to all methods in that segment (unless it is overwritten on the method itself).

```rust
delegate! {
  #[unwrap]
  to self.inner {
    fn foo(&self) -> u32; // calls self.inner.foo().unwrap()
    fn bar(&self) -> u32; // calls self.inner.bar().unwrap()
  }
}
```

- Add new `#[unwrap]` method modifier. Adding it on top of a delegated method will cause the generated
  code to `.unwrap()` the result.

```rust
#[unwrap]
fn foo(&self) -> u32;  // foo().unwrap()
```

- Add new `#[through(<trait>)]` method modifier. Adding it on top of a delegated method will cause the generated
  code to call the method through the provided trait
  using [UFCS](https://doc.rust-lang.org/reference/expressions/call-expr.html#disambiguating-function-calls).

```rust
#[through(MyTrait)]
delegate! {
  to &self.inner {
    #[through(MyTrait)]
    fn foo(&self) -> u32;  // MyTrait::foo(&self.inner)
  }
}
```

- Removed `#[try_into(unwrap)]`. It can now be replaced with the combination of `#[try_into]` and `#[unwrap]`:

```rust
#[try_into]
#[unwrap]
fn foo(&self) -> u32;  // TryInto::try_into(foo()).unwrap()
```

- Add the option to specify explicit type path to the `#[into]` expression modifier:

```rust
#[into(u64)]
fn foo(&self) -> u64; // Into::<u64>::into(foo())
```

- Expression modifiers `#[into]`, `#[try_into]` and `#[unwrap]` can now be used multiple times. The order
  of their usage dictates in what order they will be applied:

```rust
#[into]
#[unwrap]
fn foo(&self) -> u32;  // Into::into(foo()).unwrap()

#[unwrap]
#[into]
fn foo(&self) -> u32;  // Into::into(foo().unwrap())
```

# 0.9.0 (16. 1. 2023)

- Add new `#[as_ref]` function parameter modifier ([#47](https://github.com/Kobzol/rust-delegate/pull/47)).
  Implemented by [trueegorletov](https://github.com/trueegorletov).

# 0.8.0 (7. 9. 2022)

- Allow simple delegation to enum variants ([#45](https://github.com/Kobzol/rust-delegate/pull/45)).
  Implemented by [gfreezy](https://github.com/gfreezy).

# 0.7.0 (6. 6. 2022)

- Add new `#[into]` attribute for delegated function parameters. If specified, the parameter will be
  converted using the `From` trait before being passed as an argument to the called function.
- Add new `#[try_from]` attribute to delegated functions. You can use it to convert the delegated
  expression using the `TryFrom` trait. You can also use `#[try_from(unwrap)]` to unwrap the result of
  the conversion.

# 0.6.2 (31. 1. 2022)

- Add new `#[await(true/false)]` attribute to delegated functions. You can use it to control whether
  `.await` will be appended to the delegated expression. It will be generated by default if the delegation
  method signature is `async`.

# 0.6.1 (25. 7. 2021)

- add support for `async` functions. The delegated call will now use `.await`.

# 0.6.0 (7. 7. 2021)

- add the option to specify inline expressions that will be used as arguments for the delegated
  call (https://github.com/kobzol/rust-delegate/pull/34)
- removed `append_args` attribute, which is superseded by the mentioned expression in signature support

# 0.5.2 (16. 6. 2021)

- add the `append_args` attribute to append attributes to delegated
  calls (https://github.com/kobzol/rust-delegate/pull/31)

# 0.5.1 (6. 1. 2021)

- fix breaking change caused by using `syn` private API (https://github.com/kobzol/rust-delegate/issues/28)

# 0.5.0 (16. 11. 2020)

- `self` can now be used as the delegation target
- Rust 1.46 introduced a change that makes it a bit difficult to use `rust-delegate` implementations
  generated by macros. If you have this use case, please
  use [this workaround](https://github.com/kobzol/rust-delegate/issues/25#issuecomment-716774685).
