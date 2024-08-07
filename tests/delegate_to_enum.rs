use delegate::delegate;

enum Enum {
    A(A),
    B(B),
    C { v: C },
}

struct A {
    val: usize,
}

impl A {
    fn dbg_inner(&self) -> usize {
        dbg!(self.val);
        1
    }

    fn into_num(&self) -> u8 {
        1
    }
}
struct B {
    val_a: String,
}

impl B {
    fn dbg_inner(&self) -> usize {
        dbg!(self.val_a.clone());
        2
    }

    fn into_num(&self) -> u64 {
        2
    }
}

struct C {
    val_c: f64,
}

impl C {
    fn dbg_inner(&self) -> usize {
        dbg!(self.val_c);
        3
    }

    fn into_num(&self) -> usize {
        3
    }
}

impl Enum {
    delegate! {
        to match self {
            Enum::A(a) => a,
            Enum::B(b) => { println!("i am b"); b },
            Enum::C { v: c } => { c },
        } {
            fn dbg_inner(&self) -> usize;

            #[into]
            fn into_num(&self) -> IntoUsize;
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
struct IntoUsize(usize);

impl From<usize> for IntoUsize {
    fn from(value: usize) -> Self {
        Self(value)
    }
}
impl From<u64> for IntoUsize {
    fn from(value: u64) -> Self {
        Self(value as usize)
    }
}
impl From<u8> for IntoUsize {
    fn from(value: u8) -> Self {
        Self(value as usize)
    }
}

#[test]
fn test_delegate_enum_method() {
    let a = Enum::A(A { val: 1 });
    assert_eq!(a.dbg_inner(), 1);
    let b = Enum::B(B {
        val_a: "a".to_string(),
    });
    assert_eq!(b.dbg_inner(), 2);
    let c = Enum::C {
        v: C { val_c: 1.0 },
    };
    assert_eq!(c.dbg_inner(), 3);
}

#[test]
fn test_delegate_enum_into() {
    let a = Enum::A(A { val: 1 });
    assert_eq!(a.into_num(), IntoUsize(1));
    let b = Enum::B(B {
        val_a: "".to_string(),
    });
    assert_eq!(b.into_num(), IntoUsize(2));
    let c = Enum::C {
        v: C { val_c: 0.0 },
    };
    assert_eq!(c.into_num(), IntoUsize(3));
}
