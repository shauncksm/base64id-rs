use base64id::Id64;

fn main() {
    let id_i64 = Id64::from(1i64);
    let id_u64 = Id64::from(1u64);

    println!("{id_i64} {id_u64}"); // AAAAAAAAAAE AAAAAAAAAAE
}
