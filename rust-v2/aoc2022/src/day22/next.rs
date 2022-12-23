use crate::direction::Direction;
use crate::face::Face;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Next {
    pub position: (usize, usize),
    pub direction: Direction,
    pub face: Face,
}

impl Next {
    pub fn x(&self) -> usize {
        self.position.0
    }

    pub fn y(&self) -> usize {
        self.position.1
    }
}
