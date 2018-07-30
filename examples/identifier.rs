#![cfg(feature = "serde")]

extern crate arrayvec;
#[macro_use] extern crate new_type_derive;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate bincode;

use arrayvec::ArrayString;
use new_type_derive::NewTypeRef;

new_type_pair! {
   #[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
   /// A short identifier
   pub struct ShortId(ArrayString<[u8;8]>);

   #[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
   /// A reference to a short identifier
   pub struct ShortIdRef(str);
}

impl NewTypeRef for ShortIdRef {
    type Owned = ShortId;
    type InnerRef = str;
    type ValidationError = &'static str;

    fn validate(value: &Self::InnerRef) -> Result<(), Self::ValidationError> {
        if value.is_empty() {
            return Err("Empty string");
        }
        if value.len() > 8 {
            return Err("Too long");
        }
        Ok(())
    }

    fn to_owned(&self) -> Self::Owned {
        let inner =
            ArrayString::from(&self.inner)
                .expect("This should never fail because we pre-test the string length");
        Self::Owned { inner }
    }
}

impl ShortIdRef {
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct RefContainer<'a> {
    #[serde(borrow)]
    ident: &'a ShortIdRef,
    hash: u64,
}

fn main() {
    use bincode;
    use std::env;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let id = {
        let mut args_iter = env::args().into_iter();
        args_iter.next();
        args_iter.next().unwrap_or_else(|| String::from("generic"))
    };

    let short_id = ShortIdRef::try_as_ref(&id).expect("valid short id");
    println!("Using ShortId: {}", AsRef::<str>::as_ref(short_id));
    let hash = {
        let hasher = &mut DefaultHasher::new();
        short_id.hash(hasher);
        hasher.finish()
    };

    let in_container = RefContainer {
        ident: short_id,
        hash,
    };
    println!("In container: {:?}", in_container);

    println!("Serializing to bincode…");
    let serialized = bincode::serialize(&in_container).expect("serialization to succeed");
    println!("Serialized as: {:02x?}", serialized);

    println!("Deserializing from bincode…");
    let deserialized: RefContainer =
        bincode::deserialize(&serialized).expect("deserialization to succeed");
    println!("Deserialized as: {:?}", deserialized);

    assert_eq!(in_container, deserialized);
}
