extern crate delegate;
use delegate::delegate;

#[test]
fn test_rettype() {
    struct Inner;
    impl Inner {
        pub fn method(&self, num: u32) -> u32 {
            num
        }
    }

    struct Wrapper {
        inner: Inner,
    }

    impl Wrapper {
        delegate! {
            target self.inner {
                pub(crate) fn method(&self, num: u32);

                #[into]
                #[target_method(method)]
                pub(crate) fn method_conv(&self, num: u32) -> u64;
            }
        }
    }

    let wrapper = Wrapper { inner: Inner };

    assert_eq!(wrapper.method(3), ());
    assert_eq!(wrapper.method_conv(3), 3u64);
}

#[test]
fn test_rettype_generic() {
    trait TestTrait {
        fn create(num: u32) -> Self;
    }
    impl TestTrait for u32 {
        fn create(num: u32) -> Self {
            num
        }
    }

    struct Inner;
    impl Inner {
        pub fn method<T: TestTrait>(&self) -> T {
            T::create(0)
        }
    }

    struct Wrapper<T> {
        inner: Inner,
        s: T,
    }

    impl<T: TestTrait> Wrapper<T> {
        delegate! {
            target self.inner {
                pub(crate) fn method(&self) -> T;
            }
        }
    }

    let wrapper = Wrapper { inner: Inner, s: 3 };

    assert_eq!(wrapper.method(), 0);
}
