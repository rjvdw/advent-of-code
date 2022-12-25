//! The solution for [advent of code 2022, day 25](https://adventofcode.com/2022/day/25)

use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::snafu::Snafu;

mod snafu;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 25")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let sum = InputReader::from(args.input)
        .parse_lines(Snafu::from_str)
        .sum::<Snafu>();

    println!("Enter the number {}", sum);
}
