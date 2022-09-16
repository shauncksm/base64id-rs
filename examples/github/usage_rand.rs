fn main() {
    use base64id::Id64;
    use rand::random;

    let id: Id64 = random();

    println!("{id}"); // 11 random base64url characters
}
