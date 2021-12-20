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

    /// Divides the region specified by these bounds into (at most) `nr_regions` smaller regions. If
    /// the region is too small, there may be fewer than `nr_regions`. This function will never
    /// return _empty_ regions.
    ///
    /// Currently only divides the region in horizontal strips. This is suboptimal if the region
    /// isn't very high, but is very wide. This however is such a specific edge case that I cannot
    /// be bothered to deal with it...
    pub(crate) fn divide(&self, mut nr_regions: i64, min_size: i64) -> Vec<Bounds> {
        assert!(nr_regions > 0, "Invalid number of regions supplied.");

        let top = self.top_left.row;
        let bottom = self.bottom_right.row;
        let left = self.top_left.col;
        let right = self.bottom_right.col;

        let mut regions = vec![];
        let min_size = min_size / (bottom - top).abs();
        let mut i = top;
        while i <= bottom {
            let mut j = i + (bottom - i).abs() / nr_regions;

            // if the region is too small, adjust
            if j - i - 1 < min_size {
                j = i + min_size - 1;
            }

            // if there are too few points left to form a new region, add them to the current region
            if (bottom - j).abs() < min_size {
                j = bottom;
            }

            regions.push(Bounds {
                top_left: Point::new(i, left),
                bottom_right: Point::new(j, right),
            });
            nr_regions -= 1;
            i = j + 1;
        }
        regions
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
    fn test_divide_1() {
        let bounds = Bounds::new(1, 1, 100, 100);
        let regions = bounds.divide(10, 0);
        assert_eq!(
            regions,
            vec![
                Bounds::new(1, 1, 10, 100),
                Bounds::new(11, 1, 20, 100),
                Bounds::new(21, 1, 30, 100),
                Bounds::new(31, 1, 40, 100),
                Bounds::new(41, 1, 50, 100),
                Bounds::new(51, 1, 60, 100),
                Bounds::new(61, 1, 70, 100),
                Bounds::new(71, 1, 80, 100),
                Bounds::new(81, 1, 90, 100),
                Bounds::new(91, 1, 100, 100),
            ]
        );
    }

    #[test]
    fn test_divide_2() {
        let bounds = Bounds::new(1, 1, 50, 50);
        let regions = bounds.divide(10, 500);
        assert_eq!(
            regions,
            vec![
                Bounds::new(1, 1, 10, 50),
                Bounds::new(11, 1, 20, 50),
                Bounds::new(21, 1, 30, 50),
                Bounds::new(31, 1, 40, 50),
                Bounds::new(41, 1, 50, 50),
            ]
        );
    }

    #[test]
    fn test_divide_3() {
        let bounds = Bounds::new(1, 1, 51, 50);
        let regions = bounds.divide(10, 500);
        assert_eq!(
            regions,
            vec![
                Bounds::new(1, 1, 10, 50),
                Bounds::new(11, 1, 20, 50),
                Bounds::new(21, 1, 30, 50),
                Bounds::new(31, 1, 40, 50),
                Bounds::new(41, 1, 51, 50),
            ]
        );
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
