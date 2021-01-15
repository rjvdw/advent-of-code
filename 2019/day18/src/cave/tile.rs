use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Tile {
    Entrance,
    Wall,
    Open,
    Door(char),
    Key(char),
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Entrance => write!(f, "@"),
            Tile::Wall => write!(f, "#"),
            Tile::Open => write!(f, "."),
            Tile::Door(ch) => write!(f, "{}", ch.to_ascii_uppercase()),
            Tile::Key(ch) => write!(f, "{}", ch),
        }
    }
}
