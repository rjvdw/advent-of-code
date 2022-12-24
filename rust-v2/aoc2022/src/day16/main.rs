//! The solution for [advent of code 2022, day 16](https://adventofcode.com/2022/day/16)

extern crate core;

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::label::Label;
use crate::volcano::Volcano;

mod label;
mod tunnels;
mod valve;
mod volcano;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 16")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The starting point.
    #[clap(long, value_parser, default_value = "AA")]
    starting_point: Label,

    /// The time limit.
    #[clap(long, value_parser, default_value_t = 30)]
    time_limit: usize,

    /// How long does it take to train an elephant?
    #[clap(long, value_parser, default_value_t = 4)]
    training_time: usize,
}

fn main() {
    let args: Args = Args::parse();
    let volcano = InputReader::from(args.input).parse::<Volcano>();

    let max_relief = volcano.find_max_pressure_relief(args.starting_point, args.time_limit);
    println!(
        "The maximum pressure that can be relieved is {}",
        max_relief
    );

    let max_relief = volcano.find_max_pressure_relief_with_elephant(
        args.starting_point,
        args.time_limit - args.training_time,
    );
    println!(
        "If you were to train an elephant first, then the maximum pressure that can be relieved is {}",
        max_relief
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Volcano {
        InputReader::from("./src/day16/test.txt").parse()
    }

    #[test]
    fn test_find_max_pressure_relief() {
        assert_eq!(
            test_data().find_max_pressure_relief(Label('A', 'A'), 30),
            1651
        );
    }

    #[test]
    fn test_find_max_pressure_relief_with_elephant() {
        assert_eq!(
            test_data().find_max_pressure_relief_with_elephant(Label('A', 'A'), 26),
            1707
        );
    }
}
