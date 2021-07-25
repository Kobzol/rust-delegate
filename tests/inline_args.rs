use delegate::delegate;

#[test]
fn test_inline_args() {
    struct Inner;

    impl Inner {
        fn fun_generic<S: Copy>(self, s: S) -> S {
            s
        }
        fn fun0(self) -> u32 {
            42
        }
        fn fun1(self, a: u32) -> u32 {
            a
        }
        fn fun2(self, a: u32, b: u32) -> u32 {
            a + b
        }
        fn fun3(&self, a: u32, b: u32, c: u32) -> u32 {
            a + b + c
        }
    }

    struct Outer {
        inner: Inner,
        value: u32,
    }

    impl Outer {
        pub fn new() -> Outer {
            Outer {
                inner: Inner,
                value: 42,
            }
        }

        delegate! {
            to self.inner {
                #[call(fun_generic)]
                fn fun_generic(self, [ 42 ]) -> u32;
                #[call(fun1)]
                fn fun1_with_0(self, [ 0 ]) -> u32;
                #[call(fun1)]
                fn fun1_with_0_no_spaces(self, [0]) -> u32;
                #[call(fun1)]
                fn fun1_with_def(self, [ self.value ] ) -> u32;
                fn fun2(self, [ 0 ], b: u32) -> u32;
            }
        }
    }

    assert_eq!(Outer::new().fun_generic(), 42);
    assert_eq!(Outer::new().fun1_with_0(), 0);
    assert_eq!(Outer::new().fun1_with_0_no_spaces(), 0);
    assert_eq!(Outer::new().fun1_with_def(), 42);
    assert_eq!(Outer::new().fun2(2), 2);
}

#[test]
fn test_mixed_args() {
    use delegate::delegate;
    struct Inner;
    impl Inner {
        pub fn polynomial(&self, a: i32, x: i32, b: i32, y: i32, c: i32) -> i32 {
            a + x * x + b * y + c
        }
    }
    struct Wrapper {
        inner: Inner,
        a: i32,
        b: i32,
        c: i32,
    }
    impl Wrapper {
        delegate! {
            to self.inner {
                pub fn polynomial(&self, [ self.a ], x: i32, [ self.b ], y: i32, [ self.c ]) -> i32 ;

                #[call(polynomial)]
                pub fn linear(&self, [ 0 ], [ 0 ], [ self.b ], y: i32, [ self.c ]) -> i32 ;
            }
        }

        pub fn new() -> Wrapper {
            Wrapper {
                inner: Inner,
                a: 1,
                b: 3,
                c: 5,
            }
        }
    }

    assert_eq!(Wrapper::new().polynomial(2, 3), 19i32);
    assert_eq!(Wrapper::new().linear(3), 14i32);
}
