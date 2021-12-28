use std::cmp::{max, min};

#[derive(Clone)]
struct Range {
    from: i64,
    to: i64,
}

#[derive(Clone)]
struct Ranges {
    x: Range,
    y: Range,
    z: Range,
}

#[derive(Clone)]
struct Cube {
    on: bool,
    ranges: Ranges,
}

fn parse_range(s: &str) -> Range {
    let mut range = s.split('=').last().unwrap().split("..");
    let from = range.next().unwrap().parse().unwrap();
    let to = range.next().unwrap().parse().unwrap();

    Range { from, to }
}

fn parse(input: &str) -> Vec<Cube> {
    input
        .lines()
        .map(|l| {
            let on = l.starts_with("on");
            let mut ranges = l.split(' ').last().unwrap().split(',').map(parse_range);

            let x = ranges.next().unwrap();
            let y = ranges.next().unwrap();
            let z = ranges.next().unwrap();
            let ranges = Ranges { x, y, z };

            Cube { on, ranges }
        })
        .collect()
}

fn intersect(a: &Range, b: &Range) -> Option<Range> {
    if b.from > a.to || a.from > b.to {
        None
    } else {
        Some(Range {
            from: max(a.from, b.from),
            to: min(a.to, b.to),
        })
    }
}

fn intersection(a: &Cube, b: &Cube, on: bool) -> Option<Cube> {
    let x = intersect(&a.ranges.x, &b.ranges.x)?;
    let y = intersect(&a.ranges.y, &b.ranges.y)?;
    let z = intersect(&a.ranges.z, &b.ranges.z)?;

    Some(Cube {
        on,
        ranges: Ranges { x, y, z },
    })
}

fn cube_diffs(instructions: Vec<Cube>) -> Vec<Cube> {
    instructions.iter().fold(Vec::new(), |mut acc, curr| {
        // add `on` instructions to the diff list.
        let mut to_add = if curr.on { vec![curr.clone()] } else { vec![] };
        // for every intersect with a previous diff, add a diff with opposite sign.
        // this works because we only track `on` cubes and subtractions to them by default:
        // 1. `off` instructions turn off intersecting parts of previous diffs.
        // 2. previously `off` diffs can be turned back on by `on` instructions.
        // 3. if both instructions were `on`, the new instruction cancel out old diffs to prevent duplicates.
        to_add.extend(acc.iter().filter_map(|c| intersection(curr, c, !c.on)));
        acc.extend(to_add);
        acc
    })
}

fn vol(r: Range) -> i64 {
    r.to - r.from + 1
}

fn volume(c: Cube) -> i64 {
    let sign = if c.on { 1 } else { -1 };
    sign * vol(c.ranges.x) * vol(c.ranges.y) * vol(c.ranges.z)
}

pub fn part_one(input: &str) -> i64 {
    let bounds = Cube {
        on: true,
        ranges: Ranges {
            x: Range { from: -50, to: 50 },
            y: Range { from: -50, to: 50 },
            z: Range { from: -50, to: 50 },
        },
    };

    cube_diffs(parse(input))
        .into_iter()
        .filter_map(|c| intersection(&c, &bounds, c.on))
        .map(volume)
        .sum()
}

pub fn part_two(input: &str) -> i64 {
    cube_diffs(parse(input)).into_iter().map(volume).sum()
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
        assert_eq!(part_two(&input), 2758514936282235);
    }
}
