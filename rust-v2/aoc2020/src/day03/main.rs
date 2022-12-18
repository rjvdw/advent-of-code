//! The solution for [advent of code 2020, day 3](https://adventofcode.com/2020/day/3)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::map::Map;
use crate::slope::Slope;

mod map;
mod slope;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2020, day 3")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// One or more slopes to evaluate (in the form "RIGHT/DOWN").
    #[clap(long, short, value_parser, required(true))]
    slope: Vec<Slope>,
}

fn main() {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);
    let map = Map::parse(input.read_lines());

    let mut product = 1;

    for &slope in &args.slope {
        let count = count_trees(&map, slope);
        product *= count;
        println!(
            "Following a slope of {} right, {} down, you will encounter {} trees",
            slope.0, slope.1, count
        );
    }

    if args.slope.len() > 1 {
        println!("The product of all these results is {}", product);
    }
}

fn count_trees(map: &Map, Slope(right, down): Slope) -> usize {
    let mut col = 0;
    let mut row = 0;
    let mut count = 0;

    while row < map.get_height() {
        if map.has_tree(row, col) {
            count += 1;
        }

        row += down;
        col = (col + right) % map.get_width();
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Map {
        Map::parse(InputReader::from("./src/day03/test.txt").read_lines())
    }

    #[test]
    fn test_slope_1_1() {
        assert_eq!(count_trees(&test_data(), Slope(1, 1)), 2);
    }

    #[test]
    fn test_slope_3_1() {
        assert_eq!(count_trees(&test_data(), Slope(3, 1)), 7);
    }

    #[test]
    fn test_slope_5_1() {
        assert_eq!(count_trees(&test_data(), Slope(5, 1)), 3);
    }

    #[test]
    fn test_slope_7_1() {
        assert_eq!(count_trees(&test_data(), Slope(7, 1)), 4);
    }

    #[test]
    fn test_slope_1_2() {
        assert_eq!(count_trees(&test_data(), Slope(1, 2)), 2);
    }
}
