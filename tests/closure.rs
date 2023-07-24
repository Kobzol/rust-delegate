use delegate::delegate;
use std::cell::OnceCell;

struct Inner(u32);
impl Inner {
    pub fn new(m: u32) -> Self {
        // some "very complex" constructing work
        Self(m)
    }
    pub fn method(&self, n: u32) -> u32 {
        self.0 + n
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
        }
    }
}

#[test]
fn test_delegate_closure() {
    let wrapper = Wrapper::new();
    assert_eq!(wrapper.method(1, 2), 3);
    assert_eq!(wrapper.method(1, 3), 4);
}
