//! Determine the neighbours of a point in a `grid::Grid<T>`.

use grid::Grid;
use itertools::Itertools;

/// Allows easy fetching of neighbours of a given position.
pub trait WithNeighbours {
    /// Returns all neighbours of a given position.
    fn neighbours(&self, position: (usize, usize), include_diagonals: bool) -> Vec<(usize, usize)>;
}

impl<T> WithNeighbours for Grid<T> {
    fn neighbours(
        &self,
        (row, col): (usize, usize),
        include_diagonals: bool,
    ) -> Vec<(usize, usize)> {
        if include_diagonals {
            let row_range = (
                row.saturating_sub(1),
                (if row + 1 < self.rows() { row + 1 } else { row }),
            );
            let col_range = (
                col.saturating_sub(1),
                (if col + 1 < self.cols() { col + 1 } else { col }),
            );

            (row_range.0..=row_range.1)
                .cartesian_product(col_range.0..=col_range.1)
                .filter(|&p| p != (row, col))
                .collect()
        } else {
            let mut n = Vec::with_capacity(4);
            if row > 0 {
                n.push((row - 1, col));
            }
            if col > 0 {
                n.push((row, col - 1));
            }
            if row + 1 < self.rows() {
                n.push((row + 1, col));
            }
            if col + 1 < self.cols() {
                n.push((row, col + 1));
            }
            n
        }
    }
}

#[cfg(test)]
#[rustfmt::skip::macros(vec, assert_eq)]
mod tests {
    use super::*;
    use grid::grid;

    #[test]
    fn test_neighbours_excluding_diagonals() {
        let grid = grid![
            [1, 2, 3]
            [4, 5, 6]
            [7, 8, 9]
        ];
        let n = |row, col| grid.neighbours((row, col), false);

        assert_eq!(n(0, 0), vec![(1, 0), (0, 1)]);
        assert_eq!(n(0, 1), vec![(0, 0), (1, 1), (0, 2)]);
        assert_eq!(n(0, 2), vec![(0, 1), (1, 2)]);
        assert_eq!(n(1, 0), vec![(0, 0), (2, 0), (1, 1)]);
        assert_eq!(n(1, 1), vec![(0, 1), (1, 0), (2, 1), (1, 2)]);
        assert_eq!(n(1, 2), vec![(0, 2), (1, 1), (2, 2)]);
        assert_eq!(n(2, 0), vec![(1, 0), (2, 1)]);
        assert_eq!(n(2, 1), vec![(1, 1), (2, 0), (2, 2)]);
        assert_eq!(n(2, 2), vec![(1, 2), (2, 1)]);
    }

    #[test]
    fn test_neighbours_including_diagonals() {
        let grid = grid![
            [1, 2, 3]
            [4, 5, 6]
            [7, 8, 9]
        ];
        let n = |row, col| grid.neighbours((row, col), true);

        assert_eq!(n(0, 0), vec![(0, 1), (1, 0), (1, 1)]);
        assert_eq!(n(0, 1), vec![(0, 0), (0, 2), (1, 0), (1, 1), (1, 2)]);
        assert_eq!(n(0, 2), vec![(0, 1), (1, 1), (1, 2)]);
        assert_eq!(n(1, 0), vec![(0, 0), (0, 1), (1, 1), (2, 0), (2, 1)]);
        assert_eq!(n(1, 1), vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1), (2, 2)]);
        assert_eq!(n(1, 2), vec![(0, 1), (0, 2), (1, 1), (2, 1), (2, 2)]);
        assert_eq!(n(2, 0), vec![(1, 0), (1, 1), (2, 1)]);
        assert_eq!(n(2, 1), vec![(1, 0), (1, 1), (1, 2), (2, 0), (2, 2)]);
        assert_eq!(n(2, 2), vec![(1, 1), (1, 2), (2, 1)]);
    }
}
