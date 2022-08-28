use core::fmt;

/// Error type for base64url decoding
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// returned when input data does not contain exactly 11 characters
    InvalidLength,
    /// returned when input data conatins a character that is not within the base64url alphabet
    InvalidCharacter,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        
        match self {
            InvalidLength => write!(f, "invalid length. expected 11 characters"),
            InvalidCharacter => write!(f, "invalid character(s). expected only base64url characters"),
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
}