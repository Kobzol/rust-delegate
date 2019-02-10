extern crate delegate;
use delegate::delegate;

struct Inner;

impl Inner {
    fn fun_generic<S: Copy>(self, s: S) -> S {
        s
    }
    fn fun1(self, a: u32, b: u32) -> u32 {
        a + b
    }
    fn fun2(mut self, a: u32, b: u32) -> u32 {
        a + b
    }
    fn fun3(&self, a: u32, b: u32) -> u32 {
        a + b
    }
    fn fun4(&mut self, a: u32, b: u32) -> u32 {
        a + b
    }
    fn fun5(self: Self, a: u32, b: u32) -> u32 {
        a + b
    }
    fn fun6(mut self: Self, a: u32, b: u32) -> u32 {
        a + b
    }
}

struct Outer {
    inner: Inner,
}

impl Outer {
    pub fn new() -> Outer {
        Outer { inner: Inner }
    }

    delegate! { self.inner
        fn fun_generic<S: Copy>(self, s: S) -> S;
        fn fun1(self, a: u32, b: u32) -> u32;
        fn fun2(mut self, a: u32, b: u32) -> u32;
        fn fun3(&self, a: u32, b: u32) -> u32;
        fn fun4(&mut self, a: u32, b: u32) -> u32;
        fn fun5(self: Self, a: u32, b: u32) -> u32;
        fn fun6(mut self: Self, a: u32, b: u32) -> u32;
    }
}

#[test]
fn test_delegation() {
    assert_eq!(Outer::new().fun_generic(5), 5);
    assert_eq!(Outer::new().fun1(1, 2), 3);
    assert_eq!(Outer::new().fun2(1, 2), 3);
    assert_eq!(Outer::new().fun3(1, 2), 3);
    assert_eq!(Outer::new().fun4(1, 2), 3);
    assert_eq!(Outer::new().fun5(1, 2), 3);
    assert_eq!(Outer::new().fun6(1, 2), 3);
}
