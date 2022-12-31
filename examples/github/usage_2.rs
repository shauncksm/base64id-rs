use base64id::{Error, Id64};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let id_str = Id64::from_str("PDFehCFVGqA")?;
    let id_int = i64::from(id_str);

    println!("{}", id_int); // 4337351837722417824

    Ok(())
}
