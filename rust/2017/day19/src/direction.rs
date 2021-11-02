#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn step(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Right => (x + 1, y),
            Direction::Left => (x - 1, y),
        }
    }
}
