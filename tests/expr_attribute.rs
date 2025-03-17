extern crate delegate;
use delegate::delegate;

#[test]
fn delegate_with_custom_expr() {
    struct A(Vec<u8>);

    impl A {
        delegate! {
            to self.0 {
                #[expr(*$.unwrap())]
                fn get(&self, idx: usize) -> u8;

                #[call(get)]
                #[expr($?.checked_pow(2))]
                fn get_checked_pow_2(&self, idx: usize) -> Option<u8>;
            }
        }
    }

    let a = A(vec![2, 3, 4, 5]);

    assert_eq!(a.get(0), 2);
    assert_eq!(a.get_checked_pow_2(0), Some(4));

    // out-of-bounds behavior
    assert!(std::panic::catch_unwind(|| a.get(4)).is_err());
    assert_eq!(a.get_checked_pow_2(4), None);
}

#[test]
fn delegate_without_placeholder() {
    struct A(Vec<u8>);

    impl A {
        delegate! {
            to self.0 {
                #[expr(Some("test"))]
                fn get_name(&self) -> Option<&'static str>;
            }
        }
    }

    let a = A(vec![2, 3, 4, 5]);

    assert_eq!(a.get_name(), Some("test"));
}
