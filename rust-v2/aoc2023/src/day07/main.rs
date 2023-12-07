//! The solution for [advent of code 2023, day 7](https://adventofcode.com/2023/day/7)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::MainResult;

use crate::hand::Hand;

mod card;
mod hand;

type InputLine = (Hand, usize);

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 7")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let mut input = InputReader::from(args.input)
        .parse_lines(parse_line)
        .collect::<Vec<InputLine>>();

    let score = score_game(&input);
    println!("The total winnings are {score}");

    turn_jacks_into_jokers(&mut input);
    let score = score_game(&input);
    println!("Using jokers, the total winnings are {score}");

    Ok(())
}

fn score_game(input: &[InputLine]) -> usize {
    let mut input = input.to_vec();
    input.sort_by_key(|l| l.0);

    input
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| bid * (idx + 1))
        .sum()
}

fn turn_jacks_into_jokers(input: &mut [InputLine]) {
    for line in input.iter_mut() {
        line.0.turn_jacks_into_jokers()
    }
}

fn parse_line(line: &str) -> Result<InputLine, ParseError> {
    let i = line.find(' ').ok_or(())?;
    Ok((line[0..i].parse()?, line[i + 1..].parse()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<InputLine> {
        InputReader::from("./src/day07/test.txt")
            .parse_lines(parse_line)
            .collect::<Vec<InputLine>>()
    }

    #[test]
    fn test_score_game() {
        assert_eq!(score_game(&test_data()), 6440);
    }

    #[test]
    fn test_score_game_with_jokers() {
        let mut input = test_data();
        turn_jacks_into_jokers(&mut input);
        assert_eq!(score_game(&input), 5905);
    }
}
