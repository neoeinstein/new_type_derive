#[macro_export]
/// Creates a wrapper new type around a chosen owned type, along with a
/// matching reference type. The reference type must implement `NewTypeRef`.
///
/// In order to add additional implementation for both types, add an `impl`
/// block for the reference type after the macro invocation.
///
/// This macro can currently only build new types on `str` string slices.
///
/// # Example
///
/// ```
/// # #[macro_use] extern crate new_type_derive;
/// # #[macro_use] extern crate static_assertions;
/// # #[cfg(feature = "serde")]
/// # extern crate serde;
/// #
/// use new_type_derive::NewTypeRef;
///
/// new_type_pair! {
///    #[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
///    /// Add your type documentation here
///    pub struct MyNewType(String);
///
///    #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
///    /// And add similar documentation for the reference type
///    pub struct MyNewTypeRef(str);
/// }
///
/// impl NewTypeRef for MyNewTypeRef {
///     type Owned = MyNewType;
///     type InnerRef = str;
///     type ValidationError = String;
///
///     fn validate(value: &Self::InnerRef) -> Result<(), Self::ValidationError> {
///         if value.is_empty() {
///             return Err(String::from("Bad string"));
///         }
///         Ok(())
///     }
///
///     fn to_owned(&self) -> Self::Owned {
///         let inner = self.inner.into();
///         MyNewType { inner }
///     }
/// }
///
/// impl MyNewTypeRef {
///     pub fn len(&self) -> usize {
///         self.inner.len()
///     }
/// }
///
/// # pub fn main() {
/// assert!(MyNewTypeRef::new("").is_err());
/// assert!(MyNewType::new("Test").is_ok());
/// assert_eq!(
///     MyNewType::new("X").unwrap(),
///     MyNewTypeRef::new("X").unwrap(),
/// );
/// assert_eq!(
///     MyNewType::new("X").unwrap(),
///     MyNewType::from(MyNewTypeRef::new("X").unwrap()),
/// );
/// assert_eq!("X", MyNewType::new("X").unwrap(),);
/// assert_eq!(1, MyNewType::new("X").unwrap().as_ref().len(),);
/// # }
/// ```
macro_rules! new_type_pair {
    (   $(#[$ometa:meta])*
        pub struct $otype:ident($itype:ty);

        $(#[$rmeta:meta])*
        pub struct $rtype:ident($stype:ty);
    ) => {
$(#[$ometa])*
pub struct $otype {
    inner: $itype
}

impl $otype {
    /// Creates a new type by consuming and validating `value` and then returning the wrapped value or an error
    pub fn new(value: impl ::std::convert::Into<$itype>) -> ::std::result::Result<Self, <$rtype as NewTypeRef>::ValidationError> {
        let inner = ::std::convert::Into::into(value);
        <$rtype as NewTypeRef>::validate(::std::convert::AsRef::<$stype>::as_ref(&inner))?;
        Ok($otype { inner })
    }

    #[inline]
    /// Consumes the wrapper, returning the unwrapped inner value
    pub fn into_inner(self) -> $itype {
        self.inner
    }
}

$(#[$rmeta])*
pub struct $rtype {
    inner: $stype
}

impl $rtype {
    /// Creates a reference by validating `value` and then returning a typed reference to the value or an error
    #[allow(unsafe_code)]
    pub fn new<S: ::std::convert::AsRef<$stype> + ?Sized>(value: &S) -> ::std::result::Result<&Self, <$rtype as NewTypeRef>::ValidationError> {
        let inner_ref = ::std::convert::AsRef::as_ref(value);
        <Self as NewTypeRef>::validate(inner_ref)?;
        Ok(unsafe { Self::from_unchecked(inner_ref) })
    }

    #[inline]
    #[allow(trivial_casts, unsafe_code)]
    unsafe fn from_unchecked(s: &$stype) -> &$rtype {
        &*(s as *const $stype as *const $rtype)
    }
}

impl ::std::ops::Deref for $otype {
    type Target = $rtype;

    #[inline]
    fn deref(&self) -> &$rtype {
        ::std::convert::AsRef::as_ref(self)
    }
}

impl ::std::borrow::Borrow<$rtype> for $otype {
    fn borrow(&self) -> &$rtype {
        ::std::convert::AsRef::as_ref(self)
    }
}

impl ::std::borrow::Borrow<$stype> for $otype {
    fn borrow(&self) -> &$stype {
        ::std::convert::AsRef::as_ref(::std::convert::AsRef::as_ref(self))
    }
}

impl ::std::borrow::Borrow<$stype> for $rtype {
    fn borrow(&self) -> &$stype {
        &self.inner
    }
}

impl ::std::convert::AsRef<$rtype> for $otype {
    #[allow(unsafe_code)]
    #[inline]
    fn as_ref(&self) -> &$rtype {
        unsafe { $rtype::from_unchecked(::std::convert::AsRef::as_ref(&self.inner)) }
    }
}

impl ::std::convert::AsRef<$stype> for $rtype {
    #[inline]
    fn as_ref(&self) -> &$stype {
        &self.inner
    }
}

impl<'a> ::std::cmp::PartialEq<$otype> for &'a $rtype {
    #[inline]
    fn eq(&self, other: &$otype) -> bool {
         ::std::cmp::PartialEq::eq(::std::convert::AsRef::<$stype>::as_ref(&self), ::std::convert::AsRef::<$stype>::as_ref(::std::convert::AsRef::<$rtype>::as_ref(&other)))
    }
}

impl<'a> ::std::cmp::PartialEq<$otype> for &'a $stype {
    #[inline]
    fn eq(&self, other: &$otype) -> bool {
         ::std::cmp::PartialEq::eq(*self, ::std::convert::AsRef::<$stype>::as_ref(::std::convert::AsRef::<$rtype>::as_ref(&other)))
    }
}

impl<'a> ::std::cmp::PartialEq<&'a $stype> for $otype {
    #[inline]
    fn eq(&self, other: &&'a $stype) -> bool {
         ::std::cmp::PartialEq::eq(::std::convert::AsRef::<$stype>::as_ref(::std::convert::AsRef::<$rtype>::as_ref(&self)), *other)
    }
}

impl<'a> ::std::cmp::PartialEq<&'a $rtype> for $otype {
    #[inline]
    fn eq(&self, other: &&'a $rtype) -> bool {
         ::std::cmp::PartialEq::eq(::std::convert::AsRef::<$stype>::as_ref(::std::convert::AsRef::<$rtype>::as_ref(&self)), ::std::convert::AsRef::<$stype>::as_ref(&other))
    }
}

impl ::std::cmp::PartialEq<$rtype> for $stype {
    #[inline]
    fn eq(&self, other: &$rtype) -> bool {
         ::std::cmp::PartialEq::eq(self, &other.inner)
    }
}

impl ::std::cmp::PartialEq<$stype> for $rtype {
    #[inline]
    fn eq(&self, other: &$stype) -> bool {
         ::std::cmp::PartialEq::eq(&self.inner, other)
    }
}

impl<'a> ::std::cmp::PartialOrd<$otype> for &'a $rtype {
    #[inline]
    fn partial_cmp(&self, other: &$otype) -> ::std::option::Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(::std::convert::AsRef::<$stype>::as_ref(&self), ::std::convert::AsRef::<$stype>::as_ref(::std::convert::AsRef::<$rtype>::as_ref(&other)))
    }
}

impl<'a> ::std::cmp::PartialOrd<$otype> for &'a $stype {
    #[inline]
    fn partial_cmp(&self, other: &$otype) -> ::std::option::Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(*self, ::std::convert::AsRef::<$stype>::as_ref(::std::convert::AsRef::<$rtype>::as_ref(&other)))
    }
}

impl<'a> ::std::cmp::PartialOrd<&'a $stype> for $otype {
    #[inline]
    fn partial_cmp(&self, other: &&'a $stype) -> ::std::option::Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(::std::convert::AsRef::<$stype>::as_ref(::std::convert::AsRef::<$rtype>::as_ref(&self)), *other)
    }
}

impl<'a> ::std::cmp::PartialOrd<&'a $rtype> for $otype {
    #[inline]
    fn partial_cmp(&self, other: &&'a $rtype) -> ::std::option::Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(::std::convert::AsRef::<$stype>::as_ref(::std::convert::AsRef::<$rtype>::as_ref(&self)), ::std::convert::AsRef::<$stype>::as_ref(other))
    }
}

impl ::std::cmp::PartialOrd<$rtype> for $stype {
    #[inline]
    fn partial_cmp(&self, other: &$rtype) -> ::std::option::Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(self, &other.inner)
    }
}

impl ::std::cmp::PartialOrd<$stype> for $rtype {
    #[inline]
    fn partial_cmp(&self, other: &$stype) -> ::std::option::Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(&self.inner, other)
    }
}

impl<'a> From<&'a $rtype> for $otype {
    #[inline]
    fn from(r: &'a $rtype) -> Self {
        r.to_owned()
    }
}

#[cfg(feature = "serde")]
impl ::serde::Serialize for $otype {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where S: ::serde::Serializer {
        ::serde::Serializer::serialize_str(serializer, ::std::convert::AsRef::as_ref(::std::convert::AsRef::<$rtype>::as_ref(&self)))
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(all(test, feature = "mutate"), mutate)]
impl<'de> ::serde::Deserialize<'de> for $otype {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error> where
        D: ::serde::Deserializer<'de> {
        let inner: $itype = ::serde::Deserialize::deserialize(deserializer)?;
        Ok($otype::new(inner).map_err(|e| ::serde::de::Error::custom(e.to_string()))?)
    }
}

#[cfg(feature = "serde")]
impl ::serde::Serialize for $rtype {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: ::serde::Serializer {
        ::serde::Serializer::serialize_str(serializer, ::std::convert::AsRef::as_ref(&self))
    }
}

#[cfg(feature = "serde")]
impl<'de> ::serde::Deserialize<'de> for &'de $rtype {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error> where
        D: ::serde::Deserializer<'de> {
        struct InnerVisitor;

        impl<'de> ::serde::de::Visitor<'de> for InnerVisitor {
            type Value = &'de $rtype;

            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::result::Result<(), ::std::fmt::Error> {
                formatter.write_str("a valid, borrowable MCP SKU")
            }

            fn visit_borrowed_str<E>(self, v: &'de str) -> ::std::result::Result<Self::Value, E> where
                E: ::serde::de::Error, {
                Ok($rtype::new(v).map_err(|e| ::serde::de::Error::custom(::std::string::ToString::to_string(&e)))?)
            }
        }

        ::serde::Deserializer::deserialize_str(deserializer, InnerVisitor)
    }
}
    };
}

#[cfg(test)]
mod test {
    use arrayvec::ArrayString;
    use bincode;
    use std::fmt;
    use NewTypeRef;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct EmptyStringError;

    impl fmt::Display for EmptyStringError {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            f.write_str("string must not be empty")
        }
    }

    new_type_pair! {
        #[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
        /// And now it's documented!
        pub struct StrWrap(String);

        #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
        /// Even the reference type is documented!
        pub struct StrWrapRef(str);
    }

    impl NewTypeRef for StrWrapRef {
        type Owned = StrWrap;
        type InnerRef = str;
        type ValidationError = EmptyStringError;

        fn validate(value: &Self::InnerRef) -> Result<(), Self::ValidationError> {
            if value.is_empty() {
                return Err(EmptyStringError);
            }
            Ok(())
        }

        fn to_owned(&self) -> Self::Owned {
            let inner = String::from(self.as_ref());
            StrWrap { inner }
        }
    }

    new_type_pair! {
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
        /// And now it's documented!
        pub struct ArrStrWrap(ArrayString<[u8;16]>);

        #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
        /// Even the reference type is documented!
        pub struct ArrStrWrapRef(str);
    }

    impl NewTypeRef for ArrStrWrapRef {
        type Owned = ArrStrWrap;
        type InnerRef = str;
        type ValidationError = String;

        fn validate(value: &Self::InnerRef) -> Result<(), Self::ValidationError> {
            if value.is_empty() {
                return Err(String::from("Empty!"));
            } else if value.len() > 16 {
                return Err(String::from("Too Long!"));
            }
            Ok(())
        }

        fn to_owned(&self) -> Self::Owned {
            let inner = ArrayString::from(self.as_ref()).unwrap();
            ArrStrWrap { inner }
        }
    }

    #[test]
    fn minimal() {
        assert!(StrWrap::new("x").is_ok());
        assert!(StrWrapRef::new("").is_err());
    }

    const TEST_STRING: &str = "TESTING";
    lazy_static! {
        static ref SERIALIZED_TEST_STRING: Vec<u8> = bincode::serialize(TEST_STRING).unwrap();
    }

    #[test]
    fn str_wrap_is_serializable() {
        let value = bincode::serialize(&StrWrap::new(TEST_STRING).unwrap())
            .expect("serialization should succeed");
        assert_eq!(*SERIALIZED_TEST_STRING, value);
    }

    #[test]
    fn arr_str_wrap_is_serializable() {
        let value = bincode::serialize(
            &ArrStrWrap::new(ArrayString::from(TEST_STRING).unwrap()).unwrap(),
        ).expect("serialization should succeed");
        assert_eq!(*SERIALIZED_TEST_STRING, value);
    }

    #[test]
    fn str_wrap_ref_is_serializable() {
        let value = bincode::serialize(StrWrapRef::new(TEST_STRING).unwrap())
            .expect("serialization should succeed");
        assert_eq!(*SERIALIZED_TEST_STRING, value);
    }

    #[test]
    fn arr_str_wrap_ref_is_serializable() {
        let value = bincode::serialize(
            ArrStrWrapRef::new(&ArrayString::<[u8; 16]>::from(TEST_STRING).unwrap()).unwrap(),
        ).expect("serialization should succeed");
        assert_eq!(*SERIALIZED_TEST_STRING, value);
    }

    #[test]
    fn str_wrap_is_deserializable() {
        let value: StrWrap =
            bincode::deserialize(&SERIALIZED_TEST_STRING).expect("deserialization to succeed");
        assert_eq!(value, TEST_STRING);
    }

    #[test]
    fn arr_str_wrap_is_deserializable() {
        let value: ArrStrWrap =
            bincode::deserialize(&SERIALIZED_TEST_STRING).expect("deserialization to succeed");
        assert_eq!(value, TEST_STRING);
    }

    #[test]
    fn str_wrap_ref_is_deserializable() {
        let value: &StrWrapRef =
            bincode::deserialize(&SERIALIZED_TEST_STRING).expect("deserialization to succeed");
        assert_eq!(value, TEST_STRING);
    }

    #[test]
    fn arr_str_wrap_ref_is_deserializable() {
        let value: &ArrStrWrapRef =
            bincode::deserialize(&SERIALIZED_TEST_STRING).expect("deserialization to succeed");
        assert_eq!(value, TEST_STRING);
    }

    #[test]
    fn sizes_of_types_match_expectations() {
        use std::mem::size_of_val;
        const TEST_STR: &str = "TESTING!";
        println!("Source string: {}", TEST_STR);
        println!("str: {}", size_of_val(TEST_STR));
        println!("&str: {}", size_of_val(&TEST_STR));
        println!("String: {}", size_of_val(&String::from(TEST_STR)));
        println!(
            "StrWrapRef: {}",
            size_of_val(StrWrapRef::new(TEST_STR).unwrap())
        );
        println!(
            "&StrWrapRef: {}",
            size_of_val(&StrWrapRef::new(TEST_STR).unwrap())
        );
        println!("StrWrap: {}", size_of_val(&StrWrap::new(TEST_STR).unwrap()));
        println!(
            "ArrStrWrapRef: {}",
            size_of_val(ArrStrWrapRef::new(TEST_STR).unwrap())
        );
        println!(
            "&ArrStrWrapRef: {}",
            size_of_val(&ArrStrWrapRef::new(TEST_STR).unwrap())
        );
        println!(
            "ArrStrWrap: {}",
            size_of_val(&ArrStrWrap::new(ArrayString::from(TEST_STR).unwrap()).unwrap())
        );
        println!(
            "[ArrStrWrap;2]: {}",
            size_of_val(&[
                ArrStrWrap::new(ArrayString::from(TEST_STR).unwrap()).unwrap(),
                ArrStrWrap::new(ArrayString::from(TEST_STR).unwrap()).unwrap()
            ])
        );
        assert_eq_size_ptr!(&TEST_STR, &StrWrapRef::new(TEST_STR).unwrap());
        assert_eq_size_ptr!(&TEST_STR, &ArrStrWrapRef::new(TEST_STR).unwrap());
        assert_eq_size_val!(
            String::from(TEST_STR),
            StrWrap::new(String::from(TEST_STR)).unwrap()
        );
        assert_eq_size_val!(
            ArrayString::<[u8; 16]>::from(TEST_STR).unwrap(),
            ArrStrWrap::new(ArrayString::from(TEST_STR).unwrap()).unwrap()
        );
    }

    proptest! {
        #[test]
        fn wrapped_equal_or_error_same(ref s in ".*") {
            let or = StrWrap::new(s.to_owned());
            let rr = StrWrapRef::new(s);

            match (or, rr) {
                (Ok(o), Ok(r)) => assert_eq!(o, r),
                (Err(oe), Err(re)) => assert_eq!(oe, re),
                (Ok(_), Err(e)) => panic!("Owned succeeded while ref failed with: {:?}", e),
                (Err(e), Ok(_)) => panic!("Ref succeeded while owned failed with: {:?}", e),
            }
        }
    }

    proptest! {
        #[test]
        fn arr_wrapped_equal_or_error_same(ref s in ".*") {
            let or = ArrayString::from(s).map_err(|e| format!("{:?}", e)).and_then(|s| ArrStrWrap::new(s).map_err(|e| format!("{:?}", e)));
            let rr = ArrStrWrapRef::new(s).map_err(|e| format!("{:?}", e));

            match (or, rr) {
                (Ok(o), Ok(r)) => assert_eq!(o, r),
                (Err(_), Err(_)) => {},
                (Ok(_), Err(e)) => panic!("Owned succeeded while ref failed with: {:?}", e),
                (Err(e), Ok(_)) => panic!("Ref succeeded while owned failed with: {:?}", e),
            }
        }
    }

}
