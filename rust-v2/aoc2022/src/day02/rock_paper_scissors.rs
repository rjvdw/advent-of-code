//! Encapsulates the logic needed to play rock-paper-scissors.

use crate::choice::Choice;
use crate::outcome::Outcome;

pub trait RockPaperScissors {
    fn eval(&self, other: Choice) -> Outcome;
    fn choice_for_desired_outcome(&self, desired_outcome: Outcome) -> Choice;
}
