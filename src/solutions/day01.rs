use itertools::Itertools;

fn to_u32(x: &str) -> u32 {
    x.parse::<u32>().unwrap()
}

// identify value increases across iterator by using a sliding window.
fn count_increases(it: impl Iterator<Item = u32>) -> u32 {
    let win = it.tuple_windows();
    win.fold(0, |acc, (a, b)| if b > a { acc + 1 } else { acc })
}

pub fn part_one(input: &str) -> u32 {
    let it = input.lines().map(to_u32);
    count_increases(it)
}

pub fn part_two(input: &str) -> u32 {
    let it = input
        .lines()
        .map(to_u32)
        .tuple_windows()
        .map(|(a, b, c)| a + b + c);

    count_increases(it)
}

#[test]
fn example() {
    let input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
    assert_eq!(part_one(&input), 7);
    assert_eq!(part_two(&input), 5);
}
