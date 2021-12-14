use std::{collections::HashMap, convert::TryInto};

struct Point {
    x: i32,
    y: i32,
}

type Line = (Point, Point);
type Grid = HashMap<(i32, i32), u32>;

trait PointGrid {
    fn add_point(&mut self, x: i32, y: i32);
    fn add_points(&mut self, line: &str, skip_diagonals: bool);
    fn overlaps(&self) -> u32;
}

impl PointGrid for Grid {
    fn add_point(&mut self, x: i32, y: i32) {
        *self.entry((x, y)).or_default() += 1;
    }

    fn add_points(&mut self, line: &str, skip_diagonals: bool) {
        let (p1, p2) = parse_line(line);

        if skip_diagonals && (p1.x != p2.x && p1.y != p2.y) {
            return;
        }

        let mut x = p1.x;
        let mut y = p1.y;

        let dx = (p2.x - p1.x).signum();
        let dy = (p2.y - p1.y).signum();

        while (x, y) != (p2.x + dx, p2.y + dy) {
            *self.entry((x, y)).or_default() += 1;
            x += dx;
            y += dy;
        }
    }

    fn overlaps(&self) -> u32 {
        self.values()
            .filter(|v| **v > 1)
            .count()
            .try_into()
            .unwrap()
    }
}

fn parse_line(l: &str) -> Line {
    let mut parts = l.split(" -> ").map(|p| {
        let mut nums = p.split(',').map(|x| x.parse().unwrap());
        Point {
            x: nums.next().unwrap(),
            y: nums.next().unwrap(),
        }
    });

    (parts.next().unwrap(), parts.next().unwrap())
}

pub fn part_one(input: &str) -> u32 {
    let mut grid: Grid = HashMap::new();
    input.lines().for_each(|l| grid.add_points(l, true));
    grid.overlaps()
}

pub fn part_two(input: &str) -> u32 {
    let mut grid: Grid = HashMap::new();
    input.lines().for_each(|l| grid.add_points(l, false));
    grid.overlaps()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 5);
        assert_eq!(part_one(&input), 5);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 5);
        assert_eq!(part_two(&input), 12);
    }
}
