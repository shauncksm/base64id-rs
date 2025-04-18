# base64id-rs
[![crates.io](https://img.shields.io/crates/v/base64id.svg)](https://crates.io/crates/base64id)
[![official website](https://img.shields.io/badge/official-website-166534
)](https://base64id.cksm.cc/)
[![docs.rs](https://img.shields.io/docsrs/base64id)](https://docs.rs/base64id/latest/base64id)
[![Rust Validation](https://github.com/shauncksm/base64id-rs/actions/workflows/rust-validate.yml/badge.svg)](https://github.com/shauncksm/base64id-rs/actions/workflows/rust-validate.yml)
<picture><img alt="license" src="https://img.shields.io/crates/l/base64id"></picture>

A pure rust library for representing 64, 32 and 16 bit integers as [base64url](https://datatracker.ietf.org/doc/html/rfc4648#section-5) encoded strings.
```txt
base64url    i64                   u64
-----------  --------------------  --------------------
B21CkMCtWZA    535157120202267024    535157120202267024
fHH_W21Typg   8967229101212682904   8967229101212682904
kjsG-f3NhxI  -7909720649771415790  10537023423938135826
jHamKFSl5oM  -8325284168998721917  10121459904710829699
```

An integer is efficiently stored and manipulated in memory.
However the integer cannot be sent to/from a web client in a url safe manor, without some encoding scheme.
This library allows you to encode the integer to/from an base64url character string.

For a video of the underlying concept in action, see [here](https://www.youtube.com/watch?v=gocwRvLhDf8).

## Benefits
- Integers are made url safe
- Encoded integers use fewer bytes as compared to hex or decimal encoding
- Tests for [RFC 4648](https://www.rfc-editor.org/rfc/rfc4648) compliance where implemented from the start and across the entire libary
- base64id uses `#![no_std]` with no heap allocation required
- base64id uses `#![forbid(unsafe_code)]`

## Website
You can use [base64id.cksm.cc](https://base64id.cksm.cc) to test, debug and generate random base64id strings instantly.

All conversions are run locally in the browser using JavaScript and Web Assembly, with no server backend needed. You can view the GitHub repo for this website [here](https://github.com/shauncksm/base64id-rs-website).

## Motivation
I've used this concept a number of times in personal and work projects as I find it very useful.
The problem is I've had to reimplement the functionality everytime.

The motivation for this library was to design and implement the core concept once, while paying attention to metrics such as performance, correctness and compatability.

## Installation
Add the following to your `Cargo.toml` file
```toml
[dependencies]
base64id = "0.4"
```

### Migrating From v0.3
For users of v0.3.x or less migrating to v0.4.0 or greater, please see the [migration guide](docs/MIGRATION.0.4.md).

## Usage
You start by creating your own tuple struct with a single `i64`, `i32`, `i16` or `i128`. Then apply the `Base64Id` derive macro to your struct.

```rust
use base64id::Base64Id;

#[derive(Base64Id)]
struct MyId(i64);
```

### Encoding
You can convert signed or unsigned integers to a Base64Id struct as follows:
```rust
use base64id::Base64Id;

#[derive(Base64Id)]
struct MyId(i64);

fn main() {
    let int: i64 = 1;
    let id = MyId::from(int);

    println!("{id}"); // AAAAAAAAAAE
}
```

### Decoding
You can use `FromStr` and `From<{integer}>` to convert a `String` to a Base64Id struct and then into an `i64` as follows:
```rust
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
```

## Serde
Support for [Serde](https://serde.rs/) is possible through the use of the `base64id` derive macro helper attribute.

```rust
use base64id::Base64Id;
use serde_json::Result;

#[derive(Base64Id)]
#[base64id(Serialize, Deserialize)]
struct MyId(i32);

fn main() -> Result<()> {
    let id = MyId(897100256);

    println!("{}", serde_json::to_string(&id)?); // "NXip4A"

    Ok(())
}
```

This will apply Base64Id specific implementations of `Serialize` and `Deserialize` to your struct.

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
