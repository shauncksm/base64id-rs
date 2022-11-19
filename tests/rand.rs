use base64id::{Id32, Id64};

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
