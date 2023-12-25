//! The solution for [advent of code 2023, day 25](https://adventofcode.com/2023/day/25)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::wiring::WiringDiagram;

mod wiring;

const CUTS: usize = 3;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 25")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let diagram = InputReader::from(args.input).parse::<WiringDiagram>();

    match diagram.find_bisection(CUTS) {
        Some(bisection) => {
            println!(
                "A bisection was found: {bisection:?}. The final answer is: {}",
                bisection.0 * bisection.1
            );
        }
        None => {
            eprintln!("Could not find a bisection");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> WiringDiagram {
        InputReader::from("./src/day25/test.txt").parse()
    }

    #[test]
    fn test_find_bisection() {
        let bisection = test_data().find_bisection(CUTS);
        assert!(bisection.is_some());
        let bisection = bisection.unwrap();
        assert_eq!(bisection.0 * bisection.1, 54);
    }
}
