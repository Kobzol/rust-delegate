extern crate delegate;
use delegate::delegate;

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

            #[target_method(method)]
            pub(crate) fn method_conv(&self, num: u32) -> u64;
        }
    }
}

#[test]
fn test_rettype() {
    let wrapper = Wrapper { inner: Inner };

    assert_eq!(wrapper.method(3), ());
    assert_eq!(wrapper.method_conv(3), 3u64);
}
