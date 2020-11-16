///
/// Tests that a macro can expand into a delegate block,
/// where the target comes from a macro variable.
///
use delegate::delegate;

trait Trait {
    fn method_to_delegate(&self) -> bool;
}

struct Struct1();
impl Trait for Struct1 {
    fn method_to_delegate(&self) -> bool {
        true
    }
}

struct Struct2(pub Struct1);

// We use a macro to impl 'Trait' for 'Struct2' such that
// the target is a variable in the macro.
macro_rules! some_macro {
    (|$self:ident| $delegate_to:expr) => {
        impl Trait for Struct2 {
            delegate! {
                // '$delegate_to' will expand to 'self.0' before ´delegate!' is expanded.
                to $delegate_to {
                    fn method_to_delegate(&$self) -> bool;
                }
            }
        }
    };
}
some_macro! { |self | self.0 }

#[test]
fn test() {
    assert!(Struct2(Struct1()).method_to_delegate());
}
