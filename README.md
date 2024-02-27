
# Platform Info

[![Cross-platform tests](https://github.com/karip/platform-info/actions/workflows/rust.yml/badge.svg)](https://github.com/karip/platform-info/actions/workflows/rust.yml)

A simple Rust application to print out platform CPU architecture,
endianness and pointer width.

This repository also shows how to use GitHub actions to test
the code on 64-bit x86_64 little-endian and 32-bit PowerPC big-endian CPUs.
It uses [setup-cross-toolchain-action](https://github.com/marketplace/actions/setup-toolchains-for-cross-compilation-and-cross-testing-for-rust) to run the actions for `x86_64-unknown-linux-gnu` and `powerpc-unknown-linux-gnu`.
The [rust.yml](.github/workflows/rust.yml) file shows how the action is configured.

## Running

    > cargo run
    CPU architecture: aarch64
    Endianness: little-endian
    Pointer width: 64-bit

## Testing

The application includes tests, which ensure that the values are correct.
The tests also print out the platform info:

    cargo test -- --nocapture

## License

All files and source code are licensed under Creative Commons Zero (CC0-1.0).
