use itertools::Itertools;
use std::collections::HashMap;

type Pair = (char, char);
type Instructions = HashMap<Pair, char>;

/// Compact representation of a polymer string. This is a necessity to solve part 2 where my naive first approach failed.
/// `pairs` counts the # of times a character combination is present in the polymer.
/// example: `{ (N, N): 1, (N, C): 1, (C, B): 1 }` for the example polymer.
/// A polymerization step creates a new set of pairs by processing `instructions` for every pair currently in the polymer.
/// example: `{ (N, N): 3 }` produces `{ (N, C): 2, (C, N): 2 }`.
/// Looking at `pairs` is not enough to derive character counts, so we need a `chars` map to count these as well.
/// whenever a character is added to a pair, we increment a running counter for that character.
/// example: `{ (N, N): 3 }` would increment `C` by 2.
#[derive(Clone)]
struct Polymer {
    pairs: HashMap<Pair, u64>,
    chars: HashMap<char, u64>,
}

impl Polymer {
    fn new() -> Self {
        Polymer {
            pairs: HashMap::new(),
            chars: HashMap::new(),
        }
    }

    fn from_string(line: &str) -> Self {
        let mut poly = Polymer::new();

        line.chars().for_each(|c| {
            *poly.chars.entry(c).or_default() += 1;
        });

        line.chars().tuple_windows().for_each(|(a, b)| {
            *poly.pairs.entry((a, b)).or_default() += 1;
        });

        poly
    }

    fn process(&mut self, instructions: &Instructions) {
        let pairs = self.pairs.clone();
        self.pairs.clear();

        for (pair, count) in pairs {
            let to_add = instructions.get(&pair).unwrap().to_owned();
            *self.pairs.entry((pair.0, to_add)).or_default() += count;
            *self.pairs.entry((to_add, pair.1)).or_default() += count;
            *self.chars.entry(to_add).or_default() += count;
        }
    }

    fn process_times(&mut self, times: u8, instructions: &Instructions) -> &Self {
        for _ in 0..times {
            self.process(instructions);
        }
        self
    }

    fn to_solution(&self) -> u64 {
        let mut values: Vec<&u64> = self.chars.values().collect();
        values.sort_unstable();
        values[values.len() - 1] - values[0]
    }
}

fn parse_input(input: &str) -> (Polymer, Instructions) {
    let mut lines = input.lines();

    let polymer = Polymer::from_string(lines.next().unwrap());

    let instructions = lines.fold(HashMap::new(), |mut acc, l| {
        if !l.is_empty() {
            let parts: Vec<Vec<char>> = l.split(" -> ").map(|s| s.chars().collect()).collect();
            let pair = (parts[0][0], parts[0][1]);
            acc.insert(pair, parts[1][0]);
        }

        acc
    });

    (polymer, instructions)
}

pub fn part_one(input: &str) -> u64 {
    let (mut polymer, instructions) = parse_input(input);
    polymer.process_times(10, &instructions).to_solution()
}

pub fn part_two(input: &str) -> u64 {
    let (mut polymer, instructions) = parse_input(input);
    polymer.process_times(40, &instructions).to_solution()
}

#[test]
fn test_part_one() {
    use aoc::read_file;
    let input = read_file("examples", 14);
    assert_eq!(part_one(&input), 1588);
}

#[test]
fn test_part_two() {
    use aoc::read_file;
    let input = read_file("examples", 14);
    assert_eq!(part_two(&input), 2188189693529);
}
