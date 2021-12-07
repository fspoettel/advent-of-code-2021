fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

pub fn part_one(input: &str) -> i32 {
    let mut positions = parse_input(input);
    let median = aoc::median(&mut positions);
    positions.iter().map(|x| (x - median).abs()).sum()
}

pub fn part_two(input: &str) -> i32 {
    let positions = parse_input(input);

    let mean = positions.iter().sum::<i32>() / positions.len() as i32;

    // the mean does not always return an int, there is some variance to the result.
    // the right answer is within the surrounding candidates of the mean though.
    let candidates = mean - 2..mean + 2;

    // @see https://en.wikipedia.org/wiki/Triangular_number
    fn triangular(a: i32) -> i32 {
        a * (a + 1) / 2
    }

    candidates
        .map(|i| positions.iter().map(|p| triangular((p - i).abs())).sum())
        .min()
        .unwrap()
}

#[test]
fn test_part_one() {
    use aoc::read_file;
    let input = read_file("examples", 7);
    assert_eq!(part_one(&input), 37);
}

#[test]
fn test_part_two() {
    use aoc::read_file;
    let input = read_file("examples", 7);
    assert_eq!(part_two(&input), 168);
}
