# 🎄 [Advent of Code 2021](https://adventofcode.com/2021/) 🎄

## Usage

1. add file `./inputs/day{day}.txt` with your inputs. example: `./inputs/day01.txt`
2. add file `./solutions/day{day}.rs` with your solutions. example: `./solutions/day01.rs`
3. reference the module in `./solutions/mod.rs`.
4. add day to `./main.rs`.
5. execute `cargo run <day>`. example: `cargo run 01`

## Templates

```rs
/// <dayXX.rs>
pub fn part_one(input: &str) -> u32 {
    0
}

pub fn part_two(input: &str) -> u32 {
    0
}

#[test]
fn example() {
    let input = "".to_string();
    assert_eq!(part_one(&input), 0);
    assert_eq!(part_two(&input), 0);
}
```

```rs
/// solutions/mod.rs
pub mod day01;
```

```rs
/// main.rs
1 => solve_day!(day01, &input),
```
