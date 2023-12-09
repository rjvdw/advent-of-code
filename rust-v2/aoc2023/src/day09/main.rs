//! The solution for [advent of code 2023, day 9](https://adventofcode.com/2023/day/9)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::parser::parse_whitespace_separated_to_vec;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 9")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() {
    let args: Args = Args::parse();
    let input: Vec<Vec<i64>> = InputReader::from(args.input)
        .parse_lines(parse_whitespace_separated_to_vec)
        .collect();

    let result = input
        .iter()
        .map(|seq| extrapolate(seq))
        .collect::<Option<Vec<_>>>();

    match result {
        Some(result) => {
            println!(
                "Extrapolating to the future, the sum of all extrapolated values is {}",
                result.iter().map(|pair| pair.1).sum::<i64>()
            );

            println!(
                "Extrapolating to the past, the sum of all extrapolated values is {}",
                result.iter().map(|pair| pair.0).sum::<i64>()
            );
        }
        None => println!("There was at least one sequence of which no value could be extrapolated"),
    };
}

fn extrapolate(seq: &[i64]) -> Option<(i64, i64)> {
    if seq.iter().all(|&v| v == 0) {
        Some((0, 0))
    } else if let Some(&first) = seq.first() {
        let mut differences = Vec::with_capacity(seq.len() - 1);
        let mut prev = first;
        for &v in seq.iter().skip(1) {
            differences.push(v - prev);
            prev = v;
        }
        let next = extrapolate(&differences)?;
        let last = seq.last().copied()?;

        Some((first - next.0, last + next.1))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extrapolate1() {
        assert_eq!(extrapolate(&[0, 3, 6, 9, 12, 15]), Some((-3, 18)));
    }

    #[test]
    fn test_extrapolate2() {
        assert_eq!(extrapolate(&[1, 3, 6, 10, 15, 21]), Some((0, 28)));
    }

    #[test]
    fn test_extrapolate3() {
        assert_eq!(extrapolate(&[10, 13, 16, 21, 30, 45]), Some((5, 68)));
    }
}
