use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::err_parse_error;
use rdcl_aoc_helpers::error::ParseError;

pub fn parse_inventory_file(path: &str) -> Result<Vec<Item>, ParseError> {
    let file = File::open(path)?;
    let mut category = ItemCategory::Unknown;
    let mut inventory = Vec::new();
    let mut columns = (0, 0, 0);
    for line in BufReader::new(file).lines() {
        let line = line?;
        if line.starts_with("Weapons:") {
            category = ItemCategory::Weapon;
            match get_columns(&line) {
                Some(c) => columns = c,
                None => return err_parse_error!("invalid line: {}", line),
            }
        } else if line.starts_with("Armor:") {
            category = ItemCategory::Armor;
            match get_columns(&line) {
                Some(c) => columns = c,
                None => return err_parse_error!("invalid line: {}", line),
            }
        } else if line.starts_with("Rings:") {
            category = ItemCategory::Ring;
            match get_columns(&line) {
                Some(c) => columns = c,
                None => return err_parse_error!("invalid line: {}", line),
            }
        } else if !line.is_empty() {
            let (cost_idx, damage_idx, armor_idx) = columns;

            let name = line[..cost_idx].trim().to_string();
            let cost = line[cost_idx..damage_idx].trim().parse()?;
            let damage = line[damage_idx..armor_idx].trim().parse()?;
            let armor = line[armor_idx..].trim().parse()?;

            inventory.push(Item {
                category,
                name,
                cost,
                damage,
                armor,
            });
        }
    }
    Ok(inventory)
}

fn get_columns(line: &str) -> Option<(usize, usize, usize)> {
    let cost_idx = line.find("Cost")?;
    let damage_idx = line.find("Damage")?;
    let armor_idx = line[1..].find("Armor")?;

    Some((cost_idx, damage_idx, armor_idx))
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ItemCategory {
    Unknown,
    Weapon,
    Armor,
    Ring,
}

#[derive(Debug, Clone)]
pub struct Item {
    category: ItemCategory,
    name: String,
    cost: u32,
    pub(in crate::game_objects) damage: u32,
    pub(in crate::game_objects) armor: u32,
}

impl Item {
    pub fn get_category(&self) -> ItemCategory {
        self.category
    }

    pub fn get_cost(&self) -> u32 {
        self.cost
    }
}
