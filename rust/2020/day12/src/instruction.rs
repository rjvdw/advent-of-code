use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

use crate::coordinates::Coordinates;

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

#[derive(Debug)]
pub struct Instruction {
    action: Action,
    value: i32,
}

impl Instruction {
    pub fn travel(&self, ship: Coordinates, heading: i32) -> (Coordinates, i32) {
        match self.action {
            Action::North => (ship + (0, self.value), heading),
            Action::South => (ship - (0, self.value), heading),
            Action::East => (ship + (self.value, 0), heading),
            Action::West => (ship - (self.value, 0), heading),
            Action::Left => (ship, (heading + self.value) % 360),
            Action::Right => (ship, (360 + heading - self.value) % 360),
            Action::Forward => match heading {
                0 => (ship + (self.value, 0), heading),
                90 => (ship + (0, self.value), heading),
                180 => (ship - (self.value, 0), heading),
                270 => (ship - (0, self.value), heading),
                _ => panic!("Invalid heading: {}", heading),
            },
        }
    }

    pub fn move_waypoint(
        &self,
        ship: Coordinates,
        waypoint: Coordinates,
    ) -> (Coordinates, Coordinates) {
        match self.action {
            Action::North => (ship, waypoint + (0, self.value)),
            Action::South => (ship, waypoint - (0, self.value)),
            Action::East => (ship, waypoint + (self.value, 0)),
            Action::West => (ship, waypoint - (self.value, 0)),
            Action::Left => (ship, waypoint.rotate(self.value)),
            Action::Right => (ship, waypoint.rotate(360 - self.value)),
            Action::Forward => (ship + self.value * waypoint, waypoint),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = match s.chars().next() {
            Some('N') => Ok(Action::North),
            Some('S') => Ok(Action::South),
            Some('E') => Ok(Action::East),
            Some('W') => Ok(Action::West),
            Some('L') => Ok(Action::Left),
            Some('R') => Ok(Action::Right),
            Some('F') => Ok(Action::Forward),
            _ => Err(parse_error!("Invalid action in line '{}'", s)),
        }?;

        let value = s[1..].parse::<i32>()?;
        let value = match action {
            Action::Left | Action::Right => value % 360,
            _ => value,
        };

        Ok(Instruction { action, value })
    }
}
