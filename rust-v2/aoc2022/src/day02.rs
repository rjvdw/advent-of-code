//! The solution for [advent of code 2022, day 2](https://adventofcode.com/2022/day/2)

use std::path::PathBuf;

use clap::Parser;

use rdcl_aoc_core::input::InputReader;
use rdcl_aoc_core::{DynResult, MainResult};

use crate::day02_lib::choice::Choice;
use crate::day02_lib::outcome::Outcome;
use crate::day02_lib::rock_paper_scissors::RockPaperScissors;
use crate::day02_lib::with_score::WithScore;

mod day02_lib;

#[derive(Parser, Debug)]
#[clap(about = "The solution for advent of code 2022, day 2")]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() -> MainResult {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);

    let (score_part_1, score_part_2) = play(input.read_lines())?;
    println!("Your score for part 1 will be {}", score_part_1);
    println!("Your score for part 2 will be {}", score_part_2);

    Ok(())
}

fn play<T>(lines: T) -> DynResult<(u32, u32)>
where
    T: Iterator<Item = String>,
{
    let mut score_p1 = 0;
    let mut score_p2 = 0;

    for line in lines {
        let chars = line.as_bytes();

        let opponent = Choice::from(chars[0]);
        let you_p1 = Choice::from(chars[2]);

        let desired_outcome = Outcome::from(chars[2]);
        let you_p2 = opponent.choice_for_desired_outcome(desired_outcome);

        score_p1 += compute_score(you_p1, opponent);
        score_p2 += compute_score(you_p2, opponent);
    }

    Ok((score_p1, score_p2))
}

fn compute_score(you: Choice, opponent: Choice) -> u32 {
    you.eval(opponent).get_score() + you.get_score()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = String> {
        InputReader::from(PathBuf::from("./test-inputs/day02.txt")).read_lines()
    }

    #[test]
    fn test_play() {
        assert_eq!(play(test_data()).unwrap(), (15, 12));
    }
}
