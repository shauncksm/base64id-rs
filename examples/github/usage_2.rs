use base64id::{Error, Id64};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let id_str = Id64::from_str("PDFehCFVGqA")?;

    println!("{}", i64::from(id_str)); // 4337351837722417824

    Ok(())
}
