use std::io;

use grid::{grid, Grid};
use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::grid::iterators::WithGridIterator;

use crate::sea_cucumber::SeaCucumber;

pub trait SeaFloor {
    fn next(&self) -> Option<Self>
    where
        Self: Sized;

    fn parse_input<I>(lines: I) -> Result<Self, ParseError>
    where
        Self: Sized,
        I: Iterator<Item = io::Result<String>>;
}

impl SeaFloor for Grid<SeaCucumber> {
    /// Determines the next generation of the sea floor.
    fn next(&self) -> Option<Self>
    where
        Self: Sized,
    {
        let rows = self.rows();
        let cols = self.cols();
        let mut next_value = Grid::new(rows, cols);
        let mut anyone_moved = false;

        for (y, x) in self.iter_row_col() {
            match self[y][x] {
                SeaCucumber::East => {
                    let nx = (x + 1) % cols;
                    if self[y][nx].is_empty() {
                        next_value[y][nx] = SeaCucumber::East;
                        anyone_moved = true;
                    } else {
                        next_value[y][x] = SeaCucumber::East;
                    }
                }
                SeaCucumber::South => {
                    let px = (x + cols - 1) % cols;
                    let nx = (x + 1) % cols;
                    let ny = (y + 1) % rows;

                    let target_is_free = self[ny][x].is_empty() && !self[ny][px].is_east();
                    let target_will_be_free = self[ny][x].is_east() && self[ny][nx].is_empty();

                    if target_is_free || target_will_be_free {
                        next_value[ny][x] = SeaCucumber::South;
                        anyone_moved = true;
                    } else {
                        next_value[y][x] = SeaCucumber::South;
                    }
                }
                SeaCucumber::Empty => {}
            }
        }

        if anyone_moved {
            Some(next_value)
        } else {
            None
        }
    }

    /// Parses the sea floor from the puzzle input.
    fn parse_input<I>(lines: I) -> Result<Self, ParseError>
    where
        Self: Sized,
        I: Iterator<Item = io::Result<String>>,
    {
        let mut grid = grid![];
        for line in lines {
            grid.push_row(
                line?
                    .chars()
                    .map(|ch| match ch {
                        '>' => SeaCucumber::East,
                        'v' => SeaCucumber::South,
                        _ => SeaCucumber::Empty,
                    })
                    .collect(),
            );
        }
        Ok(grid)
    }
}
