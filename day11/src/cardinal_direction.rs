#[derive(Clone, Copy, Debug, Hash)]
pub enum CardinalDirection {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

pub const ALL_DIRECTIONS: [CardinalDirection; 8] = [
    CardinalDirection::North,
    CardinalDirection::NorthEast,
    CardinalDirection::East,
    CardinalDirection::SouthEast,
    CardinalDirection::South,
    CardinalDirection::SouthWest,
    CardinalDirection::West,
    CardinalDirection::NorthWest,
];

impl CardinalDirection {
    pub fn predicate(&self, x: usize, y: usize, max_x: usize, max_y: usize, i: usize) -> bool {
        match self {
            Self::North => x >= i,
            Self::NorthEast => x >= i && y + i < max_y,
            Self::East => y + i < max_y,
            Self::SouthEast => x + i < max_x && y + i < max_y,
            Self::South => x + i < max_x,
            Self::SouthWest => x + i < max_x && y >= i,
            Self::West => y >= i,
            Self::NorthWest => x >= i && y >= i,
        }
    }

    pub fn mapper(&self, x: usize, y: usize, i: usize) -> (usize, usize) {
        match self {
            Self::North => (x - i, y),
            Self::NorthEast => (x - i, y + i),
            Self::East => (x, y + i),
            Self::SouthEast => (x + i, y + i),
            Self::South => (x + i, y),
            Self::SouthWest => (x + i, y - i),
            Self::West => (x, y - i),
            Self::NorthWest => (x - i, y - i),
        }
    }
}
