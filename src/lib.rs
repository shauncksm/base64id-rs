//! This crate allows for fixed length 64 bit integers to be represented as [base64url](https://datatracker.ietf.org/doc/html/rfc4648#section-5) encoded strings.
//! This is useful for exchanging unique identifiers in a web based contexts; eg. sending an SQL primary key to a client with as few character as possible.
//!
//! This crate is `#![no_std]` by default.
//! You can use the `std` cargo feature flag to enable support for the standard library
//!
//! ## Quick Start
//! Add the following to your `Cargo.toml` file.
//! ```toml
//! [dependencies]
//! base64id = { version = "0.1", features = ["std", "rand"] }
//! ```
//!
//! #### Encoding
//! You can use the `rand` feature flag to generate a random ID like so.
//! ```
//! use rand::random;
//! use base64id::Id64;
//!
//! fn main() {
//! # #[cfg(feature = "rand")]
//! # {
//!     let id: Id64 = random();
//!     println!("{id}"); // 3Zohppb9XMw
//! # }
//! }
//! ```
//!
//! #### Decoding
//! You can decode a string into an `Id64` using it's `TryFrom` impl.
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
//! ## Random Values for Development
//! From the command line you can quickly generate your own random `Id64` values, along with their corosponding `i64` and `u64` integers.
//! ```sh
//! cargo run --example random_sample
//! ```
//! ***Warning!** The output of this command is not guarentted to be stable, and may change at anytime.*

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "std")]
extern crate std;

use core::{cmp::Ordering, fmt, str::FromStr};

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

impl From<Id64> for i64 {
    fn from(id: Id64) -> Self {
        id.0
    }
}

impl From<i64> for Id64 {
    fn from(id: i64) -> Self {
        Self(id)
    }
}

impl From<Id64> for u64 {
    fn from(id: Id64) -> Self {
        u64::from_be_bytes(id.0.to_be_bytes())
    }
}

impl From<u64> for Id64 {
    fn from(id: u64) -> Self {
        Self(i64::from_be_bytes(id.to_be_bytes()))
    }
}

impl From<&u64> for Id64 {
    fn from(id: &u64) -> Self {
        Self::from(*id)
    }
}

impl From<&i64> for Id64 {
    fn from(id: &i64) -> Self {
        Self::from(*id)
    }
}

impl From<&Id64> for i64 {
    fn from(id: &Id64) -> Self {
        Self::from(*id)
    }
}

impl From<&Id64> for u64 {
    fn from(id: &Id64) -> Self {
        Self::from(*id)
    }
}

impl TryFrom<[char; 11]> for Id64 {
    type Error = Error;

    fn try_from(input: [char; 11]) -> Result<Self, Self::Error> {
        Ok(Self(base64::decode_i64(input)?))
    }
}

impl FromStr for Id64 {
    type Err = Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        let mut array = ['A'; 11];
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

        Id64::try_from(array)
    }
}

impl fmt::Display for Id64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = base64::encode_i64(self.0);
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}{}{}",
            c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7], c[8], c[9], c[10]
        )
    }
}

impl PartialOrd for Id64 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Id64 {
    fn cmp(&self, other: &Self) -> Ordering {
        let this = u64::from(*self);
        let other = u64::from(*other);

        this.cmp(&other)
    }
}

// ############################### //
// ########----------------####### //
// ######--- 32 Bit Value ---##### //
// ########----------------####### //
// ############################### //

// To be done

// ############################### //
// ########----------------####### //
// ######--- 16 Bit Value ---##### //
// ########----------------####### //
// ############################### //

// To be done
