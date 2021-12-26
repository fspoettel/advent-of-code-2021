#[derive(Clone)]
enum Occupant {
    EastBound,
    SouthBound,
    Empty,
}

type Line = Vec<Occupant>;

fn parse(input: &str) -> Vec<Line> {
    input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(
                    l.chars()
                        .map(|c| match c {
                            '>' => Occupant::EastBound,
                            'v' => Occupant::SouthBound,
                            '.' => Occupant::Empty,
                            c => panic!("unexpected input: {}", c),
                        })
                        .collect(),
                )
            }
        })
        .collect()
}

fn simulate_step(grid: &mut Vec<Line>) -> u32 {
    let mut moved = 0;

    let w = grid[0].len();
    let h = grid.len();

    let reference = grid.clone();
    // eastbound traffic
    for y in 0..h {
        for x in 0..w {
            let x2 = if x == 0 { w - 1 } else { x - 1 };

            if let Occupant::Empty = reference[y][x] {
                if let Occupant::EastBound = reference[y][x2] {
                    grid[y][x2] = Occupant::Empty;
                    grid[y][x] = Occupant::EastBound;
                    moved += 1;
                }
            }
        }
    }

    let reference = grid.clone();
    // southbound traffic
    for y in 0..h {
        for x in 0..w {
            let y2 = if y == 0 { h - 1 } else { y - 1 };

            if let Occupant::Empty = reference[y][x] {
                if let Occupant::SouthBound = reference[y2][x] {
                    grid[y2][x] = Occupant::Empty;
                    grid[y][x] = Occupant::SouthBound;
                    moved += 1;
                }
            }
        }
    }

    moved
}

pub fn part_one(input: &str) -> u32 {
    let mut grid = parse(input);
    let mut step = 0;

    loop {
        step += 1;

        if simulate_step(&mut grid) == 0 {
            break;
        }
    }

    step
}

pub fn part_two(_input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 25);
        assert_eq!(part_one(&input), 58);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 25);
        assert_eq!(part_two(&input), 0);
    }
}
