#![no_std]

use core::fmt;

pub(self) mod base64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id64(u64);

impl Id64 {
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

impl From<u64> for Id64 {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl TryFrom<[char; 11]> for Id64 {
    type Error = &'static str;

    fn try_from(input: [char; 11]) -> Result<Self, Self::Error> {
        Ok(
            Self(
                base64::decode_u64(input)?
            )
        )
    }
}

impl fmt::Display for Id64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = base64::encode_u64(self.0);
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}{}{}",
            c[0],
            c[1],
            c[2],
            c[3],
            c[4],
            c[5],
            c[6],
            c[7],
            c[8],
            c[9],
            c[10]
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::Id64;
    #[test]
    fn create_id64_from_u64() {
        let number: u64 = 25519;
        let id = Id64::from(number);
        assert_eq!(number, id.into_inner());
    }
}