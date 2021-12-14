#![allow(clippy::needless_range_loop)]
use aoc::Point;
use std::cmp::max;

type Points = Vec<Point>;

type Line = Vec<bool>;
type Grid = Vec<Line>;

#[derive(Debug)]
enum Instruction {
    X(usize),
    Y(usize),
}
type Instructions = Vec<Instruction>;

fn parse_input(input: &str) -> (Grid, Instructions) {
    let mut points: Points = Vec::new();
    let mut instructions: Instructions = Vec::new();

    let mut x_ceil: usize = 0;
    let mut y_ceil: usize = 0;

    input.lines().for_each(|l| {
        // line is an instruction.
        if l.starts_with('f') {
            let instruction = parse_instruction(l);

            // infer grid size from first instructions.
            // looking at max. point size might fail if last lines or columns are empty.
            if x_ceil == 0 || y_ceil == 0 {
                match instruction {
                    Instruction::X(x) => x_ceil = max(x * 2 + 1, x_ceil),
                    Instruction::Y(y) => y_ceil = max(y * 2 + 1, y_ceil),
                }
            }

            instructions.push(instruction);
        // line is a point.
        } else if !l.is_empty() {
            points.push(parse_point(l));
        }
    });

    (make_grid(&points, x_ceil, y_ceil), instructions)
}

fn parse_point(l: &str) -> Point {
    let mut coords = l.split(',');
    let x: usize = coords.next().unwrap().parse().unwrap();
    let y: usize = coords.next().unwrap().parse().unwrap();
    Point(x, y)
}

fn parse_instruction(l: &str) -> Instruction {
    let mut instr = l.split(' ').last().unwrap().split('=');
    let axis = instr.next().unwrap();
    let amount: usize = instr.next().unwrap().parse().unwrap();

    if axis == "x" {
        Instruction::X(amount)
    } else {
        Instruction::Y(amount)
    }
}

fn make_grid(points: &[Point], x_ceil: usize, y_ceil: usize) -> Grid {
    let mut grid: Grid = vec![vec![false; x_ceil]; y_ceil];

    for Point(x, y) in points {
        grid[*y][*x] = true;
    }

    grid
}

fn fold_y(grid: &[Line], fold_at: usize, x_ceil: usize, y_ceil: usize) -> Grid {
    let mut points: Points = Vec::new();

    for y in 0..fold_at {
        for x in 0..x_ceil {
            if grid[y][x] || grid[y_ceil - y - 1][x] {
                points.push(Point(x, y));
            }
        }
    }

    make_grid(&points, x_ceil, fold_at)
}

fn fold_x(grid: &[Line], fold_at: usize, x_ceil: usize, y_ceil: usize) -> Grid {
    let mut points: Points = Vec::new();

    for y in 0..y_ceil {
        for x in 0..fold_at {
            if grid[y][x] || grid[y][x_ceil - x - 1] {
                points.push(Point(x, y));
            }
        }
    }

    make_grid(&points, fold_at, y_ceil)
}

fn fold(grid: &[Line], instruction: &Instruction) -> Grid {
    let y_ceil = grid.len();
    let x_ceil = grid[0].len();

    match instruction {
        Instruction::X(fold_at) => fold_x(grid, *fold_at, x_ceil, y_ceil),
        Instruction::Y(fold_at) => fold_y(grid, *fold_at, x_ceil, y_ceil),
    }
}

fn count_grid(grid: &[Line]) -> u32 {
    grid.iter().flatten().filter(|x| **x).count() as u32
}

pub fn part_one(input: &str) -> u32 {
    let (grid, instructions) = parse_input(input);
    count_grid(&fold(&grid, &instructions[0]))
}

fn print_grid(grid: &[Line]) {
    for line in grid {
        let chars: String = line.iter().map(|x| if *x { '#' } else { '.' }).collect();
        println!("{}", chars);
    }
}

pub fn part_two(input: &str) -> u32 {
    let (grid, instructions) = parse_input(input);

    let code = instructions.iter().fold(grid, |acc, curr| fold(&acc, curr));

    print_grid(&code);
    count_grid(&code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 13);
        assert_eq!(part_one(&input), 17);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 13);
        assert_eq!(part_two(&input), 16);
    }
}
