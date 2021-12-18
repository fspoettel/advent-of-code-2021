use itertools::Itertools;
use std::cmp::max;

#[derive(Clone, Copy)]
enum Symbol {
    Open,
    Close,
    Comma,
    Num(u32),
}

impl Symbol {
    fn from_char(c: char) -> Self {
        match c {
            '[' => Symbol::Open,
            ']' => Symbol::Close,
            ',' => Symbol::Comma,
            c => Symbol::Num(c.to_digit(10).unwrap()),
        }
    }
}

type Snail = Vec<Symbol>;

fn add(a: &[Symbol], b: &[Symbol]) -> Snail {
    let mut snail: Snail = vec![Symbol::Open];
    snail.extend(a.iter().cloned());
    snail.push(Symbol::Comma);
    snail.extend(b.iter().cloned());
    snail.push(Symbol::Close);
    reduce(&mut snail);
    snail
}

fn from_str(s: &str) -> Snail {
    s.chars().map(Symbol::from_char).collect()
}

fn reduce(snail: &mut Snail) {
    while explode(snail) || split(snail) {}
}

fn split(snail: &mut Snail) -> bool {
    let pos = snail.iter().position(|s| match s {
        Symbol::Num(x) => *x > 9,
        _ => false,
    });

    match pos {
        None => false,
        Some(index) => {
            if let Symbol::Num(x) = snail[index] {
                let l = x / 2;
                let r = x - l;
                snail.splice(
                    index..index + 1,
                    [
                        Symbol::Open,
                        Symbol::Num(l),
                        Symbol::Comma,
                        Symbol::Num(r),
                        Symbol::Close,
                    ],
                );
                true
            } else {
                unreachable!()
            }
        }
    }
}

fn explode(snail: &mut Snail) -> bool {
    let mut depth = 0;

    let pos = snail.iter().position(|s| {
        match s {
            Symbol::Open => {
                depth += 1;
            }
            Symbol::Close => {
                depth -= 1;
            }
            _ => (),
        };

        depth == 5
    });

    match pos {
        None => false,
        Some(index) => {
            let l = match snail[index + 1] {
                Symbol::Num(x) => x,
                _ => unreachable!(),
            };

            let r = match snail[index + 3] {
                Symbol::Num(x) => x,
                _ => unreachable!(),
            };

            snail[..index].iter_mut().rev().find_map(|x| {
                if let Symbol::Num(x) = x {
                    *x += l;
                    Some(())
                } else {
                    None
                }
            });

            snail[index + 4..].iter_mut().find_map(|x| {
                if let Symbol::Num(x) = x {
                    *x += r;
                    Some(())
                } else {
                    None
                }
            });

            snail.splice(index..index + 5, [Symbol::Num(0)]);
            true
        }
    }
}

fn parse(input: &str) -> Vec<Snail> {
    input.lines().map(from_str).collect()
}

// this previously used a recursive function based on casting to json.
// found this on a random reddit thread and it is both faster and more elegant.
fn calc_magnitude(snail: &[Symbol]) -> u32 {
    let mut multiplier = 1;
    let mut output = 0;

    for symbol in snail {
        match symbol {
            Symbol::Close => multiplier /= 2,
            Symbol::Comma => multiplier = (multiplier / 3) * 2,
            Symbol::Num(x) => output += *x * multiplier,
            Symbol::Open => multiplier *= 3,
        }
    }

    output
}

pub fn part_one(input: &str) -> u32 {
    calc_magnitude(
        &parse(input)
            .into_iter()
            .fold1(|acc, curr| add(&acc, &curr))
            .unwrap(),
    )
}

pub fn part_two(input: &str) -> u32 {
    parse(input).iter().combinations(2).fold(0, |acc, snails| {
        max(
            acc,
            max(
                calc_magnitude(&add(snails[0], snails[1])),
                calc_magnitude(&add(snails[1], snails[0])),
            ),
        )
    })
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
