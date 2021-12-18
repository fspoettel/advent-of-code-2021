use crate::helpers::math::{median, nth_triangular};

fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> u64 {
    let mut positions = parse(input);
    let median = median(&mut positions);
    positions
        .iter()
        .map(|x| (*x as i32 - median as i32).abs() as u64)
        .sum()
}

pub fn part_two(input: &str) -> u64 {
    let mut positions = parse(input);
    positions.sort_unstable();

    (0..*positions.last().unwrap())
        .map(|i| {
            positions
                .iter()
                .map(|p| nth_triangular((*p as i32 - i as i32).abs() as u64))
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
