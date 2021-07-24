extern crate delegate;
use delegate::delegate;

#[test]
fn test_async_await() {
    struct Inner;
    impl Inner {
        pub async fn method(&self, num: u32) -> u32 {
            num
        }
    }

    struct Wrapper {
        inner: Inner,
    }

    impl Wrapper {
        delegate! {
            to self.inner {
                pub(crate) async fn method(&self, num: u32) -> u32;

                #[call(method)]
                pub(crate) async fn unit(&self, num: u32);

                #[into]
                #[call(method)]
                pub(crate) async fn with_into(&self, num: u32) -> u64;
            }
        }
    }

    let wrapper = Wrapper { inner: Inner };

    assert_eq!(::futures::executor::block_on(wrapper.method(3)), 3);
    assert_eq!(::futures::executor::block_on(wrapper.unit(3)), ());
    assert_eq!(::futures::executor::block_on(wrapper.with_into(3)), 3_u64);
}

#[test]
fn test_async_trait() {
    use async_trait::async_trait;

    #[async_trait]
    trait Inner {
        async fn method(&self, num: u32) -> u32;
    }

    struct InnerImpl;
    #[async_trait]
    impl Inner for InnerImpl {
        async fn method(&self, num: u32) -> u32 {
            num
        }
    }

    struct Wrapper<T> {
        inner: T,
    }

    impl<T: Inner> Wrapper<T> {
        delegate! {
            to self.inner {
                pub(crate) async fn method(&self, num: u32) -> u32;

                #[call(method)]
                pub(crate) async fn unit(&self, num: u32);

                #[into]
                #[call(method)]
                pub(crate) async fn with_into(&self, num: u32) -> u64;
            }
        }
    }

    let wrapper = Wrapper { inner: InnerImpl };

    assert_eq!(::futures::executor::block_on(wrapper.method(3)), 3);
    assert_eq!(::futures::executor::block_on(wrapper.unit(3)), ());
    assert_eq!(::futures::executor::block_on(wrapper.with_into(3)), 3_u64);
}
