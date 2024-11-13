use core::fmt;

/// Enum for base64url decoding errors
#[derive(Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// Returned when input data contains an invalid number of characters
    ///
    /// ## Expected Lengths
    /// - `i64` or `u64`: 11 characters
    /// - `i32` or `u32`: 6 characters
    /// - `i16` or `u16`: 3 characters
    InvalidLength,
    /// Returned when input data conatins a character that is not within the base64url alphabet
    InvalidCharacter,
    /// Returned when the last character of input data is out of bounds
    ///
    /// For `i64`, `u64`, `i16` and `u16` values, the last character must be one of the following:
    /// ```txt
    /// AEIMQUYcgkosw048
    /// ```
    ///
    /// For `i32` and `u32` values, the last character must be one of the following:
    /// ```txt
    /// AQgw
    /// ```
    OutOfBoundsCharacter,
}

impl core::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidLength => write!(f, "invalid length. number of characters was invalid"),
            Error::InvalidCharacter => write!(
                f,
                "invalid character(s). expected only base64url characters"
            ),
            Error::OutOfBoundsCharacter => {
                write!(f, "invalid character. last character was out of bounds")
            }
        }
    }
}
