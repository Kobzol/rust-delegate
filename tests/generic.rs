use delegate::delegate;
use std::collections::HashSet;
use std::hash::Hash;

#[test]
fn test_generics_method() {
    struct Foo;
    impl Foo {
        fn foo<'a, P, X>(&self) {}
    }

    struct Bar(Foo);
    impl Bar {
        delegate! {
            to &self.0 {
                fn foo<'a, P, X>(&self);
            }
        }
    }
}

#[test]
fn test_generics_function() {
    struct Foo;
    impl Foo {
        fn foo<P, X>() {}
    }

    struct Bar(Foo);
    impl Bar {
        delegate! {
            to Foo {
                fn foo<P, X>();
            }
        }
    }
}

#[test]
fn test_generics_through_trait() {
    trait A {
        fn f<P>(&self) -> u32;
    }

    struct Foo;

    impl A for Foo {
        fn f<P>(&self) -> u32 {
            0
        }
    }

    struct Bar(Foo);

    impl Bar {
        delegate! {
            to &self.0 {
                #[through(A)]
                fn f<P>(&self) -> u32;
            }
        }
    }

    let bar = Bar(Foo);
    assert_eq!(bar.f::<u32>(), 0);
}

#[test]
fn test_generics_complex() {
    struct Foo;
    impl Foo {
        fn foo<'a: 'static, X: Copy, #[allow(unused)] T>(&self) {}
    }

    struct Bar(Foo);
    impl Bar {
        delegate! {
            to &self.0 {
                fn foo<'a: 'static, X: Copy, #[allow(unused)] T>(&self);
            }
        }
    }
}

#[test]
fn test_lifetime_late_bound() {
    trait QSet<T>
    where
        T: PartialEq,
    {
        fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T>
        where
            Self: Sized,
            T: 'a;
    }

    impl<T: Eq + Hash> QSet<T> for HashSet<T> {
        delegate! {
            to self {
                #[through(HashSet)]
                fn iter<'a>(&'a self) -> impl Iterator<Item = &T>
                where
                    Self: Sized, T: 'a;
            }
        }
    }
}
