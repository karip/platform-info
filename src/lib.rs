//! A silly little library to return CPU architecture, endianness and pointer width (`usize::BITS`).

#![no_std]

/// Returns the cfg target_arch value.
pub fn arch() -> &'static str {
    if cfg!(target_arch = "aarch64") {
        "aarch64"
    } else if cfg!(target_arch = "arm") {
        "arm"
    } else if cfg!(target_arch = "mips") {
        "mips"
    } else if cfg!(target_arch = "powerpc") {
        "powerpc"
    } else if cfg!(target_arch = "powerpc64") {
        "powerpc64"
    } else if cfg!(target_arch = "x86") {
        "x86"
    } else if cfg!(target_arch = "x86_64") {
        "x86_64"
    } else if cfg!(target_arch = "avr") {
        "avr"
    } else {
        "unknown"
    }
}

/// Returns the cfg target_endian value.
pub fn endianness() -> &'static str {
    if cfg!(target_endian = "big") {
        "big-endian"
    } else {
        "little-endian"
    }

}

/// Returns the `usize::BITS` value.
pub fn pointer_width() -> u32 {
    usize::BITS
}

pub fn vendor() -> &'static str {
    if cfg!(target_vendor = "apple") {
        "apple"
    } else if cfg!(target_vendor = "espressif") {
        "espressif"
    } else if cfg!(target_vendor = "fortanix") {
        "fortanix"
    } else if cfg!(target_vendor = "ibm") {
        "ibm"
    } else if cfg!(target_vendor = "kmc") {
        "kmc"
    } else if cfg!(target_vendor = "nintendo") {
        "nintendo"
    } else if cfg!(target_vendor = "nvidia") {
        "nvidia"
    } else if cfg!(target_vendor = "pc") {
        "pc"
    } else if cfg!(target_vendor = "risc0") {
        "risc0"
    } else if cfg!(target_vendor = "sony") {
        "sony"
    } else if cfg!(target_vendor = "sun") {
        "sun"
    } else if cfg!(target_vendor = "unikraft") {
        "unikraft"
    } else if cfg!(target_vendor = "uwp") {
        "uwp"
    } else if cfg!(target_vendor = "win7") {
        "win7"
    } else if cfg!(target_vendor = "wrs") {
        "wrs"
    } else {
        "unknown"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pointer_width() {
        assert_eq!(usize::BITS, u32::from(pointer_width()));
        let value = usize::MAX as u128;
        let bw: u128 = (1 << pointer_width()) - 1;
        assert_eq!(bw, value);
        let (overflown_value, _) = usize::MAX.overflowing_add(1);
        assert_eq!(overflown_value, 0);
    }

    #[test]
    fn test_endianness() {
        let value = 513u16;
        if endianness() == "little-endian" {
            assert_eq!(value.to_ne_bytes(), [ 1, 2 ]);
        } else {
            assert_eq!(value.to_ne_bytes(), [ 2, 1 ]);
        }
    }

    // conditional tests based on pointer width

    #[test]
    #[cfg(target_pointer_width = "16")]
    fn test_for_pointer_width_16() {
        assert_eq!(pointer_width(), 16);
    }

    #[test]
    #[cfg(target_pointer_width = "32")]
    fn test_for_pointer_width_32() {
        assert_eq!(pointer_width(), 32);
    }

    #[test]
    #[cfg(target_pointer_width = "64")]
    fn test_for_pointer_width_64() {
        assert_eq!(pointer_width(), 64);
    }
}
