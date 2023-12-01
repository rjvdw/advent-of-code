//! The solution for [advent of code 2023, day 1](https://adventofcode.com/2023/day/1)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::{input::InputReader, DynResult, MainResult};

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 1")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);

    let (solution1, solution2) = solve(input.read_lines())?;

    println!(
        "Only counting numeric digits, the sum of all calibration values is {}",
        solution1
    );

    println!(
        "Also counting digits that are written out, the sum of all calibration values is {}",
        solution2
    );

    Ok(())
}

fn solve<T>(lines: T) -> DynResult<(u32, u32)>
where
    T: Iterator<Item = String>,
{
    // A couple of assumptions:
    // - The digit `0` is never encountered in the puzzle input.
    // - If a line contains no digits, it may be ignored in the calculation.

    let mut sum_p1: u32 = 0;
    let mut sum_p2: u32 = 0;

    for line in lines {
        let mut first_p1 = 0;
        let mut last_p1 = 0;
        let mut first_p2 = 0;
        let mut last_p2 = 0;

        for (i, ch) in line.bytes().enumerate() {
            let digit = if (0x31..=0x39).contains(&ch) {
                let digit = (ch - 0x30) as u32;

                if first_p1 == 0 {
                    first_p1 = digit;
                }
                last_p1 = digit;

                Some(digit)
            } else {
                DIGITS
                    .iter()
                    .enumerate()
                    .find(|&(_, digit)| matches_digit(&line, digit, i))
                    .map(|(i, _)| (i + 1) as u32)
            };

            if let Some(v) = digit {
                if first_p2 == 0 {
                    first_p2 = v;
                }
                last_p2 = v;
            }
        }

        sum_p1 += 10 * first_p1 + last_p1;
        sum_p2 += 10 * first_p2 + last_p2;
    }

    Ok((sum_p1, sum_p2))
}

fn matches_digit(line: &str, digit: &str, start: usize) -> bool {
    let end = start + digit.len();
    end <= line.len() && &line[start..end] == digit
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data_1() -> impl Iterator<Item = String> {
        InputReader::from("./src/day01/test1.txt").read_lines()
    }

    fn test_data_2() -> impl Iterator<Item = String> {
        InputReader::from("./src/day01/test2.txt").read_lines()
    }

    #[test]
    fn test_part_1() {
        let (sum, _) = solve(test_data_1()).unwrap();
        assert_eq!(sum, 142);
    }

    #[test]
    fn test_part_2() {
        let (_, sum) = solve(test_data_2()).unwrap();
        assert_eq!(sum, 281);
    }
}
