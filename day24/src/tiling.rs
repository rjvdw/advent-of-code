use std::ops::Add;
use std::str::FromStr;

use helpers::parse_error::ParseError;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Coord(pub i32, pub i32);

impl Coord {
    pub fn neighbours(self) -> [Coord; 6] {
        [
            Direction::East.walk(self),
            Direction::SouthEast.walk(self),
            Direction::SouthWest.walk(self),
            Direction::West.walk(self),
            Direction::NorthWest.walk(self),
            Direction::NorthEast.walk(self),
        ]
    }
}

impl Add<(i32, i32)> for Coord {
    type Output = Coord;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    fn walk(&self, position: Coord) -> Coord {
        match self {
            Direction::East => position + (1, 0),
            Direction::SouthEast => position + (0, -1),
            Direction::SouthWest => position + (-1, -1),
            Direction::West => position + (-1, 0),
            Direction::NorthWest => position + (0, 1),
            Direction::NorthEast => position + (1, 1),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Path(Vec<Direction>);

impl Path {
    pub fn walk(&self, position: Coord) -> Coord {
        self.0
            .iter()
            .fold(position, |position, step| step.walk(position))
    }
}

impl FromStr for Path {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut directions = Vec::new();

        let mut chars = s.chars();

        while let Some(ch1) = chars.next() {
            if let Some(direction) = match ch1 {
                'e' => Some(Direction::East),
                'w' => Some(Direction::West),
                's' | 'n' => {
                    if let Some(ch2) = chars.next() {
                        match (ch1, ch2) {
                            ('s', 'e') => Some(Direction::SouthEast),
                            ('s', 'w') => Some(Direction::SouthWest),
                            ('n', 'e') => Some(Direction::NorthEast),
                            ('n', 'w') => Some(Direction::NorthWest),
                            _ => None,
                        }
                    } else {
                        None
                    }
                }
                _ => None,
            } {
                directions.push(direction);
            } else {
                return Err(ParseError(format!("Invalid input line: {}", s)));
            }
        }

        Ok(Path(directions))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let path = "esenee".parse::<Path>().unwrap();
        assert_eq!(path.walk(Coord(0, 0)), Coord(3, 0));
    }

    #[test]
    fn test_2() {
        let path = "esew".parse::<Path>().unwrap();
        assert_eq!(path.walk(Coord(0, 0)), Coord(0, -1));
    }

    #[test]
    fn test_3() {
        let path = "nwwswee".parse::<Path>().unwrap();
        assert_eq!(path.walk(Coord(0, 0)), Coord(0, 0));
    }
}
