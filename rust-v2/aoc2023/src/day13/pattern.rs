use std::fmt;

use grid::Grid;

use rdcl_aoc_core::input::VecFromInput;
use rdcl_aoc_core::{err_parse_error, ParseResult};

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum Field {
    #[default]
    Ash,
    Rock,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Reflection {
    Row(usize),
    Column(usize),
    None,
}

impl Reflection {
    pub fn summarize(self) -> Option<usize> {
        match self {
            Reflection::Row(v) => Some(100 * v),
            Reflection::Column(v) => Some(v),
            Reflection::None => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pattern: Grid<Field>,
}

impl Pattern {
    pub fn find_reflection(&self) -> Reflection {
        for row in 1..self.pattern.rows() {
            if self.reflects_at_row(row) {
                return Reflection::Row(row);
            }
        }

        for col in 1..self.pattern.cols() {
            if self.reflects_at_col(col) {
                return Reflection::Column(col);
            }
        }

        Reflection::None
    }

    pub fn find_smudge(&self) -> Reflection {
        for row in 1..self.pattern.rows() {
            if self.almost_reflects_at_row(row) {
                return Reflection::Row(row);
            }
        }

        for col in 1..self.pattern.cols() {
            if self.almost_reflects_at_col(col) {
                return Reflection::Column(col);
            }
        }

        Reflection::None
    }

    fn reflects_at_row(&self, row: usize) -> bool {
        let mut row1 = row;
        let mut row2 = row;

        while row1 > 0 && row2 < self.pattern.rows() {
            row1 -= 1;

            if self.compare_rows(row1, row2) != 0 {
                return false;
            }

            row2 += 1;
        }

        true
    }

    fn almost_reflects_at_row(&self, row: usize) -> bool {
        let mut row1 = row;
        let mut row2 = row;
        let mut smudge_found = false;

        while row1 > 0 && row2 < self.pattern.rows() {
            row1 -= 1;

            let diff = self.compare_rows(row1, row2);
            if diff == 1 && !smudge_found {
                smudge_found = true;
            } else if diff != 0 {
                return false;
            }

            row2 += 1;
        }

        smudge_found
    }

    fn compare_rows(&self, row1: usize, row2: usize) -> usize {
        (0..self.pattern.cols())
            .filter(|&col| self.pattern[(row1, col)] != self.pattern[(row2, col)])
            .count()
    }

    fn reflects_at_col(&self, col: usize) -> bool {
        let mut col1 = col;
        let mut col2 = col;

        while col1 > 0 && col2 < self.pattern.cols() {
            col1 -= 1;

            if self.compare_cols(col1, col2) != 0 {
                return false;
            }

            col2 += 1;
        }

        true
    }

    fn almost_reflects_at_col(&self, col: usize) -> bool {
        let mut col1 = col;
        let mut col2 = col;
        let mut smudge_found = false;

        while col1 > 0 && col2 < self.pattern.cols() {
            col1 -= 1;

            let diff = self.compare_cols(col1, col2);
            if diff == 1 && !smudge_found {
                smudge_found = true;
            } else if diff != 0 {
                return false;
            }

            col2 += 1;
        }

        smudge_found
    }

    fn compare_cols(&self, col1: usize, col2: usize) -> usize {
        (0..self.pattern.rows())
            .filter(|&row| self.pattern[(row, col1)] != self.pattern[(row, col2)])
            .count()
    }
}

impl VecFromInput for Pattern {
    fn parse<T>(input: T) -> ParseResult<Vec<Self>>
    where
        T: Iterator<Item = String>,
    {
        let mut result = vec![];
        let mut pattern = Pattern::default();

        for line in input {
            if line.is_empty() {
                if !pattern.pattern.is_empty() {
                    result.push(pattern);
                }
                pattern = Pattern::default();
            } else {
                let mut row = vec![];
                for ch in line.chars() {
                    row.push(match ch {
                        '.' => Field::Ash,
                        '#' => Field::Rock,
                        _ => {
                            return err_parse_error!();
                        }
                    });
                }
                pattern.pattern.push_row(row);
            }
        }
        if !pattern.pattern.is_empty() {
            result.push(pattern);
        }

        Ok(result)
    }
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.pattern.rows() {
            if row != 0 {
                writeln!(f)?;
            }
            for col in 0..self.pattern.cols() {
                match self.pattern[(row, col)] {
                    Field::Ash => write!(f, ".")?,
                    Field::Rock => write!(f, "#")?,
                }
            }
        }

        Ok(())
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Pattern {
            pattern: Grid::new(0, 0),
        }
    }
}
