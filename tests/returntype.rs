use delegate::delegate;
use std::convert::TryFrom;

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
            to self.inner {
                pub(crate) fn method(&self, num: u32);

                #[into]
                #[call(method)]
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
            to self.inner {
                pub(crate) fn method(&self) -> T;
            }
        }
    }

    let wrapper = Wrapper { inner: Inner, s: 3 };

    assert_eq!(wrapper.method(), 0);
}

#[test]
fn test_try_into() {
    struct A;

    #[derive(Debug, PartialEq)]
    struct B;

    impl TryFrom<A> for B {
        type Error = u32;

        fn try_from(_value: A) -> Result<Self, Self::Error> {
            Ok(B)
        }
    }

    struct Inner;
    impl Inner {
        pub fn method(&self) -> A {
            A
        }
    }

    struct Wrapper {
        inner: Inner,
    }

    impl Wrapper {
        delegate! {
            to self.inner {
                #[try_into]
                fn method(&self) -> Result<B, u32>;

                #[try_into(unwrap)]
                #[call(method)]
                fn method2(&self) -> B;
            }
        }
    }

    let wrapper = Wrapper { inner: Inner };

    assert_eq!(wrapper.method(), Ok(B));
    assert_eq!(wrapper.method2(), B);
}
