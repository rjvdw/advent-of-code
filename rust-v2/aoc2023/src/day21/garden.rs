use std::collections::{HashSet, VecDeque};

use grid::Grid;

use rdcl_aoc_core::input::FromInput;
use rdcl_aoc_core::{err_parse_error, ParseResult};

#[derive(Debug, Clone)]
pub struct Garden {
    map: Grid<Plot>,
    starting_position: (i64, i64),
}

impl Garden {
    pub fn start_walking(&self, steps: usize, infinite: bool) -> usize {
        let mut positions: VecDeque<((i64, i64), usize)> = VecDeque::new();
        positions.push_back((self.starting_position, steps));
        let mut seen: HashSet<(i64, i64)> = HashSet::new();

        while let Some((position, steps)) = positions.pop_front() {
            if steps == 0 {
                continue;
            }
            let steps = steps - 1;
            for neighbour in self.map.neighbours(position, infinite) {
                if !seen.contains(&neighbour) {
                    seen.insert(neighbour);
                    positions.push_back((neighbour, steps));
                }
            }
        }

        seen.iter()
            .filter(|p| self.starting_position.compare_colors(p, steps))
            .count()
    }
}

impl FromInput for Garden {
    fn parse<T>(input: T) -> ParseResult<Self>
    where
        T: Iterator<Item = String>,
    {
        let mut garden = Garden {
            map: Grid::new(0, 0),
            starting_position: (0, 0),
        };

        for (row_idx, line) in input.enumerate() {
            let mut row = vec![];
            for (col_idx, ch) in line.chars().enumerate() {
                row.push(match ch {
                    'S' => {
                        garden.starting_position = (row_idx as i64, col_idx as i64);
                        Plot::GardenPlot
                    }
                    '#' => Plot::Rock,
                    '.' => Plot::GardenPlot,
                    _ => {
                        return err_parse_error!("invalid plot encountered: {}", ch);
                    }
                });
            }
            garden.map.push_row(row);
        }

        Ok(garden)
    }
}

#[derive(Debug, Copy, Clone, Default)]
enum Plot {
    #[default]
    GardenPlot,
    Rock,
}

impl Plot {
    fn is_garden_plot(&self) -> bool {
        matches![self, Plot::GardenPlot]
    }
}

trait Traversable {
    fn at(&self, position: (i64, i64)) -> Plot;

    fn neighbours(&self, position: (i64, i64), infinite: bool) -> Vec<(i64, i64)>;
}

impl Traversable for Grid<Plot> {
    fn at(&self, (row, col): (i64, i64)) -> Plot {
        let rows = self.rows() as i64;
        let row = (((row % rows) + rows) % rows) as usize;
        let cols = self.cols() as i64;
        let col = (((col % cols) + cols) % cols) as usize;
        self[(row, col)]
    }

    fn neighbours(&self, (row, col): (i64, i64), infinite: bool) -> Vec<(i64, i64)> {
        let mut neighbours = Vec::with_capacity(4);

        if infinite {
            neighbours.push((row - 1, col));
            neighbours.push((row + 1, col));
            neighbours.push((row, col - 1));
            neighbours.push((row, col + 1));
        } else {
            if row > 0 {
                neighbours.push((row - 1, col));
            }
            if row + 1 < self.rows() as i64 {
                neighbours.push((row + 1, col));
            }
            if col > 0 {
                neighbours.push((row, col - 1));
            }
            if col + 1 < self.cols() as i64 {
                neighbours.push((row, col + 1));
            }
        }

        neighbours
            .iter()
            .copied()
            .filter(|&position| self.at(position).is_garden_plot())
            .collect()
    }
}

/// Represents the coloring of a board. Tiles are either black or white.
/// Two neighboring tiles never have the same color (i.e. a tile that is
/// next to a black tile must be white and vice versa).
trait BlackOrWhite {
    /// Returns true if a tile is black.
    fn is_black(&self) -> bool;

    fn is_white(&self) -> bool {
        !self.is_black()
    }

    /// Check whether the coloring of two tiles allow them to be
    /// reachable from one another in a given number of steps.
    ///
    /// For example, a black tile can never be reached from a white
    /// tile in an even number of steps.
    fn compare_colors(&self, other: &Self, steps: usize) -> bool {
        if steps % 2 == 0 {
            self.is_black() == other.is_black()
        } else {
            self.is_black() == other.is_white()
        }
    }
}

impl BlackOrWhite for (i64, i64) {
    fn is_black(&self) -> bool {
        (2 + self.0 % 2) % 2 == (2 + self.1 % 2) % 2
    }
}
