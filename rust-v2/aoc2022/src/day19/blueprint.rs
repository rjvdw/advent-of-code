use std::str::FromStr;

use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::parser::Parser;

use crate::pool::Resources;

#[derive(Debug, Eq, PartialEq)]
pub struct Blueprint {
    pub id: u32,
    pub ore_robot: Resources,
    pub clay_robot: Resources,
    pub obsidian_robot: Resources,
    pub geode_robot: Resources,
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
            ore_robot: Resources {
                ore: ore_robot_ore,
                ..Default::default()
            },
            clay_robot: Resources {
                ore: clay_robot_ore,
                ..Default::default()
            },
            obsidian_robot: Resources {
                ore: obsidian_robot_ore,
                clay: obsidian_robot_clay,
                ..Default::default()
            },
            geode_robot: Resources {
                ore: geode_robot_ore,
                obsidian: geode_robot_obsidian,
                ..Default::default()
            },
        })
    }
}

#[cfg(test)]
pub mod tests {
    use rdcl_aoc_core::input::InputReader;

    use super::*;

    pub fn test_data() -> Vec<Blueprint> {
        InputReader::from("./src/day19/test.txt")
            .parse_lines(Blueprint::from_str)
            .collect()
    }

    #[test]
    fn test_from_str() {
        assert_eq!(
            test_data(),
            vec![
                Blueprint {
                    id: 1,
                    ore_robot: Resources::new(4, 0, 0, 0),
                    clay_robot: Resources::new(2, 0, 0, 0),
                    obsidian_robot: Resources::new(3, 14, 0, 0),
                    geode_robot: Resources::new(2, 0, 7, 0),
                },
                Blueprint {
                    id: 2,
                    ore_robot: Resources::new(2, 0, 0, 0),
                    clay_robot: Resources::new(3, 0, 0, 0),
                    obsidian_robot: Resources::new(3, 8, 0, 0),
                    geode_robot: Resources::new(3, 0, 12, 0),
                },
            ]
        );
    }
}
