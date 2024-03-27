
# Platform Info

[![Cross-platform tests](https://github.com/karip/platform-info/actions/workflows/rust.yml/badge.svg)](https://github.com/karip/platform-info/actions/workflows/rust.yml)

This repository shows how to use GitHub actions to test a Rust library
on different CPU architectures:

 - 64-bit little-endian x86_64
 - 32-bit big-endian PowerPC

It uses [setup-cross-toolchain-action](https://github.com/marketplace/actions/setup-toolchains-for-cross-compilation-and-cross-testing-for-rust) to run the actions for `x86_64-unknown-linux-gnu` and `powerpc-unknown-linux-gnu`.
The [rust.yml](.github/workflows/rust.yml) file shows how the action is configured.

## Running the example

The library can be seen in action by running the example to print out
platform CPU architecture, endianness and pointer width (`usize::BITS`).

    > cargo run --example platform-info
    CPU architecture: aarch64
    Endianness: little-endian
    Pointer width (usize::BITS): 64 bits

## Manual testing

The library includes few tests, which also print out the platform info.
Here's how to run the tests manually on different platforms.

### 64-bit little-endian x86_64

Running the tests for these systems is usually trivial because most computers are
64-bit little-endian. Just run:

    cargo test -- --nocapture

If the system is not 64-bit little-endian, then install and use [cross](https://github.com/cross-rs/cross):

    // install docker or podman, on Ubuntu: apt-get -y install podman
    cargo install cross
    cross test --target x86_64-unknown-linux-gnu -- --nocapture

### 32-bit big-endian PowerPC

Install and use [cross](https://github.com/cross-rs/cross):

    // install docker or podman, on Ubuntu: apt-get -y install podman
    cargo install cross
    cross test --target powerpc-unknown-linux-gnu -- --nocapture

## License

All files and source code are licensed under Creative Commons Zero (CC0-1.0).
