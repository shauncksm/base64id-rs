//! This crate allows for fixed length 64, 32 and 16 bit integers to be represented as [base64url](https://datatracker.ietf.org/doc/html/rfc4648#section-5) encoded strings.
//! This is useful for exchanging unique identifiers in a web based contexts; eg. sending an SQL primary key to a client with as few character as possible.
//!
//! This crate is `#![no_std]` by default.
//! You can use the `std` cargo feature flag to enable support for the standard library
//!
//! ## Quick Start
//! Add the following to your `Cargo.toml` file.
//! ```toml
//! [dependencies]
//! base64id = { version = "0.3", features = ["std", "rand"] }
//! ```
//!
//! #### Encoding
//! You can use the `rand` feature flag to generate a random ID like so.
//! ```
//! use base64id::Id64;
//!
//! fn main() {
//! # #[cfg(feature = "rand")]
//! # {
//!     let id: Id64 = rand::random();
//!     println!("{id}"); // 3Zohppb9XMw
//! # }
//! }
//! ```
//!
//! #### Decoding
//! You can decode a string into an `Id64` using it's `FromStr` impl.
//! ```
//! use std::str::FromStr;
//! use base64id::{Error, Id64};
//!
//! fn main() -> Result<(), Error> {
//!     let id = Id64::from_str("AAAAAAAAAAE")?;
//!     assert_eq!(id, Id64::from(1u64));
//!     Ok(())
//! }
//! ```
//!
//! Refer to the [Error] enum regarding decode errors.
//!
//! ## Serde
//!
//! You can use the `serde` feature flag to derive `Serialize` and `Deserialize` on `Id64`, `Id32` and `Id16`.
//!
//! #### Examples
//!
//! Deserializing a json string into an `Id32`
//! ```
//! # #[cfg(feature = "serde")]
//! # {
//! # use serde_json::Error;
//! # use base64id::Id32;
//! # fn main() -> Result<(), Error> {
//! let id: Id32 = serde_json::from_str(r#""CyRWFA""#)?;
//! assert_eq!(id, Id32::from(186930708));
//! #   Ok(())
//! # }
//! # }
//! ```
//!
//! Serializing a struct containing an `Id64`
//! ```
//! # #[cfg(feature = "serde")]
//! # {
//! # use serde::Serialize;
//! # use serde_json::Error;
//! # use base64id::Id64;
//! # fn main() -> Result<(), Error> {
//! #[derive(Serialize)]
//! struct Record {
//!     pub id: Id64,
//! }
//!
//! let record = Record { id: Id64::from(5869384017340884593i64) };
//!
//! assert_eq!(
//!     serde_json::to_string(&record)?,
//!     r#"{"id":"UXQ9qpv1ZnE"}"#
//! );
//! # Ok(())
//! # }
//! # }
//! ```
//!
//! ## SQLx
//!
//! You can use the `sqlx` feature flag to derive `Type` and `FromRow` on `Id64`, `Id32` and `Id16`.\
//! This will allow you to add, update or query values from any SQLx supported database without manual type conversion.
//!
//! Values are converted to and from SQL data types based on their `i64`, `i32` and `i16` representations.
//!
//! ## Random Values for Development
//! From the command line you can quickly generate your own random values, along with their corosponding signed and unsigned integers.
//! ```sh
//! cargo run --example random_sample ([64|32|16])
//! ```
//! ***Warning!** The output of this command is not guarentted to be stable, and may change at anytime.*

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "std")]
extern crate std;

use core::{
    cmp::Ordering,
    fmt::{self, Write},
    str::FromStr,
};

mod base64;
mod error;
mod tests;

pub use error::Error;

#[cfg(feature = "rand")]
mod rand;

#[cfg(feature = "serde")]
mod serde;

#[cfg(feature = "sqlx")]
use sqlx::{FromRow, Type};

#[cfg(feature = "derive")]
pub use base64id_derive::Base64Id;

macro_rules! generate_core_trait_impls {
    ($lib_type:ident, $lib_char_array:ty, $u_type:ident, $i_type:ident, $decode_fn:ident, $encode_fn:ident) => {
        impl From<$lib_type> for $i_type {
            fn from(id: $lib_type) -> Self {
                id.0
            }
        }

        impl From<$i_type> for $lib_type {
            fn from(id: $i_type) -> Self {
                Self(id)
            }
        }

        impl From<$lib_type> for $u_type {
            fn from(id: $lib_type) -> Self {
                $u_type::from_be_bytes(id.0.to_be_bytes())
            }
        }

        impl From<$u_type> for $lib_type {
            fn from(id: $u_type) -> Self {
                Self($i_type::from_be_bytes(id.to_be_bytes()))
            }
        }

        impl From<&$u_type> for $lib_type {
            fn from(id: &$u_type) -> Self {
                Self::from(*id)
            }
        }

        impl From<&$i_type> for $lib_type {
            fn from(id: &$i_type) -> Self {
                Self::from(*id)
            }
        }

        impl From<&$lib_type> for $i_type {
            fn from(id: &$lib_type) -> Self {
                Self::from(*id)
            }
        }

        impl From<&$lib_type> for $u_type {
            fn from(id: &$lib_type) -> Self {
                Self::from(*id)
            }
        }

        impl TryFrom<$lib_char_array> for $lib_type {
            type Error = Error;

            fn try_from(input: $lib_char_array) -> Result<Self, Self::Error> {
                Ok(Self(base64::$decode_fn(input)?))
            }
        }

        impl FromStr for $lib_type {
            type Err = Error;

            fn from_str(id: &str) -> Result<Self, Self::Err> {
                let mut array: $lib_char_array = ::core::default::Default::default();
                let mut id_iter = id.chars();

                for c in array.iter_mut() {
                    *c = match id_iter.next() {
                        Some(d) => d,
                        None => return Err(Error::InvalidLength),
                    };
                }

                if id_iter.next().is_some() {
                    return Err(Error::InvalidLength);
                }

                $lib_type::try_from(array)
            }
        }

        impl fmt::Display for $lib_type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                for c in base64::$encode_fn(self.0) {
                    f.write_char(c)?;
                }

                Ok(())
            }
        }

        impl PartialOrd for $lib_type {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $lib_type {
            fn cmp(&self, other: &Self) -> Ordering {
                let this = $u_type::from(*self);
                let other = $u_type::from(*other);

                this.cmp(&other)
            }
        }
    };
}

// ############################### //
// ########----------------####### //
// ######--- 64 Bit Value ---##### //
// ########----------------####### //
// ############################### //

/// 64 bit container with methods for base64url encoding
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "sqlx", derive(Type, FromRow), sqlx(transparent))]
pub struct Id64(i64);

impl Id64 {
    /// Binary equivalent to `u64::MIN`; All bits set to `0`.
    pub const MIN: Id64 = Id64(0);

    /// Binary equivalent to `u64::MAX`; All bits set to `1`.
    pub const MAX: Id64 = Id64(-1);

    /// Create a new `Id64` with an inner value of `0i64`
    ///
    /// This is an alias of `Id64::default()`
    pub fn new() -> Id64 {
        Id64::default()
    }
}

generate_core_trait_impls!(Id64, [char; 11], u64, i64, decode_i64, encode_i64);

// ############################### //
// ########----------------####### //
// ######--- 32 Bit Value ---##### //
// ########----------------####### //
// ############################### //

/// 32 bit container with methods for base64url encoding
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "sqlx", derive(Type, FromRow), sqlx(transparent))]
pub struct Id32(i32);

impl Id32 {
    /// Binary equivalent to `u32::MIN`; All bits set to `0`.
    pub const MIN: Id32 = Id32(0);

    /// Binary equivalent to `u32::MAX`; All bits set to `1`.
    pub const MAX: Id32 = Id32(-1);

    /// Create a new `Id32` with an inner value of `0i32`
    ///
    /// This is an alias of `Id32::default()`
    pub fn new() -> Id32 {
        Id32::default()
    }
}

generate_core_trait_impls!(Id32, [char; 6], u32, i32, decode_i32, encode_i32);

// ############################### //
// ########----------------####### //
// ######--- 16 Bit Value ---##### //
// ########----------------####### //
// ############################### //

/// 16 bit container with methods for base64url encoding
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "sqlx", derive(Type, FromRow), sqlx(transparent))]
pub struct Id16(i16);

impl Id16 {
    /// Binary equivalent to `u16::MIN`; All bits set to `0`.
    pub const MIN: Id16 = Id16(0);

    /// Binary equivalent to `u16::MAX`; All bits set to `1`.
    pub const MAX: Id16 = Id16(-1);

    /// Create a new `Id16` with an inner value of `0i16`
    ///
    /// This is an alias of `Id16::default()`
    pub fn new() -> Id16 {
        Id16::default()
    }
}

generate_core_trait_impls!(Id16, [char; 3], u16, i16, decode_i16, encode_i16);
