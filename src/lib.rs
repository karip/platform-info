//! Simple library to return CPU architecture, endianness and pointer width (`usize::BITS`).

#[cfg(target_arch = "aarch64")]
pub fn arch() -> &'static str {
    "aarch64"
}

#[cfg(target_arch = "arm")]
pub fn arch() -> &'static str {
    "arm"
}

#[cfg(target_arch = "mips")]
pub fn arch() -> &'static str {
    "mips"
}

#[cfg(target_arch = "powerpc")]
pub fn arch() -> &'static str {
    "powerpc"
}

#[cfg(target_arch = "powerpc64")]
pub fn arch() -> &'static str {
    "powerpc64"
}

#[cfg(target_arch = "x86")]
pub fn arch() -> &'static str {
    "x86"
}

#[cfg(target_arch = "x86_64")]
pub fn arch() -> &'static str {
    "x86_64"
}

#[cfg(target_arch = "avr")]
pub fn arch() -> &'static str {
    "avr"
}

#[cfg(target_endian = "big")]
pub fn endianness() -> &'static str {
    "big-endian"
}

#[cfg(target_endian = "little")]
pub fn endianness() -> &'static str {
    "little-endian"
}

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
