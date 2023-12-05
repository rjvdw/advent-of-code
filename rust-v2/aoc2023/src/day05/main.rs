//! The solution for [advent of code 2023, day 5](https://adventofcode.com/2023/day/5)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::parser::parse_whitespace_separated_to_vec;
use rdcl_aoc_core::MainResult;

use crate::mapping::{Mapping, Mappings};

mod mapping;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 5")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let mut input = InputReader::from(args.input).read_lines().peekable();

    let (naive_seeds, seed_ranges) = parse_seeds(&mut input)?;
    let mappings = <Vec<Mapping>>::parse(&mut input)?;

    let locations = evaluate_mappings(&naive_seeds, &mappings);
    match locations.iter().min() {
        Some(v) => println!("The lowest location number is {}", v),
        None => println!("No location numbers were found"),
    };

    let locations = evaluate_mappings(&seed_ranges, &mappings);
    match locations.iter().min() {
        Some(v) => println!("Using the full ranges, the lowest location number is {}", v),
        None => println!("Using the full ranges, no location numbers were found"),
    };

    Ok(())
}

fn parse_seeds<T>(input: &mut T) -> Result<(Vec<u64>, Vec<u64>), ParseError>
where
    T: Iterator<Item = String>,
{
    let line = input.next().ok_or(())?;
    let line = line.strip_prefix("seeds: ").ok_or(())?;
    input.next(); // the second line in the file is an empty line so may be ignored

    let naive_seeds = parse_whitespace_separated_to_vec(line)?;

    let mut seed_ranges = vec![];
    let mut iter = naive_seeds.iter();
    while let Some(&start) = iter.next() {
        let end = start + *iter.next().ok_or(())?;
        for i in start..end {
            seed_ranges.push(i);
        }
    }

    Ok((naive_seeds, seed_ranges))
}

fn evaluate_mappings(seeds: &[u64], mappings: &Vec<Mapping>) -> Vec<u64> {
    let mut locations: Vec<u64> = Vec::with_capacity(seeds.len());
    for seed in seeds {
        locations.push(mappings.apply(*seed));
    }
    locations
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = String> {
        InputReader::from("./src/day05/test.txt").read_lines()
    }

    #[test]
    fn test_evaluate_mappings_naive() {
        let mut input = test_data().peekable();
        let (seeds, _) = parse_seeds(&mut input).unwrap();
        let mappings = <Vec<Mapping>>::parse(&mut input).unwrap();
        let locations = evaluate_mappings(&seeds, &mappings);

        assert_eq!(locations, [82, 43, 86, 35]);
    }

    #[test]
    fn test_evaluate_mappings_full() {
        let mut input = test_data().peekable();
        let (_, seeds) = parse_seeds(&mut input).unwrap();
        let mappings = <Vec<Mapping>>::parse(&mut input).unwrap();
        let locations = evaluate_mappings(&seeds, &mappings);

        assert_eq!(
            locations,
            [
                82, 83, 84, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 60, 86, 87, 88, 89, 94, 95, 96,
                56, 57, 58, 59, 97, 98
            ]
        );
    }
}
