/// implementation of Dijkstra's algorithm.
/// borrows from the example found in the [Rust Docs](https://doc.rust-lang.org/std/collections/binary_heap/index.html#examples).
/// for further information, see:
/// [Wikipedia](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm) |
/// [Introduction to the A* Algorithm](https://www.redblobgames.com/pathfinding/a-star/introduction.html).
mod shortest_path {
    use std::cmp::Ordering;
    use std::collections::BinaryHeap;

    /// model the grid as a directed graph.
    pub type Graph = Vec<Node>;

    /// each node in the graph is an adjacency list of neighbors.
    /// the example start position would look like:
    /// vec![Edge { cost: 1, node: 1 }, { cost: 1, node: 1 }]
    pub type Node = Vec<Edge>;

    // each edge contains the connected node and the cost to move to that node.
    #[derive(Clone, Debug)]
    pub struct Edge {
        pub node: usize,
        pub cost: usize,
    }

    // while performing the search, track a sorted list of candidates (=state) to visit next on a priority queue.
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: usize,
        position: usize,
    }

    /// the algorithm expects a `min-heap` priority queue as frontier.
    /// the default std. lib implementation is a `max-heap`, so the sort order needs to be flipped for state values.
    /// also adds a tie breaker based on position.
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

    pub fn shortest_path(
        graph: &[Vec<Edge>],
        start_node: usize,
        target_node: usize,
    ) -> Option<usize> {
        // dist[node] = current shortest distance from `start` to `node`.
        let mut dist: Vec<_> = (0..graph.len()).map(|_| usize::MAX).collect();
        let mut frontier = BinaryHeap::new();

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
            for edge in &graph[position] {
                let next = State {
                    cost: cost + edge.cost,
                    position: edge.node,
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

use self::shortest_path::{shortest_path, Edge, Graph};
use crate::helpers::grid::{neighbors, Point};

type Row = Vec<u32>;
type Grid = Vec<Row>;

fn parse_input(input: &str) -> Grid {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

/// transforms the 2d grid to a directed graph, also providing `start` and `target` node.
fn grid_to_graph(grid: &[Row]) -> (Graph, usize, usize) {
    let y_ceil = grid.len();
    let x_ceil = grid[0].len();

    let mut graph: Graph = vec![Vec::new(); y_ceil * x_ceil];

    // add an edge to the adjacency list for every neighbor.
    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, _)| {
            let point = Point(x, y);
            neighbors(point, x_ceil - 1, y_ceil - 1, false)
                .into_iter()
                .for_each(|p| {
                    graph[point.to_id(x_ceil)].push(Edge {
                        cost: grid[p.1][p.0] as usize,
                        node: p.to_id(x_ceil),
                    });
                });
        })
    });

    let start_node = Point(0, 0).to_id(x_ceil);
    let target_node = Point(y_ceil - 1, x_ceil - 1).to_id(x_ceil);

    (graph, start_node, target_node)
}

pub fn part_one(input: &str) -> u32 {
    let grid = parse_input(input);
    let (graph, start_node, target_node) = grid_to_graph(&grid);
    shortest_path(&graph, start_node, target_node).unwrap() as u32
}

pub fn part_two(input: &str) -> u32 {
    let grid = parse_input(input);

    let y_ceil = grid.len();
    let x_ceil = grid[0].len();

    let expanded: Grid = (0..(5 * grid.len()))
        .map(|y| {
            (0..(5 * grid[0].len()))
                .map(|x| {
                    // increment grows by one with every horizontal *and* vertical tile.
                    let x_increment = (x / x_ceil) as u32;
                    let y_increment = (y / y_ceil) as u32;

                    // each individual value can be derived from the original value and the current distance to it.
                    let cost = grid[x % x_ceil][y % y_ceil] + x_increment + y_increment;
                    if cost == 9 {
                        cost
                    } else {
                        cost % 9
                    }
                })
                .collect()
        })
        .collect();

    let (graph, start_node, target_node) = grid_to_graph(&expanded);
    shortest_path(&graph, start_node, target_node).unwrap() as u32
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
