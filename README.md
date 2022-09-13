# base64id-rs
A pure rust library for representing 64 bit integers as [base64url](https://datatracker.ietf.org/doc/html/rfc4648#section-5) encoded strings.
This can be useful in web based applications ([like this](https://www.youtube.com/watch?v=gocwRvLhDf8)) for sending database record ID's to clients while reducing string lenth.

```txt
base64url    i64                   u64
-----------  --------------------  --------------------
B21CkMCtWZA    535157120202267024    535157120202267024
fHH_W21Typg   8967229101212682904   8967229101212682904
kjsG-f3NhxI  -7909720649771415790  10537023423938135826
jHamKFSl5oM  -8325284168998721917  10121459904710829699
```

## Motivation
I've used this concept a number of times in personal and work projects as I find it very useful.
The problem is I've had to reimplement the functionality everytime.

The motivation for this library was to design and implement the core concept once, while paying attention to metrics such as performance, correctness, and compatability. To that end:
- the library is `no_std` by default; with no heap allocation required, all execution is done on the stack
- all base64 bit manipulation code is unit tested with fixed random values

## Installation
Add the following to your `Cargo.toml` file
```toml
[dependencies]
base64id = { version = "1.0", features = ["rand"] }
```

## Usage

All work is done using the `Id64` struct.

### Encoding
You can convert an `i64` or `u64` into a `Id64` as follows
```rs
use base64id::Id64;

fn main() {
    let id_i64 = Id64::from(1i64);
    let id_u64 = Id64::from(1u64);

    println!("{id_i64} {id_u64}"); // AAAAAAAAAAE AAAAAAAAAAE
}
```

### Decoding
You can also use `FromStr` to convert strings into an `Id64`
```rs
use base64id::{Error, Id64};
use std::str::FromStr;

fn main() -> Result<(), Error> {
    let id_str = Id64::from_str("PDFehCFVGqA")?;

    println!("{}", i64::from(id_str)); // 4337351837722417824

    Ok(())
}
```

## Third Party Crates

Support for Serde, Rand and SQLx may be enabled though the use of optional cargo feature flags.

### Rand
You can use the `rand` feature flag for working with the `rand` crate
```rs
use base64id::Id64;
use rand::random;

fn main() {
    let id: Id64 = random();

    println!("{id}"); // 11 random base64url characters
}
```

### Serde
You can use the `serde` feature flag to drive `Serialize` and `Deserialize` on `Id64`
```rs
use base64id::Id64;
use serde::{Deserialize, Serialize};

fn main() {
    #[derive(Serialize, Deserialize)]
    struct Record {
        id: Id64,
    }

    let record = Record {
        id: Id64::from(0u64),
    };

    println!("{}", serde_json::to_string(&record).unwrap()); // {"id":"AAAAAAAAAAA"}
}
```

### SQLx
You can use the `sqlx` feature flag for using an `Id64` with SQLx SQL commands
```rs
let id = Id64::from_str("IkoY0lQYRrI").unwrap();
let mut conn = SqliteConnection::connect("sqlite::memory:").await?;

sqlx::query("CREATE TABLE sqlx (id INT PRIMARY KEY)")
    .execute(&mut conn)
    .await?;

sqlx::query("INSERT INTO sqlx VALUES (?)")
    .bind(id)
    .execute(&mut conn)
    .await?;

let output = sqlx::query_as::<Sqlite, Id64>("SELECT id FROM sqlx LIMIT 1")
    .fetch_one(&mut conn)
    .await?;

println!("{output}"); // IkoY0lQYRrI
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
