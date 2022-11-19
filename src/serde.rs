use core::{fmt, str::FromStr};

use serde::{
    de::{Deserializer, Unexpected, Visitor},
    Deserialize, Serialize,
};

use crate::{error::Error, Id64};

macro_rules! generate_serde_trait_impls {
    ($lib_type:ident, $visitor_type:ident, $char_count:literal, $last_char_range:literal) => {
        impl Serialize for $lib_type {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.collect_str(self)
            }
        }

        impl<'de> Deserialize<'de> for $lib_type {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                deserializer.deserialize_str($visitor_type)
            }
        }

        struct $visitor_type;

        impl<'de> Visitor<'de> for $visitor_type {
            type Value = $lib_type;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    concat!($char_count, " character base64url encoded string")
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                const EXP1: &str = concat!("exactly ", $char_count, " base64url characters");
                const EXP2: &str = concat!(
                    "the last character must be one of the following: ",
                    $last_char_range
                );

                $lib_type::from_str(v).map_err(|e| match e {
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
    };
}

generate_serde_trait_impls!(Id64, Id64Visitor, 11, "AEIMQUYcgkosw048");
