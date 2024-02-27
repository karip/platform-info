
fn main() {
    println!("CPU architecture: {}", target_arch());
    println!("Endianness: {}", endianness());
    println!("Pointer width: {}-bit", pointer_width());
}

#[cfg(target_arch = "aarch64")]
fn target_arch() -> &'static str {
    "aarch64"
}

#[cfg(target_arch = "arm")]
fn target_arch() -> &'static str {
    "arm"
}

#[cfg(target_arch = "mips")]
fn target_arch() -> &'static str {
    "mips"
}

#[cfg(target_arch = "powerpc")]
fn target_arch() -> &'static str {
    "powerpc"
}

#[cfg(target_arch = "powerpc64")]
fn target_arch() -> &'static str {
    "powerpc64"
}

#[cfg(target_arch = "x86")]
fn target_arch() -> &'static str {
    "x86"
}

#[cfg(target_arch = "x86_64")]
fn target_arch() -> &'static str {
    "x86_64"
}

#[cfg(target_arch = "avr")]
fn target_arch() -> &'static str {
    "avr"
}

#[cfg(target_endian = "big")]
fn endianness() -> &'static str {
    "big-endian"
}

#[cfg(target_endian = "little")]
fn endianness() -> &'static str {
    "little-endian"
}

#[cfg(target_pointer_width = "16")]
fn pointer_width() -> u8 {
    16
}

#[cfg(target_pointer_width = "32")]
fn pointer_width() -> u8 {
    32
}

#[cfg(target_pointer_width = "64")]
fn pointer_width() -> u8 {
    64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_values() {
        println!("CPU Architecture: {}", target_arch());
        println!("Endianness: {}", endianness());
        println!("Pointer width: {}-bit", pointer_width());
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
