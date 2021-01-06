/// A single space within the cave.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(crate) enum Tile {
    /// Walls cannot be traversed.
    Wall,

    /// Empty spaces can be traversed.
    Empty,
}

impl Tile {
    /// Indicates whether a tile can be traversed.
    pub(crate) fn is_empty(&self) -> bool {
        matches!(self, Tile::Empty)
    }
}
