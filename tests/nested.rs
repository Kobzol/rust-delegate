use delegate::delegate;

struct Inner;
impl Inner {
    pub fn method(&self, num: u32) -> u32 {
        num
    }
}

struct Inner2 {
    inner: Inner,
}

struct Wrapper {
    inner: Inner2,
}

impl Wrapper {
    delegate! {
        to self.inner.inner {
            pub(crate) fn method(&self, num: u32) -> u32;
        }
    }
}

#[test]
fn test_nested() {
    let wrapper = Wrapper {
        inner: Inner2 { inner: Inner },
    };

    assert_eq!(wrapper.method(3), 3);
}
