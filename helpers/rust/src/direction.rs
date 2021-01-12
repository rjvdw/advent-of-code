//! Represents one of the four cardinal directions.

/// There are four cardinal directions: Up, down, left, and right. You can also think of these as
/// north, south, west and east.
#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Given a position, determine the next position for a given direction.
pub trait CanTravel<T> {
    /// Travel in the specified direction.
    fn travel(&self, from: &T) -> T;

    /// Travel in the specified direction, and return None if this is not possible.
    fn travel_checked(&self, from: &T) -> Option<T>;
}

impl Direction {
    /// Turn one step to the left.
    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    /// Turn one step to the right.
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    /// Reverse direction.
    pub fn reverse(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    /// Turn an arbitrary number of steps. Positive numbers turn to the left, negative numbers turn
    /// to the right.
    pub fn turn(&self, steps: isize) -> Direction {
        let actual_steps = (4 + (steps % 4)) % 4;
        match actual_steps {
            0 => *self,
            1 => self.turn_left(),
            2 => self.reverse(),
            3 => self.turn_right(),
            _ => unreachable!(),
        }
    }
}

impl CanTravel<(i64, i64)> for Direction {
    fn travel(&self, (x, y): &(i64, i64)) -> (i64, i64) {
        match self {
            Direction::Up => (*x, *y - 1),
            Direction::Down => (*x, *y + 1),
            Direction::Left => (*x - 1, *y),
            Direction::Right => (*x + 1, *y),
        }
    }

    fn travel_checked(&self, from: &(i64, i64)) -> Option<(i64, i64)> {
        Some(self.travel(from))
    }
}

impl CanTravel<(usize, usize)> for Direction {
    fn travel(&self, (x, y): &(usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (*x, *y - 1),
            Direction::Down => (*x, *y + 1),
            Direction::Left => (*x - 1, *y),
            Direction::Right => (*x + 1, *y),
        }
    }

    fn travel_checked(&self, (x, y): &(usize, usize)) -> Option<(usize, usize)> {
        match self {
            Direction::Up if *y == 0 => None,
            Direction::Up => Some((*x, *y - 1)),
            Direction::Down => Some((*x, *y + 1)),
            Direction::Left if *x == 0 => None,
            Direction::Left => Some((*x - 1, *y)),
            Direction::Right => Some((*x + 1, *y)),
        }
    }
}
