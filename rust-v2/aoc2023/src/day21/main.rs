//! The solution for [advent of code 2023, day 21](https://adventofcode.com/2023/day/21)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::garden::Garden;

mod garden;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 21")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The number of steps the Elf wants to take.
    #[clap(long, short, value_parser, default_value_t = 64)]
    steps: usize,

    /// No, sorry, the _actual_ number of steps the Elf wants to take.
    #[clap(long, short, value_parser, default_value_t = 26501365)]
    actual_steps: usize,
}

fn main() {
    let args: Args = Args::parse();
    let garden = InputReader::from(args.input).parse::<Garden>();

    println!(
        "There are {} plots reachable in {} steps",
        garden.start_walking(args.steps, false),
        args.steps,
    );

    println!(
        "Using an infinite garden, there are {} plots reachable in {} steps",
        garden.start_walking(args.actual_steps, true),
        args.actual_steps,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Garden {
        InputReader::from("./src/day21/test.txt").parse()
    }

    #[test]
    fn test_start_walking() {
        assert_eq!(test_data().start_walking(6, false), 16);
        assert_eq!(test_data().start_walking(6, true), 16);
        assert_eq!(test_data().start_walking(10, true), 50);
        assert_eq!(test_data().start_walking(50, true), 1594);
        assert_eq!(test_data().start_walking(100, true), 6536);
        // assert_eq!(test_data().start_walking(500, true), 167004);
        // assert_eq!(test_data().start_walking(1000, true), 668697);
        // assert_eq!(test_data().start_walking(5000, true), 16733044);
    }
}
