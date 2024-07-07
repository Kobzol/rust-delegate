use delegate::delegate;
use std::cell::OnceCell;

#[test]
fn test_delegate_closure() {
    struct Inner(u32);
    impl Inner {
        fn method(&self, n: u32) -> u32 {
            self.0 + n
        }

        fn method2(&self, n: u32) -> u32 {
            self.0 - n
        }
    }

    struct Wrapper {
        inner: OnceCell<Inner>,
    }

    impl Wrapper {
        pub fn new() -> Self {
            Self {
                inner: OnceCell::new(),
            }
        }
        fn content(&self, val: u32) -> &Inner {
            self.inner.get_or_init(|| Inner(val))
        }
        delegate! {
            to |k: u32| self.content(k) {
                pub fn method(&self, num: u32) -> u32;
                pub fn method2(&self, num: u32) -> u32;
            }
        }
    }

    let wrapper = Wrapper::new();
    assert_eq!(wrapper.method(1, 2), 3);
    assert_eq!(wrapper.method2(1, 1), 0);
    assert_eq!(wrapper.method(1, 3), 4);
    assert_eq!(wrapper.method2(1, 0), 1);
}

#[test]
fn test_delegate_closure_associated_function() {
    struct Inner;
    impl Inner {
        fn method(n: u32) -> u32 {
            n + 1
        }

        fn method2(n: u32) -> u32 {
            n - 1
        }
    }

    struct Wrapper;

    impl Wrapper {
        // Doesn't really make sense, but should compile
        delegate! {
            to |k: u32| Inner {
                pub fn method(num: u32) -> u32;
                pub fn method2(num: u32) -> u32;
            }
        }
    }

    assert_eq!(Wrapper::method(1, 2), 3);
    assert_eq!(Wrapper::method2(1, 1), 0);
    assert_eq!(Wrapper::method(1, 3), 4);
    assert_eq!(Wrapper::method(1, 0), 1);
}
