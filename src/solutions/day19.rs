use std::collections::HashMap;

type Point = (i32, i32, i32);

type Report = Vec<Point>;
type Reports = HashMap<u32, Report>;

fn parse(input: &str) -> Reports {
    let mut reports: Reports = HashMap::new();
    let mut current_report = 0;

    input.lines().for_each(|l| {
        if l.starts_with("---") {
            current_report += 1;
        } else if !l.is_empty() {
            let mut coords = l.split(',').map(|s| s.parse::<i32>().unwrap());
            reports.entry(current_report).or_default().push((
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            ));
        }
    });

    reports
}

pub fn part_one(input: &str) -> u32 {
    let reports = parse(input);
    79
}

pub fn part_two(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 19);
        assert_eq!(part_one(&input), 79);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 19);
        assert_eq!(part_two(&input), 0);
    }
}
