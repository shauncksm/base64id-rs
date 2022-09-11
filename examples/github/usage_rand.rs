use base64id::Id64;
use rand::random;

fn main() {
    let id: Id64 = random();

    println!("{id}"); // 11 random base64url characters
}
