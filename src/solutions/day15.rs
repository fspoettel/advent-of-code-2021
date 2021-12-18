/// implementation of Dijkstra's algorithm for a 2d grid.
/// borrows from the example found in the [rust docs](https://doc.rust-lang.org/std/collections/binary_heap/index.html#examples).
/// in contrast to the example, we do not create a directed graph but work with the supplied grid directly.
/// for further information, see:
/// [Wikipedia](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm) |
/// [Introduction to the A* Algorithm](https://www.redblobgames.com/pathfinding/a-star/introduction.html).
mod shortest_path {
    use crate::helpers::grid::Point;
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;

    // while performing the search, track a sorted list of candidates (=state) to visit next on a priority queue.
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: usize,
        position: usize,
    }

    /// the algorithm expects a `min-heap` priority queue as frontier.
    /// the default std. lib implementation is a `max-heap`, so the sort order needs to be flipped for state values.
    /// also adds a tie breaker based on position. see [rust docs](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html#min-heap)
    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other
                .cost
                .cmp(&self.cost)
                .then_with(|| self.position.cmp(&other.position))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    pub fn shortest_path(grid: &[Vec<u32>]) -> Option<usize> {
        let height = grid.len();
        let width = grid[0].len();

        // dist[node] = current shortest distance from `start` to `node`.
        let mut dist: Vec<_> = (0..(height * width)).map(|_| usize::MAX).collect();

        let mut frontier = BinaryHeap::new();

        let start_node = Point(0, 0).to_id(width);
        let target_node = Point(height - 1, width - 1).to_id(width);

        // initialize start with a zero cost.
        dist[start_node] = 0;
        frontier.push(State {
            cost: 0,
            position: start_node,
        });

        // examine the frontier starting with the lowest cost nodes.
        while let Some(State { cost, position }) = frontier.pop() {
            if position == target_node {
                return Some(cost);
            }

            // skip: there is a better path to this node already.
            if cost > dist[position] {
                continue;
            }

            // see if we can find a path with a lower cost than previous paths for any adjacent nodes.
            for point in Point::from_id(position, width).neighbors(width - 1, height - 1, false) {
                let next = State {
                    cost: cost + grid[point.1][point.0] as usize,
                    position: point.to_id(width),
                };

                // if so, add it to the frontier and continue.
                if next.cost < dist[next.position] {
                    frontier.push(next);
                    dist[next.position] = next.cost;
                }
            }
        }

        None
    }
}

use self::shortest_path::shortest_path;

type Row = Vec<u32>;
type Grid = Vec<Row>;

fn parse(input: &str) -> Grid {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

pub fn part_one(input: &str) -> u32 {
    shortest_path(&parse(input)).unwrap() as u32
}

pub fn part_two(input: &str) -> u32 {
    let grid = parse(input);

    let height = grid.len();
    let width = grid[0].len();

    let expanded: Grid = (0..(5 * grid.len()))
        .map(|y| {
            (0..(5 * grid[0].len()))
                .map(|x| {
                    // increment grows by one with every horizontal *and* vertical tile.
                    let x_increment = (x / width) as u32;
                    let y_increment = (y / height) as u32;

                    // each individual value can be derived from the original value and the current distance to it.
                    let cost = grid[x % width][y % height] + x_increment + y_increment;
                    if cost == 9 {
                        cost
                    } else {
                        cost % 9
                    }
                })
                .collect()
        })
        .collect();

    shortest_path(&expanded).unwrap() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 15);
        assert_eq!(part_one(&input), 40);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 15);
        assert_eq!(part_two(&input), 315);
    }
}
