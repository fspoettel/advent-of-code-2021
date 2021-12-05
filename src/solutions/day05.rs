use std::cmp::{max, min};
use std::ops::Range;
use std::{collections::HashMap, convert::TryInto};

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

type Line = (Point, Point);
type Grid = HashMap<String, u32>;

trait PointGrid {
    fn add_point(&mut self, x: usize, y: usize);
    fn add_points(&mut self, l: &str, include_diagonals: bool);
    fn overlaps(&self) -> u32;
}

impl PointGrid for Grid {
    fn add_point(&mut self, x: usize, y: usize) {
        let key = format!("{}.{}", x, y);
        *self.entry(key).or_default() += 1;
    }

    fn add_points(&mut self, l: &str, include_diagonals: bool) {
        let (p1, p2) = parse_line(l);
        // horizontals
        if p1.y == p2.y {
            for i in min_to_max(p1.x, p2.x) {
                let _ = &self.add_point(i, p1.y);
            }
        // verticals
        } else if p1.x == p2.x {
            for i in min_to_max(p1.y, p2.y) {
                let _ = &self.add_point(p1.x, i);
            }
        // diagonals
        } else if include_diagonals {
            let left = if p1.x < p2.x { &p1 } else { &p2 };
            let right = if p1.x < p2.x { &p2 } else { &p1 };
            let ttb = left.y < right.y;
            for (i, x) in (left.x..right.x + 1).enumerate() {
                let _ = &self.add_point(x, if ttb { left.y + i } else { left.y - i });
            }
        }
    }

    fn overlaps(&self) -> u32 {
        self
            .values()
            .filter(|v| **v > 1)
            .count()
            .try_into()
            .unwrap()
    }
}

fn min_to_max(a: usize, b: usize) -> Range<usize> {
    min(a, b)..max(a, b) + 1
}

fn parse_line(l: &str) -> Line {
    let mut parts = l.split("->").map(|p| {
        let mut nums = p.split(',').map(|x| x.trim().parse().unwrap());
        Point {
            x: nums.next().unwrap(),
            y: nums.next().unwrap(),
        }
    });

    (parts.next().unwrap(), parts.next().unwrap())
}

pub fn part_one(input: &str) -> u32 {
    let mut grid: Grid = HashMap::new();
    input.lines().for_each(|l| grid.add_points(l, false));
    grid.overlaps()
}

pub fn part_two(input: &str) -> u32 {
    let mut grid: Grid = HashMap::new();
    input.lines().for_each(|l| grid.add_points(l, true));
    grid.overlaps()
}

#[test]
fn example() {
    use aoc2021::read_file;
    let input = read_file("examples", 5);
    assert_eq!(part_one(&input), 5);
    assert_eq!(part_two(&input), 12);
}
