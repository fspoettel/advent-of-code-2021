use crate::helpers::grid::Point;
use std::collections::HashSet;

static OCTOPUS_ROWS: usize = 10;
static OCTOPUS_COLS: usize = 10;

type Grid = [Line; 10];
type Line = [u32; 10];

fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<Line>>()
        .try_into()
        .unwrap()
}

fn process_step(grid: &mut Grid, all_points: &[Point]) -> u32 {
    let mut flashed: HashSet<Point> = HashSet::new();
    // start the flash cascade by incrementing all points in the grid.
    // `tick` calls itself recursively until either all octopus have flashed or there
    // is a `tick` where no octopus reaches required energy levels.
    tick(grid, &mut flashed, all_points);
    // reset all flashed octopus to `0`.
    reset_energy_levels(grid, all_points);
    // once there is no more processing to do for a step, we return the count of flashes observed.
    flashed.len() as u32
}

fn tick(grid: &mut Grid, flashed: &mut HashSet<Point>, points: &[Point]) {
    points.iter().for_each(|Point(x, y)| {
        grid[*y][*x] += 1;
        if grid[*y][*x] > 9 && !flashed.contains(&Point(*x, *y)) {
            let p = Point(*x, *y);
            flashed.insert(p);
            tick(
                grid,
                flashed,
                // when an octopus flashes, it increments all neighbors.
                &p.neighbors(OCTOPUS_COLS - 1, OCTOPUS_ROWS - 1, true),
            );
        }
    });
}

fn reset_energy_levels(grid: &mut Grid, points: &[Point]) {
    points.iter().for_each(|Point(x, y)| {
        if grid[*y][*x] > 9 {
            grid[*y][*x] = 0;
        }
    });
}

fn all_points() -> Vec<Point> {
    let mut points = Vec::new();

    for x in 0..OCTOPUS_COLS {
        for y in 0..OCTOPUS_ROWS {
            points.push(Point(x, y));
        }
    }

    points
}

pub fn part_one(input: &str) -> u32 {
    let mut grid = parse(input);
    // optimization: keep a reference of all points in the grid to avoid recomputing this constantly.
    let points = all_points();

    let mut flash_count: u32 = 0;
    for _ in 0..100 {
        flash_count += process_step(&mut grid, &points);
    }

    flash_count
}

pub fn part_two(input: &str) -> usize {
    let mut grid = parse(input);
    // optimization: keep a reference of all points in the grid to avoid recomputing this constantly.
    let points = all_points();

    let mut index: usize = 0;
    let mut all_flashed = false;

    while !all_flashed {
        if process_step(&mut grid, &points) == (OCTOPUS_COLS * OCTOPUS_ROWS) as u32 {
            all_flashed = true
        } else {
            index += 1;
        }
    }

    index + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 11);
        assert_eq!(part_one(&input), 1656);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 11);
        assert_eq!(part_two(&input), 195);
    }
}
