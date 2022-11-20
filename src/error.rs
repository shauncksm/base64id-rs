use core::{fmt, num::TryFromIntError};

/// Error type for base64url decoding
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// returned when input data contains an invalid number of characters
    InvalidLength,
    /// returned when input data conatins a character that is not within the base64url alphabet
    InvalidCharacter,
    /// returned when the last character of a string is out of bounds
    ///
    /// The 11th character of an `Id64` string must not have a base64 index number who's first and/or second bit is set to 1.
    ///
    /// The following base64url characters satisfy this criteria:
    /// ```txt
    /// AEIMQUYcgkosw048
    /// ```
    OutOfBoundsCharacter,
    /// returned when a `TryFromIntError` is encountered during base64url decoding
    ///
    /// This error should never occur, as a `usize` between 0 to 63 can always convert to a `u8`.
    InfallibleU8FromUsize(TryFromIntError),
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
            InfallibleU8FromUsize(_) => write!(
                f,
                "infallible. failed to convert usize between 0 - 63 to u8"
            ),
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
}
