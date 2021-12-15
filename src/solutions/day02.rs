struct Instruction<'a> {
    direction: &'a str,
    value: i32,
}

struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

fn to_instruction(line: &str) -> Instruction {
    let (direction, _value) = line.split_once(' ').unwrap();

    Instruction {
        direction,
        value: _value.parse().unwrap(),
    }
}

fn update_position(pos: Position, Instruction { direction, value }: Instruction) -> Position {
    match direction {
        "forward" => Position {
            x: pos.x + value,
            y: pos.y + pos.aim * value,
            ..pos
        },
        "down" => Position {
            aim: pos.aim + value,
            ..pos
        },
        "up" => Position {
            aim: pos.aim - value,
            ..pos
        },
        val => panic!("bad direction input: {}", val),
    }
}

pub fn part_one(input: &str) -> i32 {
    let pos = input
        .lines()
        .map(to_instruction)
        .fold(Position { x: 0, y: 0, aim: 0 }, update_position);

    // optimization: `aim` in part two mirrors `depth` in part one which allows us to reuse the positioning logic.
    pos.x * pos.aim
}

pub fn part_two(input: &str) -> i32 {
    let pos = input
        .lines()
        .map(to_instruction)
        .fold(Position { x: 0, y: 0, aim: 0 }, update_position);

    pos.x * pos.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_one(&input), 150);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 2);
        assert_eq!(part_two(&input), 900);
    }
}
