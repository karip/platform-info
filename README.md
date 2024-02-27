
# Platform Info

A simple Rust application to print out platform CPU architecture,
endianness and pointer width.

## Running

    > cargo run
    CPU Architecture: aarch64
    Endianness: little-endian
    Pointer width: 64-bit

## Testing

The application includes tests, which ensure that the values are correct.
The tests also print out the platform info:

    cargo test -- --nocapture

## License

All files and source code are licensed under Creative Commons Zero (CC0-1.0).
