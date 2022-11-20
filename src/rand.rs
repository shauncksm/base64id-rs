use crate::{Id16, Id32, Id64};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

impl Distribution<Id64> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Id64 {
        Id64(rng.gen())
    }
}

impl Distribution<Id32> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Id32 {
        Id32(rng.gen())
    }
}

impl Distribution<Id16> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Id16 {
        Id16(rng.gen())
    }
}
