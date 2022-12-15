use std::fmt;
use std::str::FromStr;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_pathfinding::taxi_cab_2d;

use crate::line::Line;
use crate::point::Point;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Sensor {
    pub coordinate: Point,
    pub closest_beacon: Point,
}

impl Sensor {
    pub fn size(&self) -> i64 {
        taxi_cab_2d(self.coordinate.as_tuple(), self.closest_beacon.as_tuple())
    }

    pub fn contains(&self, point: &Point) -> bool {
        taxi_cab_2d(self.coordinate.as_tuple(), point.as_tuple()) <= self.size()
    }

    pub fn get_edges(&self) -> Vec<Line> {
        let distance = self.size() + 1;
        vec![
            Line(1, self.coordinate.1 - (self.coordinate.0 + distance)),
            Line(-1, self.coordinate.1 - (self.coordinate.0 + distance)),
            Line(1, self.coordinate.1 + (self.coordinate.0 + distance)),
            Line(-1, self.coordinate.1 + (self.coordinate.0 + distance)),
        ]
    }
}

impl fmt::Display for Sensor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Sensor at {}: closest beacon is at {}",
            self.coordinate, self.closest_beacon
        )
    }
}

impl FromStr for Sensor {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sub = match s.strip_prefix("Sensor at ") {
            Some(s) => s,
            None => {
                return err_parse_error!("Cannot parse sensor: {}", s);
            }
        };

        let i = match sub.find(": ") {
            Some(i) => i,
            None => {
                return err_parse_error!("Cannot parse sensor: {}", s);
            }
        };

        let coordinate = sub[..i].parse::<Point>()?;

        let sub = match sub[i..].strip_prefix(": closest beacon is at ") {
            Some(s) => s,
            None => {
                return err_parse_error!("Cannot parse beacon: {}", s);
            }
        };

        let closest_beacon = sub.parse::<Point>()?;

        Ok(Sensor {
            coordinate,
            closest_beacon,
        })
    }
}
