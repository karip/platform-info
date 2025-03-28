
# Platform Info

[![Cross-platform tests](https://github.com/karip/platform-info/actions/workflows/rust.yml/badge.svg)](https://github.com/karip/platform-info/actions/workflows/rust.yml)

This repository shows how to use a GitHub action to test a Rust library
for different CPU architectures:

 - 64-bit little-endian x86_64
 - 32-bit big-endian PowerPC

It uses [setup-cross-toolchain-action](https://github.com/taiki-e/setup-cross-toolchain-action)
to run the tests for `x86_64-unknown-linux-gnu` and `powerpc-unknown-linux-gnu`.
The action can run some targets natively (x86_64 has a "native" runner) and
some targets are emulated using qemu (powerpc has a "qemu-user" runner).
The [rust.yml](.github/workflows/rust.yml) file shows how the action is configured.

## Running the example

A simple example can be run locally to print out platform CPU architecture, endianness and
pointer width (`usize::BITS`):

    > cargo run --example platform-info
    CPU architecture: aarch64
    Endianness: little-endian
    Pointer width (usize::BITS): 64 bits

## Manual testing

Here's how to run the tests manually for different platforms.

### 64-bit little-endian x86_64

Running the tests for these systems is usually trivial because most computers are
64-bit little-endian. Just run:

    cargo test -- --nocapture

If the host system is not 64-bit little-endian, then install and use
[cross](https://github.com/cross-rs/cross):

    # install docker or podman, on Ubuntu: apt-get -y install podman
    cargo install cross
    cross test --target x86_64-unknown-linux-gnu -- --nocapture

### 32-bit big-endian PowerPC

Install and use [cross](https://github.com/cross-rs/cross):

    # install docker or podman, on Ubuntu: apt-get -y install podman
    cargo install cross
    cross test --target powerpc-unknown-linux-gnu -- --nocapture

## License

All files and source code are licensed under Creative Commons Zero (CC0-1.0).
