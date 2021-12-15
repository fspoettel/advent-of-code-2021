use crate::helpers::math::{median, nth_triangular};

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> i32 {
    let mut positions = parse_input(input);
    let median = median(&mut positions);
    positions.iter().map(|x| (x - median).abs()).sum()
}

pub fn part_two(input: &str) -> i32 {
    let mut positions = parse_input(input);
    positions.sort_unstable();

    (0..*positions.last().unwrap())
        .map(|i| {
            positions
                .iter()
                .map(|p| nth_triangular((p - i).abs()))
                .sum()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 7);
        assert_eq!(part_one(&input), 37);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 7);
        assert_eq!(part_two(&input), 168);
    }
}
