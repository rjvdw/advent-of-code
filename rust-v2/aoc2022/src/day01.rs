//! The solution for [advent of code 2022, day 1](https://adventofcode.com/2022/day/1)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::{DynResult, MainResult};

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 1")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// Compute the sum of the `n` largest values.
    #[clap(short = 'n', value_parser, default_value_t = 1)]
    top_n: usize,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);
    let calories_per_elf = parse_input(input.read_lines())?;

    match max_calories(&calories_per_elf, args.top_n) {
        Some(v) => println!("The sum of the largest {} values is {}", args.top_n, v),
        None => eprintln!(
            "There are insufficient values to compute the sum of the largest {} values",
            args.top_n
        ),
    }

    Ok(())
}

fn max_calories(values: &[u32], n: usize) -> Option<u32> {
    if values.len() < n {
        None
    } else {
        let mut values = values.to_vec();
        values.sort_unstable();
        Some(values.iter().rev().take(n).sum())
    }
}

fn parse_input<T>(lines: T) -> DynResult<Vec<u32>>
where
    T: Iterator<Item = String>,
{
    let mut sums = vec![];
    let mut sum = 0;

    for line in lines {
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
        } else {
            sum += line.parse::<u32>()?;
        }
    }
    sums.push(sum);

    Ok(sums)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = String> {
        InputReader::from(PathBuf::from("./test-inputs/day01.txt")).read_lines()
    }

    fn input() -> Vec<u32> {
        parse_input(test_data()).unwrap()
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input(test_data()).unwrap(),
            vec![6000, 4000, 11000, 24000, 10000]
        );
    }

    #[test]
    fn test_max_calories_with_n_is_1() {
        assert_eq!(max_calories(&input(), 1), Some(24000));
    }

    #[test]
    fn test_max_calories_with_n_is_3() {
        assert_eq!(max_calories(&input(), 3), Some(24000 + 11000 + 10000));
    }

    #[test]
    fn test_max_calories_with_n_is_6() {
        assert_eq!(max_calories(&input(), 6), None);
    }
}
