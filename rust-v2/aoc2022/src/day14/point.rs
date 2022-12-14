use std::cmp::Ordering;
use std::str::FromStr;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point(pub usize, pub usize);

impl Point {
    pub fn next_point(&self, target: &Point) -> Point {
        match self.0.cmp(&target.0) {
            Ordering::Less => Point(self.0 + 1, self.1),
            Ordering::Equal => match self.1.cmp(&target.1) {
                Ordering::Less => Point(self.0, self.1 + 1),
                Ordering::Equal => *self,
                Ordering::Greater => Point(self.0, self.1 - 1),
            },
            Ordering::Greater => Point(self.0 - 1, self.1),
        }
    }

    pub fn down(&self) -> Point {
        Point(self.0, self.1 + 1)
    }

    pub fn down_left(&self) -> Point {
        Point(self.0 - 1, self.1 + 1)
    }

    pub fn down_right(&self) -> Point {
        Point(self.0 + 1, self.1 + 1)
    }
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(point: &str) -> Result<Self, Self::Err> {
        match point.find(',') {
            Some(i) => Ok(Point(point[..i].parse()?, point[i + 1..].parse()?)),
            None => err_parse_error!("Invalid point: {}", point),
        }
    }
}
