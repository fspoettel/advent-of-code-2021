use itertools::Itertools;
use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point(i32, i32, i32);

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        (other.0 - self.0).pow(2) + (other.1 - self.1).pow(2) + (other.2 - self.2).pow(2)
    }

    fn manhattan_distance(&self, other: &Point) -> i32 {
        (self.0 - other.0).abs() + (self.1 - other.1).abs() + (self.2 - other.2).abs()
    }

    fn rotate(&self, rot: u8) -> Self {
        let &Point(x, y, z) = self;

        match rot {
            // translation of http://www.euclideanspace.com/maths/algebra/matrix/transforms/examples/index.htm
            0 => Point(x, y, z),
            1 => Point(x, z, -y),
            2 => Point(x, -y, -z),
            3 => Point(x, -z, y),
            4 => Point(y, -x, z),
            5 => Point(y, z, x),
            6 => Point(y, x, -z),
            7 => Point(y, -z, -x),
            8 => Point(-x, -y, z),
            9 => Point(-x, -z, -y),
            10 => Point(-x, y, -z),
            11 => Point(-x, z, y),
            12 => Point(-y, x, z),
            13 => Point(-y, -z, x),
            14 => Point(-y, -x, -z),
            15 => Point(-y, z, -x),
            16 => Point(z, y, -x),
            17 => Point(z, x, y),
            18 => Point(z, -y, x),
            19 => Point(z, -x, -y),
            20 => Point(-z, -y, -x),
            21 => Point(-z, -x, y),
            22 => Point(-z, y, x),
            23 => Point(-z, x, -y),
            v => panic!("unexpected rotation {}", v),
        }
    }
}

type Neighbors = (usize, usize);

type Report = Vec<Point>;
type Reports = Vec<Report>;

type Distances = HashSet<i32>;

fn parse(input: &str) -> Reports {
    input.lines().fold(Vec::new(), |mut acc, l| {
        if l.starts_with("---") {
            acc.push(vec![]);
        } else if !l.is_empty() {
            let mut coords = l.split(',').map(|s| s.parse::<i32>().unwrap());
            let last = acc.len() - 1;

            acc[last].push(Point(
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            ));
        }

        acc
    })
}

fn distances(reports: &[Report]) -> Vec<Distances> {
    reports
        .iter()
        .map(|r| {
            r.iter()
                .tuple_combinations()
                .map(|(p1, p2)| p1.distance(p2))
                .collect()
        })
        .collect()
}

fn find_neighbors(distances: &[Distances]) -> Vec<Neighbors> {
    distances
        .iter()
        .enumerate()
        .tuple_combinations()
        .filter(|((_, d1), (_, d2))| d1.intersection(d2).count() >= 66)
        .flat_map(|((i, _), (j, _))| [(i, j), (j, i)])
        .collect()
}

fn unaligned_neighbors(neighbors: &[Neighbors], aligned: &[Report]) -> Option<Neighbors> {
    neighbors
        .iter()
        .find(|(a, b)| !aligned[*a].is_empty() && aligned[*b].is_empty())
        .map(|(a, b)| (*a, *b))
}

fn find_pair_by_distance(reports: &[Point], distance: i32) -> (&Point, &Point) {
    reports
        .iter()
        .tuple_combinations()
        .find(|(a, b)| a.distance(b) == distance)
        .unwrap()
}

fn align(reports: &[Report]) -> (Vec<Report>, Vec<Point>) {
    let distances = distances(reports);
    let neighbors = find_neighbors(&distances);

    let mut alignments: Vec<Point> = vec![Point(0, 0, 0)];
    let mut aligned: Vec<Report> = vec![vec![]; reports.len()];
    aligned[0] = reports[0].clone();

    while let Some((a, b)) = unaligned_neighbors(&neighbors, &aligned) {
        let common_distance = distances[a].intersection(&distances[b]).next().unwrap();

        let (c0, c1) = find_pair_by_distance(&aligned[a], *common_distance);
        let (t0, t1) = find_pair_by_distance(&reports[b], *common_distance);

        let mut alignment: Option<Point> = None;
        let mut rot = 0;

        while rot < 24 {
            let r0 = t0.rotate(rot);
            let r1 = t1.rotate(rot);

            let saligned = r0 - *c0 == r1 - *c1;
            let asaligned = r0 - *c1 == r1 - *c0;

            if saligned || asaligned {
                alignment = if saligned {
                    Some(*c0 - r0)
                } else {
                    Some(*c1 - r0)
                };
                break;
            } else {
                rot += 1;
            }
        }

        if let Some(alignment) = alignment {
            alignments.push(alignment);
            aligned[b] = reports[b]
                .iter()
                .map(|p| p.rotate(rot) + alignment)
                .collect();
        } else {
            panic!("could not find a canonical orientation for all reports!");
        }
    }

    (aligned, alignments)
}

pub fn part_one(input: &str) -> usize {
    let reports = parse(input);
    align(&reports).0.iter().flatten().unique().count()
}

pub fn part_two(input: &str) -> i32 {
    let reports = parse(input);
    let (_, alignments) = align(&reports);

    alignments
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.manhattan_distance(b))
        .max()
        .unwrap()
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
        assert_eq!(part_two(&input), 3621);
    }
}
