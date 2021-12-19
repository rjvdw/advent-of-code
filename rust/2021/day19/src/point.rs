use std::fmt;
use std::ops::{Add, Sub};
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::math::taxi_cab_3d;
use rdcl_aoc_helpers::parse_error;

pub const ORIENTATIONS: u8 = 24;

#[derive(Copy, Clone, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point {
    pub fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }

    pub fn rotate(&self, orientation: u8) -> Point {
        match orientation {
            0 => *self,
            1 => Point::new(self.x, self.z, -self.y),
            2 => Point::new(self.x, -self.z, self.y),
            3 => Point::new(self.x, -self.y, -self.z),
            4 => Point::new(self.y, self.x, -self.z),
            5 => Point::new(self.y, self.z, self.x),
            6 => Point::new(self.y, -self.z, -self.x),
            7 => Point::new(self.y, -self.x, self.z),
            8 => Point::new(self.z, self.x, self.y),
            9 => Point::new(self.z, self.y, -self.x),
            10 => Point::new(self.z, -self.y, self.x),
            11 => Point::new(self.z, -self.x, -self.y),
            12 => Point::new(-self.z, self.x, -self.y),
            13 => Point::new(-self.z, self.y, self.x),
            14 => Point::new(-self.z, -self.y, -self.x),
            15 => Point::new(-self.z, -self.x, self.y),
            16 => Point::new(-self.y, self.x, self.z),
            17 => Point::new(-self.y, self.z, -self.x),
            18 => Point::new(-self.y, -self.z, self.x),
            19 => Point::new(-self.y, -self.x, -self.z),
            20 => Point::new(-self.x, self.y, -self.z),
            21 => Point::new(-self.x, self.z, self.y),
            22 => Point::new(-self.x, -self.z, -self.y),
            _ => Point::new(-self.x, -self.y, self.z),
        }
    }

    pub fn distance_to(&self, other: &Point) -> i64 {
        taxi_cab_3d((self.x, self.y, self.z), (other.x, other.y, other.z))
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point {{ x: {}, y: {}, z: {} }}", self.x, self.y, self.z)
    }
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(i0) = s.find(',') {
            if let Some(i1) = s[i0 + 1..].find(',') {
                let i1 = i1 + i0 + 1;
                return Ok(Point::new(
                    s[..i0].parse()?,
                    s[i0 + 1..i1].parse()?,
                    s[i1 + 1..].parse()?,
                ));
            }
        }

        Err(parse_error!("Invalid point: {}", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotations() {
        assert_eq!(Point::new(3, 2, 1).rotate(0), Point::new(3, 2, 1));
        assert_eq!(Point::new(3, 2, 1).rotate(1), Point::new(3, 1, -2));
        assert_eq!(Point::new(3, 2, 1).rotate(2), Point::new(3, -1, 2));
        assert_eq!(Point::new(3, 2, 1).rotate(3), Point::new(3, -2, -1));
        assert_eq!(Point::new(3, 2, 1).rotate(4), Point::new(2, 3, -1));
        assert_eq!(Point::new(3, 2, 1).rotate(5), Point::new(2, 1, 3));
        assert_eq!(Point::new(3, 2, 1).rotate(6), Point::new(2, -1, -3));
        assert_eq!(Point::new(3, 2, 1).rotate(7), Point::new(2, -3, 1));
        assert_eq!(Point::new(3, 2, 1).rotate(8), Point::new(1, 3, 2));
        assert_eq!(Point::new(3, 2, 1).rotate(9), Point::new(1, 2, -3));
        assert_eq!(Point::new(3, 2, 1).rotate(10), Point::new(1, -2, 3));
        assert_eq!(Point::new(3, 2, 1).rotate(11), Point::new(1, -3, -2));
        assert_eq!(Point::new(3, 2, 1).rotate(12), Point::new(-1, 3, -2));
        assert_eq!(Point::new(3, 2, 1).rotate(13), Point::new(-1, 2, 3));
        assert_eq!(Point::new(3, 2, 1).rotate(14), Point::new(-1, -2, -3));
        assert_eq!(Point::new(3, 2, 1).rotate(15), Point::new(-1, -3, 2));
        assert_eq!(Point::new(3, 2, 1).rotate(16), Point::new(-2, 3, 1));
        assert_eq!(Point::new(3, 2, 1).rotate(17), Point::new(-2, 1, -3));
        assert_eq!(Point::new(3, 2, 1).rotate(18), Point::new(-2, -1, 3));
        assert_eq!(Point::new(3, 2, 1).rotate(19), Point::new(-2, -3, -1));
        assert_eq!(Point::new(3, 2, 1).rotate(20), Point::new(-3, 2, -1));
        assert_eq!(Point::new(3, 2, 1).rotate(21), Point::new(-3, 1, 2));
        assert_eq!(Point::new(3, 2, 1).rotate(22), Point::new(-3, -1, -2));
        assert_eq!(Point::new(3, 2, 1).rotate(23), Point::new(-3, -2, 1));
    }

    #[test]
    fn test_distance_to() {
        assert_eq!(
            Point::new(5, 5, 5).distance_to(&Point::new(-10, 10, 0)),
            15 + 5 + 5
        );
    }

    #[test]
    fn test_add() {
        assert_eq!(
            Point::new(1, 2, 3) + Point::new(4, 5, 6),
            Point::new(5, 7, 9)
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            Point::new(1, 2, 3) - Point::new(4, 5, 6),
            Point::new(-3, -3, -3)
        );
    }

    #[test]
    fn test_fmt_display() {
        assert_eq!(format!("{}", Point::new(1, 2, 3)), "1,2,3");
    }

    #[test]
    fn test_fmt_debug() {
        assert_eq!(
            format!("{:?}", Point::new(1, 2, 3)),
            "Point { x: 1, y: 2, z: 3 }"
        );
    }

    #[test]
    fn test_from_str() {
        assert_eq!("1,2,3".parse::<Point>(), Ok(Point::new(1, 2, 3)));
        assert_eq!("-1,-2,-3".parse::<Point>(), Ok(Point::new(-1, -2, -3)));
        assert!("a,b,c".parse::<Point>().is_err());
        assert!("invalid".parse::<Point>().is_err());
    }
}
