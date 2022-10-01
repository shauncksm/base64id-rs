# Project Vision
The ultimate end goal for this project is to have a Rust library that can be used to represent 64, 32 and 16 bit integers as base64url encoded strings.

I aim for this project to

- be simple; both in terms of public API and implementation
- be as performant as possible
- have strictly minimal dependancies
- have maximum compatability with other crates

I do **NOT** aim for this project to incorporate

- unsafe rust code

  I believe the goals of this project are achievable in safe Rust.

- support for integers or base64url strings of arbitrary length

  I believe focusing on fixed length values, provides advantages in areas of compatability with other environments and simplicity, both for the developer using this library and in the libraries implementation.

## Future Work
- At this time only 64 bit integers are supported.

- The primary goal for future work will be incorporating 32 and 16 bit value support.

- I am happy to implement, or to receive pull requests that implement common Traits from the Rust core or standard libraries where it makes sense.

  If there's a choice between implementing on `core` or `std`, `core` should always be preferred, for maximum compatability.

- I am open to additional base64 alphabets beyond base64url, though this is not a priority for me, and I don't plan on implementing this at this time.

  If you have a use case for this please open an issue on GitHub to elaborate.