use delegate::delegate;
use std::convert::TryFrom;

#[test]
fn test_segment_unwrap() {
    struct Inner;

    impl Inner {
        fn foo(&self, a: u32) -> Result<u32, ()> {
            Ok(a)
        }
        fn bar(&self, a: u32) -> Result<u32, ()> {
            Ok(a)
        }
    }

    struct Wrapper {
        inner: Inner,
    }

    impl Wrapper {
        delegate! {
            #[unwrap]
            to self.inner {
                fn foo(&self, a: u32) -> u32;
                fn bar(&self, a: u32) -> u32;
            }
        }
    }

    let wrapper = Wrapper { inner: Inner };
    assert_eq!(wrapper.foo(1), 1);
    assert_eq!(wrapper.bar(2), 2);
}

#[test]
fn test_segment_try_into() {
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
            #[try_into]
            to self.inner {
                fn method(&self) -> Result<B, u32>;
            }
        }
    }

    let wrapper = Wrapper { inner: Inner };
    let wrapper = Wrapper { inner: Inner };

    assert_eq!(wrapper.method(), Ok(B));
}

#[test]
fn test_segment_into() {
    struct A(u32);

    impl From<A> for Result<u32, ()> {
        fn from(value: A) -> Self {
            Ok(value.0)
        }
    }

    impl From<A> for u64 {
        fn from(value: A) -> Self {
            value.0 as u64
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
            #[into(Result<u32, ()>)]
            to self.inner {
                #[unwrap]
                fn method(&self) -> u32;

                #[into]
                #[call(method)]
                fn method2(&self) -> u64;
            }
        }
    }

    let wrapper = Wrapper { inner: Inner };

    assert_eq!(wrapper.method(), 0);
    assert_eq!(wrapper.method2(), 0);
}

#[test]
fn test_segment_await() {
    struct UserRepo;
    impl UserRepo {
        fn foo(&self) {}
        async fn bar(&self) -> u32 {
            1
        }
    }

    struct SharedRepo(tokio::sync::Mutex<UserRepo>);

    impl SharedRepo {
        delegate! {
            #[await(false)]
            to self.0.lock().await {
                async fn foo(&self);
                #[await(true)]
                async fn bar(&self) -> u32;
            }
        }
    }
}
