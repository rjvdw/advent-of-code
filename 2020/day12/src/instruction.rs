use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

use crate::coordinates::Coordinates;

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
pub struct Instruction {
    action: Action,
    value: i32,
}

impl Instruction {
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

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = match s.chars().next() {
            Some('N') => Ok(Action::NORTH),
            Some('S') => Ok(Action::SOUTH),
            Some('E') => Ok(Action::EAST),
            Some('W') => Ok(Action::WEST),
            Some('L') => Ok(Action::LEFT),
            Some('R') => Ok(Action::RIGHT),
            Some('F') => Ok(Action::FORWARD),
            _ => Err(ParseError(format!("Invalid action in line '{}'", s))),
        }?;

        let value = s[1..].parse::<i32>()?;
        let value = match action {
            Action::LEFT | Action::RIGHT => value % 360,
            _ => value,
        };

        Ok(Instruction { action, value })
    }
}
