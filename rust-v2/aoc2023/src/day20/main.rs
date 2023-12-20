//! The solution for [advent of code 2023, day 20](https://adventofcode.com/2023/day/20)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::MainResult;

use crate::machine::{ButtonModule, Module, ModuleMap, ModuleStates};

mod machine;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 20")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// How often should the button be pressed?
    #[clap(long, short, value_parser, default_value_t = 1000)]
    button_push_count: usize,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let modules = Module::parse_input(InputReader::from(args.input).read_lines())?;

    let (low, high) = do_button_presses(&modules, args.button_push_count);
    println!(
        "After {} presses, the low count is {low}, the high count is {high}, and their product is {}",
        args.button_push_count,
        low * high
    );

    Ok(())
}

fn do_button_presses(modules: &ModuleMap, count: usize) -> (usize, usize) {
    let mut states = ModuleStates::new();
    let mut counts = (0, 0);
    for _ in 0..count {
        let (low_count, high_count) = modules.push_button(&mut states);
        counts.0 += low_count;
        counts.1 += high_count;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    const BUTTON_PUSH_COUNT: usize = 1000;

    fn test_data_1() -> ModuleMap {
        Module::parse_input(InputReader::from("./src/day20/test1.txt").read_lines()).unwrap()
    }

    fn test_data_2() -> ModuleMap {
        Module::parse_input(InputReader::from("./src/day20/test2.txt").read_lines()).unwrap()
    }

    #[test]
    fn test_do_button_presses_1() {
        let map = test_data_1();
        assert_eq!(do_button_presses(&map, BUTTON_PUSH_COUNT), (8000, 4000));
    }

    #[test]
    fn test_do_button_presses_2() {
        let map = test_data_2();
        assert_eq!(do_button_presses(&map, BUTTON_PUSH_COUNT), (4250, 2750));
    }
}
