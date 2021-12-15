use std::fmt::Debug;

/// A point describes a location `x, y` in a grid with two axis.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point(pub usize, pub usize);

impl Point {
    /// Get a unique id for a point in a given grid.
    pub fn to_id(self, width: usize) -> usize {
        self.0 + width * self.1
    }

    /// Get a point from a unique id in a grid.
    pub fn from_id(id: usize, width: usize) -> Self {
        Point(id % width, id / width)
    }

    /// Get all neighbors for a point in a grid, respecting the boundaries of the input.
    pub fn neighbors(self, max_x: usize, max_y: usize, include_diagonals: bool) -> Vec<Point> {
        let mut neighbors: Vec<Point> = Vec::new();
        let Point(x, y) = self;

        let bound_top = y == 0;
        let bound_left = x == 0;

        let bound_bottom = y == max_y;
        let bound_right = x == max_x;

        if !bound_top {
            neighbors.push(Point(x, y - 1));

            if include_diagonals && !bound_left {
                neighbors.push(Point(x - 1, y - 1));
            }

            if include_diagonals && !bound_right {
                neighbors.push(Point(x + 1, y - 1));
            }
        }

        if !bound_bottom {
            neighbors.push(Point(x, y + 1));

            if include_diagonals && !bound_left {
                neighbors.push(Point(x - 1, y + 1));
            }

            if include_diagonals && !bound_right {
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
}
