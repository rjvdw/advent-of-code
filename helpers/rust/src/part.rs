use std::fmt;
use std::str::FromStr;

use crate::error::ParseError;

pub enum Part {
    One,
    Two,
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Part::One => write!(f, "part 1"),
            Part::Two => write!(f, "part 2"),
        }
    }
}

impl FromStr for Part {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "part1" | "part 1" | "Part 1" | "Part1" => Ok(Part::One),
            "part2" | "part 2" | "Part 2" | "Part2" => Ok(Part::Two),
            _ => Err(ParseError(format!("Invalid part: {}", s))),
        }
    }
}
