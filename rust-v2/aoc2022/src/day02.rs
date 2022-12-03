use clap::Parser;
use rdcl_aoc_core::input::InputReader;
use std::error;
use std::path::PathBuf;

/// The solution for advent of code 2022, day 2
#[derive(Parser, Debug)]
struct Args {
    /// The file which contains the puzzle input.
    input: PathBuf,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Args = Args::parse();
    let input = InputReader::from(args.input);

    let score_part_1 = play_part_1(input.read_lines())?;
    println!("Your score for part 1 will be {}", score_part_1);

    let score_part_2 = play_part_2(input.read_lines())?;
    println!("Your score for part 2 will be {}", score_part_2);

    Ok(())
}

fn play_part_1<T>(lines: T) -> Result<u32, Box<dyn error::Error>>
where
    T: Iterator<Item = String>,
{
    let mut score = 0;

    for line in lines {
        let chars = line.as_bytes();

        let opponent = Choice::from(chars[0]);
        let you = Choice::from(chars[2]);

        score += compute_score(you, opponent);
    }

    Ok(score)
}

fn play_part_2<T>(lines: T) -> Result<u32, Box<dyn error::Error>>
where
    T: Iterator<Item = String>,
{
    let mut score = 0;

    for line in lines {
        let chars = line.as_bytes();

        let opponent = Choice::from(chars[0]);
        let desired_outcome = Outcome::from(chars[2]);
        let you = opponent.choice_for_desired_outcome(desired_outcome);

        score += compute_score(you, opponent);
    }

    Ok(score)
}

fn compute_score(you: Choice, opponent: Choice) -> u32 {
    you.eval(opponent).get_score() + you.get_score()
}

#[derive(PartialEq, Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl From<u8> for Choice {
    fn from(ch: u8) -> Self {
        match ch {
            b'A' | b'X' => Choice::Rock,
            b'B' | b'Y' => Choice::Paper,
            b'C' | b'Z' => Choice::Scissors,
            _ => panic!("Invalid input: {}", ch),
        }
    }
}

impl From<u8> for Outcome {
    fn from(ch: u8) -> Self {
        match ch {
            b'X' => Outcome::Lose,
            b'Y' => Outcome::Draw,
            b'Z' => Outcome::Win,
            _ => panic!("Invalid input: {}", ch),
        }
    }
}

trait RockPaperScissors {
    fn eval(&self, other: Choice) -> Outcome;
    fn choice_for_desired_outcome(&self, desired_outcome: Outcome) -> Choice;
}

impl RockPaperScissors for Choice {
    fn eval(&self, other: Choice) -> Outcome {
        match (self, &other) {
            (Choice::Rock, Choice::Scissors) => Outcome::Win,
            (Choice::Scissors, Choice::Paper) => Outcome::Win,
            (Choice::Paper, Choice::Rock) => Outcome::Win,
            (a, b) if a == b => Outcome::Draw,
            _ => Outcome::Lose,
        }
    }

    fn choice_for_desired_outcome(&self, desired_outcome: Outcome) -> Choice {
        match desired_outcome {
            Outcome::Win => match self {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock,
            },
            Outcome::Draw => *self,
            Outcome::Lose => match self {
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper,
            },
        }
    }
}

trait Scorable {
    fn get_score(&self) -> u32;
}

impl Scorable for Choice {
    fn get_score(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

impl Scorable for Outcome {
    fn get_score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_data() -> impl Iterator<Item = String> {
        InputReader::from(PathBuf::from("./test-inputs/day02.txt")).read_lines()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(play_part_1(test_data()).unwrap(), 15);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(play_part_2(test_data()).unwrap(), 12);
    }
}
