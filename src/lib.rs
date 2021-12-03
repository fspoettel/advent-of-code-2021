pub fn bits_to_u32(bits: &[bool]) -> u32 {
    bits.iter().fold(0, |acc, &b| acc * 2 + (b as u32))
}

pub fn str_to_u32(str: &str) -> u32 {
    u32::from_str_radix(str, 2).unwrap()
}
