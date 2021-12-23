use std::collections::HashMap;

fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split(": ").last().unwrap().parse::<u64>().unwrap())
        .collect()
}

fn deterministic_roll(position: &mut u64, score: &mut u64, dice: &mut u64) {
    let roll: u64 = (*dice..(*dice + 3)).sum();
    let next_position = ((*position + roll - 1) % 10) + 1;
    *position = next_position;
    *score += next_position;
    *dice += 3;
}

pub fn part_one(input: &str) -> u64 {
    let positions = parse(input);
    let mut p1_position = positions[0];
    let mut p2_position = positions[1];
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut dice = 1;

    loop {
        deterministic_roll(&mut p1_position, &mut p1_score, &mut dice);
        if p1_score >= 1000 {
            return p2_score * (dice - 1);
        }

        deterministic_roll(&mut p2_position, &mut p2_score, &mut dice);
        if p2_score >= 1000 {
            return p1_score * (dice - 1);
        }
    }
}

// possible rolls for a 3-sided die.
static ROLLS: [(u64, u64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

type Cache = HashMap<(u64, u64, u64, u64), (u64, u64)>;

fn play(
    p1_position: u64,
    p2_position: u64,
    p1_score: u64,
    p2_score: u64,
    cache: &mut Cache,
) -> (u64, u64) {
    let cache_key = (p1_position, p2_position, p1_score, p2_score);

    // if there is a cached resolution for this game state, return it.
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }

    // the game loop works by swapping p1 & p2 every iteration:
    //  1. increment position and score for p1 for every possible dice roll in a turn.
    //  2. call play with p1 and p2 swapped since it's now p2's turn.
    //  3. if any of the previous positions won, count it as a win (=p2 is previous p1) and return.
    //  4. once a loop returns, increment the win counter **in a swapped fashion** since the wins reported for p2 belong to p1.
    if p2_score >= 21 {
        (0, 1)
    } else {
        let res = ROLLS.iter().fold((0, 0), |acc, (roll, n)| {
            let position = ((p1_position + roll - 1) % 10) + 1;
            let wins = play(p2_position, position, p2_score, p1_score + position, cache);
            (acc.0 + n * wins.1, acc.1 + n * wins.0)
        });

        cache.insert(cache_key, res);

        res
    }
}

pub fn part_two(input: &str) -> u64 {
    let mut cache = HashMap::new();

    let positions = parse(input);
    let p1_position = positions[0];
    let p2_position = positions[1];

    let (p1_wins, p2_wins) = play(p1_position, p2_position, 0, 0, &mut cache);
    std::cmp::max(p1_wins, p2_wins)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 21);
        assert_eq!(part_one(&input), 739785);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 21);
        assert_eq!(part_two(&input), 444356092776315);
    }
}
