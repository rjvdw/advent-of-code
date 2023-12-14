//! The solution for [advent of code 2023, day 14](https://adventofcode.com/2023/day/14)

use std::collections::HashMap;
use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;

use crate::platform::Platform;

mod platform;

const CYCLE_COUNT: usize = 1_000_000_000;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 14")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let platform = InputReader::from(args.input).parse::<Platform>();

    println!(
        "After tilting the platform north, the load is {}",
        platform.tilt_north().compute_load()
    );
    let platform = cycle(platform, CYCLE_COUNT);
    println!(
        "After completing {CYCLE_COUNT} cycles, the load is {}",
        platform.compute_load()
    );
}

fn cycle(mut platform: Platform, count: usize) -> Platform {
    let mut seen: HashMap<String, usize> = HashMap::new();

    let mut i = 0;
    while i < count {
        platform = platform.cycle();
        let key = format!("{platform}");

        if let Some(v) = seen.get(&key) {
            // loop detected
            println!(
                "after {i} iterations, the platform is identical as it was after {v} iterations"
            );
            let modulus = i - v;
            i = count - ((count - v) % modulus);
            seen.clear();
        }

        seen.insert(key, i);
        i += 1;
    }

    platform
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Platform {
        InputReader::from("./src/day14/test.txt").parse()
    }

    #[test]
    fn test_tilt_north() {
        let platform = test_data().tilt_north();
        assert_eq!(platform.compute_load(), 136);
    }

    #[test]
    fn test_platform_cycle() {
        let platform = test_data().cycle();
        assert_eq!(platform.compute_load(), 87);

        let platform = platform.cycle();
        assert_eq!(platform.compute_load(), 69);

        let platform = platform.cycle();
        assert_eq!(platform.compute_load(), 69);
    }

    #[test]
    fn test_cycle() {
        let platform = cycle(test_data(), CYCLE_COUNT);
        assert_eq!(platform.compute_load(), 64);
    }
}
