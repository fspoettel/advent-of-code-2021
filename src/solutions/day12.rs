use std::collections::{HashMap, HashSet};

static START: &str = "start";
static END: &str = "end";

#[derive(Debug)]
struct Graph<'a> {
    // nodes are stored as <id, size> map where size is a boolean.
    // `true` indicates that a room is big, `false` small.
    nodes: HashMap<&'a str, bool>,
    // edges are stored as an adjacency list for each node.
    edges: HashMap<&'a str, HashSet<&'a str>>,
}

impl Graph<'_> {
    fn new() -> Self {
        Graph {
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    fn get_adjacent_nodes(&self, node_id: &str) -> Vec<&&str> {
        self.edges.get(node_id).unwrap().iter().collect()
    }
}

fn parse(input: &str) -> Graph {
    let mut graph = Graph::new();

    input.lines().for_each(|l| {
        let mut node_pair = l.split('-').map(|id| (id, id.to_uppercase() == id));

        let (from, from_type) = node_pair.next().unwrap();
        let (to, to_type) = node_pair.next().unwrap();

        graph.nodes.insert(from, from_type);
        graph.nodes.insert(to, to_type);
        graph.edges.entry(from).or_default().insert(to);
        graph.edges.entry(to).or_default().insert(from);
    });

    graph
}

fn search(graph: &Graph, seen: &HashSet<&str>, id: &str, small_room_counter: u8) -> u32 {
    let mut small_room_counter = small_room_counter;

    if id == END {
        return 1;
    }

    if seen.contains(&id) {
        if id == START {
            return 0;
        // in part one, any small room can only be visited once.
        // in part two, **one** room may be visited twice.
        // to reflect that, a counter is decremented the first time a small room is encountered.
        } else if !*graph.nodes.get(id).unwrap() {
            if small_room_counter == 0 {
                return 0;
            } else {
                small_room_counter -= 1;
            }
        }
    }

    let mut seen_here = seen.clone();
    seen_here.insert(id);

    let result = graph
        .get_adjacent_nodes(id)
        .iter()
        .map(|adjacent_node| search(graph, &seen_here, adjacent_node, small_room_counter))
        .sum();

    result
}

pub fn part_one(input: &str) -> u32 {
    let graph = parse(input);
    let seen: HashSet<&str> = HashSet::new();
    search(&graph, &seen, START, 0)
}

pub fn part_two(input: &str) -> u32 {
    let graph = parse(input);
    let seen: HashSet<&str> = HashSet::new();
    search(&graph, &seen, START, 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 12);
        assert_eq!(part_one(&input), 226);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 12);
        assert_eq!(part_two(&input), 3509);
    }
}
