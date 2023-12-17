use std::fmt;

use grid::Grid;

use rdcl_aoc_core::input::FromInput;
use rdcl_aoc_core::ParseResult;
use rdcl_aoc_pathfinding::{taxi_cab_2d, AStar};

const NORMAL_CRUCIBLE: Crucible = Crucible {
    min_count: 0,
    max_count: 3,
};
const ULTRA_CRUCIBLE: Crucible = Crucible {
    min_count: 4,
    max_count: 10,
};

#[derive(Debug, Clone)]
pub struct City {
    layout: Grid<u8>,
    crucible: Crucible,
    start: (usize, usize),
    end: (usize, usize),
}

impl City {
    pub fn find_optimal_route(&self) -> Option<u64> {
        let start = Step {
            point: self.start,
            direction: Direction::Right,
            count: 0,
        };
        let optimal_route = self.find_shortest_path(&start, &self.end)?;

        Some(
            optimal_route
                .iter()
                .skip(1)
                .map(|step| self.layout[step.point] as u64)
                .sum(),
        )
    }

    pub fn upgrade_to_ultra_crucibles(&mut self) {
        self.crucible = ULTRA_CRUCIBLE;
    }

    fn check_neighbour(&self, direction: Direction, step: Step) -> Option<(u64, Step)> {
        self.crucible
            .try_travel(direction, step, self.end)
            .map(|next| (self.layout[next.point] as u64, next))
    }
}

impl AStar for City {
    type Point = Step;
    type EndPoint = (usize, usize);

    fn distance_score(&self, a: &Self::Point, b: &Self::EndPoint) -> u64 {
        taxi_cab_2d(a.point, *b) as u64
    }

    fn get_neighbours(&self, point: &Self::Point) -> Vec<(u64, Self::Point)> {
        let mut neighbours = Vec::with_capacity(4);
        let (row, col) = point.point;

        if row > 0 {
            if let Some(next) = self.check_neighbour(Direction::Up, *point) {
                neighbours.push(next);
            }
        }
        if row + 1 < self.layout.rows() {
            if let Some(next) = self.check_neighbour(Direction::Down, *point) {
                neighbours.push(next);
            }
        }
        if col + 1 < self.layout.cols() {
            if let Some(next) = self.check_neighbour(Direction::Right, *point) {
                neighbours.push(next);
            }
        }
        if col > 0 {
            if let Some(next) = self.check_neighbour(Direction::Left, *point) {
                neighbours.push(next);
            }
        }

        neighbours
    }
}

impl FromInput for City {
    fn parse<T>(input: T) -> ParseResult<Self>
    where
        T: Iterator<Item = String>,
    {
        let mut layout = Grid::new(0, 0);

        for line in input {
            layout.push_row(line.bytes().map(|b| b - b'0').collect());
        }

        let start = (0, 0);
        let end = (layout.rows() - 1, layout.cols() - 1);

        Ok(City {
            layout,
            crucible: NORMAL_CRUCIBLE,
            start,
            end,
        })
    }
}

impl fmt::Display for City {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.layout.rows() {
            if row != 0 {
                writeln!(f)?;
            }
            for col in 0..self.layout.cols() {
                write!(f, "{}", self.layout[(row, col)])?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Step {
    point: (usize, usize),
    direction: Direction,
    count: usize,
}

impl Step {
    fn travel(&self, direction: Direction) -> Step {
        Step {
            point: direction.apply(self.point),
            direction,
            count: if direction == self.direction {
                self.count + 1
            } else {
                1
            },
        }
    }
}

impl PartialEq<(usize, usize)> for Step {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.point.eq(other)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn apply(&self, (row, col): (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Right => (row, col + 1),
            Direction::Left => (row, col - 1),
        }
    }

    fn is_reversed(&self, other: Direction) -> bool {
        match self {
            Direction::Up => other == Direction::Down,
            Direction::Down => other == Direction::Up,
            Direction::Right => other == Direction::Left,
            Direction::Left => other == Direction::Right,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Crucible {
    min_count: usize,
    max_count: usize,
}

impl Crucible {
    fn try_travel(&self, direction: Direction, step: Step, end: (usize, usize)) -> Option<Step> {
        let next = step.travel(direction);

        let is_legal = if direction.is_reversed(step.direction) {
            false
        } else if direction == step.direction || next.point == end {
            !self.must_turn(step.count)
        } else {
            self.can_turn(step.count)
        };

        if is_legal {
            Some(next)
        } else {
            None
        }
    }

    fn can_turn(&self, step_count: usize) -> bool {
        step_count >= self.min_count
    }

    fn must_turn(&self, step_count: usize) -> bool {
        step_count >= self.max_count
    }
}
