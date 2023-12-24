//! The solution for [advent of code 2023, day 24](https://adventofcode.com/2023/day/24)

use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::sky::Trajectory;

mod fraction;
mod sky;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 24")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// Lower bound for estimation.
    #[clap(long, short, value_parser, default_value_t = 200_000_000_000_000)]
    lower_bound: i64,

    /// Upper bound for estimation.
    #[clap(long, short, value_parser, default_value_t = 400_000_000_000_000)]
    upper_bound: i64,
}

fn main() {
    let args: Args = Args::parse();
    let hailstones = InputReader::from(args.input)
        .parse_lines(Trajectory::from_str)
        .collect::<Vec<_>>();
    let bounds = (args.lower_bound, args.upper_bound);

    println!(
        "There are an estimated {} possible intersections between hailstones",
        count_possible_intersections(&hailstones, bounds)
    );
}

fn count_possible_intersections(hailstones: &[Trajectory], bounds: (i64, i64)) -> usize {
    let mut count = 0;

    for i in 1..hailstones.len() {
        for j in 0..i {
            if hailstones[i].estimate_collision_between(hailstones[j], bounds) {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<Trajectory> {
        InputReader::from("./src/day24/test.txt")
            .parse_lines(Trajectory::from_str)
            .collect()
    }

    #[test]
    fn test_count_possible_intersections() {
        assert_eq!(count_possible_intersections(&test_data(), (7, 27)), 2);
    }
}
