use std::collections::{HashSet, VecDeque};
use std::fmt;

use grid::Grid;

use rdcl_aoc_core::input::FromInput;
use rdcl_aoc_core::ParseResult;
use Either::*;

type Point = (usize, usize);
type Beam = (Direction, Point);

#[derive(Debug, Copy, Clone)]
enum Either<L, R> {
    Left(L),
    Right(R),
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Space {
    #[default]
    Empty,
    DiagonalMirror,
    AntiDiagonalMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

impl Space {
    fn apply(&self, direction: Direction) -> Either<Direction, (Direction, Direction)> {
        match self {
            Space::Empty => Left(direction),
            Space::DiagonalMirror => match direction {
                Direction::North => Left(Direction::East),
                Direction::South => Left(Direction::West),
                Direction::East => Left(Direction::North),
                Direction::West => Left(Direction::South),
            },
            Space::AntiDiagonalMirror => match direction {
                Direction::North => Left(Direction::West),
                Direction::South => Left(Direction::East),
                Direction::East => Left(Direction::South),
                Direction::West => Left(Direction::North),
            },
            Space::HorizontalSplitter => match direction {
                Direction::North | Direction::South => Right((Direction::West, Direction::East)),
                direction => Left(direction),
            },
            Space::VerticalSplitter => match direction {
                Direction::East | Direction::West => Right((Direction::North, Direction::South)),
                direction => Left(direction),
            },
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone)]
pub struct Contraption {
    layout: Grid<Space>,
}

impl Contraption {
    pub fn count_energized_spaces_from_top_left(&self) -> usize {
        self.count_energized_spaces((Direction::East, (0, 0)))
    }

    pub fn find_optimal_entry_point(&self) -> (Point, usize) {
        let mut optimal = ((0, 0), 0);
        let rows = self.layout.rows();
        let cols = self.layout.cols();

        for row in 0..rows {
            let start = (row, 0);
            let count = self.count_energized_spaces((Direction::East, start));
            if count > optimal.1 {
                optimal = (start, count);
            }

            let start = (row, cols - 1);
            let count = self.count_energized_spaces((Direction::West, start));
            if count > optimal.1 {
                optimal = (start, count);
            }
        }

        for col in 0..cols {
            let start = (0, col);
            let count = self.count_energized_spaces((Direction::South, start));
            if count > optimal.1 {
                optimal = (start, count);
            }

            let start = (rows - 1, col);
            let count = self.count_energized_spaces((Direction::North, start));
            if count > optimal.1 {
                optimal = (start, count);
            }
        }

        optimal
    }

    fn count_energized_spaces(&self, start: Beam) -> usize {
        let mut beams: VecDeque<Beam> = VecDeque::new();
        let mut seen: HashSet<Beam> = HashSet::new();
        let mut energized: HashSet<Point> = HashSet::new();

        beams.push_back(start);

        while let Some(beam) = beams.pop_front() {
            energized.insert(beam.1);
            if seen.contains(&beam) {
                continue;
            }
            seen.insert(beam);

            for beam in self.propagate(beam) {
                beams.push_back(beam);
            }
        }

        energized.len()
    }

    fn propagate(&self, (direction, point): Beam) -> Vec<Beam> {
        let mut beams = Vec::with_capacity(2);

        match self.layout[point].apply(direction) {
            Left(d1) => {
                if let Some(p) = self.travel((d1, point)) {
                    beams.push((d1, p));
                }
            }
            Right((d1, d2)) => {
                if let Some(p) = self.travel((d1, point)) {
                    beams.push((d1, p));
                }
                if let Some(p) = self.travel((d2, point)) {
                    beams.push((d2, p));
                }
            }
        }

        beams
    }

    fn travel(&self, (direction, (row, col)): Beam) -> Option<Point> {
        let (row, col) = match direction {
            Direction::North => (row.checked_sub(1)?, col),
            Direction::South => (row + 1, col),
            Direction::East => (row, col + 1),
            Direction::West => (row, col.checked_sub(1)?),
        };

        if row < self.layout.rows() && col < self.layout.cols() {
            Some((row, col))
        } else {
            None
        }
    }
}

impl FromInput for Contraption {
    fn parse<T>(input: T) -> ParseResult<Self>
    where
        T: Iterator<Item = String>,
    {
        let mut contraption = Contraption {
            layout: Grid::new(0, 0),
        };

        for line in input {
            contraption.layout.push_row(
                line.chars()
                    .map(|ch| match ch {
                        '/' => Space::DiagonalMirror,
                        '\\' => Space::AntiDiagonalMirror,
                        '-' => Space::HorizontalSplitter,
                        '|' => Space::VerticalSplitter,
                        _ => Space::Empty,
                    })
                    .collect(),
            );
        }

        Ok(contraption)
    }
}

impl fmt::Display for Contraption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.layout.rows() {
            if row != 0 {
                writeln!(f)?;
            }
            for col in 0..self.layout.cols() {
                match self.layout[(row, col)] {
                    Space::Empty => write!(f, ".")?,
                    Space::DiagonalMirror => write!(f, "/")?,
                    Space::AntiDiagonalMirror => write!(f, "\\")?,
                    Space::HorizontalSplitter => write!(f, "-")?,
                    Space::VerticalSplitter => write!(f, "|")?,
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beam_path_1() {
        let mut contraption = Contraption {
            layout: Grid::new(2, 5),
        };
        contraption.layout[(0, 0)] = Space::AntiDiagonalMirror;

        assert_eq!(contraption.count_energized_spaces_from_top_left(), 2);
    }

    #[test]
    fn test_beam_path_2() {
        let mut contraption = Contraption {
            layout: Grid::new(2, 5),
        };
        contraption.layout[(0, 0)] = Space::DiagonalMirror;

        assert_eq!(contraption.count_energized_spaces_from_top_left(), 1);
    }

    #[test]
    fn test_beam_path_3() {
        let mut contraption = Contraption {
            layout: Grid::new(2, 5),
        };
        contraption.layout[(0, 0)] = Space::Empty;

        assert_eq!(contraption.count_energized_spaces_from_top_left(), 5);
    }

    #[test]
    fn test_beam_path_4() {
        let mut contraption = Contraption {
            layout: Grid::new(2, 5),
        };
        contraption.layout[(0, 0)] = Space::HorizontalSplitter;

        assert_eq!(contraption.count_energized_spaces_from_top_left(), 5);
    }

    #[test]
    fn test_beam_path_5() {
        let mut contraption = Contraption {
            layout: Grid::new(2, 5),
        };
        contraption.layout[(0, 0)] = Space::VerticalSplitter;

        assert_eq!(contraption.count_energized_spaces_from_top_left(), 2);
    }
}
