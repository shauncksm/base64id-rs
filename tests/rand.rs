use base64id::Id64;

use rand::random;

#[test]
#[cfg(all(feature = "rand"))]
fn rand_id64() {
    let id: Id64 = random();
    println!("{id}");
}
