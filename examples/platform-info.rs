//! Example program to print out platform info.

fn main() {
    println!("CPU architecture: {}", platform_info::arch());
    println!("Family: {}", platform_info::family());
    println!("Vendor: {}", platform_info::vendor());
    println!("Endianness: {}", platform_info::endianness());
    println!("Pointer width (usize::BITS): {} bits", platform_info::pointer_width());
}
