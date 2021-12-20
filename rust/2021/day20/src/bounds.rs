use std::fmt;
use std::ops::RangeInclusive;

use itertools::Itertools;

use crate::point::Point;

pub type RowColIterator = itertools::Product<RangeInclusive<i64>, RangeInclusive<i64>>;

#[derive(Copy, Clone)]
pub struct Bounds {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Bounds {
    pub fn update_with(&mut self, row: i64, col: i64) {
        if row < self.top_left.row {
            self.top_left.row = row;
        }
        if row > self.bottom_right.row {
            self.bottom_right.row = row;
        }
        if col < self.top_left.col {
            self.top_left.col = col;
        }
        if col > self.bottom_right.col {
            self.bottom_right.col = col;
        }
    }

    pub fn iter_row_col(&self) -> RowColIterator {
        let row_range = (self.top_left.row - 1)..=(self.bottom_right.row + 1);
        let col_range = (self.top_left.col - 1)..=(self.bottom_right.col + 1);
        row_range.cartesian_product(col_range)
    }

    pub fn contains(&self, point: &Point) -> bool {
        point.row >= self.top_left.row
            && point.row <= self.bottom_right.row
            && point.col >= self.top_left.col
            && point.col <= self.bottom_right.col
    }
}

impl fmt::Display for Bounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.top_left, self.bottom_right)
    }
}

impl fmt::Debug for Bounds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Bounds {{ top_left: {:?}, bottom_right: {:?} }}",
            self.top_left, self.bottom_right
        )
    }
}

impl Default for Bounds {
    fn default() -> Self {
        Bounds {
            top_left: Point {
                row: i64::MAX,
                col: i64::MAX,
            },
            bottom_right: Point {
                row: i64::MIN,
                col: i64::MIN,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_with() {
        let mut bound = Bounds::default();
        bound.update_with(-5, 5);
        bound.update_with(5, -5);
        assert_eq!(bound.top_left, Point::new(-5, -5));
        assert_eq!(bound.bottom_right, Point::new(5, 5));
    }

    #[test]
    #[rustfmt::skip::macros(vec)]
    fn test_iter_row_col() {
        let bound = Bounds {
            top_left: Point::new(-1, -1),
            bottom_right: Point::new(1, 1),
        };
        assert_eq!(
            bound.iter_row_col().collect::<Vec<(i64, i64)>>(),
            vec![
                (-2, -2), (-2, -1), (-2, 0), (-2, 1), (-2, 2),
                (-1, -2), (-1, -1), (-1, 0), (-1, 1), (-1, 2),
                (0, -2), (0, -1), (0, 0), (0, 1), (0, 2),
                (1, -2), (1, -1), (1, 0), (1, 1), (1, 2),
                (2, -2), (2, -1), (2, 0), (2, 1), (2, 2),
            ]
        );
    }

    #[test]
    fn test_contains() {
        let bounds = Bounds {
            top_left: Point::new(-5, -5),
            bottom_right: Point::new(5, 5),
        };

        assert!(bounds.contains(&Point::new(0, 0)));
        assert!(bounds.contains(&Point::new(5, 5)));
        assert!(bounds.contains(&Point::new(5, -5)));
        assert!(bounds.contains(&Point::new(-5, 5)));
        assert!(bounds.contains(&Point::new(-5, -5)));
        assert!(!bounds.contains(&Point::new(-6, -5)));
        assert!(!bounds.contains(&Point::new(-5, -6)));
        assert!(!bounds.contains(&Point::new(6, 5)));
        assert!(!bounds.contains(&Point::new(5, 6)));
    }

    #[test]
    fn test_fmt_display() {
        let bounds = Bounds {
            top_left: Point::new(-5, -5),
            bottom_right: Point::new(5, 5),
        };
        assert_eq!(format!("{}", bounds), "[(-5, -5), (5, 5)]");
    }

    #[test]
    fn test_fmt_debug() {
        let bounds = Bounds {
            top_left: Point::new(-5, -5),
            bottom_right: Point::new(5, 5),
        };
        assert_eq!(format!("{:?}", bounds), "Bounds { top_left: Point { row: -5, col: -5 }, bottom_right: Point { row: 5, col: 5 } }");
    }
}
