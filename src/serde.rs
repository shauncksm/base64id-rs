use core::{fmt, str::FromStr};

use serde::{
    de::{Deserializer, Unexpected, Visitor},
    Deserialize, Serialize,
};

use crate::{error::Error, Id64};

impl Serialize for Id64 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl<'de> Deserialize<'de> for Id64 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(Id64Visitor)
    }
}

struct Id64Visitor;

impl<'de> Visitor<'de> for Id64Visitor {
    type Value = Id64;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "11 character base64url encoded string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        const EXP1: &str = "exactly 11 base64url characters";
        const EXP2: &str = "the last character must be one of the following: AEIMQUYcgkosw048";

        Id64::from_str(v).map_err(|e| match e {
            Error::InvalidLength => E::invalid_length(v.len(), &EXP1),
            Error::InvalidCharacter => E::invalid_value(
                Unexpected::Other("1 or more non-base64url characters"),
                &EXP1,
            ),
            Error::OutOfBoundsCharacter => E::invalid_value(
                Unexpected::Other("the last character was out of bounds"),
                &EXP2,
            ),
            Error::InfallibleU8FromUsize(_) => E::custom(e),
        })
    }
}
