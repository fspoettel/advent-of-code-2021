use std::collections::{HashMap, HashSet};

static BOARD_SIZE: usize = 5;

// boards are stored as a hash_map. this facilitates the removal of winning boards in the game loop.
struct Board {
    // holds unique numbers on this board in a set. this makes summing uncrossed numbers easy and efficient.
    nums: HashSet<u32>,
    // holds both `horizontal` and `vertical` rows of ths board.
    // optimization: pre-calculate a 'transposed' set of rows & columns once in the beginning.
    // optimization: use signed integers for keys (- for column ids) to avoid expensive string operations.
    rows: HashMap<i32, Vec<u32>>,
}

type Boards = HashMap<usize, Board>;

pub fn str_to_u32(s: &str) -> u32 {
    s.trim().parse().unwrap()
}

fn to_draw(line: &str) -> Vec<u32> {
    line.split(',').map(str_to_u32).collect()
}

fn to_board(lines: &[&str]) -> Board {
    lines.iter().enumerate().fold(
        Board {
            nums: HashSet::new(),
            rows: HashMap::new(),
        },
        |mut board, (_row, l)| {
            l.trim()
                .split_whitespace()
                .enumerate()
                .for_each(|(_col, s)| {
                    let parsed: u32 = str_to_u32(s);
                    let col: i32 = _col as i32;
                    let row: i32 = _row as i32;
                    board.nums.insert(parsed);
                    board.rows.entry(-(col + 1)).or_default().push(parsed);
                    board.rows.entry(row + 1).or_default().push(parsed);
                });

            board
        },
    )
}

fn to_boards<'a>(lines: impl Iterator<Item = &'a str>) -> Boards {
    let mut boards = HashMap::new();

    lines
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>()
        .chunks(BOARD_SIZE)
        .map(to_board)
        .enumerate()
        .for_each(|(i, b)| {
            boards.insert(i, b);
        });

    boards
}

fn find_winners(draw: &[u32], boards: &Boards, i: usize) -> Vec<(usize, u32)> {
    let current_draw = &draw[i - 1];
    let draws = &draw[..i];

    boards
        .iter()
        .filter(|(_, b)| {
            // optimization: skip board processing if it does not contain the drawn number.
            b.nums.contains(current_draw)
            // check if any row consists of crossed numbers only.
            && b.rows.values().any(|v| v.iter().all(|n| draws.contains(n)))
        })
        .map(|(key, b)| {
            let uncrossed_nums: u32 = b.nums.iter().filter(|n| !draws.contains(n)).sum();
            (*key, current_draw * uncrossed_nums)
        })
        .collect()
}

fn find_first_winner(draw: &[u32], boards: &mut Boards, i: usize) -> u32 {
    let winners = find_winners(draw, boards, i);

    if winners.is_empty() {
        find_first_winner(draw, boards, i + 1)
    } else {
        let (_, score) = winners.first().unwrap();
        *score
    }
}

fn find_last_winner(draw: &[u32], boards: &mut Boards) -> u32 {
    let mut winners: Vec<u32> = Vec::new();

    for i in BOARD_SIZE..draw.len() {
        find_winners(draw, boards, i)
            .iter()
            .for_each(|(key, score)| {
                boards.remove(key);
                winners.push(*score);
            });
    }

    *winners.last().unwrap()
}

pub fn part_one(input: &str) -> u32 {
    let mut lines = input.lines();
    let draw: Vec<u32> = to_draw(lines.next().unwrap());
    let mut boards = to_boards(lines);

    find_first_winner(&draw, &mut boards, BOARD_SIZE)
}

pub fn part_two(input: &str) -> u32 {
    let mut lines = input.lines();
    let draw: Vec<u32> = to_draw(lines.next().unwrap());
    let mut boards = to_boards(lines);

    find_last_winner(&draw, &mut boards)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 4);
        assert_eq!(part_one(&input), 4512);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 4);
        assert_eq!(part_two(&input), 1924);
    }
}
