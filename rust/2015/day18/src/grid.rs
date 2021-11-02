use std::fmt;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::MultilineFromStr;
use rdcl_aoc_helpers::parse_error;

#[derive(Debug, Clone)]
pub struct Grid {
    rows: usize,
    cols: usize,
    cells: Vec<bool>,
    pub with_stuck_corners: bool,
}

impl Grid {
    pub fn next(&self) -> Grid {
        let mut active_neighbour_count = vec![0; self.cells.len()];
        for (idx, &cell) in self.cells.iter().enumerate() {
            if cell {
                for neighbour in self.neighbours(idx) {
                    active_neighbour_count[neighbour] += 1;
                }
            }
        }

        let mut next_cells = vec![false; self.cells.len()];
        for (idx, &count) in active_neighbour_count.iter().enumerate() {
            next_cells[idx] = count == 3 || (self.cells[idx] && count == 2);
        }

        if self.with_stuck_corners {
            next_cells[0] = true;
            next_cells[self.cols - 1] = true;
            next_cells[self.cols * (self.rows - 1)] = true;
            next_cells[self.cols * self.rows - 1] = true;
        }

        Grid {
            rows: self.rows,
            cols: self.cols,
            cells: next_cells,
            with_stuck_corners: self.with_stuck_corners,
        }
    }

    pub fn count_active_cells(&self) -> usize {
        self.cells.iter().filter(|v| **v).count()
    }

    fn neighbours(&self, idx: usize) -> Vec<usize> {
        let mut neighbours = Vec::new();

        // left edge
        if idx % self.cols != 0 {
            let left = idx - 1;
            neighbours.push(left); // left

            if left >= self.cols {
                neighbours.push(left - self.cols); // left & up
            }

            if left + self.cols < self.cells.len() {
                neighbours.push(left + self.cols); // left & down
            }
        }

        // right edge
        if idx % self.cols != self.cols - 1 {
            let right = idx + 1;
            neighbours.push(right); // right

            if right >= self.cols {
                neighbours.push(right - self.cols); // right & up
            }

            if right + self.cols < self.cells.len() {
                neighbours.push(right + self.cols); // right & down
            }
        }

        if idx >= self.cols {
            neighbours.push(idx - self.cols); // up
        }

        if idx + self.cols < self.cells.len() {
            neighbours.push(idx + self.cols); // down
        }

        neighbours
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (idx, &cell) in self.cells.iter().enumerate() {
            if idx != 0 && idx % self.cols == 0 {
                writeln!(f)?;
            } else if idx != 0 {
                write!(f, " ")?;
            }
            write!(f, "{}", if cell { '#' } else { '.' })?;
        }
        Ok(())
    }
}

impl MultilineFromStr for Grid {
    type Err = ParseError;

    fn new() -> Self {
        Grid {
            rows: 0,
            cols: 0,
            cells: Vec::new(),
            with_stuck_corners: false,
        }
    }

    fn indicates_new_record(&self, _line: &str) -> bool {
        false
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        if self.cols == 0 {
            self.cols = line.len()
        } else if self.cols != line.len() {
            return Err(parse_error!("The lines have inconsistent width."));
        }
        self.rows += 1;

        for ch in line.chars() {
            self.cells.push(ch == '#');
        }

        Ok(())
    }
}

#[cfg(test)]
#[rustfmt::skip::macros(vec)]
mod tests {
    use super::*;

    const O: bool = false;
    const X: bool = true;

    #[test]
    fn test_without_stuck_corners() {
        let mut grid = Grid {
            cols: 6,
            rows: 6,
            cells: vec![
                O, X, O, X, O, X,
                O, O, O, X, X, O,
                X, O, O, O, O, X,
                O, O, X, O, O, O,
                X, O, X, O, O, X,
                X, X, X, X, O, O,
            ],
            with_stuck_corners: false,
        };

        assert_eq!(grid.count_active_cells(), 15);

        // after 1 step
        grid = grid.next();
        assert_eq!(
            grid.cells,
            vec![
                O, O, X, X, O, O,
                O, O, X, X, O, X,
                O, O, O, X, X, O,
                O, O, O, O, O, O,
                X, O, O, O, O, O,
                X, O, X, X, O, O,
            ]
        );
        assert_eq!(grid.count_active_cells(), 11);

        // after 2 steps
        grid = grid.next();
        assert_eq!(
            grid.cells,
            vec![
                O, O, X, X, X, O,
                O, O, O, O, O, O,
                O, O, X, X, X, O,
                O, O, O, O, O, O,
                O, X, O, O, O, O,
                O, X, O, O, O, O,
            ]
        );
        assert_eq!(grid.count_active_cells(), 8);

        // after 3 steps
        grid = grid.next();
        assert_eq!(
            grid.cells,
            vec![
                O, O, O, X, O, O,
                O, O, O, O, O, O,
                O, O, O, X, O, O,
                O, O, X, X, O, O,
                O, O, O, O, O, O,
                O, O, O, O, O, O,
            ]
        );
        assert_eq!(grid.count_active_cells(), 4);

        // after 4 steps
        grid = grid.next();
        assert_eq!(
            grid.cells,
            vec![
                O, O, O, O, O, O,
                O, O, O, O, O, O,
                O, O, X, X, O, O,
                O, O, X, X, O, O,
                O, O, O, O, O, O,
                O, O, O, O, O, O,
            ]
        );
        assert_eq!(grid.count_active_cells(), 4);
    }

    #[test]
    fn test_with_stuck_corners() {
        let mut grid = Grid {
            cols: 6,
            rows: 6,
            cells: vec![
                X, X, O, X, O, X,
                O, O, O, X, X, O,
                X, O, O, O, O, X,
                O, O, X, O, O, O,
                X, O, X, O, O, X,
                X, X, X, X, O, X,
            ],
            with_stuck_corners: true,
        };

        assert_eq!(grid.count_active_cells(), 17);

        // after 1 step
        grid = grid.next();
        assert_eq!(
            grid.cells,
            vec![
                X, O, X, X, O, X,
                X, X, X, X, O, X,
                O, O, O, X, X, O,
                O, O, O, O, O, O,
                X, O, O, O, X, O,
                X, O, X, X, X, X,
            ]
        );
        assert_eq!(grid.count_active_cells(), 18);

        // after 2 steps
        grid = grid.next();
        assert_eq!(
            grid.cells,
            vec![
                X, O, O, X, O, X,
                X, O, O, O, O, X,
                O, X, O, X, X, O,
                O, O, O, X, X, O,
                O, X, O, O, X, X,
                X, X, O, X, X, X,
            ]
        );
        assert_eq!(grid.count_active_cells(), 18);

        // after 3 steps
        grid = grid.next();
        assert_eq!(
            grid.cells,
            vec![
                X, O, O, O, X, X,
                X, X, X, X, O, X,
                O, O, X, X, O, X,
                O, O, O, O, O, O,
                X, X, O, O, O, O,
                X, X, X, X, O, X,
            ]
        );
        assert_eq!(grid.count_active_cells(), 18);

        // after 4 steps
        grid = grid.next();
        assert_eq!(
            grid.cells,
            vec![
                X, O, X, X, X, X,
                X, O, O, O, O, X,
                O, O, O, X, O, O,
                O, X, X, O, O, O,
                X, O, O, O, O, O,
                X, O, X, O, O, X,
            ]
        );
        assert_eq!(grid.count_active_cells(), 14);

        // after 5 steps
        grid = grid.next();
        assert_eq!(
            grid.cells,
            vec![
                X, X, O, X, X, X,
                O, X, X, O, O, X,
                O, X, X, O, O, O,
                O, X, X, O, O, O,
                X, O, X, O, O, O,
                X, X, O, O, O, X,
            ]
        );
        assert_eq!(grid.count_active_cells(), 17);
    }
}
