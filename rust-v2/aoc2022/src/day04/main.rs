//! The solution for [advent of code 2022, day 4](https://adventofcode.com/2022/day/4)

use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::assignment_pair::AssignmentPair;

mod assignment;
mod assignment_pair;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 4")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input).parse_lines(AssignmentPair::from_str);
    let (nr_contains, nr_overlaps) = count(input);

    println!(
        "There are {} assignment pairs that fully overlap",
        nr_contains
    );

    println!(
        "There are {} assignment pairs that partially overlap",
        nr_overlaps
    );
}

fn count<T>(assignment_pairs: T) -> (usize, usize)
where
    T: Iterator<Item = AssignmentPair>,
{
    assignment_pairs
        .map(|ap| (ap.contains(), ap.overlaps()))
        .map(|(contains, overlaps)| (usize::from(contains), usize::from(overlaps)))
        .fold((0, 0), |acc, r| (acc.0 + r.0, acc.1 + r.1))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = AssignmentPair> {
        InputReader::from("./src/day04/test.txt").parse_lines(AssignmentPair::from_str)
    }

    #[test]
    fn test_count() {
        assert_eq!(count(test_data()), (2, 4));
    }
}
