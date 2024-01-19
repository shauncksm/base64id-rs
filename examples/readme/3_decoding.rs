use base64id::{Base64Id, Error};
use std::str::FromStr;

#[derive(Base64Id)]
struct MyId(i64);

fn main() -> Result<(), Error> {
    let id_str = MyId::from_str("PDFehCFVGqA")?;
    let id_int = i64::from(id_str);

    println!("{}", id_int); // 4337351837722417824

    Ok(())
}
