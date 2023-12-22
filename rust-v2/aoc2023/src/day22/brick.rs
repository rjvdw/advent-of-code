use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

use rdcl_aoc2023::overlap::Overlappable;
use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;

#[derive(Debug, Copy, Clone, Default)]
pub struct Brick {
    start: Point,
    end: Point,
}

impl Brick {
    /// Returns true if this brick hangs directly above some other brick.
    pub fn supported_by(&self, other: &Brick) -> bool {
        self.get_z().0 > other.get_z().1
            && self.get_x().overlaps_with(other.get_x())
            && self.get_y().overlaps_with(other.get_y())
    }

    fn get_x(&self) -> (usize, usize) {
        (self.start.x, self.end.x)
    }

    fn get_y(&self) -> (usize, usize) {
        (self.start.y, self.end.y)
    }

    pub fn get_z(&self) -> (usize, usize) {
        (self.start.z, self.end.z)
    }

    pub fn set_z(&mut self, z: usize) {
        let d = self.end.z - self.start.z;
        self.start.z = z;
        self.end.z = d + z;
    }
}

impl FromStr for Brick {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut brick = Brick::default();
        let mut parts = s.split('~').map(|p| p.parse::<Point>());

        brick.start = parts.next().ok_or(())??;
        brick.end = parts.next().ok_or(())??;

        assert!(brick.start.x <= brick.end.x);
        assert!(brick.start.y <= brick.end.y);
        assert!(brick.start.z <= brick.end.z);

        if parts.next().is_some() {
            err_parse_error!()
        } else {
            Ok(brick)
        }
    }
}

impl fmt::Display for Brick {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}~{}", self.start, self.end)
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl FromStr for Point {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut point = Point::default();
        let mut parts = s.split(',').map(|nr| nr.parse::<usize>());

        point.x = parts.next().ok_or(())??;
        point.y = parts.next().ok_or(())??;
        point.z = parts.next().ok_or(())??;

        if parts.next().is_some() {
            err_parse_error!()
        } else {
            Ok(point)
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.z)
    }
}
