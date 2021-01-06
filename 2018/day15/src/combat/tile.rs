/// A single space within the cave.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::combat) enum Tile {
    /// Walls cannot be traversed.
    Wall,

    /// Empty spaces can be traversed.
    Empty,
}

impl Tile {
    /// Indicates whether a tile can be traversed.
    pub(in crate::combat) fn is_empty(&self) -> bool {
        matches!(self, Tile::Empty)
    }
}
