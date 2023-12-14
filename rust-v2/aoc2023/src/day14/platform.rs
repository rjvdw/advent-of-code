use std::fmt;

use grid::Grid;

use rdcl_aoc_core::input::FromInput;
use rdcl_aoc_core::ParseResult;

#[derive(Debug, Clone)]
pub struct Platform {
    platform: Grid<Rock>,
}

impl Platform {
    pub fn cycle(&self) -> Platform {
        self.tilt_north().tilt_west().tilt_south().tilt_east()
    }

    pub fn tilt_north(&self) -> Platform {
        self.tilt(self.platform.rows(), self.platform.cols(), |col, row| {
            (row, col)
        })
    }

    pub fn tilt_south(&self) -> Platform {
        self.tilt(self.platform.rows(), self.platform.cols(), |col, row| {
            (self.platform.rows() - 1 - row, col)
        })
    }

    pub fn tilt_west(&self) -> Platform {
        self.tilt(self.platform.cols(), self.platform.rows(), |row, col| {
            (row, col)
        })
    }

    pub fn tilt_east(&self) -> Platform {
        self.tilt(self.platform.cols(), self.platform.rows(), |row, col| {
            (row, self.platform.cols() - 1 - col)
        })
    }

    fn tilt<C>(&self, r1: usize, r2: usize, coord: C) -> Platform
    where
        C: Fn(usize, usize) -> (usize, usize),
    {
        let mut next = Grid::new(self.platform.rows(), self.platform.cols());

        for i in 0..r1 {
            let mut free = 0;
            for j in 0..r2 {
                match self.platform[coord(i, j)] {
                    Rock::None => {}
                    Rock::Cube => {
                        next[coord(i, j)] = Rock::Cube;
                        free = j + 1;
                    }
                    Rock::Round => {
                        next[coord(i, free)] = Rock::Round;
                        free += 1;
                    }
                }
            }
        }

        Platform { platform: next }
    }

    pub fn compute_load(&self) -> usize {
        let mut total = 0;

        for row in 0..self.platform.rows() {
            let load = self.platform.rows() - row;
            for col in 0..self.platform.cols() {
                if matches![self.platform[(row, col)], Rock::Round] {
                    total += load;
                }
            }
        }

        total
    }
}

impl FromInput for Platform {
    fn parse<T>(input: T) -> ParseResult<Self>
    where
        T: Iterator<Item = String>,
    {
        let mut platform = Grid::new(0, 0);

        for line in input {
            platform.push_row(
                line.chars()
                    .map(|ch| match ch {
                        'O' => Rock::Round,
                        '#' => Rock::Cube,
                        _ => Rock::None,
                    })
                    .collect(),
            )
        }

        Ok(Platform { platform })
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.platform.rows() {
            if row != 0 {
                writeln!(f)?;
            }
            for col in 0..self.platform.cols() {
                match self.platform[(row, col)] {
                    Rock::None => write!(f, ".")?,
                    Rock::Cube => write!(f, "#")?,
                    Rock::Round => write!(f, "O")?,
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub enum Rock {
    #[default]
    None,
    Cube,
    Round,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_load() {
        let platform =
            Platform::parse(["O..", "OO.", "OOO"].iter().map(|s| s.to_string())).unwrap();

        assert_eq!(platform.compute_load(), 10);
    }
}
