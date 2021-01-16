#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum Tile {
    Wall,
    Open,
    Portal(String),
    Empty,
}

impl Tile {
    pub(crate) fn is_open(&self) -> bool {
        matches!(self, Tile::Open) || matches!(self, Tile::Portal(_))
    }
}
