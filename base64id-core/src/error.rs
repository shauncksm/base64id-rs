use core::fmt;

/// Error enum for base64url decoding
///
/// This enum will only implement the `std::error::Error` trait when the `std` feature flag is enabled. This is not enabled by default.
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// returned when input data contains an invalid number of characters
    ///
    /// ## Expected Lengths
    /// - `Id64` 11 characters
    /// - `Id32` 6 characters
    /// - `Id16` 3 characters
    InvalidLength,
    /// returned when input data conatins a character that is not within the base64url alphabet
    InvalidCharacter,
    /// returned when the last character of input data is out of bounds
    ///
    /// For `Id64` and `Id16` values, the last character must be one of the following:
    /// ```txt
    /// AEIMQUYcgkosw048
    /// ```
    ///
    /// For `Id32` values, the last character must be one of the following:
    /// ```txt
    /// AQgw
    /// ```
    ///
    /// ## Example
    /// ```rust
    /// # use std::str::FromStr;
    /// # use base64id::{Error, Id64, Id32, Id16};
    /// assert_eq!(Id64::from_str("AAAAAAAAAAB"), Err(Error::OutOfBoundsCharacter));
    /// assert_eq!(Id32::from_str("AAAAAB"), Err(Error::OutOfBoundsCharacter));
    /// assert_eq!(Id16::from_str("AAB"), Err(Error::OutOfBoundsCharacter));
    /// ```
    OutOfBoundsCharacter,
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            InvalidLength => write!(f, "invalid length. number of characters was invalid"),
            InvalidCharacter => write!(
                f,
                "invalid character(s). expected only base64url characters"
            ),
            OutOfBoundsCharacter => {
                write!(f, "invalid character. last character was out of bounds")
            }
        }
    }
}

mod tests {
    macro_rules! generate_error_test_suite {
        ($lib_type:ident, $lib_type_name:ident, $bad_length:expr, $bad_char:expr) => {
            #[cfg(test)]
            mod $lib_type_name {
                use crate::{
                    $lib_type,
                    Error::{InvalidCharacter, InvalidLength},
                };
                use core::str::FromStr;

                #[test]
                fn bad_length() {
                    let id = $lib_type::from_str($bad_length).unwrap_err();
                    assert_eq!(id, InvalidLength);
                }

                #[test]
                fn invalid_character() {
                    let id = $lib_type::from_str($bad_char).unwrap_err();
                    debug_assert_eq!(id, InvalidCharacter);
                }
            }
        };
    }

    generate_error_test_suite!(Id64, id64, "A", "AAAAAAAAAA=");

    generate_error_test_suite!(Id32, id32, "A", "AAAAA=");

    generate_error_test_suite!(Id16, id16, "A", "AA=");
}
