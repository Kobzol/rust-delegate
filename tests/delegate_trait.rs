use delegate::{delegatable_trait, delegate_trait};

#[delegatable_trait]
trait Greet {
    fn hello(&self) -> String;
    fn goodbye(&self) -> String;
}

struct Inner;

impl Greet for Inner {
    fn hello(&self) -> String {
        "hello from inner".into()
    }
    fn goodbye(&self) -> String {
        "goodbye from inner".into()
    }
}

struct Wrapper {
    inner: Inner,
}

// `hello` is overridden, `goodbye` is entirely absent -> delegated
#[delegate_trait(target = inner)]
impl Greet for Wrapper {
    fn hello(&self) -> String {
        "hello from wrapper".into()
    }
}

#[test]
fn test_delegate_trait_partial() {
    let w = Wrapper { inner: Inner };
    assert_eq!(w.hello(), "hello from wrapper");
    assert_eq!(w.goodbye(), "goodbye from inner");
}

// Test delegating ALL methods (empty impl block)
#[delegatable_trait]
trait Math {
    fn add(&self, a: u32, b: u32) -> u32;
    fn mul(&self, a: u32, b: u32) -> u32;
}

struct MathImpl;

impl Math for MathImpl {
    fn add(&self, a: u32, b: u32) -> u32 { a + b }
    fn mul(&self, a: u32, b: u32) -> u32 { a * b }
}

struct MathWrapper {
    inner: MathImpl,
}

#[delegate_trait(target = inner)]
impl Math for MathWrapper {}

#[test]
fn test_delegate_trait_all() {
    let w = MathWrapper { inner: MathImpl };
    assert_eq!(w.add(2, 3), 5);
    assert_eq!(w.mul(4, 5), 20);
}

// Test with generics
#[delegatable_trait]
trait Len {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

struct GenericWrapper<T: Len> {
    inner: T,
}

// Only override is_empty; len is delegated
#[delegate_trait(target = inner)]
impl<T: Len> Len for GenericWrapper<T> {
    fn is_empty(&self) -> bool {
        self.inner.len() > 10
    }
}

struct MyVec(Vec<u8>);

impl Len for MyVec {
    fn len(&self) -> usize { self.0.len() }
    fn is_empty(&self) -> bool { self.0.is_empty() }
}

#[test]
fn test_delegate_trait_generic() {
    let w = GenericWrapper {
        inner: MyVec(vec![1, 2, 3]),
    };
    assert_eq!(w.len(), 3);
    assert!(!w.is_empty());
}

// Test that default methods are NOT delegated (use their default impl)
#[delegatable_trait]
trait WithDefault {
    fn required(&self) -> u32;
    fn optional(&self) -> u32 {
        1 + self.required()
    }
}

struct DefaultInner;
impl WithDefault for DefaultInner {
    fn required(&self) -> u32 { 42 }
    fn optional(&self) -> u32 { 200 }
}

struct DefaultWrapper { inner: DefaultInner }

#[delegate_trait(target = inner)]
impl WithDefault for DefaultWrapper {
    fn optional(&self) -> u32 { 100 }
}

struct DefaultWrapper2 { inner: DefaultInner }

#[delegate_trait(target = inner)]
impl WithDefault for DefaultWrapper2 {
}

#[test]
fn test_default_methods_not_delegated() {
    let w = DefaultWrapper { inner: DefaultInner };
    // required() is delegated -> calls inner.required() -> 42
    assert_eq!(w.required(), 42);
    // optional() is explicitly overridden in the impl block -> 100
    assert_eq!(w.optional(), 100);

    // DefaultWrapper2 has an empty impl block:
    // required() is delegated, optional() keeps its default (999)
    let w2 = DefaultWrapper2 { inner: DefaultInner };
    assert_eq!(w2.required(), 42);
    assert_eq!(w2.optional(), 43);
}
