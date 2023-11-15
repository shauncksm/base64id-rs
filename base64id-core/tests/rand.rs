use base64id_core::{Id16, Id32, Id64};

use rand::random;

#[test]
#[cfg(feature = "rand")]
fn rand_id64() {
    let id: Id64 = random();
    println!("{id}");
}

#[test]
#[cfg(feature = "rand")]
fn rand_id32() {
    let id: Id32 = random();
    println!("{id}");
}

#[test]
#[cfg(feature = "rand")]
fn rand_id16() {
    let id: Id16 = random();
    println!("{id}");
}
