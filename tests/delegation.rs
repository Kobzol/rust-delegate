use delegate::delegate;

#[test]
fn test_delegation() {
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
        inner2: Inner,
    }

    impl Outer {
        pub fn new() -> Outer {
            Outer {
                inner: Inner,
                inner2: Inner,
            }
        }

        delegate! {
            to self.inner {
                fn fun_generic<S: Copy>(self, s: S) -> S;
                fn fun1(self, a: u32, b: u32) -> u32;
                fn fun2(mut self, a: u32, b: u32) -> u32;
                fn fun3(&self, a: u32, b: u32) -> u32;
            }
            to self.inner2 {
                fn fun4(&mut self, a: u32, b: u32) -> u32;
                fn fun5(self: Self, a: u32, b: u32) -> u32;
                fn fun6(mut self: Self, a: u32, b: u32) -> u32;
            }
        }
    }

    assert_eq!(Outer::new().fun_generic(5), 5);
    assert_eq!(Outer::new().fun1(1, 2), 3);
    assert_eq!(Outer::new().fun2(1, 2), 3);
    assert_eq!(Outer::new().fun3(1, 2), 3);
    assert_eq!(Outer::new().fun4(1, 2), 3);
    assert_eq!(Outer::new().fun5(1, 2), 3);
    assert_eq!(Outer::new().fun6(1, 2), 3);
}

#[test]
fn test_delegate_self() {
    trait Foo {
        fn foo(&self) -> u32;
    }

    struct S;

    impl S {
        fn foo(&self) -> u32 {
            1
        }
    }

    impl Foo for S {
        delegate! {
            to self {
                fn foo(&self) -> u32;
            }
        }
    }

    let s = S;
    assert_eq!(Foo::foo(&s), 1);
}

#[test]
fn test_delegate_tuple() {
    trait Foo {
        fn foo(&self) -> u32;
    }

    struct S;
    impl S {
        fn foo(&self) -> u32 {
            1
        }
    }

    struct T(S);

    impl Foo for T {
        delegate! {
            to self.0 {
                fn foo(&self) -> u32;
            }
        }
    }

    let s = S;
    let t = T(s);
    assert_eq!(Foo::foo(&t), 1);
}
