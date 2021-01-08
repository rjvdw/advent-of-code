use crate::tool::Tool;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Tile {
    Rocky,
    Narrow,
    Wet,
}

impl Tile {
    pub fn suitable_tools(&self) -> Vec<Tool> {
        match self {
            Tile::Rocky => vec![Tool::ClimbingGear, Tool::Torch],
            Tile::Narrow => vec![Tool::Torch, Tool::Neither],
            Tile::Wet => vec![Tool::ClimbingGear, Tool::Neither],
        }
    }
}
