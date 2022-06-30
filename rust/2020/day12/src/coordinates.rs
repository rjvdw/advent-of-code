use std::fmt;
use std::ops::{Add, Mul, Sub};
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

pub const ORIGIN: Coordinates = Coordinates(0, 0);

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct Coordinates(pub i32, pub i32);

impl Coordinates {
    pub fn manhattan_distance(self) -> u32 {
        self.0.unsigned_abs() + self.1.unsigned_abs()
    }

    pub fn rotate(self, degrees: i32) -> Coordinates {
        match degrees {
            0 => self,
            90 => Coordinates(-self.1, self.0),
            180 => Coordinates(-self.0, -self.1),
            270 => Coordinates(self.1, -self.0),
            _ => panic!("Invalid rotation: {}", degrees),
        }
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Add for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinates(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<(i32, i32)> for Coordinates {
    type Output = Coordinates;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Coordinates(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinates(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Sub<(i32, i32)> for Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: (i32, i32)) -> Self::Output {
        Coordinates(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<i32> for Coordinates {
    type Output = (i32, i32);

    fn mul(self, rhs: i32) -> Self::Output {
        (self.0 * rhs, self.1 * rhs)
    }
}

impl Mul<Coordinates> for i32 {
    type Output = (i32, i32);

    fn mul(self, rhs: Coordinates) -> Self::Output {
        (self * rhs.0, self * rhs.1)
    }
}

impl FromStr for Coordinates {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(idx) = s.find(',') {
            let x = s[..idx].parse::<i32>();
            let y = s[idx + 1..].parse::<i32>();

            if let (Ok(x), Ok(y)) = (x, y) {
                return Ok(Coordinates(x, y));
            }
        }

        Err(parse_error!("Invalid coordinates '{}'", s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_0() {
        assert_eq!(Coordinates(2, 1).rotate(0), Coordinates(2, 1))
    }

    #[test]
    fn test_rotate_90() {
        assert_eq!(Coordinates(2, 1).rotate(90), Coordinates(-1, 2))
    }

    #[test]
    fn test_rotate_180() {
        assert_eq!(Coordinates(2, 1).rotate(180), Coordinates(-2, -1))
    }

    #[test]
    fn test_rotate_270() {
        assert_eq!(Coordinates(2, 1).rotate(270), Coordinates(1, -2))
    }
}
