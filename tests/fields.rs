
use delegate::delegate;

struct Datum {
    name: String,
    value: u32,
    error: u32,
    xy: (f32, f32)
}

struct DatumWrapper(Datum);

impl DatumWrapper {
    fn get_inner(&self) -> &Datum {
        &self.0
    }
    delegate! {
        to self.0 {
            /// Expands to `self.0.value`
            #[field]
            fn value(&self) -> u32;

            /// Expands to `self.0.value`
            #[field(value)]
            fn renamed_value(&self) -> u32;

            /// Expands to `&self.0.value` (equivalent to `#[field(&value)]`)
            #[field(ref value)]
            fn renamed_value_ref(&self) -> &u32;

            /// Expands to `&mut self.0.value` (equivalent to `#[field(&mut value)]`)
            #[field(ref mut value)]
            fn renamed_value_ref_mut(&mut self) -> &mut u32;

            /// Expands to `&self.0.error` (demonstrates `ref` without a field name)
            #[field(ref)]
            fn error(&self) -> &u32;
        }
        to self.0.xy {
            /// Expands to `self.0.xy.0` (demonstrates unnamed field access by value)
            #[field(0)]
            fn x(&self) -> f32;
            /// Expands to `&self.0.xy.1` (demonstrates unnamed field access by reference)
            #[field(ref 1)]
            fn y(&self) -> &f32;
        }
    }
}

#[test]
fn test_fields() {
    let mut wrapper = DatumWrapper(Datum {
        name: "foo".to_string(),
        value: 1,
        error: 2,
        xy: (3.0, 4.0),
    });
    assert_eq!(wrapper.value(), wrapper.0.value);
    assert_eq!(wrapper.renamed_value(), wrapper.0.value);
    assert_eq!(wrapper.renamed_value_ref(), &wrapper.0.value);
    assert_eq!(wrapper.renamed_value_ref_mut(), &mut 1);
    assert_eq!(wrapper.error(), &wrapper.0.error);
    assert_eq!(wrapper.x(), wrapper.0.xy.0);
    assert_eq!(wrapper.y(), &wrapper.0.xy.1);
}
