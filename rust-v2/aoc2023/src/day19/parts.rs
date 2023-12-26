use std::fmt;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::{err_parse_error, ParseResult};

pub const NR_RATINGS: usize = 4;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
pub enum Rating {
    X = 0,
    M = 1,
    A = 2,
    S = 3,
}

impl FromStr for Rating {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Rating::X),
            "m" => Ok(Rating::M),
            "a" => Ok(Rating::A),
            "s" => Ok(Rating::S),
            _ => err_parse_error!("Invalid rating: {}", s),
        }
    }
}

impl fmt::Display for Rating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rating::X => write!(f, "x"),
            Rating::M => write!(f, "m"),
            Rating::A => write!(f, "a"),
            Rating::S => write!(f, "s"),
        }
    }
}

pub type Part = [usize; NR_RATINGS];

pub trait HasRatings {
    fn sum_ratings(&self) -> usize;
}

impl HasRatings for Part {
    fn sum_ratings(&self) -> usize {
        self.iter().sum()
    }
}

impl Index<Rating> for Part {
    type Output = usize;

    fn index(&self, index: Rating) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<Rating> for Part {
    fn index_mut(&mut self, index: Rating) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

pub fn parse_part(s: &str) -> ParseResult<Part> {
    let mut part = [0usize; NR_RATINGS];
    for sub in s[1..s.len() - 1].split(',') {
        let rating = sub[0..1].parse::<Rating>()?;
        let value = sub[2..].parse::<usize>()?;
        part[rating] = value;
    }
    Ok(part)
}
