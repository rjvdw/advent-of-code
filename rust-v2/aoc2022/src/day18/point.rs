use core::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Point {
    pub fn new(x: i32, y: i32, z: i32) -> Point {
        Point { x, y, z }
    }

    pub fn within_bounds(&self, min: Point, max: Point) -> bool {
        self.x >= min.x
            && self.y >= min.y
            && self.z >= min.z
            && self.x <= max.x
            && self.y <= max.y
            && self.z <= max.z
    }

    pub fn neighbours(self) -> [Point; 6] {
        [
            self + (1, 0, 0),
            self - (1, 0, 0),
            self + (0, 1, 0),
            self - (0, 1, 0),
            self + (0, 0, 1),
            self - (0, 0, 1),
        ]
    }

    pub fn upto(&self, other: Point) -> impl Iterator<Item = Point> + '_ {
        (self.x..=other.x).flat_map(move |x| {
            (self.y..=other.y)
                .flat_map(move |y| (self.z..=other.z).map(move |z| Point::new(x, y, z)))
        })
    }
}

impl Add<(i32, i32, i32)> for Point {
    type Output = Point;

    fn add(self, rhs: (i32, i32, i32)) -> Self::Output {
        Point {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
            z: self.z + rhs.2,
        }
    }
}

impl Sub<(i32, i32, i32)> for Point {
    type Output = Point;

    fn sub(self, rhs: (i32, i32, i32)) -> Self::Output {
        Point {
            x: self.x - rhs.0,
            y: self.y - rhs.1,
            z: self.z - rhs.2,
        }
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{};{};{}]", self.x, self.y, self.z)
    }
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');

        let x = match iter.next() {
            Some(v) => v.parse()?,
            None => {
                return err_parse_error!("Invalid input: {}", s);
            }
        };

        let y = match iter.next() {
            Some(v) => v.parse()?,
            None => {
                return err_parse_error!("Invalid input: {}", s);
            }
        };

        let z = match iter.next() {
            Some(v) => v.parse()?,
            None => {
                return err_parse_error!("Invalid input: {}", s);
            }
        };

        Ok(Point { x, y, z })
    }
}
