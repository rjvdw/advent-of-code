use std::fmt;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn travel(&self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
        }
    }

    pub fn neighbours(position: (usize, usize)) -> Vec<(usize, usize)> {
        let mut positions = Vec::with_capacity(5);
        positions.push(position);
        positions.push(Direction::Right.travel(position));
        positions.push(Direction::Down.travel(position));
        if position.0 > 0 {
            positions.push(Direction::Left.travel(position));
        }
        if position.1 > 0 {
            positions.push(Direction::Up.travel(position));
        }

        positions
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => panic!("{value} is not a valid direction"),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Right => write!(f, ">"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
        }
    }
}
