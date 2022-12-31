use base64id::Id64;

fn main() {
    let int: i64 = 1;
    let id = Id64::from(int);

    println!("{id}"); // AAAAAAAAAAE
}
