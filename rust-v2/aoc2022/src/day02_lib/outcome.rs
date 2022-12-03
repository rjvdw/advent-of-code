//! Represents the possible outcomes of a game of rock-paper-scissors.

use crate::day02_lib::with_score::WithScore;

pub enum Outcome {
    Win,
    Draw,
    Lose,
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

impl WithScore for Outcome {
    fn get_score(&self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}
