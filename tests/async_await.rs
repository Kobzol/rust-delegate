extern crate delegate;

use delegate::delegate;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

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

#[test]
fn test_bridge_async_to_sync() {
    struct UserRepo;
    impl UserRepo {
        fn foo(&self) {}
    }

    struct SharedRepo(tokio::sync::Mutex<UserRepo>);

    impl SharedRepo {
        delegate! {
            to self.0.lock().await {
                #[await(false)]
                async fn foo(&self);
            }
        }
    }
}

#[test]
fn test_bridge_sync_to_async() {
    struct Fut;
    impl Future for Fut {
        type Output = u32;

        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            todo!()
        }
    }

    struct UserRepo;
    impl UserRepo {
        fn foo(&self) -> Fut {
            Fut
        }
    }

    struct SharedRepo(UserRepo);

    impl SharedRepo {
        delegate! {
            to self.0 {
                #[await(true)]
                async fn foo(&self) -> u32;
            }
        }
    }
}
