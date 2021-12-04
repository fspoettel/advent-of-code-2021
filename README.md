# [Advent of Code](https://adventofcode.com/2021/) 🎄

![Language](https://badgen.net/badge/Language/Rust)

## Setup

* Install the [Rust toolchain](https://www.rust-lang.org/tools/install).
* (recommended) Install [rust-analyzer](https://rust-analyzer.github.io/manual.html) for your editor.

## Commands

### Setup new day

```sh
# example: `./scaffold 1`
./scaffold <day>

# output:
# Created module `src/solutions/day01.rs`
# Created input file `src/inputs/day01.txt`
# Created example file `src/examples/day01.txt`
# Linked new module in `src/main.rs`
# Linked new module in `src/solutions/mod.rs`
# Have fun! 🎄
```

Every solution file has a _unit test_ referencing the example input file. You can use this test to develop and debug your solution. When editing a solution file, `rust-analyzer`  will display buttons for these actions above the unit test.

### Run solutions for a day

```sh
# example: `cargo run 1`
cargo run <day>

# output:
#     Running `target/debug/aoc2021 1`
# ----
#
# 🎄 Part 1 🎄
#
# 0
#
# 🎄 Part 2 🎄
#
# 0
#
# ----
```

### Run all solutions against example input

```sh
cargo test
```

### Format code

```sh
cargo fmt
```

### Lint code

```sh
cargo clippy
```
