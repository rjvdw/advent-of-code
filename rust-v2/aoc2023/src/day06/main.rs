//! The solution for [advent of code 2023, day 6](https://adventofcode.com/2023/day/6)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::parser::parse_whitespace_separated_to_vec;
use rdcl_aoc_core::{assert_or_parse_error, err_parse_error, MainResult, ParseResult};

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 6")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input)
        .read_lines()
        .collect::<Vec<_>>();

    let parsed = parse_input(&input)?;
    let margin_of_error = compute_margin_of_error(&parsed);
    println!("Your total margin of error is {margin_of_error}");

    let parsed = parse_concatenated_input(&input)?;
    let margin_of_error = compute_margin_of_error(&parsed);
    println!("Your total margin of error after accounting for bad kerning is {margin_of_error}");

    Ok(())
}

fn compute_margin_of_error(input: &[(u64, u64)]) -> u64 {
    input
        .iter()
        .map(|&(time, distance)| find_wins(time, distance))
        .map(|opt| match opt {
            Some((start, end)) => 1 + end - start,
            None => 0,
        })
        .product::<u64>()
}

fn find_wins(time: u64, distance_to_beat: u64) -> Option<(u64, u64)> {
    // the goal is to find x in 0..=time such that:
    //     x * (time - x) > distance_to_beat
    // which boils down to solving the quadratic equation:
    //     x^2 - time*x + distance_to_beat = 0

    let d = (time * time).checked_sub(4 * distance_to_beat)?;
    let left = (time as f64) / 2.0;
    let right = (d as f64).sqrt() / 2.0;

    let mut x1 = (left - right) as u64 - 1;
    let mut x2 = (left + right) as u64 + 1;

    // deal with edge cases:
    while race(time, x1) <= distance_to_beat {
        x1 += 1;
    }
    while race(time, x2) <= distance_to_beat {
        x2 -= 1;
    }

    Some((x1, x2))
}

fn race(time: u64, push_time: u64) -> u64 {
    (time - push_time) * push_time
}

fn parse_input(input: &[String]) -> ParseResult<Vec<(u64, u64)>> {
    assert_or_parse_error!(input.len() == 2, "Input has an incorrect number of lines.");

    let line1 =
        parse_whitespace_separated_to_vec::<u64>(input[0].strip_prefix("Time:").ok_or(())?)?;
    let line2 =
        parse_whitespace_separated_to_vec::<u64>(input[1].strip_prefix("Distance:").ok_or(())?)?;

    assert_or_parse_error!(
        line1.len() == line2.len(),
        "Number of times do not match number of distances."
    );

    let mut parsed = vec![];
    for i in 0..line1.len() {
        parsed.push((line1[i], line2[i]));
    }
    Ok(parsed)
}

fn parse_concatenated_input(input: &[String]) -> ParseResult<Vec<(u64, u64)>> {
    if input.len() != 2 {
        return err_parse_error!("Input has an incorrect number of lines.");
    }

    let line1 = input[0].strip_prefix("Time:").ok_or(())?.replace(' ', "");

    let line2 = input[1]
        .strip_prefix("Distance:")
        .ok_or(())?
        .replace(' ', "");

    Ok(vec![(line1.parse()?, line2.parse()?)])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_wins() {
        assert_eq!(find_wins(7, 9), Some((2, 5)));
        assert_eq!(find_wins(15, 40), Some((4, 11)));
        assert_eq!(find_wins(30, 200), Some((11, 19)));
    }

    #[test]
    fn test_compute_margin_of_error_1() {
        let input = parse_input(&[
            "Time: 7 15 30".to_string(),
            "Distance: 9 40 200".to_string(),
        ])
        .unwrap();
        assert_eq!(compute_margin_of_error(&input), 288);
    }

    #[test]
    fn test_compute_margin_of_error_2() {
        let input = parse_concatenated_input(&[
            "Time: 7 15 30".to_string(),
            "Distance: 9 40 200".to_string(),
        ])
        .unwrap();
        assert_eq!(compute_margin_of_error(&input), 71503);
    }
}
