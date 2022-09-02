//! This crate allows for 64 bit integers to be represented as [base64url](https://datatracker.ietf.org/doc/html/rfc4648#section-5) encoded strings.
//! This is useful for exchanging unique identifiers in a web based contexts; eg. sending an SQL primary key to a client with as few character as possible.
//!
//! This crate is `#![no_std]`.
//! *Future plans include the use of cargo feature flags to enable `std` as needed.*
//!
//! ## Example
//! Here are some examples of encoded and raw random `i64` integers.
//! ```txt
//! Id64        i64
//! ----------- --------------------
//! zQpPkyvSY4c -3672035052653223033
//! fRN6Rpu717I 9012681722977572786
//! Hvbo3OHRk8s 2231226700787717067
//! hhKuR_uLu5g -8785768298860266600
//! e502o-aw89M 8907335715586634707
//! CEopSRThix8 597335294439688991
//! RSSE_NwKa1U 4982253309336906581
//! C4xz60HkBPA 832167485416670448
//! H0rwQK6dVoE 2254878724050474625
//! 3g4P8dXEqjU -2446000016267630027
//! ```
//! You can generate your own sample values using `cargo run --example random_sample`.

#![no_std]
#![warn(missing_docs)]

use core::{fmt, str::FromStr};

pub(self) mod base64;
pub(self) mod error;

pub use error::Error;

#[cfg(feature = "rand")]
pub(self) mod rand;

#[cfg(feature = "serde")]
pub(self) mod serde;

#[cfg(feature = "sqlx")]
extern crate std;
#[cfg(feature = "sqlx")]
use sqlx::{FromRow, Type};

/// 64 bit container with methods for base64url encoding
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "sqlx", derive(Type, FromRow), sqlx(transparent))]
pub struct Id64(i64);

impl Id64 {
    /// Binary equivalent to `u64::MIN`; All bits set to `0`.
    pub const MIN: Id64 = Id64(0);

    /// Binary equivalent to `u64::MAX`; All bits set to `1`.
    pub const MAX: Id64 = Id64(-1);
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

impl TryFrom<[char; 11]> for Id64 {
    type Error = Error;

    fn try_from(input: [char; 11]) -> Result<Self, Self::Error> {
        Ok(Self(base64::decode_u64(input)?))
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
        let c = base64::encode_u64(self.0);
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}{}{}",
            c[0], c[1], c[2], c[3], c[4], c[5], c[6], c[7], c[8], c[9], c[10]
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Id64;
    use core::str::FromStr;

    #[test]
    fn create_id64_from_u64() {
        let number: u64 = 25519;
        let id = Id64::from(number);
        assert_eq!(number, u64::from(id));
    }

    #[test]
    fn create_id64_from_i64() {
        let number: i64 = -25519;
        let id = Id64::from(number);
        assert_eq!(number, i64::from(id));
    }

    #[test]
    fn create_id64_from_str() {
        let id = Id64::from_str("AAAAAAAAAAA").unwrap();
        assert_eq!(Id64::from(0u64), id);
    }

    #[test]
    fn id64_min_const() {
        assert_eq!(u64::MIN.to_be_bytes(), u64::from(Id64::MIN).to_be_bytes());
    }

    #[test]
    fn id64_max_const() {
        assert_eq!(u64::MAX.to_be_bytes(), u64::from(Id64::MAX).to_be_bytes());
    }
}
