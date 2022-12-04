//! Represents either Rock, Paper or Scissors.

use crate::outcome::Outcome;
use crate::rock_paper_scissors::RockPaperScissors;
use crate::with_score::WithScore;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
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

impl WithScore for Choice {
    fn get_score(&self) -> u32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}
