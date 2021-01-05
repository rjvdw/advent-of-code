#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub(in crate::combat) enum Tile {
    Wall,
    Empty,
}

impl Tile {
    // pub(in crate::combat) fn is_wall(&self) -> bool {
    //     matches!(self, Tile::Wall)
    // }

    pub(in crate::combat) fn is_empty(&self) -> bool {
        matches!(self, Tile::Empty)
    }
}
