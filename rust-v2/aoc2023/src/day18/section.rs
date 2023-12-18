use std::fmt;

use crate::direction::Direction;
use crate::point::Point;

#[derive(Copy, Clone)]
pub struct Section {
    pub direction: Direction,
    pub from: Point,
    pub to: Point,
}

impl Section {
    pub fn new(from: Point, direction: Direction, length: usize) -> Section {
        Section {
            direction,
            from,
            to: direction.step(from, length),
        }
    }

    pub fn contains(&self, point: Point) -> bool {
        let row_min = self.from.row.min(self.to.row);
        let row_max = self.from.row.max(self.to.row);
        let col_min = self.from.col.min(self.to.col);
        let col_max = self.from.col.max(self.to.col);

        (row_min..=row_max).contains(&point.row) && (col_min..=col_max).contains(&point.col)
    }

    pub fn is_vertical(&self) -> bool {
        self.direction.is_down() || self.direction.is_up()
    }

    pub fn goes_up(&self, point: Point) -> bool {
        (self.direction.is_down() && self.from != point)
            || (self.direction.is_up() && self.to != point)
    }

    pub fn goes_down(&self, point: Point) -> bool {
        (self.direction.is_up() && self.from != point)
            || (self.direction.is_down() && self.to != point)
    }
}

impl fmt::Debug for Section {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Section({:?} -> {:?} [{:?}])",
            self.from, self.to, self.direction
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_1() {
        let section = Section::new(Point::new(0, 0), Direction::Down, 3);
        assert!(!section.contains(Point::new(-1, 0)));
        assert!(section.contains(Point::new(0, 0)));
        assert!(section.contains(Point::new(1, 0)));
        assert!(section.contains(Point::new(2, 0)));
        assert!(section.contains(Point::new(3, 0)));
        assert!(!section.contains(Point::new(4, 0)));
        assert!(!section.contains(Point::new(0, 1)));
        assert!(!section.contains(Point::new(1, 1)));
        assert!(!section.contains(Point::new(2, 1)));
        assert!(!section.contains(Point::new(3, 1)));
        assert!(!section.contains(Point::new(0, -1)));
        assert!(!section.contains(Point::new(1, -1)));
        assert!(!section.contains(Point::new(2, -1)));
        assert!(!section.contains(Point::new(3, -1)));
    }

    #[test]
    fn test_contains_2() {
        let section = Section::new(Point::new(3, 0), Direction::Up, 3);
        assert!(!section.contains(Point::new(-1, 0)));
        assert!(section.contains(Point::new(0, 0)));
        assert!(section.contains(Point::new(1, 0)));
        assert!(section.contains(Point::new(2, 0)));
        assert!(section.contains(Point::new(3, 0)));
        assert!(!section.contains(Point::new(4, 0)));
        assert!(!section.contains(Point::new(0, 1)));
        assert!(!section.contains(Point::new(1, 1)));
        assert!(!section.contains(Point::new(2, 1)));
        assert!(!section.contains(Point::new(3, 1)));
        assert!(!section.contains(Point::new(0, -1)));
        assert!(!section.contains(Point::new(1, -1)));
        assert!(!section.contains(Point::new(2, -1)));
        assert!(!section.contains(Point::new(3, -1)));
    }

    #[test]
    fn test_is_vertical() {
        assert!(Section::new(Point::new(0, 0), Direction::Up, 3).is_vertical());
        assert!(Section::new(Point::new(0, 0), Direction::Down, 3).is_vertical());
        assert!(!Section::new(Point::new(0, 0), Direction::Left, 3).is_vertical());
        assert!(!Section::new(Point::new(0, 0), Direction::Right, 3).is_vertical());
    }

    #[test]
    fn test_goes_up_1() {
        let section = Section::new(Point::new(0, 0), Direction::Down, 3);

        assert!(!section.goes_up(Point::new(0, 0)));
        assert!(section.goes_up(Point::new(1, 0)));
        assert!(section.goes_up(Point::new(2, 0)));
        assert!(section.goes_up(Point::new(3, 0)));
    }

    #[test]
    fn test_goes_up_2() {
        let section = Section::new(Point::new(3, 0), Direction::Up, 3);

        assert!(!section.goes_up(Point::new(0, 0)));
        assert!(section.goes_up(Point::new(1, 0)));
        assert!(section.goes_up(Point::new(2, 0)));
        assert!(section.goes_up(Point::new(3, 0)));
    }

    #[test]
    fn test_goes_down_1() {
        let section = Section::new(Point::new(0, 0), Direction::Down, 3);

        assert!(section.goes_down(Point::new(0, 0)));
        assert!(section.goes_down(Point::new(1, 0)));
        assert!(section.goes_down(Point::new(2, 0)));
        assert!(!section.goes_down(Point::new(3, 0)));
    }

    #[test]
    fn test_goes_down_2() {
        let section = Section::new(Point::new(3, 0), Direction::Up, 3);

        assert!(section.goes_down(Point::new(0, 0)));
        assert!(section.goes_down(Point::new(1, 0)));
        assert!(section.goes_down(Point::new(2, 0)));
        assert!(!section.goes_down(Point::new(3, 0)));
    }
}
