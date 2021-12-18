type Matrix = Vec<Vec<u32>>;

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: usize,
    y: usize,
    val: u32,
}

fn parse(input: &str) -> Matrix {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn surounding_points(matrix: &[Vec<u32>], p: &Point) -> [Point; 4] {
    let line = &matrix[p.y];
    // using an array istd. of a vec here achieves a 5x speedup by utilising the stack.
    // in this case, we need to return "bogus" values for points that would otherwise result in `-1`.
    // convention: we use index `99` and set value to `9` to mark it as an edge.
    [
        Point {
            x: if p.x > 0 { p.x - 1 } else { 99 },
            y: p.y,
            val: if p.x > 0 { line[p.x - 1] } else { 9 },
        },
        Point {
            x: p.x + 1,
            y: p.y,
            val: if p.x < line.len() - 1 {
                line[p.x + 1]
            } else {
                9
            },
        },
        Point {
            x: p.x,
            y: if p.y > 0 { p.y - 1 } else { 99 },
            val: if p.y > 0 { matrix[p.y - 1][p.x] } else { 9 },
        },
        Point {
            x: p.x,
            y: p.y + 1,
            val: if p.y < matrix.len() - 1 {
                matrix[p.y + 1][p.x]
            } else {
                9
            },
        },
    ]
}

fn is_minimum(matrix: &[Vec<u32>], p: &Point) -> bool {
    surounding_points(matrix, p).iter().all(|x| x.val > p.val)
}

fn get_minimums(matrix: &[Vec<u32>]) -> Vec<Point> {
    let mut minimums: Vec<Point> = Vec::new();

    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            let val = matrix[y][x];
            let p = Point { x, y, val };
            if is_minimum(matrix, &p) {
                minimums.push(p);
            }
        }
    }

    minimums
}

pub fn part_one(input: &str) -> u32 {
    let matrix = parse(input);
    get_minimums(&matrix).iter().map(|p| p.val + 1).sum()
}

fn flood_fill<'a>(matrix: &'a [Vec<u32>], p: &'a Point, basin: &'a mut Vec<Point>) {
    surounding_points(matrix, p)
        .iter()
        .filter(|x| x.val != 9 && x.val > p.val)
        .for_each(|x| {
            flood_fill(matrix, x, basin);
        });

    if !basin.contains(p) {
        basin.push(*p);
    }
}

pub fn part_two(input: &str) -> usize {
    let matrix = parse(input);

    let mut basins = get_minimums(&matrix)
        .iter()
        .map(|p| {
            let mut basin: Vec<Point> = Vec::new();
            flood_fill(&matrix, p, &mut basin);
            basin.len()
        })
        .collect::<Vec<usize>>();

    let len = basins.len();
    basins.sort_unstable();

    basins[len - 1] * basins[len - 2] * basins[len - 3]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        use aoc::read_file;
        let input = read_file("examples", 9);
        assert_eq!(part_one(&input), 15);
    }

    #[test]
    fn test_part_two() {
        use aoc::read_file;
        let input = read_file("examples", 9);
        assert_eq!(part_two(&input), 1134);
    }
}
