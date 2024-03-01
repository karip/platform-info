//! A silly little library to return CPU architecture, endianness and pointer width (`usize::BITS`).

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_values() {
        println!("-*-*-*-");
        println!("CPU architecture: {}", arch());
        println!("Endianness: {}", endianness());
        println!("Pointer width (usize::BITS): {} bits", pointer_width());
    }

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
}
