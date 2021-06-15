extern crate delegate;

use delegate::delegate;

#[test]
fn test_append_args() {
    struct Inner;

    impl Inner {
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
                #[append_args()]
                fn fun0(self) -> u32;
                #[append_args()]
                fn fun1(self, a: u32) -> u32;
                #[append_args(0)]
                #[call(fun1)]
                fn fun1_with_0(self) -> u32;
                #[append_args(self.value)]
                #[call(fun1)]
                fn fun1_with_def(self) -> u32;
                #[append_args(1, 2)]
                fn fun2(self) -> u32;
                #[append_args(1, 2)]
                fn fun3(self, a: u32) -> u32;
            }
        }
    }

    assert_eq!(Outer::new().fun0(), 42);
    assert_eq!(Outer::new().fun1(1), 1);
    assert_eq!(Outer::new().fun1_with_0(), 0);
    assert_eq!(Outer::new().fun1_with_def(), 42);
    assert_eq!(Outer::new().fun2(), 3);
    assert_eq!(Outer::new().fun3(3), 6);
}