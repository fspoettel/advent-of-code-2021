use std::env;
use std::fs;

mod solutions;
use crate::solutions::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let day: u8 = args[1].clone().parse().unwrap();

    let input = read_input_file(day);

    match day {
        1 => {
            print_solution(day01::part_one(&input), day01::part_two(&input));
        }
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
    return f.expect("could not open input file");
}

fn print_solution(solution_one: u32, solution_two: u32) {
    println!("part 1 = {:?}, part 2 = {:?}", solution_one, solution_two)
}
