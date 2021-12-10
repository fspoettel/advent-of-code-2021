use std::collections::VecDeque;

use aoc::median;

type ParsingResult = Result<CharacterStack, ParsingError>;
type CharacterStack = VecDeque<char>;
struct ParsingError {
    token: char,
}

fn opener(c: char) -> Option<char> {
    match c {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        '>' => Some('<'),
        _ => None,
    }
}

/// go through the line character-by-character.
/// opening chars are pushed to the front of the stack.
/// closing chars are checked against the front of the stack.
/// if it matches, we continue processing the line.
/// if it does not match, we throw a ParsingError containing the offending token.
/// once the line completes parsing without errors, we return the rest of the stack.
fn parse(line: &str) -> ParsingResult {
    let mut stack: CharacterStack = VecDeque::new();
    let mut offending_token: Option<char> = None;

    for c in line.chars() {
        let opener = opener(c);

        if let Some(opener) = opener {
            match stack.pop_front() {
                Some(last_open) => {
                    if opener != last_open {
                        offending_token = Some(c);
                        break;
                    }
                }
                // case doesn't seem to occur in puzzle input.
                None => {
                    offending_token = Some(c);
                    break;
                }
            }
        } else {
            stack.push_front(c);
        }
    }

    match offending_token {
        Some(token) => Err(ParsingError { token }),
        None => Ok(stack),
    }
}

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|l| match parse(l) {
            Ok(_) => 0,
            Err(err) => match err.token {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => 0,
            },
        })
        .sum()
}

pub fn part_two(input: &str) -> u64 {
    let mut scores: Vec<u64> = input
        .lines()
        .filter_map(|l| match parse(l) {
            Ok(stack) => {
                let score = stack.iter().fold(0, |acc, char| {
                    acc * 5
                        + match char {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4,
                            _ => 0,
                        }
                });

                Some(score)
            }
            Err(_) => None,
        })
        .collect();

    median(&mut scores)
}

#[test]
fn test_part_one() {
    use aoc::read_file;
    let input = read_file("examples", 10);
    assert_eq!(part_one(&input), 26397);
}

#[test]
fn test_part_two() {
    use aoc::read_file;
    let input = read_file("examples", 10);
    assert_eq!(part_two(&input), 288957);
}
