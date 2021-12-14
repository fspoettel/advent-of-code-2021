use std::env;
use std::fmt::Debug;
use std::fs;

use num_traits::NumCast;
use num_traits::PrimInt;

pub fn read_file(folder: &str, day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd
        .join("src")
        .join(folder)
        .join(format!("day{:02}.txt", day));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

/// get a vector's median value.
/// the median is the value separating the higher half from the lower half of a data sample.
/// [Wikipedia](https://en.wikipedia.org/wiki/Median)
pub fn median<T: PrimInt + Debug>(vec: &mut Vec<T>) -> T {
    let len = vec.len();
    let mid = len / 2;

    vec.sort_unstable();

    if len % 2 == 0 {
        (vec[mid - 1] + vec[mid]) / NumCast::from(2).unwrap()
    } else {
        vec[mid]
    }
}

/// sum a sequence of integers. (e.g. `1, 2, 3, 4`)
/// [Wikipedia](https://en.wikipedia.org/wiki/Triangular_number)
pub fn nth_triangular<T: PrimInt>(a: T) -> T {
    a * (a + NumCast::from(1).unwrap()) / NumCast::from(2).unwrap()
}

// Grid Helpers

/// A point describes a location `x,y` in a grid with two axis.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point(pub usize, pub usize);

/// Get all neighbors for a point in a grid, respecting the boundaries of the input.
pub fn neighbors(point: Point, max_x: usize, max_y: usize) -> Vec<Point> {
    let mut neighbors: Vec<Point> = Vec::new();
    let Point(x, y) = point;

    let bound_top = y == 0;
    let bound_left = x == 0;

    let bound_bottom = y == max_y;
    let bound_right = x == max_x;

    if !bound_top {
        neighbors.push(Point(x, y - 1));

        if !bound_left {
            neighbors.push(Point(x - 1, y - 1));
        }

        if !bound_right {
            neighbors.push(Point(x + 1, y - 1));
        }
    }

    if !bound_bottom {
        neighbors.push(Point(x, y + 1));

        if !bound_left {
            neighbors.push(Point(x - 1, y + 1));
        }

        if !bound_right {
            neighbors.push(Point(x + 1, y + 1));
        }
    }

    if !bound_left {
        neighbors.push(Point(x - 1, y));
    }

    if !bound_right {
        neighbors.push(Point(x + 1, y));
    }

    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_median() {
        assert_eq!(median(&mut [1, 4, 7].to_vec()), 4);
        assert_eq!(median(&mut [3, 10, 36, 255, 79, 24, 5, 8].to_vec()), 17);
    }

    #[test]
    fn test_nth_triangular() {
        assert_eq!(nth_triangular(7), 28);
    }
}
