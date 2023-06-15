use delegate::delegate;

#[test]
fn test_call_through_trait() {
    trait A {
        fn f(&self) -> u32;
    }

    trait B {
        fn f(&self) -> u32;
    }

    struct Foo;

    impl A for Foo {
        fn f(&self) -> u32 {
            0
        }
    }
    impl B for Foo {
        fn f(&self) -> u32 {
            1
        }
    }

    struct Bar(Foo);

    impl Bar {
        delegate! {
            to &self.0 {
                #[through(A)]
                fn f(&self) -> u32;
                #[call(f)]
                #[through(B)]
                fn f2(&self) -> u32;
            }
        }
    }

    let bar = Bar(Foo);
    assert_eq!(bar.f(), 0);
    assert_eq!(bar.f2(), 1);
}
