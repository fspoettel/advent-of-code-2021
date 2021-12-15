use crate::helpers::math::median;

/// tracks open tokens (e.g. `(`) in sequence of occurence.
type CharacterStack = Vec<char>;

/// error thrown if parser fails to parse a line.
struct ParsingError {
    token: char,
}

type ParsingResult = Result<CharacterStack, ParsingError>;

fn opener(c: char) -> Option<char> {
    match c {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        '>' => Some('<'),
        _ => None,
    }
}

/// go through the line char-by-char.
/// opening chars are added to a stack.
/// when closing char is encountered, pop the first item of the stack.
/// if the closing char can be used to close the pair, continue processing the line.
/// if it does not match, throw a `ParsingError` referencing the offending token.
/// once the line completes parsing without errors, return the rest of the stack.
fn parse(line: &str) -> ParsingResult {
    let mut stack: CharacterStack = Vec::new();
    let mut offending_token: Option<char> = None;

    for c in line.chars() {
        let opener = opener(c);

        if let Some(opener) = opener {
            match stack.pop() {
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
            stack.push(c);
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
                let score = stack.iter().rev().fold(0, |acc, char| {
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
