use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) struct Point {
    pub(crate) row: i64,
    pub(crate) col: i64,
}

impl Point {
    pub(crate) fn new(row: i64, col: i64) -> Point {
        Point { row, col }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point {{ row: {:?}, col: {:?} }}", self.row, self.col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fmt_display() {
        let point = Point::new(3, 5);
        assert_eq!(format!("{}", point), "(3, 5)");
    }

    #[test]
    fn test_fmt_debug() {
        let point = Point::new(3, 5);
        assert_eq!(format!("{:?}", point), "Point { row: 3, col: 5 }");
    }
}
