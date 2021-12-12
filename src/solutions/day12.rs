use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Graph {
    // nodes are stored as <id, size> map where size is a boolean.
    // `true` indicates that a room is big, `false` small.
    nodes: HashMap<String, bool>,
    // edges are stored as an adjacency list for each node.
    edges: HashMap<String, HashSet<String>>,
}

impl Graph {
    fn get_adjacent_nodes(&self, node_id: &str) -> Vec<&String> {
        self.edges.get(node_id).unwrap().iter().collect()
    }
}

fn parse_input(input: &str) -> Graph {
    let mut graph = Graph {
        nodes: HashMap::new(),
        edges: HashMap::new(),
    };

    input.lines().for_each(|l| {
        let mut node_pair = l
            .split('-')
            .map(|id| (id.to_string(), id.to_uppercase() == id));

        let (from, from_type) = node_pair.next().unwrap();
        let (to, to_type) = node_pair.next().unwrap();

        graph.nodes.insert(from.clone(), from_type);
        graph.nodes.insert(to.clone(), to_type);
        graph
            .edges
            .entry(from.clone())
            .or_default()
            .insert(to.clone());
        graph.edges.entry(to).or_default().insert(from);
    });

    graph
}

fn search(graph: &Graph, seen: &HashSet<&str>, id: &str, small_room_counter: u8) -> u32 {
    let mut small_room_counter = small_room_counter;

    if id == "end" {
        return 1;
    }

    if seen.contains(&id) {
        if id == "start" {
            return 0;
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
        .get_adjacent_nodes(&id)
        .iter()
        .map(|adjacent_node| search(graph, &seen_here, adjacent_node, small_room_counter))
        .sum();

    result
}

pub fn part_one(input: &str) -> u32 {
    let graph = parse_input(input);
    let seen: HashSet<&str> = HashSet::new();
    search(&graph, &seen, "start", 0)
}

pub fn part_two(input: &str) -> u32 {
    let graph = parse_input(input);
    let seen: HashSet<&str> = HashSet::new();
    search(&graph, &seen, "start", 1)
}

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