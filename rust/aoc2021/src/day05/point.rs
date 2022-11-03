use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find(',') {
            Some(p) => Ok(Point {
                x: s[..p].parse()?,
                y: s[p + 1..].parse()?,
            }),
            None => Err(parse_error!("Invalid input: {}", s)),
        }
    }
}
