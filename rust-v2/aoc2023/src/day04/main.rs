//! The solution for [advent of code 2023, day 4](https://adventofcode.com/2023/day/4)

use std::path::PathBuf;
use std::str::FromStr;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::MainResult;

use crate::scratch_card::ScratchCard;

mod scratch_card;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2023, day 4")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let cards = InputReader::from(args.input)
        .parse_lines(ScratchCard::from_str)
        .collect::<Vec<_>>();

    let score = cards.iter().map(|card| card.naive_score()).sum::<u32>();
    println!(
        "Using the naive scoring system, the total score is {}",
        score
    );

    let score = evaluate(&cards).iter().sum::<usize>();
    println!(
        "Using the correct scoring system, you end up with {} cards",
        score
    );

    Ok(())
}

fn evaluate(cards: &[ScratchCard]) -> Vec<usize> {
    let mut ids = vec![1usize; cards.len()];

    for card in cards {
        let id = card.id();
        let count = ids[id - 1];
        let matching = card.count_matching_numbers();

        for item in ids.iter_mut().skip(id).take(matching) {
            *item += count;
        }
    }

    ids
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> Vec<ScratchCard> {
        InputReader::from("./src/day04/test.txt")
            .parse_lines(ScratchCard::from_str)
            .collect()
    }

    #[test]
    fn test_naive_score() {
        let cards = test_data();

        assert_eq!(cards[0].naive_score(), 8);
        assert_eq!(cards[1].naive_score(), 2);
        assert_eq!(cards[2].naive_score(), 2);
        assert_eq!(cards[3].naive_score(), 1);
        assert_eq!(cards[4].naive_score(), 0);
        assert_eq!(cards[5].naive_score(), 0);
    }

    #[test]
    fn test_evaluate() {
        let cards = test_data();

        assert_eq!(evaluate(&cards), vec![1, 2, 4, 8, 14, 1]);
    }
}
