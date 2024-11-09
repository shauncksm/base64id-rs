# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.4.0]
### Added
- New `Base64Id` derive macro in new `base64id_derive` crate
- Support for unsigned integer types (u64, u32, u16)

### Changed
- (Breaking) Replaced serde feature flag with Base64Id derive macro helper attribute
- Moved majority of crate code to new `base64id_core` crate re-exported via `pub use`
- Replaced std::error::Error with core::error::Error on crate Error enum

### Removed
- (Breaking) Removed `Id64`, `Id32` and `Id16` structs (See Base64Id derive macro)
- (Breaking) Removed direct SQLx support and all related documentation
- (Breaking) Removed direct rand support and most related documentation
- (Breaking) Removed `std` cargo feature flag
- Removed Cargo.lock from version control

## [v0.3.1]
### Security
- Updated all cargo dependencies, to update spin 0.9.4 to 0.9.8 https://github.com/shauncksm/base64id-rs/security/dependabot/3

## [v0.3.0]
### Added
- `Id32` struct for 32 bit value support
- `Id16` struct for 16 bit value support

### Removed
- (Breaking) Removed InfallibleU8FromUsize Error variant

### Changes
- Updated explanatory text and code examples in README.md
- Updated rustdoc documentation

### Fixed
- Inaccurate version number in README.md

### Security
- Updated bumpalo dependency to 3.12.0 after [use after free](https://github.com/shauncksm/base64id-rs/security/dependabot/1) bug was detected

## [v0.2.0]
### Added
- Added reference based impls of the `From` trait for `&Id64`, `&u64` and `&i64`.

### Fixed
- Code examples not linted correctly in README.md

## [v0.1.1]
### Changed
- Patch version bumps for sqlx and serde
- Activate all feature flags within docs.rs build

### Fixed
- Unable to compile due to conflicting sqlx feature flags (#1)

## [v0.1.0] - 2022-10-04
### Added
- Support for fixed length 64 bit integers
- Support for base64url alphabet encoding & decoding
- Optional rand, serde and sqlx support