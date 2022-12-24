use grid::Grid;

use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::input::FromInput;
use rdcl_aoc_pathfinding::{taxi_cab_2d, AStar};

#[derive(Debug)]
pub struct Heightmap {
    grid: Grid<u8>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Heightmap {
    pub fn find_shortest_path(&self) -> Vec<(usize, usize)> {
        AStar::find_shortest_path(self, &self.start, &self.end).unwrap()
    }

    pub fn find_shortest_path_with_alternative_starting_point(
        &self,
    ) -> ((usize, usize), Vec<(usize, usize)>) {
        let mut best_so_far = (self.start, self.find_shortest_path());
        for y in 0..self.grid.rows() {
            for x in 0..self.grid.cols() {
                if (x, y) != self.start && self.height_at(&(x, y)) == b'a' {
                    let option = AStar::find_shortest_path(self, &(x, y), &self.end);
                    if let Some(path) = option {
                        if path.len() < best_so_far.1.len() {
                            best_so_far = ((x, y), path);
                        }
                    }
                }
            }
        }
        best_so_far
    }

    fn height_at(&self, point: &(usize, usize)) -> u8 {
        let x = point.0;
        let y = point.1;
        self.grid[y][x]
    }
}

impl FromInput for Heightmap {
    fn parse<T>(input: T) -> Result<Self, ParseError>
    where
        T: Iterator<Item = String>,
    {
        let mut grid = Grid::new(0, 0);
        let mut start = (0, 0);
        let mut end = (0, 0);

        for (y, line) in input.enumerate() {
            let mut row = Vec::with_capacity(line.len());
            for (x, ch) in line.chars().enumerate() {
                let height = match ch {
                    'S' => {
                        start = (x, y);
                        b'a'
                    }
                    'E' => {
                        end = (x, y);
                        b'z'
                    }
                    v => v as u8,
                };
                row.push(height);
            }
            grid.push_row(row);
        }

        Ok(Heightmap { grid, start, end })
    }
}

impl AStar for Heightmap {
    type Point = (usize, usize);

    fn distance_score(&self, a: &Self::Point, b: &Self::Point) -> u64 {
        let a = (a.0 as u64, a.1 as u64);
        let b = (b.0 as u64, b.1 as u64);
        taxi_cab_2d(a, b)
    }

    fn get_neighbours(&self, point: &Self::Point) -> Vec<(u64, Self::Point)> {
        let max_height = self.height_at(point) + 1;
        let mut neighbours = vec![];

        let not_too_high = |p: &Self::Point| self.height_at(p) <= max_height;

        if point.0 > 0 {
            let p = (point.0 - 1, point.1);
            if not_too_high(&p) {
                neighbours.push((1, p));
            }
        }

        if point.0 < self.grid.cols() - 1 {
            let p = (point.0 + 1, point.1);
            if not_too_high(&p) {
                neighbours.push((1, p));
            }
        }

        if point.1 > 0 {
            let p = (point.0, point.1 - 1);
            if not_too_high(&p) {
                neighbours.push((1, p));
            }
        }

        if point.1 < self.grid.rows() - 1 {
            let p = (point.0, point.1 + 1);
            if not_too_high(&p) {
                neighbours.push((1, p));
            }
        }

        neighbours
    }
}
