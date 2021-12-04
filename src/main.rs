use std::env;
use std::fs;

mod solutions;
use crate::solutions::*;

macro_rules! solve_day {
    ($day:path, $input:expr) => {{
        use $day::*;
        println!(
            "part 1 = {:?}, part 2 = {:?}",
            part_one($input),
            part_two($input)
        );
    }};
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: u8 = args[1].clone().parse().unwrap();

    let input = read_input_file(day);

    match day {
        1 => solve_day!(day01, &input),
        2 => solve_day!(day02, &input),
        3 => solve_day!(day03, &input),
        4 => solve_day!(day04, &input),
        5 => solve_day!(day05, &input),
        _ => println!("day not solved: {}", day),
    }
}

fn read_input_file(day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd
        .join("src")
        .join("inputs")
        .join(format!("day{:02}.txt", day));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}
