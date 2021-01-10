use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::err_parse_error;
use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    NorthWest,
    North,
    NorthEast,
    SouthWest,
    South,
    SouthEast,
}

impl Direction {
    pub fn walk(&self, (x, y): (i64, i64)) -> (i64, i64) {
        match self {
            Direction::NorthWest => (x - 1, y),
            Direction::North => (x, y - 1),
            Direction::NorthEast => (x + 1, y - 1),
            Direction::SouthWest => (x - 1, y + 1),
            Direction::South => (x, y + 1),
            Direction::SouthEast => (x + 1, y),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::NorthWest => write!(f, "nw"),
            Direction::North => write!(f, "n"),
            Direction::NorthEast => write!(f, "ne"),
            Direction::SouthWest => write!(f, "sw"),
            Direction::South => write!(f, "s"),
            Direction::SouthEast => write!(f, "se"),
        }
    }
}

impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nw" => Ok(Direction::NorthWest),
            "n" => Ok(Direction::North),
            "ne" => Ok(Direction::NorthEast),
            "sw" => Ok(Direction::SouthWest),
            "s" => Ok(Direction::South),
            "se" => Ok(Direction::SouthEast),
            _ => err_parse_error!("Invalid direction: {}", s),
        }
    }
}
