//! Print a minimal table with random Id64 and corosponding i64 pairs
//!
//! Run example with `cargo run --example random_sample` for 64 bit values
//!
//! Run example with `cargo run --example random_sample 32` for 32 bit values

use std::env::args;

use rand::random;

use base64id::{Id16, Id32, Id64};

fn main() {
    let default = String::from("64");
    let args: Vec<String> = args().collect();

    let id_type = args.get(1).unwrap_or(&default);

    match id_type.as_str() {
        "16" => {
            println!("base64url   i16      u16");
            println!("---------   ------   -----");

            for _ in 0..10 {
                let i16: i16 = random();
                let id = Id16::from(i16);
                let u16 = u16::from(id);

                println!("{id}  {i16:>13}  {u16:>6}");
            }
        }
        "32" => {
            println!("base64url   i32           u32");
            println!("---------   -----------   ----------");

            for _ in 0..10 {
                let i32: i32 = random();
                let id = Id32::from(i32);
                let u32 = u32::from(id);

                println!("{id}  {i32:>15}  {u32:>11}");
            }
        }
        _ => {
            println!("base64url    i64                   u64");
            println!("-----------  --------------------  --------------------");

            for _ in 0..10 {
                let i64: i64 = random();
                let id = Id64::from(i64);
                let u64 = u64::from(id);

                println!("{id}  {i64:>20}  {u64:>20}");
            }
        }
    }
}
