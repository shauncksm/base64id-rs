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
    /// TBA
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
