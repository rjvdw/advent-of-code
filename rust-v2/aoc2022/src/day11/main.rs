//! The solution for [advent of code 2022, day 11](https://adventofcode.com/2022/day/11)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::MainResult;

use crate::monkey::{parse, play_round, Monkey};

mod monkey;
mod operation;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 11")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,

    /// The number of rounds of Keep Away to play.
    #[clap(short, long, value_parser, default_value_t = 20)]
    rounds: usize,

    /// The number of monkeys to consider when computing the monkey business score.
    #[clap(short = 'n', value_parser, default_value_t = 2)]
    n: usize,

    /// Are you worried you might not ever get your items back?
    #[clap(short, long, value_parser, default_value_t = false)]
    worried: bool,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);
    let mut monkeys = parse(input.read_lines())?;

    for _ in 0..args.rounds {
        play_round(&mut monkeys, args.worried);
    }

    println!(
        "After {} rounds of Keep Away, the level of monkey business is {}",
        args.rounds,
        compute_monkey_business(&monkeys, args.n)
    );

    Ok(())
}

fn compute_monkey_business(monkeys: &[Monkey], top_n: usize) -> usize {
    let mut counts: Vec<usize> = monkeys
        .iter()
        .map(|m| m.get_inspected_items_count())
        .collect();
    counts.sort_unstable();
    counts.iter().rev().take(top_n).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = String> {
        InputReader::from("./src/day11/test.txt").read_lines()
    }

    #[test]
    fn test_compute_monkey_business_after_20_rounds_while_not_worried() {
        let mut monkeys = parse(test_data()).unwrap();
        for _ in 0..20 {
            play_round(&mut monkeys, false);
        }
        assert_eq!(compute_monkey_business(&monkeys, 2), 10605);
    }

    #[test]
    fn test_compute_monkey_business_after_10000_rounds_while_worried() {
        let mut monkeys = parse(test_data()).unwrap();
        for _ in 0..10000 {
            play_round(&mut monkeys, true);
        }
        assert_eq!(compute_monkey_business(&monkeys, 2), 2713310158);
    }
}
