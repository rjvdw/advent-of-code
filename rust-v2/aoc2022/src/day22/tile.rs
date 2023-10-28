use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Default)]
pub enum Tile {
    Open,
    Closed,
    #[default]
    OutOfBounds,
}

impl Tile {
    pub fn is_within_bounds(&self) -> bool {
        !matches!(self, Tile::OutOfBounds)
    }
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Open,
            '#' => Tile::Closed,
            _ => Tile::OutOfBounds,
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Open => write!(f, "."),
            Tile::Closed => write!(f, "#"),
            Tile::OutOfBounds => write!(f, " "),
        }
    }
}
