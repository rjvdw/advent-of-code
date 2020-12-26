use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

const OP_TURN_ON: &str = "turn on";
const OP_TURN_OFF: &str = "turn off";
const OP_TOGGLE: &str = "toggle";

type CoordinateType = u16;
pub type Coordinate = (CoordinateType, CoordinateType);

#[derive(Debug)]
enum Operation {
    On,
    Off,
    Toggle,
}

#[derive(Debug)]
pub struct Instruction {
    op: Operation,
    lights: Vec<Coordinate>,
}

impl Instruction {
    pub fn set_binary_state(&self, lit: &mut HashSet<Coordinate>) {
        match self.op {
            Operation::On => {
                lit.extend(&self.lights);
            }
            Operation::Off => {
                for light in &self.lights {
                    lit.remove(light);
                }
            }
            Operation::Toggle => {
                for light in &self.lights {
                    if lit.contains(light) {
                        lit.remove(light);
                    } else {
                        lit.insert(*light);
                    }
                }
            }
        }
    }

    pub fn update_brightness(&self, brightness: &mut HashMap<Coordinate, u32>) {
        let delta: i32 = match self.op {
            Operation::On => 1,
            Operation::Off => -1,
            Operation::Toggle => 2,
        };

        for light in &self.lights {
            let mut current_brightness = brightness.get(light).cloned().unwrap_or(0);
            if delta >= 0 {
                current_brightness += delta as u32;
            } else if current_brightness > 0 {
                current_brightness -= (-delta) as u32;
            }
            brightness.insert(*light, current_brightness);
        }
    }

    #[cfg(test)]
    pub fn count_lights(&self) -> usize {
        self.lights.len()
    }

    #[allow(clippy::type_complexity)]
    fn parse(s: &str) -> Option<(Operation, (&str, &str), (&str, &str))> {
        let (op, skip) = if s.starts_with(OP_TURN_ON) {
            (Operation::On, OP_TURN_ON.len())
        } else if s.starts_with(OP_TURN_OFF) {
            (Operation::Off, OP_TURN_OFF.len())
        } else if s.starts_with(OP_TOGGLE) {
            (Operation::Toggle, OP_TOGGLE.len())
        } else {
            return None;
        };

        let start_x = skip + 1;
        let end_x = start_x + s[start_x..].find(',')?;
        let start_y = end_x + 1;
        let end_y = start_y + s[start_y..].find(' ')?;

        let start = (&s[start_x..end_x], &s[start_y..end_y]);

        let start_x = end_y + 1 + s[end_y + 1..].find(' ')? + 1;
        let end_x = start_x + s[start_x..].find(',')?;
        let start_y = end_x + 1;

        let end = (&s[start_x..end_x], &s[start_y..]);

        Some((op, start, end))
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((op, start, end)) = Self::parse(s) {
            let start = (
                start.0.parse::<CoordinateType>()?,
                start.1.parse::<CoordinateType>()?,
            );
            let end = (
                end.0.parse::<CoordinateType>()?,
                end.1.parse::<CoordinateType>()?,
            );

            let mut lights = Vec::new();
            for x in start.0..=end.0 {
                for y in start.1..=end.1 {
                    lights.push((x, y));
                }
            }

            Ok(Instruction { op, lights })
        } else {
            Err(ParseError(format!("Invalid instruction: {}", s)))
        }
    }
}
