use delegate::delegate;
struct Inner(u32);
impl Inner {
    pub fn new(m: u32) -> Self {
        // some "very complex" constructing work
        Self(m)
    }

    pub fn method(n: u32) -> u32 {
        n
    }
}

struct Wrapper {
    inner: Inner,
}

impl Wrapper {
    delegate! {
        to |k: u32| Inner {
            pub fn method() -> u32;
        }
    }
}
