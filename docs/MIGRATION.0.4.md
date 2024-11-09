# Migration From v0.3 to v0.4

## Preamble

In all versions including and prior to v0.3, base64id exposed a set of concrete types (`Id64`, `Id32` and `Id16`) which users would include in their project.
These concrete types were what allowed library users to use 64, 32 and 16 bit integers in decimal or base64url form.

The issue with these types was that they included a set of third party trait implementations on unstable crates.
There was always a risk that third party updates could [break](https://github.com/shauncksm/base64id-rs/issues/9) functionality.
See issue [#8](https://github.com/shauncksm/base64id-rs/issues/8) for further explaination.

# Migration

v0.4 removes these concrete types and instead exposes the `Base64Id` derive macro.
This will apply base64id specific trait implementations to a tuple struct that you define.
You can then apply rand, serde, sqlx trait implementations and any other behaviour to your struct as needed.

## Before

Before you'd simply use one of the provided concrete types, for instance:

```rust
use base64id::Id64;
```

## After

To recreate the `Id64` type with all previously supported traits, replace the above `use` statement with the following struct definition:

```rust
use base64id::Base64Id;
use sqlx::{Type, FromRow};
use rand::{Rng, distributions::{Standard, Distribution}};

#[derive(Base64Id, Debug, FromRow, Type)]
#[base64id(Serialize, Deserialize)]
#[sqlx(transparent)]
struct Id64(i64);

impl Distribution<Id64> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Id64 {
        Id64(rng.gen())
    }
}
```