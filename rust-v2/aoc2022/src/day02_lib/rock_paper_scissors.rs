//! Encapsulates the logic needed to play rock-paper-scissors.

use crate::day02_lib::choice::Choice;
use crate::day02_lib::outcome::Outcome;

pub trait RockPaperScissors {
    fn eval(&self, other: Choice) -> Outcome;
    fn choice_for_desired_outcome(&self, desired_outcome: Outcome) -> Choice;
}
