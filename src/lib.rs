#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_imports,
    unused_qualifications,
    missing_docs
)]

//! This crate provides a means of easily creating a wrapper new type along
//! with a corresponding reference type

#[cfg(test)]
extern crate arrayvec;
#[cfg(all(feature = "serde", test))]
extern crate bincode;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[cfg(test)]
#[macro_use]
extern crate proptest;
#[cfg(all(feature = "serde", test))]
extern crate serde;
#[cfg(all(feature = "serde", test))]
#[macro_use]
extern crate serde_derive;
#[cfg(test)]
#[macro_use]
extern crate static_assertions;

mod traits;

pub use traits::NewTypeRef;

#[macro_use]
mod new_type_pair;
