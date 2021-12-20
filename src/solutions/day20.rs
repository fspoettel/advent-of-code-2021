type Pixels = Vec<bool>;
type Grid = Vec<Pixels>;

pub fn arr_to_int(bits: &[bool]) -> usize {
    bits.iter().fold(0, |acc, &b| acc * 2 + (b as usize))
}

fn to_pixels(s: &str) -> Pixels {
    s.chars().map(|c| c == '#').collect()
}

fn parse(input: &str) -> (Pixels, Grid) {
    let mut lines = input.lines();
    let cipher = to_pixels(lines.next().unwrap());
    let grid = lines.filter(|l| !l.is_empty()).map(to_pixels).collect();

    (cipher, grid)
}

fn pad(grid: &mut Vec<Pixels>, state: bool) {
    let empty_line = vec![state; grid.len()];
    grid.insert(0, empty_line.clone());
    grid.push(empty_line);

    for row in grid.iter_mut() {
        row.insert(0, state);
        row.push(state);
    }
}

fn expand(grid: &mut Vec<Pixels>, cipher: &[bool], state: bool) -> bool {
    pad(grid, state);

    let w = grid[0].len();
    let h = grid.len();

    let mut next_grid = grid.clone();

    for x in 0..w {
        for y in 0..h {
            let t = y == 0;
            let l = x == 0;
            let r = x == w - 1;
            let b = y == h - 1;
            let id = [
                if !t && !l { grid[y - 1][x - 1] } else { state },
                if !t { grid[y - 1][x] } else { state },
                if !t && !r { grid[y - 1][x + 1] } else { state },
                if !l { grid[y][x - 1] } else { state },
                grid[y][x],
                if !r { grid[y][x + 1] } else { state },
                if !b && !l { grid[y + 1][x - 1] } else { state },
                if !b { grid[y + 1][x] } else { state },
                if !b && !r { grid[y + 1][x + 1] } else { state },
            ];

            next_grid[y][x] = cipher[arr_to_int(&id)];
        }
    }

    *grid = next_grid;
    cipher[arr_to_int(&[state; 9])]
}

fn expand_times(input: &str, times: u32) -> Vec<Pixels> {
    let (cipher, mut grid) = parse(input);
    let mut state = false;

    for _ in 0..times {
        state = expand(&mut grid, &cipher, state);
    }

    grid
}

fn count(arr: &[Pixels]) -> usize {
    arr.iter().flatten().filter(|&&x| x).count()
}

pub fn part_one(input: &str) -> usize {
    count(&expand_times(input, 2))
}

pub fn part_two(input: &str) -> usize {
    count(&expand_times(input, 50))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 20);
        assert_eq!(part_one(&input), 35);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 20);
        assert_eq!(part_two(&input), 3351);
    }
}
