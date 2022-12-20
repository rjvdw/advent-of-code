use std::str::FromStr;

use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::parser::Parser;

#[derive(Debug)]
pub struct Blueprint {
    id: u8,
    ore_robot: u8,
    clay_robot: u8,
    obsidian_robot: (u8, u8),
    geode_robot: (u8, u8),
}

impl FromStr for Blueprint {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parser = Parser::new(s);

        parser.skip_text("Blueprint ")?;
        let id = parser.take_value_upto(": ")?;
        parser.skip_text("Each ore robot costs ")?;
        let ore_robot_ore = parser.take_value_upto(" ore. ")?;
        parser.skip_text("Each clay robot costs ")?;
        let clay_robot_ore = parser.take_value_upto(" ore. ")?;
        parser.skip_text("Each obsidian robot costs ")?;
        let obsidian_robot_ore = parser.take_value_upto(" ore and ")?;
        let obsidian_robot_clay = parser.take_value_upto(" clay. ")?;
        parser.skip_text("Each geode robot costs ")?;
        let geode_robot_ore = parser.take_value_upto(" ore and ")?;
        let geode_robot_obsidian = parser.take_value_upto(" obsidian.")?;

        Ok(Blueprint {
            id,
            ore_robot: ore_robot_ore,
            clay_robot: clay_robot_ore,
            obsidian_robot: (obsidian_robot_ore, obsidian_robot_clay),
            geode_robot: (geode_robot_ore, geode_robot_obsidian),
        })
    }
}
