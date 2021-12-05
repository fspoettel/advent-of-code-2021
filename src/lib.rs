use std::env;
use std::fs;

pub fn bits_to_u32(bits: &[bool]) -> u32 {
    bits.iter().fold(0, |acc, &b| acc * 2 + (b as u32))
}

pub fn byte_str_to_u32(str: &str) -> u32 {
    u32::from_str_radix(str, 2).unwrap()
}

pub fn str_to_u32(s: &str) -> u32 {
    s.trim().parse().unwrap()
}

pub fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd
        .join("src")
        .join(folder)
        .join(format!("day{:02}.txt", day));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}
