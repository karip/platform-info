
#![no_std]

// conditional integration tests

mod common;
use platform_info;

#[test]
#[cfg(target_pointer_width = "16")]
fn integration_test_16() {
    assert_eq!(common::setup(3), 4);
    assert_eq!(platform_info::pointer_width(), 16);
}

#[test]
#[cfg(target_pointer_width = "32")]
fn integration_test_32() {
    assert_eq!(common::setup(4), 5);
    assert_eq!(platform_info::pointer_width(), 32);
}

#[test]
#[cfg(target_pointer_width = "64")]
fn integration_test_64() {
    assert_eq!(common::setup(5), 6);
    assert_eq!(platform_info::pointer_width(), 64);
}
