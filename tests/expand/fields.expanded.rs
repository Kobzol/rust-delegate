use delegate::delegate;
struct Datum {
    name: String,
    value: u32,
    error: u32,
    xy: (f32, f32),
}
struct DatumWrapper(Datum);
impl DatumWrapper {
    fn get_inner(&self) -> &Datum {
        &self.0
    }
    /// Expands to `self.0.value`
    #[inline]
    fn value(&self) -> u32 {
        self.0.value
    }
    /// Expands to `self.0.value`
    #[inline]
    fn renamed_value(&self) -> u32 {
        self.0.value
    }
    /// Expands to `&self.0.value` (equivalent to `#[field(ref value)]`)
    #[inline]
    fn renamed_value_ref(&self) -> &u32 {
        &self.0.value
    }
    /// Expands to `&self.0.value` (equivalent to `#[field(&value)]`)
    #[inline]
    fn renamed_value_ref_keyword(&self) -> &u32 {
        &self.0.value
    }
    /// Expands to `&mut self.0.value` (equivalent to `#[field(ref mut value)]`)
    #[inline]
    fn renamed_value_ref_mut(&mut self) -> &mut u32 {
        &mut self.0.value
    }
    /// Expands to `&mut self.0.value` (equivalent to `#[field(&mut value)]`)
    #[inline]
    fn renamed_value_ref_mut_keyword(&mut self) -> &mut u32 {
        &mut self.0.value
    }
    /// Expands to `&self.0.error` (demonstrates `ref` without a field name)
    #[inline]
    fn error(&self) -> &u32 {
        &self.0.error
    }
    /// Expands to `self.0.xy.0` (demonstrates unnamed field access by value)
    #[inline]
    fn x(&self) -> f32 {
        self.0.xy.0
    }
    /// Expands to `&self.0.xy.1` (demonstrates unnamed field access by reference)
    #[inline]
    fn y(&self) -> &f32 {
        &self.0.xy.1
    }
    /// Expands to `&self.get_inner().value`
    #[inline]
    fn value_ref_via_get_inner(&self) -> &u32 {
        &self.get_inner().value
    }
}
