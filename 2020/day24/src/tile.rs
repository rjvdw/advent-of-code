use std::ops::Add;
use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Tile(pub i32, pub i32);

impl Tile {
    const ORIGIN: Tile = Tile(0, 0);

    pub fn neighbours(&self) -> [Tile; 6] {
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

impl Add<(i32, i32)> for Tile {
    type Output = Tile;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Tile(self.0 + rhs.0, self.1 + rhs.1)
    }
}

enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    fn walk(&self, position: &Tile) -> Tile {
        match self {
            Direction::East => position.add((1, 0)),
            Direction::SouthEast => position.add((0, -1)),
            Direction::SouthWest => position.add((-1, -1)),
            Direction::West => position.add((-1, 0)),
            Direction::NorthWest => position.add((0, 1)),
            Direction::NorthEast => position.add((1, 1)),
        }
    }
}

impl FromStr for Tile {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let mut position = Tile::ORIGIN;

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
                position = direction.walk(&position);
            } else {
                return Err(parse_error!("Invalid input line: {}", s));
            }
        }

        Ok(position)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let tile = "esenee".parse::<Tile>().unwrap();
        assert_eq!(tile, Tile(3, 0));
    }

    #[test]
    fn test_2() {
        let tile = "esew".parse::<Tile>().unwrap();
        assert_eq!(tile, Tile(0, -1));
    }

    #[test]
    fn test_3() {
        let tile = "nwwswee".parse::<Tile>().unwrap();
        assert_eq!(tile, Tile(0, 0));
    }
}
