//! This crate allows for fixed length 64, 32 and 16 bit integers to be represented as [base64url](https://datatracker.ietf.org/doc/html/rfc4648#section-5) encoded strings.
//! This is useful for exchanging unique identifiers in a web based contexts; eg. sending an SQL primary key to a client with as few character as possible.
//!
//! This crate is `#![no_std]`.
//!
//! ## Quick Start
//! Add the following to your `Cargo.toml` file.
//! ```toml
//! [dependencies]
//! base64id = "0.4"
//! ```
//!
//! ### Encoding
//! You can convert signed or unsigned integers to a Base64Id struct as follows:
//! ```rust
//! use base64id::Base64Id;
//!
//! #[derive(Base64Id)]
//! struct MyId(i64);
//!
//! fn main() {
//!     let int: i64 = 1;
//!     let id = MyId::from(int);
//!
//!     println!("{id}"); // AAAAAAAAAAE
//! }
//! ```
//!
//! ### Decoding
//! You can use `FromStr` and `From<{integer}>` to convert a `String` to a Base64Id struct and then into an `i64` as follows:
//! ```rust
//! use base64id::{Base64Id, Error};
//! use std::str::FromStr;
//!
//! #[derive(Base64Id)]
//! struct MyId(i64);
//!
//! fn main() -> Result<(), Error> {
//!     let id_str = MyId::from_str("PDFehCFVGqA")?;
//!     let id_int = i64::from(id_str);
//!
//!     println!("{}", id_int); // 4337351837722417824
//!
//!     Ok(())
//! }
//! ```
//!
//! Refer to the [Error] enum regarding decode errors.
//!
//! ## Serde
//! Support for [Serde](https://serde.rs/) is possible through the use of the `base64id` derive macro helper attribute.
//!
//! ```rust
//! use base64id::Base64Id;
//! use serde_json::Result;
//!
//! #[derive(Base64Id)]
//! #[base64id(Serialize, Deserialize)]
//! struct MyId(i32);
//!
//! fn main() -> Result<()> {
//!     let id = MyId(897100256);
//!
//!     println!("{}", serde_json::to_string(&id)?); // "NXip4A"
//!
//!     Ok(())
//! }
//! ```

pub use base64id_core::Error;

pub use base64id_derive::Base64Id;
