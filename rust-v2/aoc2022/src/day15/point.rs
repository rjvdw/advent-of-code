use std::fmt;
use std::str::FromStr;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point(pub i64, pub i64);

impl Point {
    pub fn as_tuple(&self) -> (i64, i64) {
        (self.0, self.1)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x={}, y={}", self.0, self.1)
    }
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sub = match s.strip_prefix("x=") {
            Some(s) => s,
            None => {
                return err_parse_error!("Cannot parse x: {}", s);
            }
        };

        let i = match sub.find(", ") {
            Some(i) => i,
            None => {
                return err_parse_error!("Cannot parse x: {}", s);
            }
        };

        let x = sub[..i].parse::<i64>()?;

        let sub = match sub[i..].strip_prefix(", y=") {
            Some(s) => s,
            None => {
                return err_parse_error!("Cannot parse y: {}", s);
            }
        };

        let y = sub.parse::<i64>()?;

        Ok(Point(x, y))
    }
}
