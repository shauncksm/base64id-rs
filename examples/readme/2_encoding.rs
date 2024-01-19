use base64id::Base64Id;

#[derive(Base64Id)]
struct MyId(i64);

fn main() {
    let int: i64 = 1;
    let id = MyId::from(int);

    println!("{id}"); // AAAAAAAAAAE
}
