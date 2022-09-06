//! Print a minimal table with random Id64 and corosponding i64 pairs
//!
//! Run example with `cargo run --example random_sample`

use rand::random;

use base64id::Id64;

fn main() {
    println!("Id64        i64");
    println!("----------- --------------------");

    for _ in 0..10 {
        let int: i64 = random();
        let id = Id64::from(int);

        println!("{id} {int}");
    }
}
