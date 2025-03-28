
#![no_std]

// normal integration tests

mod common;
use platform_info;

#[test]
fn integration_test_endianness() {
    assert_eq!(common::setup(1), 2);
    let value = 513u16;
    if platform_info::endianness() == "little-endian" {
        assert_eq!(value.to_ne_bytes(), [ 1, 2 ]);
    } else {
        assert_eq!(value.to_ne_bytes(), [ 2, 1 ]);
    }
}
