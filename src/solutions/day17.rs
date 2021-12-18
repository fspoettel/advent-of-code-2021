use std::cmp::max;

type Point = (isize, isize);
type Velocity = (isize, isize);

struct Bounds {
    left: isize,
    right: isize,
    top: isize,
    bottom: isize,
}

impl Bounds {
    fn contains(&self, (x, y): &Point) -> bool {
        x >= &self.left && x <= &self.right && y <= &self.top && y >= &self.bottom
    }
}

fn parse(line: &str) -> Bounds {
    let values: Vec<isize> = line
        .split(',')
        .flat_map(|part| {
            part.split('=')
                .last()
                .unwrap()
                .split("..")
                .map(|x| x.parse().unwrap())
        })
        .collect();

    Bounds {
        left: values[0],
        right: values[1],
        bottom: values[2],
        top: values[3],
    }
}

fn simulate_point(
    initial_point: Point,
    initial_velocity: Velocity,
    bounds: &Bounds,
) -> Option<isize> {
    let mut point = initial_point;
    let mut velocity = initial_velocity;
    let mut y_max = point.1;

    // terminate if point has overshot bounds
    while point.0 <= bounds.right && point.1 >= bounds.bottom {
        point = (point.0 + velocity.0, point.1 + velocity.1);
        y_max = max(point.1, y_max);

        if bounds.contains(&point) {
            return Some(y_max);
        } else {
            velocity = (max(0, velocity.0 - 1), velocity.1 - 1);
        }
    }

    None
}

fn find_hits(bounds: &Bounds) -> Vec<isize> {
    let mut max_y = Vec::new();
    let initial_position = (0, 0);

    for x in 0..=bounds.right {
        for y in bounds.bottom..=-bounds.bottom {
            if let Some(y) = simulate_point(initial_position, (x, y), bounds) {
                max_y.push(y);
            }
        }
    }

    max_y
}

pub fn part_one(input: &str) -> isize {
    let bounds = parse(input.lines().next().unwrap());
    *find_hits(&bounds).iter().max().unwrap()
}

pub fn part_two(input: &str) -> usize {
    let bounds = parse(input.lines().next().unwrap());
    find_hits(&bounds).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 17);
        assert_eq!(part_one(&input), 45);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 17);
        assert_eq!(part_two(&input), 112);
    }
}
