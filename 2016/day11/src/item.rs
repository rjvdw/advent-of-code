use std::str::FromStr;

use rdcl_aoc_helpers::err_parse_error;
use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Item {
    Generator(String),
    Microchip(String),
}

impl Item {
    pub fn is_generator(&self) -> bool {
        matches!(self, Item::Generator(_))
    }

    pub fn is_microchip(&self) -> bool {
        matches!(self, Item::Microchip(_))
    }

    pub fn get_label(&self) -> String {
        match self {
            Item::Generator(label) => label.to_string(),
            Item::Microchip(label) => label.to_string(),
        }
    }

    pub fn set_label(&mut self, new_label: String) {
        match self {
            Item::Generator(label) => *label = new_label,
            Item::Microchip(label) => *label = new_label,
        }
    }
}

impl FromStr for Item {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(r) = s.strip_suffix(" generator") {
            Ok(Item::Generator(r.to_string()))
        } else if let Some(r) = s.strip_suffix("-compatible microchip") {
            Ok(Item::Microchip(r.to_string()))
        } else {
            err_parse_error!("Invalid input: {}", s)
        }
    }
}
