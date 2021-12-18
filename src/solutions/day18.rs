use itertools::Itertools;
use serde_json::Value;
use std::cmp::max;

#[derive(Clone, Copy, Debug)]
enum Symbol {
    LeftBracket,
    RightBracket,
    Comma,
    Number(u32),
}

impl Symbol {
    fn from_char(c: char) -> Self {
        match c {
            '[' => Symbol::LeftBracket,
            ']' => Symbol::RightBracket,
            ',' => Symbol::Comma,
            c => Symbol::Number(c.to_digit(10).unwrap()),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Symbol::LeftBracket => '[',
            Symbol::RightBracket => ']',
            Symbol::Comma => ',',
            Symbol::Number(v) => char::from_digit(*v, 10).unwrap(),
        }
    }
}

type Snail = Vec<Symbol>;

trait SnailMethods {
    fn add(a: &Self, b: &Self) -> Self;
    fn explode(&mut self) -> bool;
    fn from_str(s: &str) -> Snail;
    fn reduce(&mut self);
    fn split(&mut self) -> bool;
    fn to_json(&self) -> Value;
}

impl SnailMethods for Snail {
    fn from_str(s: &str) -> Snail {
        s.chars().map(Symbol::from_char).collect()
    }

    // cast to a nested JSON array in order to do an easy, recursive magnitude calculation.
    // there certainly is a more efficient solution :P
    fn to_json(&self) -> Value {
        let s: String = self.iter().map(Symbol::to_char).collect();
        serde_json::from_str(&s).unwrap()
    }

    fn add(a: &Snail, b: &Snail) -> Self {
        let mut snail: Self = vec![Symbol::LeftBracket];
        snail.extend(a.iter().map(|x| *x));
        snail.push(Symbol::Comma);
        snail.extend(b.iter().map(|x| *x));
        snail.push(Symbol::RightBracket);
        snail.reduce();
        snail
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() || self.split() {
                continue;
            } else {
                break;
            }
        }
    }

    fn split(&mut self) -> bool {
        let pos = self.iter().position(|s| {
            if let Symbol::Number(x) = s {
                *x > 9
            } else {
                false
            }
        });

        match pos {
            Some(index) => {
                if let Symbol::Number(x) = self[index] {
                    let l = x / 2;
                    let r = x - l;
                    self.splice(
                        index..index + 1,
                        [
                            Symbol::LeftBracket,
                            Symbol::Number(l),
                            Symbol::Comma,
                            Symbol::Number(r),
                            Symbol::RightBracket,
                        ],
                    );
                    true
                } else {
                    unreachable!()
                }
            }
            None => false,
        }
    }

    fn explode(&mut self) -> bool {
        let mut depth = 0;

        let pos = self.iter().position(|s| {
            match s {
                Symbol::LeftBracket => {
                    depth += 1;
                }
                Symbol::RightBracket => {
                    depth -= 1;
                }
                _ => (),
            };

            depth == 5
        });

        match pos {
            None => false,
            Some(index) => {
                let mut i = index;
                let mut j = index + 5;

                let l = if let Symbol::Number(x) = self[index + 1] {
                    x
                } else {
                    unreachable!()
                };

                let r = if let Symbol::Number(x) = self[index + 3] {
                    x
                } else {
                    unreachable!()
                };

                while i > 0 {
                    if let Symbol::Number(x) = self[i] {
                        self.splice(i..i + 1, [Symbol::Number(x + l)]);
                        break;
                    } else {
                        i -= 1;
                    }
                }

                while j < self.len() {
                    if let Symbol::Number(x) = self[j] {
                        self.splice(j..j + 1, [Symbol::Number(x + r)]);
                        break;
                    } else {
                        j += 1;
                    }
                }

                self.splice(index..index + 5, [Symbol::Number(0)]);
                true
            }
        }
    }
}

fn parse(input: &str) -> Vec<Snail> {
    input.lines().map(Snail::from_str).collect()
}

fn calc_magnitude(arr: &Vec<Value>) -> u64 {
    fn calc(val: &Value, modifier: u64) -> u64 {
        match val {
            Value::Array(a) => Some(calc_magnitude(a)),
            Value::Number(x) => Some(x.as_u64().unwrap()),
            _ => None,
        }
        .unwrap()
            * modifier
    }

    calc(&arr[0], 3) + calc(&arr[1], 2)
}

pub fn part_one(input: &str) -> u64 {
    let snail_as_json = parse(input)
        .into_iter()
        .fold1(|acc, curr| Snail::add(&acc, &curr))
        .unwrap()
        .to_json();

    match snail_as_json {
        Value::Array(x) => calc_magnitude(&x),
        _ => unreachable!(),
    }
}

pub fn part_two(input: &str) -> u64 {
    let mut max_magnitude = 0;

    parse(input).iter().combinations(2).for_each(|snails| {
        let mut magnitude = |a: &Snail, b: &Snail| {
            match Snail::add(a, b).to_json() {
                Value::Array(x) => {
                    max_magnitude = max(calc_magnitude(&x), max_magnitude);
                }
                _ => unreachable!(),
            };
        };

        magnitude(snails[1], snails[0]);
        magnitude(snails[0], snails[1]);
    });

    max_magnitude
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 18);
        assert_eq!(part_one(&input), 4140);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 18);
        assert_eq!(part_two(&input), 3993);
    }
}
