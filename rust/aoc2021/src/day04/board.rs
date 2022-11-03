use std::collections::HashSet;
use std::num::ParseIntError;

use grid::{grid, Grid};
use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::grid::iterators::WithGridIterator;
use rdcl_aoc_helpers::input::MultilineFromStr;
use rdcl_aoc_helpers::parse_error;

#[derive(Debug, Clone)]
pub struct Board {
    numbers: Grid<u8>,
    marked: HashSet<(usize, usize)>,
}

impl Board {
    #[cfg(test)]
    pub fn new(grid: Grid<u8>) -> Board {
        Board {
            numbers: grid,
            marked: HashSet::new(),
        }
    }

    fn dim(&self) -> usize {
        self.numbers.rows()
    }

    pub fn mark(&mut self, nr: u8) {
        for (row, col) in self.numbers.iter_row_col() {
            if self.numbers[row][col] == nr {
                self.marked.insert((row, col));
            }
        }
    }

    pub fn reset(&mut self) {
        self.marked.clear();
    }

    pub fn score(&self) -> u32 {
        let mut score = 0;

        for (row, col) in self.numbers.iter_row_col() {
            if !self.marked.contains(&(row, col)) {
                score += self.numbers[row][col] as u32;
            }
        }

        score
    }

    pub fn bingo(&self) -> bool {
        for i in 0..self.dim() {
            let mut full_col = true;
            let mut full_row = true;

            for j in 0..self.dim() {
                if !self.marked.contains(&(i, j)) {
                    full_col = false;
                }
                if !self.marked.contains(&(j, i)) {
                    full_row = false;
                }
            }

            if full_col || full_row {
                return true;
            }
        }

        false
    }
}

impl Default for Board {
    fn default() -> Self {
        Board {
            numbers: grid![],
            marked: HashSet::new(),
        }
    }
}

impl MultilineFromStr for Board {
    type Err = ParseError;

    fn new() -> Self {
        Board::default()
    }

    fn indicates_new_record(&self, line: &str) -> bool {
        line.is_empty()
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        let nrs = line
            .split_whitespace()
            .map(str::parse::<u8>)
            .collect::<Result<Vec<u8>, ParseIntError>>()?;

        if self.numbers.cols() == 0 {
            for nr in nrs {
                self.numbers.push_col(vec![nr]);
            }
        } else {
            let mut row = Vec::with_capacity(self.numbers.cols());
            for nr in nrs {
                row.push(nr);
            }
            self.numbers.push_row(row);
        }

        if self.numbers.rows() > self.numbers.cols() {
            return Err(parse_error!("Bingo boards must be square!"));
        }

        Ok(())
    }
}
