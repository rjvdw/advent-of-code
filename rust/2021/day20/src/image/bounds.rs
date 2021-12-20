use std::fmt;
use std::ops::RangeInclusive;

use itertools::Itertools;

use crate::point::Point;

pub(crate) type RowColIterator = itertools::Product<RangeInclusive<i64>, RangeInclusive<i64>>;

#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) struct Bounds {
    pub(crate) top_left: Point,
    pub(crate) bottom_right: Point,
}

impl Bounds {
    fn new(top: i64, left: i64, bottom: i64, right: i64) -> Bounds {
        Bounds {
            top_left: Point::new(top, left),
            bottom_right: Point::new(bottom, right),
        }
    }

    pub(crate) fn update_with(&mut self, row: i64, col: i64) {
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

    pub(crate) fn join_with(&mut self, other: &Bounds) {
        self.update_with(other.top_left.row, other.top_left.col);
        self.update_with(other.bottom_right.row, other.bottom_right.col);
    }

    pub(crate) fn stretched(&self, by: i64) -> Bounds {
        Bounds::new(
            self.top_left.row - by,
            self.top_left.col - by,
            self.bottom_right.row + by,
            self.bottom_right.col + by,
        )
    }

    pub(crate) fn height(&self) -> usize {
        (self.bottom_right.row + 1 - self.top_left.row).abs() as usize
    }

    pub(crate) fn width(&self) -> usize {
        (self.bottom_right.col + 1 - self.top_left.col).abs() as usize
    }

    pub(crate) fn size(&self) -> usize {
        self.height() * self.width()
    }

    pub(crate) fn iter_row_col(&self) -> RowColIterator {
        let row_range = self.top_left.row..=self.bottom_right.row;
        let col_range = self.top_left.col..=self.bottom_right.col;
        row_range.cartesian_product(col_range)
    }

    pub(crate) fn contains(&self, point: &Point) -> bool {
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
        let mut bounds = Bounds::default();
        bounds.update_with(-5, 5);
        bounds.update_with(5, -5);
        assert_eq!(bounds.top_left, Point::new(-5, -5));
        assert_eq!(bounds.bottom_right, Point::new(5, 5));
    }

    #[test]
    fn test_join_with() {
        let mut bounds1 = Bounds::new(-2, -2, 3, 3);
        let bounds2 = Bounds::new(1, -3, 2, 5);
        bounds1.join_with(&bounds2);

        assert_eq!(bounds1.top_left, Point::new(-2, -3));
        assert_eq!(bounds1.bottom_right, Point::new(3, 5));
    }

    #[test]
    #[rustfmt::skip::macros(vec)]
    fn test_iter_row_col() {
        let bounds = Bounds::new(-1, -1, 1, 1);
        assert_eq!(
            bounds.iter_row_col().collect::<Vec<(i64, i64)>>(),
            vec![
                (-1, -1), (-1, 0), (-1, 1),
                (0, -1), (0, 0), (0, 1),
                (1, -1), (1, 0), (1, 1),
            ]
        );
    }

    #[test]
    fn test_contains() {
        let bounds = Bounds::new(-5, -5, 5, 5);

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
        let bounds = Bounds::new(-5, -5, 5, 5);
        assert_eq!(format!("{}", bounds), "[(-5, -5), (5, 5)]");
    }

    #[test]
    fn test_fmt_debug() {
        let bounds = Bounds::new(-5, -5, 5, 5);
        assert_eq!(format!("{:?}", bounds), "Bounds { top_left: Point { row: -5, col: -5 }, bottom_right: Point { row: 5, col: 5 } }");
    }
}
