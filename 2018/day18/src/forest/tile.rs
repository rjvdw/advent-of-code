#[derive(Debug, Copy, Clone)]
pub(crate) enum Tile {
    Open,
    Trees,
    Lumberyard,
}

impl Tile {
    pub(crate) fn get_state(&self) -> char {
        match self {
            Tile::Open => 'o',
            Tile::Trees => 't',
            Tile::Lumberyard => 'y',
        }
    }
}
