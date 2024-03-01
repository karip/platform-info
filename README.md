
# Platform Info

[![Cross-platform tests](https://github.com/karip/platform-info/actions/workflows/rust.yml/badge.svg)](https://github.com/karip/platform-info/actions/workflows/rust.yml)

This repository shows how to use GitHub actions to test a Rust library
on different CPU architectures:

 - 64-bit little-endian x86_64
 - 32-bit big-endian PowerPC

It uses [setup-cross-toolchain-action](https://github.com/marketplace/actions/setup-toolchains-for-cross-compilation-and-cross-testing-for-rust) to run the actions for `x86_64-unknown-linux-gnu` and `powerpc-unknown-linux-gnu`.
The [rust.yml](.github/workflows/rust.yml) file shows how the action is configured.

## Running the example

The library can be seen in action by running a simple Rust example to print out
platform CPU architecture, endianness and pointer width (`usize::BITS`).

    > cargo run --example platform-info
    CPU architecture: aarch64
    Endianness: little-endian
    Pointer width (usize::BITS): 64 bits

## Testing

The application includes tests, which check that the library returns correct values.
The tests also print out the platform info:

    cargo test -- --nocapture

## License

All files and source code are licensed under Creative Commons Zero (CC0-1.0).
