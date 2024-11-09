//! This crate contains the core library code for [base64id-rs](https://github.com/shauncksm/base64id-rs).
//! You shouldn't use this crate directly. See [here](https://docs.rs/base64id/latest/base64id/) instead.

#![no_std]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[allow(missing_docs)]
pub mod base64;
mod error;

pub use error::Error;
