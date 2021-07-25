use delegate::delegate;
use std::sync::Mutex;

struct Inner;
impl Inner {
    pub fn method(&self, num: u32) -> u32 {
        num
    }
    pub fn method2(&self, num: u32) -> u32 {
        num
    }
}

struct Wrapper {
    inner: Mutex<Inner>,
}

fn global_fn() -> Inner {
    Inner
}

impl Wrapper {
    delegate! {
        to self.inner.lock().unwrap() {
            pub(crate) fn method(&self, num: u32) -> u32;
        }
        to global_fn() {
            pub(crate) fn method2(&self, num: u32) -> u32;
        }
    }
}

#[test]
fn test_mutex() {
    let wrapper = Wrapper {
        inner: Mutex::new(Inner),
    };

    assert_eq!(wrapper.method(3), 3);
    assert_eq!(wrapper.method2(3), 3);
}

#[test]
fn test_index() {
    struct Inner;
    impl Inner {
        pub fn method(&self, num: u32) -> u32 {
            num
        }
    }

    struct Wrapper {
        index: usize,
        items: Vec<Inner>,
    }

    impl Wrapper {
        delegate! {
            to self.items[self.index] {
                fn method(&self, num: u32) -> u32;
            }
        }
    }

    let wrapper = Wrapper {
        index: 0,
        items: vec![Inner],
    };

    assert_eq!(wrapper.method(3), 3);
}
