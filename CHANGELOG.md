# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Added
- `Id32` struct for 32 bit value support
- `Id16` struct for 16 bit value support

### Changes
- Updated explanatory text and code examples in README.md
- Updated rustdoc documentation

### Fixed
- Inaccurate version number in README.md

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