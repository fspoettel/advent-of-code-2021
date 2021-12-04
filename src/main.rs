use aoc2021::read_file;
use std::env;

mod solutions;
use crate::solutions::*;

static ANSI_BOLD: &str = "\x1b[1m";
static ANSI_RESET: &str = "\x1b[0m";

macro_rules! solve_day {
    ($day:path, $input:expr) => {{
        use $day::*;
        println!("----");
        println!("");
        println!("🎄 {}Part 1{} 🎄", ANSI_BOLD, ANSI_RESET);
        println!("");
        println!("{}", part_one($input));
        println!("");
        println!("🎄 {}Part 2{} 🎄", ANSI_BOLD, ANSI_RESET);
        println!("");
        println!("{}", part_two($input));
        println!("");
        println!("----");
    }};
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: u8 = args[1].clone().parse().unwrap();

    let input = read_file("inputs", day);

    match day {
        1 => solve_day!(day01, &input),
        2 => solve_day!(day02, &input),
        3 => solve_day!(day03, &input),
        4 => solve_day!(day04, &input),
        5 => solve_day!(day05, &input),
        _ => println!("day not solved: {}", day),
    }
}
