use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(in crate::cave) enum Tile {
    Entrance,
    Open,
    Wall,
    Key(char),
    Door(char),
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Entrance => write!(f, "@"),
            Tile::Open => write!(f, "."),
            Tile::Wall => write!(f, "#"),
            Tile::Key(key) => write!(f, "{}", key),
            Tile::Door(key) => write!(f, "{}", key.to_ascii_uppercase()),
        }
    }
}
