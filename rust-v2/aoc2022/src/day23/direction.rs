use std::fmt;

use crate::direction::Direction::*;

pub type Headings = [[Direction; 3]; 4];

pub const ALL: [Direction; 8] = [
    North, NorthEast, East, SouthEast, South, SouthWest, West, NorthWest,
];

pub const HEADINGS: Headings = [
    [NorthWest, North, NorthEast],
    [SouthWest, South, SouthEast],
    [NorthWest, West, SouthWest],
    [NorthEast, East, SouthEast],
];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn from(&self, (x, y): (i64, i64)) -> (i64, i64) {
        match self {
            Direction::North => (x, y - 1),
            Direction::NorthEast => (x + 1, y - 1),
            Direction::East => (x + 1, y),
            Direction::SouthEast => (x + 1, y + 1),
            Direction::South => (x, y + 1),
            Direction::SouthWest => (x - 1, y + 1),
            Direction::West => (x - 1, y),
            Direction::NorthWest => (x - 1, y - 1),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::North => write!(f, "N"),
            Direction::NorthEast => write!(f, "NE"),
            Direction::East => write!(f, "E"),
            Direction::SouthEast => write!(f, "SE"),
            Direction::South => write!(f, "S"),
            Direction::SouthWest => write!(f, "SW"),
            Direction::West => write!(f, "W"),
            Direction::NorthWest => write!(f, "NW"),
        }
    }
}
