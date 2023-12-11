//! The solution for [advent of code 2023, day 11](https://adventofcode.com/2023/day/11)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::galaxy::GalaxyMap;

mod galaxy;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 11")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let map = InputReader::from(args.input).parse::<GalaxyMap>();
    println!(
        "Making empty space twice as big, the sum of the shortest paths is {}",
        map.sum_shortest_paths(2)
    );
    println!(
        "Making empty space a million times as big, the sum of the shortest paths is {}",
        map.sum_shortest_paths(1_000_000)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> GalaxyMap {
        InputReader::from("./src/day11/test.txt").parse()
    }

    #[test]
    fn test_sum_shortest_paths_after_1_expansion() {
        let map = test_data();
        assert_eq!(map.sum_shortest_paths(2), 374);
    }

    #[test]
    fn test_sum_shortest_paths_after_10_expansion() {
        let map = test_data();
        assert_eq!(map.sum_shortest_paths(10), 1030);
    }

    #[test]
    fn test_sum_shortest_paths_after_100_expansion() {
        let map = test_data();
        assert_eq!(map.sum_shortest_paths(100), 8410);
    }
}
