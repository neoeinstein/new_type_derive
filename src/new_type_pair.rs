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
/// assert!(MyNewTypeRef::try_as_ref("").is_err());
/// assert!(MyNewType::try_from("Test").is_ok());
/// assert_eq!(
///     MyNewType::try_from("X").unwrap(),
///     MyNewTypeRef::try_as_ref("X").unwrap(),
/// );
/// assert_eq!(
///     MyNewType::try_from("X").unwrap(),
///     MyNewType::from(MyNewTypeRef::try_as_ref("X").unwrap()),
/// );
/// assert_eq!("X", MyNewType::try_from("X").unwrap(),);
/// assert_eq!(1, MyNewType::try_from("X").unwrap().as_ref().len(),);
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
    pub fn try_from(value: impl Into<$itype>) -> Result<Self, <$rtype as NewTypeRef>::ValidationError> {
        let inner = value.into();
        <$rtype as NewTypeRef>::validate(inner.as_ref())?;
        Ok($otype { inner })
    }

}

$(#[$rmeta])*
pub struct $rtype {
    inner: $stype
}

impl $rtype {
    /// Creates a reference by validating `value` and then returning a typed reference to the value or an error
    #[allow(unsafe_code)]
    pub fn try_as_ref<S: AsRef<$stype> + ?Sized>(value: &S) -> Result<&Self, <$rtype as NewTypeRef>::ValidationError> {
        let inner_ref = value.as_ref();
        <Self as NewTypeRef>::validate(inner_ref)?;
        Ok(#[allow(unsafe_code)] unsafe { Self::from_unchecked(inner_ref) })
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
        self.as_ref()
    }
}

impl ::std::borrow::Borrow<$rtype> for $otype {
    #[inline]
    fn borrow(&self) -> &$rtype {
        self.as_ref()
    }
}

impl ::std::borrow::Borrow<$stype> for $otype {
    #[inline]
    fn borrow(&self) -> &$stype {
        self.as_ref().as_ref()
    }
}

impl ::std::borrow::Borrow<$stype> for $rtype {
    #[inline]
    fn borrow(&self) -> &$stype {
        self.as_ref()
    }
}

impl ::std::convert::AsRef<$rtype> for $otype {
    #[inline]
    fn as_ref(&self) -> &$rtype {
        #[allow(unsafe_code)] unsafe { $rtype::from_unchecked(self.inner.as_ref()) }
    }
}

impl ::std::convert::AsRef<$stype> for $rtype {
    #[inline]
    fn as_ref(&self) -> &$stype {
        &self.inner
    }
}

impl ::std::convert::AsRef<$rtype> for $rtype {
    #[inline]
    fn as_ref(&self) -> &$rtype {
        &self
    }
}

impl<'a> ::std::cmp::PartialEq<$otype> for &'a $rtype {
    #[inline]
    fn eq(&self, rhs: &$otype) -> bool {
        *self == rhs.as_ref()
    }
}

impl ::std::cmp::PartialEq<$otype> for $rtype {
    #[inline]
    fn eq(&self, rhs: &$otype) -> bool {
        self == rhs.as_ref()
    }
}

impl<'a> ::std::cmp::PartialEq<$otype> for &'a $stype {
    #[inline]
    fn eq(&self, rhs: &$otype) -> bool {
        self == rhs.as_ref()
    }
}

impl ::std::cmp::PartialEq<$otype> for $stype {
    #[inline]
    fn eq(&self, rhs: &$otype) -> bool {
        self == rhs.as_ref()
    }
}

impl<'a> ::std::cmp::PartialEq<&'a $stype> for $otype {
    #[inline]
    fn eq(&self, rhs: &&'a $stype) -> bool {
        self.as_ref() == *rhs
    }
}

impl ::std::cmp::PartialEq<$stype> for $otype {
    #[inline]
    fn eq(&self, rhs: &$stype) -> bool {
        self.as_ref() == *rhs
    }
}

impl<'a> ::std::cmp::PartialEq<&'a $rtype> for $otype {
    #[inline]
    fn eq(&self, rhs: &&'a $rtype) -> bool {
        self.as_ref() == *rhs
    }
}

impl ::std::cmp::PartialEq<$rtype> for $otype {
    #[inline]
    fn eq(&self, rhs: &$rtype) -> bool {
        self.as_ref() == rhs
    }
}

impl ::std::cmp::PartialEq<$rtype> for $stype {
    #[inline]
    fn eq(&self, rhs: &$rtype) -> bool {
        self == &rhs.inner
    }
}

impl<'a> ::std::cmp::PartialEq<&'a $rtype> for $stype {
    #[inline]
    fn eq(&self, rhs: &&'a $rtype) -> bool {
        self == &rhs.inner
    }
}

impl<'a> ::std::cmp::PartialEq<$rtype> for &'a $stype {
    #[inline]
    fn eq(&self, rhs: &$rtype) -> bool {
        *self == &rhs.inner
    }
}

impl ::std::cmp::PartialEq<$stype> for $rtype {
    #[inline]
    fn eq(&self, rhs: &$stype) -> bool {
        &self.inner == rhs
    }
}

impl<'a> ::std::cmp::PartialEq<&'a $stype> for $rtype {
    #[inline]
    fn eq(&self, rhs: &&'a $stype) -> bool {
        &self.inner == *rhs
    }
}

impl<'a> ::std::cmp::PartialEq<$stype> for &'a $rtype {
    #[inline]
    fn eq(&self, rhs: &$stype) -> bool {
        &self.inner == rhs
    }
}

impl<'a> ::std::cmp::PartialOrd<$otype> for &'a $rtype {
    #[inline]
    fn partial_cmp(&self, rhs: &$otype) -> Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(
            self,
            &rhs.as_ref(),
        )
    }
}

impl ::std::cmp::PartialOrd<$otype> for $rtype {
    #[inline]
    fn partial_cmp(&self, rhs: &$otype) -> Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(
            &self,
            &rhs.as_ref(),
        )
    }
}

impl<'a> ::std::cmp::PartialOrd<$otype> for &'a $stype {
    #[inline]
    fn partial_cmp(&self, rhs: &$otype) -> Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(
            self,
            &rhs.as_ref(),
        )
    }
}

impl ::std::cmp::PartialOrd<$otype> for $stype {
    #[inline]
    fn partial_cmp(&self, rhs: &$otype) -> Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(
            &self,
            &rhs.as_ref(),
        )
    }
}

impl<'a> ::std::cmp::PartialOrd<&'a $rtype> for $otype {
    #[inline]
    fn partial_cmp(&self, rhs: &&'a $rtype) -> Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(
            &self.as_ref(),
            rhs,
        )
    }
}

impl ::std::cmp::PartialOrd<$rtype> for $otype {
    #[inline]
    fn partial_cmp(&self, rhs: &$rtype) -> Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(
            &self.as_ref(),
            &rhs,
        )
    }
}

impl<'a> ::std::cmp::PartialOrd<&'a $stype> for $otype {
    #[inline]
    fn partial_cmp(&self, rhs: &&'a $stype) -> Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(
            &self.as_ref(),
            rhs,
        )
    }
}

impl ::std::cmp::PartialOrd<$stype> for $otype {
    #[inline]
    fn partial_cmp(&self, rhs: &$stype) -> Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(
            &self.as_ref(),
            &rhs,
        )
    }
}

impl ::std::cmp::PartialOrd<$rtype> for $stype {
    #[inline]
    fn partial_cmp(&self, rhs: &$rtype) -> Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(self, &rhs.inner)
    }
}

impl<'a> ::std::cmp::PartialOrd<&'a $rtype> for $stype {
    #[inline]
    fn partial_cmp(&self, rhs: &&'a $rtype) -> Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(self, &rhs.inner)
    }
}

impl<'a> ::std::cmp::PartialOrd<$rtype> for &'a $stype {
    #[inline]
    fn partial_cmp(&self, rhs: &$rtype) -> Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(*self, &rhs.inner)
    }
}

impl ::std::cmp::PartialOrd<$stype> for $rtype {
    #[inline]
    fn partial_cmp(&self, rhs: &$stype) -> Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(&self.inner, rhs)
    }
}

impl<'a> ::std::cmp::PartialOrd<&'a $stype> for $rtype {
    #[inline]
    fn partial_cmp(&self, rhs: &&'a $stype) -> Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(&self.inner, *rhs)
    }
}

impl<'a> ::std::cmp::PartialOrd<$stype> for &'a $rtype {
    #[inline]
    fn partial_cmp(&self, rhs: &$stype) -> Option<::std::cmp::Ordering> {
        ::std::cmp::PartialOrd::partial_cmp(&self.inner, rhs)
    }
}

impl<'a> From<&'a $rtype> for $otype {
    #[inline]
    fn from(r: &'a $rtype) -> Self {
        r.to_owned()
    }
}

impl From<$otype> for $itype {
    #[inline]
    fn from(o: $otype) -> Self {
        o.inner
    }
}

#[cfg(feature = "serde")]
impl ::serde::Serialize for $otype {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where S: ::serde::Serializer {
        ::serde::Serializer::serialize_str(serializer, AsRef::<$stype>::as_ref(AsRef::<$rtype>::as_ref(&self)))
    }
}

#[cfg(feature = "serde")]
impl<'de> ::serde::Deserialize<'de> for $otype {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error> where
        D: ::serde::Deserializer<'de> {
        let inner: $itype = ::serde::Deserialize::deserialize(deserializer)?;
        Ok($otype::try_from(inner).map_err(|e| ::serde::de::Error::custom(e.to_string()))?)
    }
}

#[cfg(feature = "serde")]
impl ::serde::Serialize for $rtype {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: ::serde::Serializer {
        ::serde::Serializer::serialize_str(serializer, AsRef::<$stype>::as_ref(&self))
    }
}

#[cfg(feature = "serde")]
impl<'de: 'a, 'a> ::serde::Deserialize<'de> for &'a $rtype {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error> where
        D: ::serde::Deserializer<'de> {
        let inner: &$stype = ::serde::Deserialize::deserialize(deserializer)?;
        Ok($rtype::try_as_ref(inner).map_err(|e| ::serde::de::Error::custom(e.to_string()))?)
    }
}
    };
}

#[cfg(test)]
mod test {
    use arrayvec::ArrayString;
    #[cfg(feature = "serde")]
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
        assert!(StrWrap::try_from("x").is_ok());
        assert!(StrWrapRef::try_as_ref("").is_err());
    }

    const TEST_STRING: &str = "TESTING";
    const ALT_STRING: &str = "Ĉu ĝustas?";

    #[cfg(feature = "serde")]
    lazy_static! {
        static ref SERIALIZED_TEST_STRING: Vec<u8> = bincode::serialize(TEST_STRING).unwrap();
    }

    #[test]
    fn equality() {
        assert_eq!(TEST_STRING, StrWrap::try_from(TEST_STRING).unwrap());
        assert_eq!(*TEST_STRING, StrWrap::try_from(TEST_STRING).unwrap());
        assert_eq!(TEST_STRING, StrWrapRef::try_as_ref(TEST_STRING).unwrap());
        assert_eq!(*TEST_STRING, StrWrapRef::try_as_ref(TEST_STRING).unwrap());
        assert_eq!(TEST_STRING, *StrWrapRef::try_as_ref(TEST_STRING).unwrap());
        assert_eq!(*TEST_STRING, *StrWrapRef::try_as_ref(TEST_STRING).unwrap());
        assert_eq!(StrWrapRef::try_as_ref(TEST_STRING).unwrap(), StrWrap::try_from(TEST_STRING).unwrap());
        assert_eq!(*StrWrapRef::try_as_ref(TEST_STRING).unwrap(), StrWrap::try_from(TEST_STRING).unwrap());
        assert_eq!(StrWrap::try_from(TEST_STRING).unwrap(), TEST_STRING);
        assert_eq!(StrWrap::try_from(TEST_STRING).unwrap(), *TEST_STRING);
        assert_eq!(StrWrapRef::try_as_ref(TEST_STRING).unwrap(), TEST_STRING);
        assert_eq!(StrWrapRef::try_as_ref(TEST_STRING).unwrap(), *TEST_STRING);
        assert_eq!(*StrWrapRef::try_as_ref(TEST_STRING).unwrap(), TEST_STRING);
        assert_eq!(*StrWrapRef::try_as_ref(TEST_STRING).unwrap(), *TEST_STRING);
        assert_eq!(StrWrap::try_from(TEST_STRING).unwrap(), StrWrapRef::try_as_ref(TEST_STRING).unwrap());
        assert_eq!(StrWrap::try_from(TEST_STRING).unwrap(), *StrWrapRef::try_as_ref(TEST_STRING).unwrap());
    }

    #[test]
    fn cmp() {
        use std::cmp::{Ordering, PartialOrd};
        const EQUAL: Option<Ordering> = Some(Ordering::Equal);
        assert_eq!(EQUAL, PartialOrd::partial_cmp(&TEST_STRING, &StrWrap::try_from(TEST_STRING).unwrap()));
        assert_eq!(EQUAL, PartialOrd::partial_cmp(TEST_STRING, &StrWrap::try_from(TEST_STRING).unwrap()));
        assert_eq!(EQUAL, PartialOrd::partial_cmp(&TEST_STRING, &StrWrapRef::try_as_ref(TEST_STRING).unwrap()));
        assert_eq!(EQUAL, PartialOrd::partial_cmp(TEST_STRING, &StrWrapRef::try_as_ref(TEST_STRING).unwrap()));
        assert_eq!(EQUAL, PartialOrd::partial_cmp(&TEST_STRING, StrWrapRef::try_as_ref(TEST_STRING).unwrap()));
        assert_eq!(EQUAL, PartialOrd::partial_cmp(TEST_STRING, StrWrapRef::try_as_ref(TEST_STRING).unwrap()));
        assert_eq!(EQUAL, PartialOrd::partial_cmp(&StrWrapRef::try_as_ref(TEST_STRING).unwrap(), &StrWrap::try_from(TEST_STRING).unwrap()));
        assert_eq!(EQUAL, PartialOrd::partial_cmp(StrWrapRef::try_as_ref(TEST_STRING).unwrap(), &StrWrap::try_from(TEST_STRING).unwrap()));
        assert_eq!(EQUAL, PartialOrd::partial_cmp(&StrWrap::try_from(TEST_STRING).unwrap(), &TEST_STRING));
        assert_eq!(EQUAL, PartialOrd::partial_cmp(&StrWrap::try_from(TEST_STRING).unwrap(), TEST_STRING));
        assert_eq!(EQUAL, PartialOrd::partial_cmp(&StrWrapRef::try_as_ref(TEST_STRING).unwrap(), &TEST_STRING));
        assert_eq!(EQUAL, PartialOrd::partial_cmp(&StrWrapRef::try_as_ref(TEST_STRING).unwrap(), TEST_STRING));
        assert_eq!(EQUAL, PartialOrd::partial_cmp(StrWrapRef::try_as_ref(TEST_STRING).unwrap(), &TEST_STRING));
        assert_eq!(EQUAL, PartialOrd::partial_cmp(StrWrapRef::try_as_ref(TEST_STRING).unwrap(), TEST_STRING));
        assert_eq!(EQUAL, PartialOrd::partial_cmp(&StrWrap::try_from(TEST_STRING).unwrap(), &StrWrapRef::try_as_ref(TEST_STRING).unwrap()));
        assert_eq!(EQUAL, PartialOrd::partial_cmp(&StrWrap::try_from(TEST_STRING).unwrap(), StrWrapRef::try_as_ref(TEST_STRING).unwrap()));
    }

    #[test]
    fn into_as_ref_roundtrip() {
        let start: String = String::from(TEST_STRING);
        let f1 = StrWrap::try_from(start.clone()).unwrap();
        assert_eq!(TEST_STRING, f1);
        let f2: &StrWrapRef = f1.as_ref();
        assert_eq!(TEST_STRING, f2);
        let f3: &str = f2.as_ref();
        assert_eq!(TEST_STRING, f3);
        let f4: &StrWrapRef = StrWrapRef::try_as_ref(f3).unwrap();
        assert_eq!(TEST_STRING, f4);
        let f5: StrWrap = f4.into();
        assert_eq!(TEST_STRING, f5);
        let end: String = f5.into();
        assert_eq!(start, end);
    }

    #[test]
    fn as_ref_into_roundtrip() {
        let start: &str = TEST_STRING;
        let f1: &StrWrapRef = StrWrapRef::try_as_ref(start).unwrap();
        assert_eq!(TEST_STRING, f1);
        let f2: StrWrap = f1.into();
        assert_eq!(TEST_STRING, f2);
        let f3: String = f2.into();
        assert_eq!(TEST_STRING, f3);
        let f4 = StrWrap::try_from(f3).unwrap();
        assert_eq!(TEST_STRING, f4);
        let f5: &StrWrapRef = f4.as_ref();
        assert_eq!(TEST_STRING, f5);
        let end: &str = f5.as_ref();
        assert_eq!(start, end);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn str_wrap_is_serializable() {
        let value = bincode::serialize(&StrWrap::try_from(TEST_STRING).unwrap())
            .expect("serialization should succeed");
        assert_eq!(*SERIALIZED_TEST_STRING, value);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn arr_str_wrap_is_serializable() {
        let value = bincode::serialize(
            &ArrStrWrap::try_from(ArrayString::from(TEST_STRING).unwrap()).unwrap(),
        ).expect("serialization should succeed");
        assert_eq!(*SERIALIZED_TEST_STRING, value);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn str_wrap_ref_is_serializable() {
        let value = bincode::serialize(StrWrapRef::try_as_ref(TEST_STRING).unwrap())
            .expect("serialization should succeed");
        assert_eq!(*SERIALIZED_TEST_STRING, value);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn arr_str_wrap_ref_is_serializable() {
        let value = bincode::serialize(
            ArrStrWrapRef::try_as_ref(&ArrayString::<[u8; 16]>::from(TEST_STRING).unwrap()).unwrap(),
        ).expect("serialization should succeed");
        assert_eq!(*SERIALIZED_TEST_STRING, value);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn str_wrap_is_deserializable() {
        let value: StrWrap =
            bincode::deserialize(&SERIALIZED_TEST_STRING).expect("deserialization to succeed");
        assert_eq!(value, TEST_STRING);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn arr_str_wrap_is_deserializable() {
        let value: ArrStrWrap =
            bincode::deserialize(&SERIALIZED_TEST_STRING).expect("deserialization to succeed");
        assert_eq!(value, TEST_STRING);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn str_wrap_ref_is_deserializable() {
        let value: &StrWrapRef =
            bincode::deserialize(&SERIALIZED_TEST_STRING).expect("deserialization to succeed");
        assert_eq!(value, TEST_STRING);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn arr_str_wrap_ref_is_deserializable() {
        let value: &ArrStrWrapRef =
            bincode::deserialize(&SERIALIZED_TEST_STRING).expect("deserialization to succeed");
        assert_eq!(value, TEST_STRING);
    }

    #[cfg(feature = "serde")]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    struct RefHolder<'a, 'b> {
        #[serde(borrow)]
        str_wrap: &'a StrWrapRef,
        #[serde(borrow)]
        arr_wrap: &'b ArrStrWrapRef,
    }

    #[test]
    #[cfg(feature = "serde")]
    fn refs_in_structure_can_roundtrip() {
        let source: RefHolder = RefHolder {
            str_wrap: StrWrapRef::try_as_ref(&TEST_STRING).unwrap(),
            arr_wrap: ArrStrWrapRef::try_as_ref(&ALT_STRING).unwrap(),
        };

        let intermediate = bincode::serialize(&source).expect("serialization to succeed");

        let actual: RefHolder = bincode::deserialize(&intermediate).expect("deserialization to succeed");

        assert_eq!(source, actual);
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
            size_of_val(StrWrapRef::try_as_ref(TEST_STR).unwrap())
        );
        println!(
            "&StrWrapRef: {}",
            size_of_val(&StrWrapRef::try_as_ref(TEST_STR).unwrap())
        );
        println!("StrWrap: {}", size_of_val(&StrWrap::try_from(TEST_STR).unwrap()));
        println!(
            "ArrStrWrapRef: {}",
            size_of_val(ArrStrWrapRef::try_as_ref(TEST_STR).unwrap())
        );
        println!(
            "&ArrStrWrapRef: {}",
            size_of_val(&ArrStrWrapRef::try_as_ref(TEST_STR).unwrap())
        );
        println!(
            "ArrStrWrap: {}",
            size_of_val(&ArrStrWrap::try_from(ArrayString::from(TEST_STR).unwrap()).unwrap())
        );
        println!(
            "[ArrStrWrap;2]: {}",
            size_of_val(&[
                ArrStrWrap::try_from(ArrayString::from(TEST_STR).unwrap()).unwrap(),
                ArrStrWrap::try_from(ArrayString::from(TEST_STR).unwrap()).unwrap()
            ])
        );
        assert_eq_size_ptr!(&TEST_STR, &StrWrapRef::try_as_ref(TEST_STR).unwrap());
        assert_eq_size_ptr!(&TEST_STR, &ArrStrWrapRef::try_as_ref(TEST_STR).unwrap());
        assert_eq_size_val!(
            String::from(TEST_STR),
            StrWrap::try_from(String::from(TEST_STR)).unwrap()
        );
        assert_eq_size_val!(
            ArrayString::<[u8; 16]>::from(TEST_STR).unwrap(),
            ArrStrWrap::try_from(ArrayString::from(TEST_STR).unwrap()).unwrap()
        );
    }

    proptest! {
        #[test]
        fn wrapped_equal_or_error_same(ref s in ".*") {
            let or = StrWrap::try_from(s.to_owned());
            let rr = StrWrapRef::try_as_ref(s);

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
            let or = ArrayString::from(s).map_err(|e| format!("{:?}", e)).and_then(|s| ArrStrWrap::try_from(s).map_err(|e| format!("{:?}", e)));
            let rr = ArrStrWrapRef::try_as_ref(s).map_err(|e| format!("{:?}", e));

            match (or, rr) {
                (Ok(o), Ok(r)) => assert_eq!(o, r),
                (Err(_), Err(_)) => {},
                (Ok(_), Err(e)) => panic!("Owned succeeded while ref failed with: {:?}", e),
                (Err(e), Ok(_)) => panic!("Ref succeeded while owned failed with: {:?}", e),
            }
        }
    }

}
