use core::fmt;

/// Error type for base64url decoding
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// returned when input data does not contain exactly 11 characters
    InvalidLength,
    /// returned when input data conatins a character that is not within the base64url alphabet
    InvalidCharacter,
    /// returned when the last character of a string is out of bounds
    ///
    /// The 11th character of a Id64 string must not have a base64 index number who's first and/or second bit is set to 1
    OutOfBoundsCharacter,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            InvalidLength => write!(f, "invalid length. expected 11 characters"),
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
