#![no_std]

use core::{fmt, str::FromStr};

pub(self) mod base64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Id64(u64);

impl From<Id64> for u64 {
    fn from(id: Id64) -> Self {
        id.0
    }
}

impl From<u64> for Id64 {
    fn from(id: u64) -> Self {
        Self(id)
    }
}

impl From<Id64> for i64 {
    fn from(id: Id64) -> Self {
        i64::from_be_bytes(id.0.to_be_bytes())
    }
}

impl From<i64> for Id64 {
    fn from(id: i64) -> Self {
        Self(u64::from_be_bytes(id.to_be_bytes()))
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

impl FromStr for Id64 {
    type Err = &'static str;
    
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        const BAD_LEN: &str = "invalid length. expected 11 characters";

        let mut array = ['A'; 11];
        let mut id_iter = id.chars();

        for c in array.iter_mut() {
            *c = match id_iter.next() {
                Some(d) => d,
                None => return Err(BAD_LEN),
            };
        }

        if id_iter.next().is_some() {
            return Err(BAD_LEN);
        }

        Ok(Id64::try_from(array)?)
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
        assert_eq!(number, u64::from(id));
    }
    
    #[test]
    fn create_id64_from_i64() {
        let number: i64 = -25519;
        let id = Id64::from(number);
        assert_eq!(number, i64::from(id));
    }

    #[test]
    fn create_id64_from_str() {
        use core::str::FromStr;

        let id = Id64::from_str("AAAAAAAAAAA").unwrap();
        assert_eq!(Id64::from(0u64), id);
    }
}