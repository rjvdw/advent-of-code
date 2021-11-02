use std::fmt;

#[derive(Debug, Copy, Clone)]
pub(crate) enum Direction {
    Right,
    Left,
}

impl Direction {
    pub(crate) fn run(&self, cursor: i64) -> i64 {
        match self {
            Direction::Right => cursor + 1,
            Direction::Left => cursor - 1,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Right => write!(f, "right"),
            Direction::Left => write!(f, "left"),
        }
    }
}
