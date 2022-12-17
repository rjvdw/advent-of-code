//! The solution for [advent of code 2022, day 17](https://adventofcode.com/2022/day/17)

use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::chamber::Chamber;

mod chamber;
mod point;
mod rock;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 17")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The amount of rocks that need to fall to convince the elephants.
    #[clap(long, value_parser, default_value_t = 2022)]
    prove_after: usize,

    /// The number of units rocks are to the left wall when they start falling.
    #[clap(long, value_parser, default_value_t = 2)]
    left: usize,

    /// The number of units rocks are above the last rock when they start falling.
    #[clap(long, value_parser, default_value_t = 3)]
    bottom: usize,
}

fn main() {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input).read_line();
    let chamber = Chamber::new(args.left, args.bottom);

    println!(
        "After {} rocks have fallen, the tower has a height of {}",
        args.prove_after,
        simulate(chamber, input, args.prove_after)
    );
}

fn simulate(mut chamber: Chamber, input: String, mut rock_count: usize) -> usize {
    let mut wind_iter = input.chars();
    let mut seen: HashMap<String, (usize, usize)> = HashMap::new();
    let mut extra_height = 0;

    while chamber.fallen_rocks_count() < rock_count {
        let wind = match wind_iter.next() {
            Some(wind) => wind,
            None => {
                if extra_height == 0 {
                    // check if we are looping
                    let summary = chamber.summarize();
                    if let Some((prev_height, prev_rocks)) = seen.get(&summary) {
                        // loop detected
                        let period = chamber.fallen_rocks_count() - prev_rocks;
                        let remaining_rocks = rock_count - chamber.fallen_rocks_count();
                        let d_height = chamber.height() - prev_height;

                        extra_height = (remaining_rocks / period) * d_height;
                        rock_count = chamber.fallen_rocks_count() + remaining_rocks % period;
                    }
                    seen.insert(summary, (chamber.height(), chamber.fallen_rocks_count()));
                }

                // reset the iterator
                wind_iter = input.chars();
                wind_iter.next().unwrap()
            }
        };

        chamber.step(wind);
    }

    extra_height + chamber.height()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> String {
        InputReader::from("./src/day17/test.txt").read_line()
    }

    #[test]
    fn test_simulate_2022() {
        let chamber = Chamber::new(2, 3);
        assert_eq!(simulate(chamber, test_data(), 2022), 3068);
    }

    #[test]
    fn test_simulate_1_000_000_000_000() {
        let chamber = Chamber::new(2, 3);
        assert_eq!(
            simulate(chamber, test_data(), 1_000_000_000_000),
            1_514_285_714_288
        );
    }
}
