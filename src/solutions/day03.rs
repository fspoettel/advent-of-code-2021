use itertools::Itertools;
use std::collections::HashMap;

pub fn arr_to_int(bits: &[bool]) -> u32 {
    bits.iter().fold(0, |acc, &b| acc * 2 + (b as u32))
}

pub fn str_to_int(str: &str) -> u32 {
    u32::from_str_radix(str, 2).unwrap()
}

pub fn part_one(input: &str) -> u32 {
    // counter that maps character indices to signed integers.
    // we can later calculate the gamma value for a given index by checking the entry's sign.
    let mut counter: HashMap<usize, i32> = HashMap::new();

    // for every character in a line:
    // increment (1) or decrement (0) the counter entry for this index.
    input.lines().for_each(|l| {
        l.chars().enumerate().for_each(|(i, c)| {
            let val = counter.entry(i).or_default();
            *val += if c == '1' { 1 } else { -1 };
        })
    });

    // collect counter into a sorted byte array.
    let gamma = counter
        .keys()
        .sorted_unstable()
        .map(|i| *counter.get(i).unwrap() >= 0)
        .collect_vec();

    // derive epsilon by flipping each bit of gamma.
    let epsilon = gamma.iter().map(|b| !(*b)).collect_vec();

    arr_to_int(&gamma) * arr_to_int(&epsilon)
}

pub fn part_two(input: &str) -> u32 {
    let lines = input.lines().collect_vec();

    let oxy_rating =
        find_line_by_bit_criteria(|a, b| if a.len() >= b.len() { a } else { b }, &lines);

    let co2_rating =
        find_line_by_bit_criteria(|a, b| if a.len() >= b.len() { b } else { a }, &lines);

    str_to_int(oxy_rating) * str_to_int(co2_rating)
}

fn find_line_by_bit_criteria<'a>(
    bit_criteria: impl Fn(Vec<&'a str>, Vec<&'a str>) -> Vec<&'a str>,
    candidates: &[&'a str],
) -> &'a str {
    let mut i = 0;
    let mut survivors = candidates.to_vec();

    while survivors.len() > 1 {
        // partition lines by the dominant bit (1 or 0) in column `i`.
        let (one_dominant, zero_dominant): (Vec<&str>, Vec<&str>) = survivors
            .iter()
            .partition(|s| s.chars().nth(i).unwrap() == '1');

        // determine which group should be continued with and assign it as survivors, discarding the rest.
        survivors = bit_criteria(one_dominant, zero_dominant);
        i += 1;
    }

    survivors.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 3);
        assert_eq!(part_one(&input), 198);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 3);
        assert_eq!(part_two(&input), 230);
    }
}
