extern crate delegate;
use delegate::delegate;

#[test]
fn test_delegate_constant() {
    trait WithConst {
        const TOTO: u8;
    }

    struct A;
    impl WithConst for A {
        const TOTO: u8 = 1;
    }

    struct B;
    impl WithConst for B {
        const TOTO: u8 = 2;
    }
    struct C;
    impl WithConst for C {
        const TOTO: u8 = 2;
    }

    enum Enum {
        A(A),
        B(B),
        C(C),
    }

    impl Enum {
        delegate! {
            to match self {
                Self::A(a) => a,
                Self::B(b) => b,
                Self::C(c) => { println!("hello from c"); c },
            } {
                #[const(WithConst::TOTO)]
                fn get_toto(&self) -> u8;
            }
        }
    }

    let a = Enum::A(A);
    assert_eq!(a.get_toto(), <A as WithConst>::TOTO);
    let b = Enum::B(B);
    assert_eq!(b.get_toto(), <B as WithConst>::TOTO);
    let c = Enum::C(C);
    assert_eq!(c.get_toto(), <C as WithConst>::TOTO);
}

#[test]
fn multiple_consts() {
    trait Foo {
        const A: u32;
        const B: u32;
    }

    struct A;
    impl Foo for A {
        const A: u32 = 1;
        const B: u32 = 2;
    }

    struct Wrapper(A);
    impl Wrapper {
        delegate! {
            to &self.0 {
                #[const(Foo::A)]
                fn a(&self) -> u32;

                #[const(Foo::B)]
                fn b(&self) -> u32;
            }
        }
    }

    let wrapper = Wrapper(A);
    assert_eq!(wrapper.a(), <A as Foo>::A);
    assert_eq!(wrapper.b(), <A as Foo>::B);
}
