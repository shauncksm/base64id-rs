//! Print a minimal table with random Id64 and corosponding i64 pairs
//!
//! Run example with `cargo run --example random_sample`

use rand::random;

use base64id::Id64;

fn main() {
    println!("base64url    i64                   u64");
    println!("-----------  --------------------  --------------------");

    for _ in 0..10 {
        let i64: i64 = random();
        let id = Id64::from(i64);
        let u64 = u64::from(id);

        println!("{id}  {i64:>20}  {u64:>20}");
    }
}
