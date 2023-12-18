//! The solution for [advent of code 2023, day 18](https://adventofcode.com/2023/day/18)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::lagoons::Lagoons;

mod direction;
mod lagoon;
mod lagoons;
mod point;
mod section;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 18")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let Lagoons { simple, correct } = InputReader::from(args.input).parse::<Lagoons>();

    println!("The lagoon has size {}", simple.size());
    println!(
        "Interpreting the hex codes correctly, the lagoon has size {}",
        correct.size()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Lagoons {
        InputReader::from("./src/day18/test.txt").parse()
    }

    #[test]
    fn test_lagoon_size() {
        assert_eq!(test_data().simple.size(), 62);
    }

    #[test]
    fn test_lagoon_corrected_size() {
        assert_eq!(test_data().correct.size(), 952_408_144_115);
    }
}
