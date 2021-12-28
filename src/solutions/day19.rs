use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Point = (i32, i32, i32);

type Report = Vec<Point>;
type Reports = HashMap<u32, Report>;

fn parse(input: &str) -> Reports {
    let mut reports: Reports = HashMap::new();
    let mut current_report: i32 = -1;

    input.lines().for_each(|l| {
        if l.starts_with("---") {
            current_report += 1;
        } else if !l.is_empty() {
            let mut coords = l.split(',').map(|s| s.parse::<i32>().unwrap());
            reports
                .entry(current_report.try_into().unwrap())
                .or_default()
                .push((
                    coords.next().unwrap(),
                    coords.next().unwrap(),
                    coords.next().unwrap(),
                ));
        }
    });

    reports
}

static ROTATION_MATRICES: [[[i8; 3]; 3]; 24] = [
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
    [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
    [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
    [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
    [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
    [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
    [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
    [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
    [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
    [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
    [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
    [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
    [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
    [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
    [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
    [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
    [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
    [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
    [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
    [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
    [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
    [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
    [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
    [[0, 1, 0], [0, 0, -1], [-1, 0, 0]],
];

type Distances = HashMap<i32, (Point, Point)>;
type DistanceMap = HashMap<u32, Distances>;

fn to_distances(report: &Report) -> Distances {
    report
        .iter()
        .tuple_combinations()
        .fold(HashMap::new(), |mut acc, (p1, p2)| {
            let distance = (((p2.0 - p1.0).pow(2) + (p2.1 - p1.1).pow(2) + (p2.2 - p1.2).pow(2))
                as f64)
                .sqrt() as i32;
            acc.insert(distance.abs(), (*p1, *p2));
            acc
        })
}

fn to_distance_cache(reports: &Reports) -> DistanceMap {
    reports.iter().fold(HashMap::new(), |mut acc, curr| {
        acc.insert(*curr.0, to_distances(curr.1));
        acc
    })
}

fn match_reports(reports: &Reports, distance_cache: &DistanceMap) -> Vec<(u32, u32)> {
    let mut matching_reports: Vec<(u32, u32)> = Vec::new();
    let mut seen_reports: Vec<u32> = vec![0];

    while matching_reports.len() != reports.len() - 1 {
        let current_report = seen_reports.iter().last().unwrap();

        let distances: Vec<i32> = distance_cache
            .get(&current_report)
            .unwrap()
            .keys()
            .map(|k| *k)
            .collect();

        let matching_report = reports
            .keys()
            .find(|other_id| {
                !seen_reports.contains(*other_id)
                    && distance_cache
                        .get(*other_id)
                        .unwrap()
                        .keys()
                        .filter(|distance| distances.contains(*distance))
                        .count()
                        >= 66
            })
            .unwrap();

        matching_reports.push((*current_report, *matching_report));
        seen_reports.push(*matching_report);
    }

    matching_reports
}

pub fn part_one(input: &str) -> u32 {
    let reports = parse(input);
    let distance_cache = to_distance_cache(&reports);
    let matching_reports = match_reports(&reports, &distance_cache);

    let mut unique_points: Vec<Point> = Vec::new();

    for (r1, r2) in matching_reports {
        println!("{} <> {}", r1, r2);
    }

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
