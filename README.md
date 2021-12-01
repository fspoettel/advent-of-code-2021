# aoc-2021

## Usage

> ⚠️ filenames are padded with a leading `0`.

1. add file `./inputs/day{day}.txt` with your inputs. example: `./inputs/day02.txt`
2. add file `./solutions/day{day}.rs` with your solutions. example: `./solutions/day02.rs`
3. reference the module in `./solutions/mod.rs`.
4. add day to `./main.rs`.
5. execute `cargo run <day>`. example: `cargo run 02`

## Templates

**Day:**

```rs
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

**Main:**

```rs
1 => {
    print_solution(day01::part_one(&input), day01::part_two(&input));
}
```
