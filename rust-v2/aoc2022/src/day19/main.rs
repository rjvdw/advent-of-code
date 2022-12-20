//! The solution for [advent of code 2022, day 19](https://adventofcode.com/2022/day/19)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::blueprint::Blueprint;

mod blueprint;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 19")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The number of minutes to run the robots.
    #[clap(short, long, value_parser, default_value_t = 24)]
    minutes: usize,
}

fn main() {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);

    for line in input.read_lines() {
        println!("{:?}", line.parse::<Blueprint>().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = String> {
        InputReader::from("./src/day19/test.txt").read_lines()
    }
}
