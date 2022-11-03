extern crate rdcl_aoc_helpers;

use std::fs::File;
use std::io::{BufRead, BufReader};

use grid::Grid;
use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use crate::sea_cucumber::SeaCucumber;
use crate::sea_floor::SeaFloor;

mod sea_cucumber;
mod sea_floor;

fn main() {
    let args = get_args(&["<input file>"], 1);

    let file = File::open(&args[1]).or_exit_with(1);
    let lines = BufReader::new(file).lines();
    let mut sea_cucumbers: Grid<SeaCucumber> = SeaFloor::parse_input(lines).or_exit_with(1);

    let mut i = 0;
    while let Some(next_value) = sea_cucumbers.next() {
        sea_cucumbers = next_value;
        i += 1;
    }

    println!(
        "Step {} is the first step where no sea cucumbers move.",
        i + 1
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut sf = sea_floor();
        let mut i = 0;
        while let Some(n) = sf.next() {
            i += 1;
            sf = n;
        }
        assert_eq!(i, 57);
    }

    fn sea_floor() -> Grid<SeaCucumber> {
        let input = vec![
            Ok("v...>>.vv>".to_string()),
            Ok(".vv>>.vv..".to_string()),
            Ok(">>.>v>...v".to_string()),
            Ok(">>v>>.>.v.".to_string()),
            Ok("v>v.vv.v..".to_string()),
            Ok(">.>>..v...".to_string()),
            Ok(".vv..>.>v.".to_string()),
            Ok("v.v..>>v.v".to_string()),
            Ok("....v..v.>".to_string()),
        ];
        SeaFloor::parse_input(input.into_iter()).unwrap()
    }
}
