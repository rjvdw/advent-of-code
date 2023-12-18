use crate::point::Point;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn step(&self, Point { row, col }: Point, count: usize) -> Point {
        let count = count as i32;
        match self {
            Direction::Up => Point::new(row - count, col),
            Direction::Down => Point::new(row + count, col),
            Direction::Left => Point::new(row, col - count),
            Direction::Right => Point::new(row, col + count),
        }
    }

    pub fn is_up(&self) -> bool {
        matches![self, Direction::Up]
    }

    pub fn is_down(&self) -> bool {
        matches![self, Direction::Down]
    }
}
