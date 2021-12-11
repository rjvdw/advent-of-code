//! Additional iterators for `grid::Grid<T>`.

use std::ops::Range;

use grid::Grid;
use itertools::Itertools;

pub type ProductRange = itertools::Product<Range<usize>, Range<usize>>;

/// Allows iterating over (row, col) values of the grid.
pub trait WithGridIterator<T> {
    /// Iterate over (row, col) values of the grid.
    fn iter_row_col(&self) -> ProductRange;
}

impl<T> WithGridIterator<T> for Grid<T> {
    fn iter_row_col(&self) -> ProductRange {
        (0..self.rows()).cartesian_product(0..self.cols())
    }
}

#[cfg(test)]
mod tests {
    use grid::grid;

    use super::*;

    #[test]
    fn test_iter_row_col() {
        let grid: Grid<u8> = grid![
            [1, 2, 3]
            [4, 5, 6]
            [7, 8, 9]
        ];
        let coords = grid.iter_row_col().collect::<Vec<(usize, usize)>>();
        assert_eq!(
            coords,
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 1),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2),
            ],
        );
    }
}
