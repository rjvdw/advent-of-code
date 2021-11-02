use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub(in crate::cave) enum Tile {
    Entrance,
    Empty,
    Wall,
    Key(char),
    Door(char),
}

impl Tile {
    pub(in crate::cave) fn is_open(&self, keys: &[char]) -> bool {
        match self {
            Tile::Entrance => true,
            Tile::Empty => true,
            Tile::Key(_) => true,
            Tile::Door(d) if keys.contains(d) => true,
            _ => false,
        }
    }

    pub(in crate::cave) fn get_key(&self) -> Option<char> {
        match self {
            Tile::Entrance => None,
            Tile::Empty => None,
            Tile::Wall => None,
            Tile::Key(ch) => Some(*ch),
            Tile::Door(ch) => Some(*ch),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Entrance => write!(f, "@"),
            Tile::Empty => write!(f, "."),
            Tile::Wall => write!(f, "#"),
            Tile::Key(key) => write!(f, "{}", key),
            Tile::Door(key) => write!(f, "{}", key.to_ascii_uppercase()),
        }
    }
}
