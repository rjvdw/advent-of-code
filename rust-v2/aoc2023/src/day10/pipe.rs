use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

use grid::Grid;
use rdcl_aoc2023::Enclosure;

use rdcl_aoc_core::input::FromInput;
use rdcl_aoc_core::{err_parse_error, ParseResult};

type Point = (usize, usize);

#[derive(Debug, Clone)]
pub struct PipeMap {
    /// The starting point.
    start: Point,

    /// The grid of all pipes.
    pipes: Grid<Pipe>,

    /// Once found, will contain all points inside the loop.
    points_in_loop: HashSet<Point>,
}

impl PipeMap {
    /// Find the loop in the input by searching for a point which has
    /// multiple routes from the start. This is done by applying a
    /// flood fill, starting at S.
    pub fn find_loop(&mut self) -> Option<usize> {
        // Keeps track of the path from start to a given point.
        let mut visited_from: HashMap<Point, Point> = HashMap::new();
        visited_from.insert(self.start, self.start);

        // The current search space.
        let mut to_visit: VecDeque<(Point, usize)> = VecDeque::from([(self.start, 0)]);

        while let Some((point, steps)) = to_visit.pop_back() {
            for next_point in self.neighbors(point) {
                if visited_from.contains_key(&next_point) {
                    if let Some(&prev_point) = visited_from.get(&point) {
                        // Verify that this is actually a loop, by checking the point isn't just the point we came from.
                        if next_point != prev_point {
                            // A loop was detected.
                            self.points_in_loop.clear();
                            self.points_in_loop.insert(self.start);
                            let p1 = self.update_loop(point, &visited_from);
                            let p2 = self.update_loop(next_point, &visited_from);

                            // Now that we have the loop, we can replace the starting position with the correct pipe.
                            self.update_start(p1, p2);

                            return Some(steps + 1);
                        }
                    }
                    continue;
                }

                visited_from.insert(next_point, point);
                to_visit.push_front((next_point, steps + 1));
            }
        }

        None
    }

    /// Count the number of points inside the loop. This method assumes
    /// that the `find_loop` method was previously called.
    pub fn count_points_inside_loop(&self) -> usize {
        self.compute_enclosure_size(false)
    }

    /// Find all points that can be reached from a given point. The
    /// starting point is treated as a four-way crossing.
    fn neighbors(&self, (row, col): Point) -> Vec<Point> {
        let mut result = Vec::with_capacity(4);
        let pipe = self.pipes[(row, col)];

        if row > 0 && pipe.connects_north() {
            let next = (row - 1, col);
            if self.pipes[next].connects_south() {
                result.push(next);
            }
        }

        if row < self.pipes.rows() - 1 && pipe.connects_south() {
            let next = (row + 1, col);
            if self.pipes[next].connects_north() {
                result.push(next);
            }
        }

        if col > 0 && pipe.connects_west() {
            let next = (row, col - 1);
            if self.pipes[next].connects_east() {
                result.push(next);
            }
        }

        if col < self.pipes.cols() - 1 && pipe.connects_east() {
            let next = (row, col + 1);
            if self.pipes[next].connects_west() {
                result.push(next);
            }
        }

        result
    }

    /// Update the loop using the data found during the execution of
    /// `find_loop`. This is done by traversing back from the points
    /// that were found to complete the loop. The first point after the
    /// start is returned, so that the start point can be updated.
    fn update_loop(&mut self, point: Point, visited_from: &HashMap<Point, Point>) -> Point {
        let mut current = point;
        self.points_in_loop.insert(current);
        while let Some(&prev) = visited_from.get(&current) {
            if prev == self.start {
                break;
            }
            current = prev;
            self.points_in_loop.insert(current);
        }

        current
    }

    /// To simplify further operations, the starting point can now be
    /// replaced by its actual pipe. This is done by checking how it
    /// connects to its neighbors.
    ///
    /// - `p1` - The first point that is joined with the start.
    /// - `p2` - The second point that is joined with the start.
    fn update_start(&mut self, p1: Point, p2: Point) {
        let d1 = Direction::from(self.start, p1).unwrap();
        let d2 = Direction::from(self.start, p2).unwrap();

        let pipe = match (d1, d2) {
            (Direction::North, Direction::South) | (Direction::South, Direction::North) => {
                Pipe::NorthSouth
            }
            (Direction::East, Direction::West) | (Direction::West, Direction::East) => {
                Pipe::EastWest
            }
            (Direction::North, Direction::East) | (Direction::East, Direction::North) => {
                Pipe::NorthEast
            }
            (Direction::North, Direction::West) | (Direction::West, Direction::North) => {
                Pipe::NorthWest
            }
            (Direction::South, Direction::East) | (Direction::East, Direction::South) => {
                Pipe::SouthEast
            }
            (Direction::South, Direction::West) | (Direction::West, Direction::South) => {
                Pipe::SouthWest
            }
            _ => unreachable!(),
        };

        self.pipes[self.start] = pipe;
    }
}

impl Enclosure for PipeMap {
    type Index = usize;
    type IsEdgeResult = (usize, usize);

    fn row_indices(&self) -> Box<dyn Iterator<Item = usize>> {
        Box::new(0..self.pipes.rows())
    }

    fn col_indices(&self) -> Box<dyn Iterator<Item = usize>> {
        Box::new(0..self.pipes.cols())
    }

    fn is_edge(&self, row: usize, col: usize) -> Option<Self::IsEdgeResult> {
        let point = (row, col);
        if self.points_in_loop.contains(&point) {
            Some(point)
        } else {
            None
        }
    }

    fn edge_goes_down(&self, point: Self::IsEdgeResult) -> bool {
        self.pipes[point].connects_south()
    }

    fn edge_goes_up(&self, point: Self::IsEdgeResult) -> bool {
        self.pipes[point].connects_north()
    }
}

impl FromInput for PipeMap {
    fn parse<T>(input: T) -> ParseResult<Self>
    where
        T: Iterator<Item = String>,
    {
        let mut map = PipeMap {
            start: (0, 0),
            pipes: Grid::new(0, 0),
            points_in_loop: HashSet::new(),
        };

        for (row, line) in input.enumerate() {
            let mut pipes = Vec::with_capacity(line.len());
            for (col, ch) in line.chars().enumerate() {
                let pipe = Pipe::of(ch)?;
                pipes.push(pipe);
                if pipe.is_start() {
                    map.start = (row, col);
                }
            }
            map.pipes.push_row(pipes);
        }

        Ok(map)
    }
}

impl fmt::Display for PipeMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.pipes.rows() {
            for col in 0..self.pipes.cols() {
                let pipe = self.pipes[(row, col)];
                write!(f, "{pipe}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Pipe {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    #[default]
    Ground,
    Start,
}

impl Pipe {
    pub fn of(value: char) -> ParseResult<Pipe> {
        match value {
            '|' => Ok(Pipe::NorthSouth),
            '-' => Ok(Pipe::EastWest),
            'L' => Ok(Pipe::NorthEast),
            'J' => Ok(Pipe::NorthWest),
            '7' => Ok(Pipe::SouthWest),
            'F' => Ok(Pipe::SouthEast),
            '.' => Ok(Pipe::Ground),
            'S' => Ok(Pipe::Start),
            _ => err_parse_error!("Invalid pipe: {}", value),
        }
    }

    pub fn is_start(&self) -> bool {
        matches!(self, Pipe::Start)
    }

    pub fn connects_north(&self) -> bool {
        matches!(
            self,
            Pipe::Start | Pipe::NorthSouth | Pipe::NorthEast | Pipe::NorthWest
        )
    }

    pub fn connects_south(&self) -> bool {
        matches!(
            self,
            Pipe::Start | Pipe::NorthSouth | Pipe::SouthEast | Pipe::SouthWest
        )
    }

    pub fn connects_east(&self) -> bool {
        matches!(
            self,
            Pipe::Start | Pipe::EastWest | Pipe::NorthEast | Pipe::SouthEast
        )
    }

    pub fn connects_west(&self) -> bool {
        matches!(
            self,
            Pipe::Start | Pipe::EastWest | Pipe::NorthWest | Pipe::SouthWest
        )
    }
}

impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pipe::NorthSouth => write!(f, "│"),
            Pipe::EastWest => write!(f, "─"),
            Pipe::NorthEast => write!(f, "└"),
            Pipe::NorthWest => write!(f, "┘"),
            Pipe::SouthWest => write!(f, "┐"),
            Pipe::SouthEast => write!(f, "┌"),
            Pipe::Ground => write!(f, "."),
            Pipe::Start => write!(f, "S"),
        }
    }
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Check whether two points are adjacent and if so in which
    /// direction.
    fn from(one: Point, other: Point) -> Option<Direction> {
        if one.0 + 1 == other.0 && one.1 == other.1 {
            Some(Direction::South)
        } else if one.0 == other.0 + 1 && one.1 == other.1 {
            Some(Direction::North)
        } else if one.0 == other.0 && one.1 + 1 == other.1 {
            Some(Direction::East)
        } else if one.0 == other.0 && one.1 == other.1 + 1 {
            Some(Direction::West)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_pipe() {
        let pipe = Pipe::of('|').unwrap();
        assert!(pipe.connects_north());
        assert!(pipe.connects_south());
        assert!(!pipe.connects_east());
        assert!(!pipe.connects_west());

        let pipe = Pipe::of('-').unwrap();
        assert!(!pipe.connects_north());
        assert!(!pipe.connects_south());
        assert!(pipe.connects_east());
        assert!(pipe.connects_west());

        let pipe = Pipe::of('L').unwrap();
        assert!(pipe.connects_north());
        assert!(!pipe.connects_south());
        assert!(pipe.connects_east());
        assert!(!pipe.connects_west());

        let pipe = Pipe::of('J').unwrap();
        assert!(pipe.connects_north());
        assert!(!pipe.connects_south());
        assert!(!pipe.connects_east());
        assert!(pipe.connects_west());

        let pipe = Pipe::of('F').unwrap();
        assert!(!pipe.connects_north());
        assert!(pipe.connects_south());
        assert!(pipe.connects_east());
        assert!(!pipe.connects_west());

        let pipe = Pipe::of('7').unwrap();
        assert!(!pipe.connects_north());
        assert!(pipe.connects_south());
        assert!(!pipe.connects_east());
        assert!(pipe.connects_west());

        let pipe = Pipe::of('.').unwrap();
        assert!(!pipe.connects_north());
        assert!(!pipe.connects_south());
        assert!(!pipe.connects_east());
        assert!(!pipe.connects_west());

        let pipe = Pipe::of('S').unwrap();
        assert!(pipe.connects_north());
        assert!(pipe.connects_south());
        assert!(pipe.connects_east());
        assert!(pipe.connects_west());
    }
}
