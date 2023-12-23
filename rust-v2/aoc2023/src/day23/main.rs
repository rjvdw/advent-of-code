//! The solution for [advent of code 2023, day 23](https://adventofcode.com/2023/day/23)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::hiking::Trail;

mod hiking;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 23")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let trail = InputReader::from(args.input).parse::<Trail>();

    println!(
        "Assuming slippy slopes, the longest hike possible crosses {} tiles",
        trail.find_longest_hike(true)
    );

    println!(
        "Assuming non-slippy slopes, the longest hike possible crosses {} tiles",
        trail.find_longest_hike(false)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Trail {
        InputReader::from("./src/day23/test.txt").parse()
    }

    #[test]
    fn test_find_longest_hike_with_slippy_slopes() {
        assert_eq!(test_data().find_longest_hike(true), 94);
    }

    #[test]
    fn test_find_longest_hike_with_non_slippy_slopes() {
        assert_eq!(test_data().find_longest_hike(false), 154);
    }
}
