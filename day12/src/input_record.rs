use crate::coordinates::Coordinates;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum Action {
    NORTH,
    SOUTH,
    EAST,
    WEST,
    LEFT,
    RIGHT,
    FORWARD,
}

#[derive(Debug)]
pub struct InputRecord {
    action: Action,
    value: i32,
}

impl InputRecord {
    pub fn travel(&self, ship: Coordinates, heading: i32) -> (Coordinates, i32) {
        match self.action {
            Action::NORTH => (ship + (0, self.value), heading),
            Action::SOUTH => (ship - (0, self.value), heading),
            Action::EAST => (ship + (self.value, 0), heading),
            Action::WEST => (ship - (self.value, 0), heading),
            Action::LEFT => (ship, (heading + self.value) % 360),
            Action::RIGHT => (ship, (360 + heading - self.value) % 360),
            Action::FORWARD => match heading {
                0 => (ship + (self.value, 0), heading),
                90 => (ship + (0, self.value), heading),
                180 => (ship - (self.value, 0), heading),
                270 => (ship - (0, self.value), heading),
                _ => panic!(format!("Invalid heading: {}", heading)),
            },
        }
    }

    pub fn move_waypoint(
        &self,
        ship: Coordinates,
        waypoint: Coordinates,
    ) -> (Coordinates, Coordinates) {
        match self.action {
            Action::NORTH => (ship, waypoint + (0, self.value)),
            Action::SOUTH => (ship, waypoint - (0, self.value)),
            Action::EAST => (ship, waypoint + (self.value, 0)),
            Action::WEST => (ship, waypoint - (self.value, 0)),
            Action::LEFT => (ship, waypoint.rotate(self.value)),
            Action::RIGHT => (ship, waypoint.rotate(360 - self.value)),
            Action::FORWARD => (ship + self.value * waypoint, waypoint),
        }
    }
}

#[derive(Debug)]
pub struct InputRecordError {
    msg: String,
}

impl fmt::Display for InputRecordError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl FromStr for InputRecord {
    type Err = InputRecordError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = match s.chars().next() {
            Some('N') => Action::NORTH,
            Some('S') => Action::SOUTH,
            Some('E') => Action::EAST,
            Some('W') => Action::WEST,
            Some('L') => Action::LEFT,
            Some('R') => Action::RIGHT,
            Some('F') => Action::FORWARD,
            _ => {
                return Err(InputRecordError {
                    msg: format!("Invalid action in line '{}'", s),
                });
            }
        };

        let value = match s[1..].parse::<i32>() {
            Ok(v) => match action {
                Action::LEFT | Action::RIGHT => v % 360,
                _ => v,
            },
            Err(e) => {
                return Err(InputRecordError {
                    msg: format!("Failed to parse value in line '{}': {}", s, e),
                });
            }
        };

        Ok(InputRecord { action, value })
    }
}
