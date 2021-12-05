use std::cmp::Ordering;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

use crate::point::Point;

static SEPARATOR: &str = " -> ";

#[derive(Debug, Copy, Clone)]
pub struct Line(Point, Point);

impl Line {
    pub fn is_horizontal(&self) -> bool {
        self.0.y == self.1.y
    }

    pub fn is_vertical(&self) -> bool {
        self.0.x == self.1.x
    }

    pub fn iter(self) -> LineIterator {
        LineIterator {
            from: self.0,
            to: self.1,
            done: false,
        }
    }
}

impl IntoIterator for Line {
    type Item = Point;
    type IntoIter = LineIterator;

    fn into_iter(self) -> Self::IntoIter {
        LineIterator {
            from: self.0,
            to: self.1,
            done: false,
        }
    }
}

impl IntoIterator for &Line {
    type Item = Point;
    type IntoIter = LineIterator;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct LineIterator {
    from: Point,
    to: Point,
    done: bool,
}

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else if self.from == self.to {
            self.done = true;
            Some(self.from)
        } else {
            let point = self.from;

            self.from.x += diff(self.from.x, self.to.x);
            self.from.y += diff(self.from.y, self.to.y);

            Some(point)
        }
    }
}

fn diff(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

impl FromStr for Line {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.find(SEPARATOR) {
            Some(p) => Ok(Line(s[..p].parse()?, s[p + SEPARATOR.len()..].parse()?)),
            None => Err(parse_error!("Invalid input: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_points_1() {
        let line = "1,1 -> 1,3".parse::<Line>().unwrap();

        assert_eq!(
            line.iter().collect::<Vec<Point>>(),
            vec![
                Point { x: 1, y: 1 },
                Point { x: 1, y: 2 },
                Point { x: 1, y: 3 },
            ]
        );
    }

    #[test]
    fn test_get_points_2() {
        let line = "9,7 -> 7,7".parse::<Line>().unwrap();

        assert_eq!(
            line.iter().collect::<Vec<Point>>(),
            vec![
                Point { x: 9, y: 7 },
                Point { x: 8, y: 7 },
                Point { x: 7, y: 7 },
            ]
        );
    }

    #[test]
    fn test_get_points_3() {
        let line = "1,1 -> 3,3".parse::<Line>().unwrap();

        assert_eq!(
            line.iter().collect::<Vec<Point>>(),
            vec![
                Point { x: 1, y: 1 },
                Point { x: 2, y: 2 },
                Point { x: 3, y: 3 },
            ]
        );
    }

    #[test]
    fn test_get_points_4() {
        let line = "9,7 -> 7,9".parse::<Line>().unwrap();

        assert_eq!(
            line.iter().collect::<Vec<Point>>(),
            vec![
                Point { x: 9, y: 7 },
                Point { x: 8, y: 8 },
                Point { x: 7, y: 9 },
            ]
        );
    }
}
