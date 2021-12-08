type Signal = [Vec<char>; 10];
type Output = [Vec<char>; 4];

fn parse_input(input: &str) -> Vec<(Signal, Output)> {
    input
        .lines()
        .map(|l| {
            let patterns: Vec<Vec<char>> = l.replace(" |", "")
                .split(' ')
                .map(|s| s.chars().collect())
                .collect();
            (
                (patterns[0..10]).to_owned().try_into().unwrap(),
                (patterns[10..14]).to_owned().try_into().unwrap()
            )
        })
        .collect()
}

fn is_one(str: &[char]) -> bool {
    str.len() == 2
}

fn is_four(str: &[char]) -> bool {
    str.len() == 4
}

fn is_seven(str: &[char]) -> bool {
    str.len() == 3
}

fn is_eight(str: &[char]) -> bool {
    str.len() == 7
}

pub fn part_one(input: &str) -> usize {
    let messages = parse_input(input);

    messages
        .iter()
        .map(|(_, output)| output)
        .flatten()
        .filter(|w| is_one(w) || is_four(w) || is_seven(w) || is_eight(w))
        .count()
}

pub fn part_two(_input: &str) -> u32 {
    0
}

#[test]
fn test_part_one() {
    use aoc::read_file;
    let input = read_file("examples", 8);
    assert_eq!(part_one(&input), 26);
}

#[test]
fn test_part_two() {
    use aoc::read_file;
    let input = read_file("examples", 8);
    assert_eq!(part_two(&input), 0);
}
