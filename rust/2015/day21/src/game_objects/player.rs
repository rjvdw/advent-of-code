use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::MultilineFromStr;
use rdcl_aoc_helpers::parse_error;

use crate::game_objects::item::Item;

#[derive(Debug, Clone)]
pub struct Player {
    hit_points: u32,
    damage: u32,
    armor: u32,
}

impl Player {
    pub fn new(hit_points: u32) -> Player {
        Player {
            hit_points,
            damage: 0,
            armor: 0,
        }
    }

    /// Determines how many turns you need to defeat `other`.
    pub fn fight(&self, other: &Player) -> u32 {
        let damage_score = if other.armor >= self.damage {
            1
        } else {
            self.damage - other.armor
        };

        other.hit_points / damage_score
            + if other.hit_points % damage_score == 0 {
                0
            } else {
                1
            }
    }

    /// Equip an item
    pub fn equip(&self, item: &Item) -> Player {
        Player {
            hit_points: self.hit_points,
            damage: self.damage + item.damage,
            armor: self.armor + item.armor,
        }
    }
}

impl MultilineFromStr for Player {
    type Err = ParseError;

    fn new() -> Self {
        Player {
            hit_points: 0,
            damage: 0,
            armor: 0,
        }
    }

    fn indicates_new_record(&self, _line: &str) -> bool {
        false
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        if let Some(v) = line.strip_prefix("Hit Points: ") {
            self.hit_points = v.parse()?;
        } else if let Some(v) = line.strip_prefix("Damage: ") {
            self.damage = v.parse()?;
        } else if let Some(v) = line.strip_prefix("Armor: ") {
            self.armor = v.parse()?;
        } else {
            return Err(parse_error!("Invalid input line: {}", line));
        }

        Ok(())
    }
}
