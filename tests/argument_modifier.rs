use delegate::delegate;

struct MyNewU32(u32);

trait Foo {
    fn bar(&self, x: Self);
}

impl Foo for u32 {
    fn bar(&self, x: Self) {}
}

impl From<MyNewU32> for u32 {
    fn from(value: MyNewU32) -> Self {
        value.0
    }
}

impl Foo for MyNewU32 {
    delegate! {
        to self.0 {
            fn bar(&self, #[into] x: Self);
        }
    }
}

struct Bar {
    foo: String,
}

impl<T> PartialEq<T> for Bar
where
    T: AsRef<str> + ?Sized,
{
    delegate! {
        to self.foo {
            fn eq(&self, #[as_ref] other: &T) -> bool;
        }
    }
}
