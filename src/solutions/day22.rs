use std::cmp::{min, max};

struct Range { from: i64, to: i64 }

struct Ranges {
    x: Range,
    y: Range,
    z: Range,
}

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
            to: min(a.to, b.to)
        })
    }
}

fn intersection(a: &Cube, b: &Cube, on: bool) -> Option<Cube> {
    let x = intersect(&a.ranges.x, &b.ranges.x)?;
    let y = intersect(&a.ranges.y, &b.ranges.y)?;
    let z = intersect(&a.ranges.z, &b.ranges.z)?;

    Some(Cube {
        on,
        ranges: Ranges { x, y, z }
    })
}

fn cube_diffs(instructions: Vec<Cube>) -> Vec<Cube> {
    let mut cubes: Vec<Cube> = Vec::new();

    for inst in instructions {
        let mut to_add: Vec<Cube> = Vec::new();

        for cube in cubes.iter() {
            let inter= intersection(&inst, cube, !cube.on);

            if let Some(inter) = inter {
                to_add.push(inter);
            }
        }

        if inst.on {
            to_add.push(inst);
        }

        cubes.extend(to_add);
    }

    cubes
}

fn volume(c: Cube) -> i64 {
    let sign = if c.on { 1 } else { -1 };
    sign * (c.ranges.x.to - c.ranges.x.from + 1) * (c.ranges.y.to - c.ranges.y.from + 1) * (c.ranges.z.to - c.ranges.z.from + 1)
}

pub fn part_one(input: &str) -> i64 {
    let bounds = Cube {
        on: true,
        ranges: Ranges {
            x: Range { from: -50, to: 50 },
            y: Range { from: -50, to: 50 },
            z: Range { from: -50, to: 50 }
        }
    };

    cube_diffs(parse(input))
        .into_iter()
        .filter_map(|c| intersection(&c, &bounds, c.on))
        .map(volume)
        .sum()
}


pub fn part_two(input: &str) -> i64 {
    cube_diffs(parse(input))
        .into_iter()
        .map(volume)
        .sum()
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
