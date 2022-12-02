extern crate rdcl_aoc_helpers;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() {
    let args = get_args(&["<input file>"], 1);

    let file = File::open(&args[1]).or_exit_with(1);
    let lines = BufReader::new(file).lines();
    let score_part_1 = play_part_1(lines).or_exit_with(1);

    println!("Your score for part 1 will be {}", score_part_1);

    let file = File::open(&args[1]).or_exit_with(1);
    let lines = BufReader::new(file).lines();
    let score_part_2 = play_part_2(lines).or_exit_with(1);

    println!("Your score for part 2 will be {}", score_part_2);
}

fn play_part_1<T>(lines: T) -> Result<u32, ParseError>
where
    T: Iterator<Item = io::Result<String>>,
{
    let mut score = 0;

    for line in lines {
        let line = line?;
        let chars = line.as_bytes();

        let opponent = Choice::from(chars[0]);
        let you = Choice::from(chars[2]);

        score += compute_score(you, opponent);
    }

    Ok(score)
}

fn play_part_2<T>(lines: T) -> Result<u32, ParseError>
where
    T: Iterator<Item = io::Result<String>>,
{
    let mut score = 0;

    for line in lines {
        let line = line?;
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

    #[test]
    fn test_part_1() {
        let input = vec![
            Ok("A Y".to_string()),
            Ok("B X".to_string()),
            Ok("C Z".to_string()),
        ];
        assert_eq!(play_part_1(input.into_iter()), Ok(15));
    }

    #[test]
    fn test_part_2() {
        let input = vec![
            Ok("A Y".to_string()),
            Ok("B X".to_string()),
            Ok("C Z".to_string()),
        ];
        assert_eq!(play_part_2(input.into_iter()), Ok(12));
    }
}
