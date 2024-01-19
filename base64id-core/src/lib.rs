//! This crate allows for fixed length 64, 32 and 16 bit integers to be represented as [base64url](https://datatracker.ietf.org/doc/html/rfc4648#section-5) encoded strings.
//! This is useful for exchanging unique identifiers in a web based contexts; eg. sending an SQL primary key to a client with as few character as possible.
//!
//! This crate is `#![no_std]` by default.
//! You can use the `std` cargo feature flag to enable support for the standard library
//!
//! ## Quick Start
//! Add the following to your `Cargo.toml` file.
//! ```toml
//! [dependencies]
//! base64id = { version = "0.4", features = ["std"] }
//! ```
//!
//! #### Encoding
//! TBA
//!
//! #### Decoding
//! TBA
//!
//! Refer to the [Error] enum regarding decode errors.
//!
//! ## Serde
//! TBA
//!
//! #### Examples
//!
//! TBA
//!
//! ## Random Values for Development
//! From the command line you can quickly generate your own random values, along with their corosponding signed and unsigned integers.
//! ```sh
//! cargo run --example random_sample ([64|32|16])
//! ```
//! ***Warning!** The output of this command is not guarentted to be stable, and may change at anytime.*

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "std")]
extern crate std;

#[allow(missing_docs)]
pub mod base64;
mod error;

pub use error::Error;
