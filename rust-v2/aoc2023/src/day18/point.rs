use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Point {
    pub row: i32,
    pub col: i32,
}

impl Point {
    pub fn new(row: i32, col: i32) -> Point {
        Point { row, col }
    }
}

impl From<(i32, i32)> for Point {
    fn from((row, col): (i32, i32)) -> Self {
        Point::new(row, col)
    }
}

impl PartialEq<(i32, i32)> for Point {
    fn eq(&self, other: &(i32, i32)) -> bool {
        self.row == other.0 && self.col == other.1
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point({}, {})", self.row, self.col)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}
