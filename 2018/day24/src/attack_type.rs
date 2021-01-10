use std::fmt;
use std::str::FromStr;

use rdcl_aoc_helpers::err_parse_error;
use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum AttackType {
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
    Slashing,
}

impl fmt::Display for AttackType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttackType::Bludgeoning => write!(f, "bludgeoning"),
            AttackType::Cold => write!(f, "cold"),
            AttackType::Fire => write!(f, "fire"),
            AttackType::Radiation => write!(f, "radiation"),
            AttackType::Slashing => write!(f, "slashing"),
        }
    }
}

impl FromStr for AttackType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bludgeoning" => Ok(AttackType::Bludgeoning),
            "cold" => Ok(AttackType::Cold),
            "fire" => Ok(AttackType::Fire),
            "radiation" => Ok(AttackType::Radiation),
            "slashing" => Ok(AttackType::Slashing),
            _ => err_parse_error!("invalid attack type: {}", s),
        }
    }
}
