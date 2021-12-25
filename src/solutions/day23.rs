/// Solved today by hand on a whiteboard with my family.
/// This file contains a helper to sum moves and the correct solutions for my input.
/// Here are some pictures of the whiteboard: [#1](https://git.io/JyIvl) [#2](https://git.io/JyIvg)

fn sum(l: &str, factor: u64) -> u64 {
    l.split(' ')
        .map(|x| x.parse::<u64>().unwrap() * factor)
        .sum()
}

fn add_lines(pink: &str, blue: &str, green: &str, purple: &str) -> u64 {
    sum(pink, 1) + sum(blue, 10) + sum(green, 100) + sum(purple, 1000)
}

pub fn part_one(_: &str) -> u64 {
    add_lines("3 3 5 8", "2 3 5", "2 3 4", "9 9")
}

pub fn part_two(_: &str) -> u64 {
    add_lines(
        "8 8 4 5 5 5 9 9",
        "7 4 5 8 7 7",
        "7 2 5 6 5 6",
        "11 11 11 11",
    )
}
