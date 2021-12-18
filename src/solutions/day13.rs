#![allow(clippy::needless_range_loop)]
use crate::helpers::grid::Point;
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

fn parse(input: &str) -> (Grid, Instructions) {
    let mut points: Points = Vec::new();
    let mut instructions: Instructions = Vec::new();

    let mut width: usize = 0;
    let mut height: usize = 0;

    input.lines().for_each(|l| {
        // line is an instruction.
        if l.starts_with('f') {
            let instruction = parse_instruction(l);

            // infer grid size from first instructions.
            // looking at max. point size might fail if last lines or columns are empty.
            if width == 0 || height == 0 {
                match instruction {
                    Instruction::X(x) => width = max(x * 2 + 1, width),
                    Instruction::Y(y) => height = max(y * 2 + 1, height),
                }
            }

            instructions.push(instruction);
        // line is a point.
        } else if !l.is_empty() {
            points.push(parse_point(l));
        }
    });

    (make_grid(&points, width, height), instructions)
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

fn make_grid(points: &[Point], width: usize, height: usize) -> Grid {
    let mut grid: Grid = vec![vec![false; width]; height];

    for Point(x, y) in points {
        grid[*y][*x] = true;
    }

    grid
}

fn fold_y(grid: &[Line], fold_at: usize, width: usize, height: usize) -> Grid {
    let mut points: Points = Vec::new();

    for y in 0..fold_at {
        for x in 0..width {
            if grid[y][x] || grid[height - y - 1][x] {
                points.push(Point(x, y));
            }
        }
    }

    make_grid(&points, width, fold_at)
}

fn fold_x(grid: &[Line], fold_at: usize, width: usize, height: usize) -> Grid {
    let mut points: Points = Vec::new();

    for y in 0..height {
        for x in 0..fold_at {
            if grid[y][x] || grid[y][width - x - 1] {
                points.push(Point(x, y));
            }
        }
    }

    make_grid(&points, fold_at, height)
}

fn fold(grid: &[Line], instruction: &Instruction) -> Grid {
    let height = grid.len();
    let width = grid[0].len();

    match instruction {
        Instruction::X(fold_at) => fold_x(grid, *fold_at, width, height),
        Instruction::Y(fold_at) => fold_y(grid, *fold_at, width, height),
    }
}

fn count_grid(grid: &[Line]) -> u32 {
    grid.iter().flatten().filter(|x| **x).count() as u32
}

pub fn part_one(input: &str) -> u32 {
    let (grid, instructions) = parse(input);
    count_grid(&fold(&grid, &instructions[0]))
}

fn print_grid(grid: &[Line]) {
    for line in grid {
        let chars: String = line.iter().map(|x| if *x { '#' } else { '.' }).collect();
        println!("{}", chars);
    }
}

pub fn part_two(input: &str) -> u32 {
    let (grid, instructions) = parse(input);

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
