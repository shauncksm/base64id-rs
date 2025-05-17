# Migration From v0.3 to v0.4

## Preamble

Version v0.4 included custom PartialOrd and Ord traits. If a user wished to supply their own such traits they would have been unable to due to conflicting trait impls.

# Migration

v1.0 removes these custom traits completely.
If your application made use of order traits with your base64id derived struct, you'll need to re-add the appropriate order traits back into your struct.

## Unsigned Structs

If your base64id struct contains an unsigned integer type, then you can restore the original ordering behaviour through rusts built in derive macros.
```rust
#[derive(Base64Id, PartialOrd, Ord)]
struct MyUnsignedId(u64);
```

## Signed Structs

If your base64id struct contains a signed integer type, then you can restore the original ordering behaviour with the following trait impls (modify to use correct integer type as needed):

```rust
use core::cmp::{PartialOrd, Ord, Ordering};

#[derive(Base64Id)]
struct MySignedId(i64);

impl PartialOrd for MySignedId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for #ident {
    fn cmp(&self, other: &Self) -> Ordering {
        let this = u64::from_be_bytes(self.0.to_be_bytes());
        let other = u64::from_be_bytes(other.0.to_be_bytes());

        this.cmp(&other)
    }
}
```