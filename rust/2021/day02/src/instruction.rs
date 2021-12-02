use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

use crate::direction::Direction;

pub struct Instruction {
    pub direction: Direction,
    pub distance: i32,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find(' ') {
            Some(pos) => Ok(Instruction {
                direction: s[..pos].parse::<Direction>()?,
                distance: s[pos + 1..].parse::<i32>()?,
            }),
            None => Err(parse_error!("Invalid input line: {}", s)),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.direction, self.distance)
    }
}
