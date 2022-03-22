#![no_std]

#[derive(Debug, Clone, Copy)]
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