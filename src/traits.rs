/// A trait that provides necessary operations for creating a new type with
/// reference type that can round-trip between the two types and the original
/// wrapped value.
pub trait NewTypeRef {
    /// The owned type, must be able to take `Self` as a reference.
    type Owned: AsRef<Self>;
    /// The type of the inner value for the reference type, e.g. `str` or
    /// `[u8]`.
    ///
    /// Currently only `str` is supported.
    type InnerRef: ?Sized;
    /// The error type that is returned in the event validation fails.
    type ValidationError;

    #[allow(unused_variables)]
    /// Validate the value before allowing it to be wrapped in the new type.
    fn validate(value: &Self::InnerRef) -> Result<(), Self::ValidationError> {
        Ok(())
    }

    /// Convert the reference into an owned value.
    ///
    /// The implementation of this must not fail in order for valid values to
    /// round-trip between the owned and reference wrappers.
    fn to_owned(&self) -> Self::Owned;
}
