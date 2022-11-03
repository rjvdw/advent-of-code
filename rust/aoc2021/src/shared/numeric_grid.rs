//! Helpers methods for dealing with grids that consist entirely of numbers.
//!
//! It is assumed that no number will ever be larger than 9, so a single digit corresponds with a
//! single number.

use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use grid::{grid, Grid};
use rdcl_aoc_helpers::error::ParseError;

/// Reads a numeric grid from a file.
pub fn read(path: &str) -> Result<Grid<u8>, ParseError> {
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines();
    parse(lines)
}

/// Parses a numeric grid from an iterator over `io::Result<String>`.
pub fn parse<I>(mut lines: I) -> Result<Grid<u8>, ParseError>
where
    I: Iterator<Item = io::Result<String>>,
{
    let mut grid = grid![];
    for line in &mut lines {
        grid.push_row(
            line?
                .chars()
                .map(|ch| (ch as u8) - b'0')
                .collect::<Vec<u8>>(),
        );
    }
    Ok(grid)
}

/// Prints the numeric grid to the screen.
pub fn print(g: &Grid<u8>) {
    for row in 0..g.rows() {
        for col in 0..g.cols() {
            print!("{}", g[row][col]);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_numeric_grid() {
        let lines = vec![
            Ok("827273847".to_string()),
            Ok("019238560".to_string()),
            Ok("563402337".to_string()),
        ];

        let expected: Grid<u8> = grid![
            [8, 2, 7, 2, 7, 3, 8, 4, 7]
            [0, 1, 9, 2, 3, 8, 5, 6, 0]
            [5, 6, 3, 4, 0, 2, 3, 3, 7]
        ];

        assert_eq!(parse(lines.into_iter()), Ok(expected));
    }
}
