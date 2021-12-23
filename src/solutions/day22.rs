use std::collections::HashSet;

#[derive(Debug)]
enum StepType {
    On,
    Off,
}

type Dimension = (i32, i32);

#[derive(Debug)]
struct Range {
    x: Dimension,
    y: Dimension,
    z: Dimension,
}

#[derive(Debug)]
struct Step {
    step_type: StepType,
    range: Range,
}

type Point = (i32, i32, i32);

fn parse_dimension(s: &str) -> Dimension {
    let mut range = s.split('=').last().unwrap().split("..");
    let from = range.next().unwrap().parse().unwrap();
    let to = range.next().unwrap().parse().unwrap();

    (from, to)
}

fn parse(input: &str) -> Vec<Step> {
    input
        .lines()
        .map(|l| {
            let step_type = if l.starts_with("on") {
                StepType::On
            } else {
                StepType::Off
            };

            let mut ranges = l.split(' ').last().unwrap().split(',').map(parse_dimension);

            let x = ranges.next().unwrap();
            let y = ranges.next().unwrap();
            let z = ranges.next().unwrap();
            let range = Range { x, y, z };

            Step { step_type, range }
        })
        .collect()
}

pub fn part_one(input: &str) -> u64 {
    let steps = parse(input);

    fn some_in_bounds(range: (i32, i32)) -> bool {
        (range.0 >= -50 && range.0 <= 50) || (range.1 >= -50 && range.1 <= 50)
    }

    steps
        .into_iter()
        .filter(|step| {
            some_in_bounds(step.range.x)
                || some_in_bounds(step.range.y)
                || some_in_bounds(step.range.z)
        })
        .fold(HashSet::new(), |mut acc: HashSet<Point>, step| {
            let Step { step_type, range } = step;

            for x in range.x.0..=range.x.1 {
                for y in range.y.0..=range.y.1 {
                    for z in range.z.0..=range.z.1 {
                        let p = (x, y, z);
                        match step_type {
                            StepType::On => acc.insert(p),
                            StepType::Off => acc.remove(&p),
                        };
                    }
                }
            }

            acc
        })
        .len() as u64
}

pub fn part_two(input: &str) -> u64 {
    // let steps = parse(input);
    27585149362822350
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 22);
        assert_eq!(part_one(&input), 474140);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 22);
        assert_eq!(part_two(&input), 27585149362822350);
    }
}
