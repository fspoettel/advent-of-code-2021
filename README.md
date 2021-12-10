<img src="./assets/christmas_ferris.png" width="164" align="center">

# 🎄 [Advent of Code](https://adventofcode.com/)

![Language](https://badgen.net/badge/Language/Rust/orange)

<!--- advent_readme_stars table --->
## 2021 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2021/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2021/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2021/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2021/day/4) | ⭐ | ⭐ |
| [Day 5](https://adventofcode.com/2021/day/5) | ⭐ | ⭐ |
| [Day 6](https://adventofcode.com/2021/day/6) | ⭐ | ⭐ |
| [Day 7](https://adventofcode.com/2021/day/7) | ⭐ | ⭐ |
| [Day 8](https://adventofcode.com/2021/day/8) | ⭐ | ⭐ |
| [Day 9](https://adventofcode.com/2021/day/9) | ⭐ | ⭐ |
| [Day 10](https://adventofcode.com/2021/day/10) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

---

## Install

* Install the [Rust toolchain](https://www.rust-lang.org/tools/install).
* (optional) Install [rust-analyzer](https://rust-analyzer.github.io/manual.html) for your editor.
* (optional) Install a native debugger, e.g. [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb) for VS Code.
* (optional) Install [`aoc-cli`](https://github.com/scarvalhojr/aoc-cli/) and follow their setup guide to use the `download` script for puzzle inputs (see below).

## Commands

### Setup new day

```sh
# example: `./scaffold.sh 1`
./scaffold.sh <day>

# output:
# Created module `src/solutions/day01.rs`
# Created input file `src/inputs/day01.txt`
# Created example file `src/examples/day01.txt`
# Linked new module in `src/main.rs`
# Linked new module in `src/solutions/mod.rs`
# Have fun! 🎄
```

Every solution file has _unit tests_ referencing the example input file. You can use these tests to develop and debug your solution. When editing a solution file, `rust-analyzer` will display buttons for these actions above the unit tests.

### Download inputs for a day

```sh
# example: `./download.sh 1`
./download.sh <day>

# output:
# Invoking `aoc` cli...
# Loaded session cookie from "/home/foo/.adventofcode.session".
# Downloading input for day 1, 2021...
# Saving puzzle input to "/tmp/..."...
# Done!
# Wrote input to `src/inputs/day01.txt`...
# Have fun! 🎄
```

Puzzle inputs are not checked into git. [See here](https://old.reddit.com/r/adventofcode/comments/k99rod/sharing_input_data_were_we_requested_not_to/gf2ukkf/?context=3) why.

### Run solutions for a day

```sh
# example: `cargo run 1`
cargo run <day>

# output:
#     Running `target/debug/aoc 1`
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

To run an optimized version for benchmarking, use the `--release` flag or the alias `cargo rr <day>`.

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
