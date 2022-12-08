use std::collections::HashSet;

use grid::Grid;

#[derive(Debug, Eq, PartialEq)]
pub struct Forest {
    heights: Grid<u8>,
}

impl Forest {
    /// Constructs a new forest given the heights of its trees.
    pub fn new(heights: Grid<u8>) -> Forest {
        Forest { heights }
    }

    /// Parses a forest from an input file.
    pub fn parse<T>(input: T) -> Forest
    where
        T: Iterator<Item = String>,
    {
        let mut grid: Grid<u8> = Grid::new(0, 0);

        for line in input {
            let row: Vec<u8> = line.bytes().map(|b| b - b'0').collect();

            grid.push_row(row);
        }

        Forest::new(grid)
    }

    /// Counts the number of trees that are visible from outside the forest.
    pub fn count_visible_trees(&self) -> usize {
        let mut visible_trees: HashSet<(usize, usize)> = HashSet::new();

        for row in 0..self.heights.rows() {
            self.count_visible(self.iter_row(row), &mut visible_trees);
            self.count_visible(self.iter_row(row).rev(), &mut visible_trees);
        }

        for col in 0..self.heights.cols() {
            self.count_visible(self.iter_col(col), &mut visible_trees);
            self.count_visible(self.iter_col(col).rev(), &mut visible_trees);
        }

        visible_trees.len()
    }

    /// Finds the highest scenic score in the forest.
    pub fn find_highest_scenic_score(&self) -> usize {
        let mut best = 0;

        for row in 1..self.heights.rows() - 1 {
            for col in 1..self.heights.cols() - 1 {
                let score = self.scenic_score(row, col);
                if score > best {
                    best = score;
                }
            }
        }

        best
    }

    /// Calculates the scenic score for a given position, by multiplying the view distance in all directions.
    pub fn scenic_score(&self, row: usize, col: usize) -> usize {
        if self.is_on_edge(row, col) {
            // tree is on the edge, its scenic score is 0 by definition
            0
        } else {
            let height = self.get(row, col);

            let trees_right = (row + 1..self.heights.rows()).map(|r| self.get(r, col));
            let trees_left = (0..row).rev().map(|r| self.get(r, col));
            let trees_bottom = (col + 1..self.heights.cols()).map(|c| self.get(row, c));
            let trees_top = (0..col).rev().map(|c| self.get(row, c));

            self.view_distance(height, trees_right)
                * self.view_distance(height, trees_left)
                * self.view_distance(height, trees_bottom)
                * self.view_distance(height, trees_top)
        }
    }

    /// Gets the height at a given coordinate.
    fn get(&self, row: usize, col: usize) -> u8 {
        self.heights.get(row, col).copied().unwrap()
    }

    /// Counts the number of trees that are visible in a given line.
    fn view_distance<T>(&self, height: u8, trees: T) -> usize
    where
        T: Iterator<Item = u8>,
    {
        let mut count = 0;

        for tree in trees {
            count += 1;
            if tree >= height {
                break;
            }
        }

        count
    }

    /// Returns true if a tree is on the edge of the forest.
    fn is_on_edge(&self, row: usize, col: usize) -> bool {
        col == 0 || row == 0 || col == self.heights.cols() - 1 || row == self.heights.rows() - 1
    }

    /// Returns an iterator over the trees in a given row.
    fn iter_row(
        &self,
        row: usize,
    ) -> impl Iterator<Item = ((usize, usize), u8)> + DoubleEndedIterator + '_ {
        self.heights
            .iter_row(row)
            .copied()
            .enumerate()
            .map(move |(x, v)| ((x, row), v))
    }

    /// Returns an iterator over the trees in a given column.
    fn iter_col(
        &self,
        col: usize,
    ) -> impl Iterator<Item = ((usize, usize), u8)> + DoubleEndedIterator + '_ {
        self.heights
            .iter_col(col)
            .copied()
            .enumerate()
            .map(move |(y, v)| ((col, y), v))
    }

    /// Counts the number of visible trees in a given line.
    fn count_visible<T>(&self, mut trees: T, visible_trees: &mut HashSet<(usize, usize)>)
    where
        T: Iterator<Item = ((usize, usize), u8)>,
    {
        if let Some((xy, mut current_height)) = trees.next() {
            visible_trees.insert(xy);
            for (xy, height) in trees {
                if height > current_height {
                    visible_trees.insert(xy);
                    current_height = height;
                }
            }
        }
    }
}
