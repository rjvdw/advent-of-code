//! The solution for [advent of code 2023, day 5](https://adventofcode.com/2023/day/5)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::parser::parse_whitespace_separated_to_vec;
use rdcl_aoc_core::MainResult;

use crate::mapping::{Mappable, Mapping, Mappings};

mod mapping;

type Seeds = Vec<u64>;
type SeedRanges = Vec<(u64, u64)>;

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

    let locations = evaluate_seeds(&naive_seeds, &mappings);
    match locations.iter().min() {
        Some(v) => println!("The lowest location number is {}", v),
        None => println!("No location numbers were found"),
    };

    let locations = evaluate_seed_ranges(&seed_ranges, &mappings);
    match find_min(&locations) {
        Some(v) => println!("Using the full ranges, the lowest location number is {}", v),
        None => println!("Using the full ranges, no location numbers were found"),
    };

    Ok(())
}

fn parse_seeds<T>(input: &mut T) -> Result<(Seeds, SeedRanges), ParseError>
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
        seed_ranges.push((start, end));
    }

    Ok((naive_seeds, seed_ranges))
}

fn evaluate_seeds(seeds: &[u64], mappings: &Vec<Mapping>) -> Seeds {
    let mut locations: Seeds = Vec::with_capacity(seeds.len());
    for seed in seeds {
        locations.push(mappings.apply_to_nr(*seed));
    }
    locations
}

fn evaluate_seed_ranges(seed_ranges: &[(u64, u64)], mappings: &Vec<Mapping>) -> SeedRanges {
    mappings.apply_to_ranges(seed_ranges)
}

fn find_min(ranges: &[(u64, u64)]) -> Option<u64> {
    ranges.iter().map(|(start, _)| start).copied().min()
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
        let locations = evaluate_seeds(&seeds, &mappings);

        assert_eq!(locations, [82, 43, 86, 35]);
    }

    #[test]
    fn test_evaluate_mappings_full() {
        let mut input = test_data().peekable();
        let (_, seeds) = parse_seeds(&mut input).unwrap();
        let mappings = <Vec<Mapping>>::parse(&mut input).unwrap();
        let mut locations = evaluate_seed_ranges(&seeds, &mappings);
        locations.sort_by_key(|l| l.0);

        assert_eq!(
            locations,
            [
                (46, 56),
                (56, 60),
                (60, 61),
                (82, 85),
                (86, 90),
                (94, 97),
                (97, 99)
            ]
        );
    }
}
