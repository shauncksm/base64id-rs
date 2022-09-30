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

#[cfg(test)]
mod tests {
    use crate::Id64;
    use core::str::FromStr;

    #[test]
    fn id64_with_bad_length() {
        let id = Id64::from_str("A").unwrap_err();
        assert_eq!(id, super::Error::InvalidLength);
    }

    #[test]
    fn id64_with_invalid_character() {
        let id = Id64::from_str("AAAAAAAAAA=").unwrap_err();
        debug_assert_eq!(id, super::Error::InvalidCharacter);
    }

    // Refer to crate::base64::tests::decode_partial_16_out_of_bounds_detection for OutOfBoundsCharacter variant test
}
