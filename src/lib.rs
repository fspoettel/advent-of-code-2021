use std::env;
use std::fmt::Debug;
use std::fs;

use num_traits::NumCast;
use num_traits::PrimInt;

pub fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd
        .join("src")
        .join(folder)
        .join(format!("day{:02}.txt", day));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

pub fn median<T: PrimInt + Debug>(vec: &mut Vec<T>) -> T {
    let len = vec.len();
    let mid = len / 2;

    vec.sort();

    if len % 2 == 0 {
        (vec[mid - 1] + vec[mid]) / NumCast::from(2).unwrap()
    } else {
        vec[mid]
    }
}

#[test]
fn test_median() {
    assert_eq!(median(&mut [1, 4, 7].to_vec()), 4);
    assert_eq!(median(&mut [3, 10, 36, 255, 79, 24, 5, 8].to_vec()), 17);
}
