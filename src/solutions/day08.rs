use std::collections::HashMap;

use itertools::Itertools;

pub fn part_one(input: &str) -> usize {
    input
        .lines()
        .flat_map(|l| {
            l.split(" | ").last().unwrap().split(' ').filter(|s| {
                let len = s.len();
                (2..=4).contains(&len) || len == 7
            })
        })
        .count()
}

type Pattern = Vec<char>;

/// The display is made up of segments `a-g` mapped as follows:
///  aaaa
/// b    c
/// b    c
///  dddd
/// e    f
/// e    f
///  gggg
type Display = HashMap<char, char>;

trait DisplayMethods {
    fn decode(&self, pattern: &[char]) -> char;
}

impl DisplayMethods for Display {
    /// Once a display is fully reconstructed, we can decode digits with it.
    /// Individual digits are returned as strings since we need to join 4 digits in the caller.
    fn decode(&self, pattern: &[char]) -> char {
        if is_one(pattern) {
            '1'
        } else if is_four(pattern) {
            '4'
        } else if is_seven(pattern) {
            '7'
        } else if is_eight(pattern) {
            '8'
        } else {
            let displayed: String = pattern
                .iter()
                .map(|c| self.get(c).unwrap())
                .sorted_unstable()
                .collect();

            match displayed.as_ref() {
                "abcefg" => '0',
                "acdeg" => '2',
                "acdfg" => '3',
                "abdfg" => '5',
                "abdefg" => '6',
                "abcdfg" => '9',
                val => panic!("unexpected decoded pattern: {}", val),
            }
        }
    }
}

// c,f
fn is_one(pattern: &[char]) -> bool {
    pattern.len() == 2
}

// b,c,d,f
fn is_four(pattern: &[char]) -> bool {
    pattern.len() == 4
}

// a,c,f
fn is_seven(pattern: &[char]) -> bool {
    pattern.len() == 3
}

// a,b,c,d,e,f,g
fn is_eight(pattern: &[char]) -> bool {
    pattern.len() == 7
}

fn is_six(p: &[char], one: &[char]) -> bool {
    one.iter().any(|c| !p.contains(c))
}

fn is_zero(p: &[char], four: &[char]) -> bool {
    four.iter().any(|c| !p.contains(c))
}

/// Helper trait to make `.find()`ing the first digits less verbose.
fn find_by(signal: &[Vec<char>], find_fn: impl Fn(&[char]) -> bool) -> Pattern {
    signal.iter().find(|x| find_fn(x)).unwrap().to_owned()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|l| {
            let patterns: Vec<Pattern> = l
                .replace(" |", "")
                .split(' ')
                .map(|s| s.chars().collect())
                .collect();

            let signal = patterns[0..10].to_vec();
            let outputs = patterns[10..14].to_vec();

            let mut display = Display::new();

            let one = find_by(&signal, is_one);
            let seven = find_by(&signal, is_seven);
            let four = find_by(&signal, is_four);
            let eight = find_by(&signal, is_eight);

            // once we know `c` and `f`, we can isolate `a` by looking at `7`
            display.insert(*seven.iter().find(|c| !&one.contains(c)).unwrap(), 'a');

            // at this point, we can decode the full signal by looking at six-segment components.
            for p in signal.iter().filter(|x| x.len() == 6) {
                if is_six(p, &one) {
                    for c in &one {
                        if p.contains(c) {
                            display.insert(*c, 'f');
                        } else {
                            display.insert(*c, 'c');
                        }
                    }
                } else if is_zero(p, &four) {
                    for c in &four {
                        if !p.contains(c) {
                            display.insert(*c, 'd');
                        } else if !&one.contains(c) {
                            display.insert(*c, 'b');
                        }
                    }
                } else {
                    for c in &eight {
                        if !p.contains(c) {
                            display.insert(*c, 'e');
                        }
                    }
                }
            }

            // whatever segment is left over maps to the last needed segment `g`.
            // we can use `eight` to identify it since it has all segments.
            for c in &eight {
                if !(display.contains_key(c)) {
                    display.insert(*c, 'g');
                }
            }

            // the display is ready for decoding now.
            // We decode the 4-digit number to a string and then parse it to an int.
            let num = outputs.iter().fold(String::new(), |mut acc, p| {
                acc.push(display.decode(p));
                acc
            });

            num.parse::<u32>().unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(part_two(&input), 61229);
    }
}
