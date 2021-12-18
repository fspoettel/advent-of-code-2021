use itertools::{Itertools, MinMaxResult};
use std::collections::HashMap;

type Pair = (char, char);
type Rules = HashMap<Pair, char>;

/// Compact representation of a polymer string. This is a necessity to solve part 2 where my naive first approach failed.
/// `pairs` counts the # of times a character combination is present in the polymer.
/// example: `{ (N, N): 1, (N, C): 1, (C, B): 1 }` for the example polymer.
/// A polymerization step creates a new set of pairs by processing `rules` for every pair currently in the polymer.
/// example: `{ (N, N): 3 }` produces `{ (N, C): 2, (C, N): 2 }`.
/// Looking at `pairs` is not enough to derive character counts, so we need a `chars` map to count these as well.
/// whenever a character is added to a pair, we increment a running counter for that character.
/// example: `{ (N, N): 3 }` would increment `C` by 2.
#[derive(Clone)]
struct Polymer {
    pairs: HashMap<Pair, u64>,
    characters: HashMap<char, u64>,
}

impl Polymer {
    fn new() -> Self {
        Polymer {
            pairs: HashMap::new(),
            characters: HashMap::new(),
        }
    }

    fn from_string(line: &str) -> Self {
        let mut polymer = Polymer::new();
        let chars = line.chars();

        chars.clone().for_each(|c| {
            *polymer.characters.entry(c).or_default() += 1;
        });

        chars.clone().tuple_windows().for_each(|(a, b)| {
            *polymer.pairs.entry((a, b)).or_default() += 1;
        });

        polymer
    }

    fn expand(&mut self, rules: &Rules) {
        let pairs = self.pairs.clone();
        self.pairs.clear();

        for (pair, count) in pairs {
            let to_add = rules.get(&pair).unwrap().to_owned();
            *self.pairs.entry((pair.0, to_add)).or_default() += count;
            *self.pairs.entry((to_add, pair.1)).or_default() += count;
            *self.characters.entry(to_add).or_default() += count;
        }
    }

    fn expand_times(&mut self, times: u8, rules: &Rules) -> &Self {
        for _ in 0..times {
            self.expand(rules);
        }
        self
    }

    fn delta(&self) -> u64 {
        match self.characters.values().minmax() {
            MinMaxResult::MinMax(a, b) => b - a,
            _ => unreachable!(),
        }
    }
}

fn parse(input: &str) -> (Polymer, Rules) {
    let mut lines = input.lines();

    let polymer = Polymer::from_string(lines.next().unwrap());

    let rules = lines.fold(HashMap::new(), |mut acc, l| {
        if !l.is_empty() {
            let parts: Vec<char> = l.replace(" -> ", "").chars().collect();
            let pair = (parts[0], parts[1]);
            acc.insert(pair, parts[2]);
        }

        acc
    });

    (polymer, rules)
}

pub fn part_one(input: &str) -> u64 {
    let (mut polymer, rules) = parse(input);
    polymer.expand_times(10, &rules).delta()
}

pub fn part_two(input: &str) -> u64 {
    let (mut polymer, rules) = parse(input);
    polymer.expand_times(40, &rules).delta()
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
