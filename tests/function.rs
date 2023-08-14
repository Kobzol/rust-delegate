use delegate::delegate;

#[test]
fn test_delegate_function() {
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

    assert_eq!(B::foo(1), 2);
}
