//! Contains all logic related to dealing with structures which have enclosed areas.

use std::fmt::Display;

/// Indicates that something acts as a 2-dimensional enclosure.
/// There has to be some sort of edge which forms a loop.
pub trait Enclosure {
    /// The type of the index used in the rows and the columns.
    type Index: Copy + Display;

    /// The return type of `is_edge`, which is used as a parameter for `down_is_edge` and `up_is_edge`.
    type IsEdgeResult: Copy;

    /// Count the number of points that are confined within the enclosure.
    ///
    /// - `include_edge` - If `true`, then points on the edge are also counted as being confined within the enclosure.
    fn compute_enclosure_size(&self, include_edge: bool) -> usize {
        let mut count = 0;

        for row in self.row_indices() {
            let mut ups = 0usize;
            let mut downs = 0usize;
            for col in self.col_indices() {
                if let Some(reference) = self.is_edge(row, col) {
                    if include_edge {
                        count += 1;
                    }
                    if self.edge_goes_down(reference) {
                        downs += 1;
                    }
                    if self.edge_goes_up(reference) {
                        ups += 1;
                    }
                } else if ups % 2 == 1 && downs % 2 == 1 {
                    count += 1;
                }
            }
        }

        count
    }

    /// Return an iterator which loops over the row indices.
    fn row_indices(&self) -> Box<dyn Iterator<Item = Self::Index>>;

    /// Return an iterator which loops over the column indices.
    fn col_indices(&self) -> Box<dyn Iterator<Item = Self::Index>>;

    /// Check if a point `(row, col)` lies on the edge of the enclosure.
    fn is_edge(&self, row: Self::Index, col: Self::Index) -> Option<Self::IsEdgeResult>;

    /// Check if the edge at `(row, col)` goes down.
    fn edge_goes_down(&self, reference: Self::IsEdgeResult) -> bool;

    /// Check if the edge at `(row, col)` goes up.
    fn edge_goes_up(&self, reference: Self::IsEdgeResult) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::enclosure::Enclosure;

    const COLS: usize = 16;

    fn test_data() -> TestEnclosure {
        TestEnclosure {
            points: vec![
                0b0111100001111000,
                0b1100110001001000,
                0b1000011001101000,
                0b1100001100101000,
                0b0100000111101000,
                0b0100000000001000,
                0b0111111111111000,
            ],
        }
    }

    #[test]
    fn test_enclosure_with_edge() {
        assert_eq!(test_data().compute_enclosure_size(true), 76);
    }

    #[test]
    fn test_enclosure_without_edge() {
        assert_eq!(test_data().compute_enclosure_size(false), 30);
    }

    struct TestEnclosure {
        points: Vec<u16>,
    }

    impl Enclosure for TestEnclosure {
        type Index = usize;
        type IsEdgeResult = (usize, usize);

        fn row_indices(&self) -> Box<dyn Iterator<Item = usize>> {
            Box::new(0..self.points.len())
        }

        fn col_indices(&self) -> Box<dyn Iterator<Item = usize>> {
            Box::new(0..COLS)
        }

        fn is_edge(&self, row: usize, col: usize) -> Option<Self::IsEdgeResult> {
            let mask = 1 << col;
            if self.points[row] & mask == mask {
                Some((row, col))
            } else {
                None
            }
        }

        fn edge_goes_down(&self, (row, col): Self::IsEdgeResult) -> bool {
            row + 1 < self.points.len() && self.is_edge(row + 1, col).is_some()
        }

        fn edge_goes_up(&self, (row, col): Self::IsEdgeResult) -> bool {
            row > 0 && self.is_edge(row - 1, col).is_some()
        }
    }
}
