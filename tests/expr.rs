extern crate delegate;
use delegate::delegate;
use std::sync::Mutex;

struct Inner;
impl Inner {
    pub fn method(&self, num: u32) -> u32 {
        num
    }
}

struct Wrapper {
    inner: Mutex<Inner>,
}

impl Wrapper {
    delegate! {
        target self.inner.lock().unwrap() {
            pub(crate) fn method(&self, num: u32) -> u32;
        }
    }
}

#[test]
fn test_mutex() {
    let wrapper = Wrapper {
        inner: Mutex::new(Inner)
    };

    assert_eq!(wrapper.method(3), 3);
}
