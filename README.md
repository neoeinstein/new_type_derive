# new_type_derive

This crate provides the `new_type_pair!` macro, which allows for the creation of
a wrapping type over primitives and their references. For example, when creating
a strongly-typed wrapper for an identifier around a `String`, we may also want
an accompanying strongly-typed wrapper around the reference type `str`. This can
enable better zero-copy behavior while still keeping the benefits of a
strongly-typed wrapper.

The reference new type must implement the `NewTypeRef` trait, which provides a
mechanism for validating that the value is valid before returning the wrapped
new type. It also ensures that you are able to seamlessly transition between the
reference type, the owned type, the underlying owned type, and the underlying
reference type through the automatic implementation of `From`, `AsRef`,
`Borrow`, `PartialEq`, and `PartialOrd`, as well as `Serialize` and
`Deserialize` when the `serde` feature is enabled.

