# base64id-rs
A Rust library for representing 64 bit integers as [base64url](https://datatracker.ietf.org/doc/html/rfc4648#section-5) encoded strings.

```txt
base64url    i64                   u64
-----------  --------------------  --------------------
B21CkMCtWZA    535157120202267024    535157120202267024
fHH_W21Typg   8967229101212682904   8967229101212682904
kjsG-f3NhxI  -7909720649771415790  10537023423938135826
jHamKFSl5oM  -8325284168998721917  10121459904710829699
```

## Usage

You can convert an `i64` or `u64` into a `Id64` as follows
```rs
use base64id::Id64;

fn main() {
    let id_i64 = Id64::from(1i64);
    let id_u64 = Id64::from(1u64);

    println!("{id_i64} {id_u64}");
}
```

You can also use `FromStr` to convert strings into an `Id64`
```rs
use base64id::{Error, Id64};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let id_str = Id64::from_str("PDFehCFVGqA")?;

    println!("{id_str}");

    Ok(())
}
```

## License
Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
