use delegate::delegate;
use std::convert::TryFrom;

#[test]
fn test_generic_returntype() {
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
fn test_into() {
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
            }
        }
    }

    let wrapper = Wrapper { inner: Inner };

    assert_eq!(wrapper.method(), Ok(B));
}

#[test]
fn test_try_into_unwrap() {
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
                #[unwrap]
                fn method(&self) -> B;
            }
        }
    }

    let wrapper = Wrapper { inner: Inner };

    assert_eq!(wrapper.method(), B);
}

#[test]
fn test_unwrap_result() {
    struct Inner;
    impl Inner {
        pub fn method(&self) -> Result<u32, ()> {
            Ok(0)
        }
    }

    struct Wrapper {
        inner: Inner,
    }

    impl Wrapper {
        delegate! {
            to self.inner {
                #[unwrap]
                fn method(&self) -> u32;
            }
        }
    }

    let wrapper = Wrapper { inner: Inner };

    assert_eq!(wrapper.method(), 0);
}

#[test]
fn test_unwrap_option() {
    struct Inner;
    impl Inner {
        pub fn method(&self) -> Option<u32> {
            Some(0)
        }
    }

    struct Wrapper {
        inner: Inner,
    }

    impl Wrapper {
        delegate! {
            to self.inner {
                #[unwrap]
                fn method(&self) -> u32;
            }
        }
    }

    let wrapper = Wrapper { inner: Inner };

    assert_eq!(wrapper.method(), 0);
}

#[test]
#[should_panic]
fn test_unwrap_no_return() {
    struct Inner;
    impl Inner {
        pub fn method(&self) -> Result<u32, ()> {
            Err(())
        }
    }

    struct Wrapper {
        inner: Inner,
    }

    impl Wrapper {
        delegate! {
            to self.inner {
                #[unwrap]
                fn method(&self);
            }
        }
    }

    let wrapper = Wrapper { inner: Inner };
    wrapper.method();
}

#[test]
fn test_unwrap_into() {
    struct A(u32);

    impl From<u32> for A {
        fn from(value: u32) -> Self {
            A(value)
        }
    }

    struct Inner;
    impl Inner {
        pub fn method(&self) -> Result<u32, ()> {
            Ok(0)
        }
    }

    struct Wrapper {
        inner: Inner,
    }

    impl Wrapper {
        delegate! {
            to self.inner {
                #[unwrap]
                #[into]
                fn method(&self) -> A;
            }
        }
    }

    let wrapper = Wrapper { inner: Inner };

    assert!(matches!(wrapper.method(), A(0)));
}

#[test]
fn test_into_unwrap() {
    struct A(u32);

    impl From<A> for Result<u32, ()> {
        fn from(value: A) -> Self {
            Ok(value.0)
        }
    }

    struct Inner;
    impl Inner {
        pub fn method(&self) -> A {
            A(0)
        }
    }

    struct Wrapper {
        inner: Inner,
    }

    impl Wrapper {
        delegate! {
            to self.inner {
                #[into(Result<u32, ()>)]
                #[unwrap]
                fn method(&self) -> u32;
            }
        }
    }

    let wrapper = Wrapper { inner: Inner };

    assert_eq!(wrapper.method(), 0);
}
