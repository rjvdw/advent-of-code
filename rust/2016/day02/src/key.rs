use std::collections::HashMap;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub struct Key {
    directions: Vec<Direction>,
}

impl Key {
    pub fn solve_for_keypad(
        &self,
        keypad: &HashMap<(u8, u8), String>,
        mut pos: (u8, u8),
    ) -> (String, (u8, u8)) {
        for direction in &self.directions {
            let next_pos = match direction {
                Direction::Up => (pos.0 - 1, pos.1),
                Direction::Down => (pos.0 + 1, pos.1),
                Direction::Right => (pos.0, pos.1 + 1),
                Direction::Left => (pos.0, pos.1 - 1),
            };

            if keypad.contains_key(&next_pos) {
                pos = next_pos;
            }
        }

        (keypad.get(&pos).unwrap().to_string(), pos)
    }
}

impl FromStr for Key {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Key {
            directions: s
                .chars()
                .map(|ch| match ch {
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    'R' => Direction::Right,
                    'L' => Direction::Left,
                    _ => unreachable!(),
                })
                .collect(),
        })
    }
}
