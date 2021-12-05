<img src="https://user-images.githubusercontent.com/1682504/144760188-7962b414-9af0-4fdd-a278-67db23fa1181.png" width="164" align="center">

# 🎄 [Advent of Code](https://adventofcode.com/2021/)

![Language](https://badgen.net/badge/Language/Rust/orange)

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

Every solution file has _unit tests_ referencing the example input file. You can use these tests to develop and debug your solution. When editing a solution file, `rust-analyzer` will display buttons for these actions above the unit tests.

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
# 6 (elapsed: 37.03µs)
#
# 🎄 Part 2 🎄
#
# 9 (elapsed: 33.18µs)
#
# ----
```

To run an optimized version for benchmarking, append the `--release` flag.

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

---

Puzzle inputs are not checked into git. For the why, [see here](https://old.reddit.com/r/adventofcode/comments/k99rod/sharing_input_data_were_we_requested_not_to/gf2ukkf/?context=3).
