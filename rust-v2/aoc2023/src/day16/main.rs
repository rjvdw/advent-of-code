//! The solution for [advent of code 2023, day 16](https://adventofcode.com/2023/day/16)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::contraption::Contraption;

mod contraption;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 16")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let contraption = InputReader::from(args.input).parse::<Contraption>();

    println!(
        "There are {} energized spaces",
        contraption.count_energized_spaces_from_top_left()
    );

    let optimal = contraption.find_optimal_entry_point();
    println!(
        "The optimal entrypoint is at ({}, {}), and will energize {} spaces",
        optimal.0 .0, optimal.0 .1, optimal.1
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Contraption {
        InputReader::from("./src/day16/test.txt").parse()
    }

    #[test]
    fn test_count_energized_spaces_from_top_left() {
        assert_eq!(test_data().count_energized_spaces_from_top_left(), 46);
    }

    #[test]
    fn test_find_optimal_entry_point() {
        assert_eq!(test_data().find_optimal_entry_point(), ((0, 3), 51));
    }
}
