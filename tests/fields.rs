use delegate::delegate;

struct Datum {
    value: u32,
    error: u32,
    xy: (f32, f32),
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

            /// Expands to `&self.0.value`
            #[field(&value)]
            fn renamed_value_ref(&self) -> &u32;

            /// Expands to `&mut self.0.value`
            #[field(&mut value)]
            fn renamed_value_ref_mut(&mut self) -> &mut u32;

            /// Expands to `&self.0.error` (demonstrates `&` without a field name)
            #[field(&)]
            fn error(&self) -> &u32;
        }
        to self.0.xy {
            /// Expands to `self.0.xy.0` (demonstrates unnamed field access by value)
            #[field(0)]
            fn x(&self) -> f32;
            /// Expands to `&self.0.xy.1` (demonstrates unnamed field access by reference)
            #[field(&1)]
            fn y(&self) -> &f32;
        }
        to self.get_inner() {
            /// Expands to `&self.get_inner().value`
            #[field(&value)]
            fn value_ref_via_get_inner(&self) -> &u32;
        }
    }
}

#[test]
fn test_fields() {
    let mut wrapper = DatumWrapper(Datum {
        value: 1,
        error: 2,
        xy: (3.0, 4.0),
    });
    assert_eq!(wrapper.value(), wrapper.0.value);
    assert_eq!(wrapper.renamed_value(), wrapper.0.value);
    assert_eq!(wrapper.renamed_value_ref(), &wrapper.0.value);
    assert_eq!(wrapper.renamed_value_ref_mut(), &mut 1);
    assert_eq!(wrapper.value_ref_via_get_inner(), &1);
    assert_eq!(wrapper.error(), &wrapper.0.error);
    assert_eq!(wrapper.x(), wrapper.0.xy.0);
    assert_eq!(wrapper.y(), &wrapper.0.xy.1);
}
