use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader, Read};

use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Grid {
    bugs: Vec<bool>,
    width: usize,
}

impl Grid {
    pub fn parse<R: Read>(r: R) -> Result<Grid, ParseError> {
        let mut grid = Grid {
            bugs: vec![],
            width: 0,
        };
        for line in BufReader::new(r).lines() {
            let line = line?;
            grid.width = line.len();
            for ch in line.chars() {
                grid.bugs.push(ch == '#');
            }
        }
        Ok(grid)
    }

    pub fn tick(&self) -> Grid {
        let mut counts = vec![0; self.bugs.len()];
        let iter = self
            .bugs
            .iter()
            .enumerate()
            .filter(|(_, &b)| b)
            .map(|(i, _)| i);

        for idx in iter {
            if idx >= self.width {
                counts[idx - self.width] += 1;
            }
            if idx + self.width < self.bugs.len() {
                counts[idx + self.width] += 1;
            }
            if idx % self.width != 0 {
                counts[idx - 1] += 1;
            }
            if (idx + 1) % self.width != 0 {
                counts[idx + 1] += 1;
            }
        }

        let bugs = counts
            .iter()
            .enumerate()
            .map(|(idx, &count)| count == 1 || (count == 2 && !self.bugs[idx]))
            .collect();

        Grid {
            bugs,
            width: self.width,
        }
    }

    pub fn calculate_biodiversity_rating(&self) -> u64 {
        let mut rating = 0;
        let mut score = 1;
        for &bug in &self.bugs {
            if bug {
                rating += score;
            }
            score *= 2;
        }
        rating
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, t) in self.bugs.iter().enumerate() {
            if i != 0 && i % self.width == 0 {
                writeln!(f)?;
            }
            if *t {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
#[rustfmt::skip::macros(vec)]
mod tests {
    use super::*;

    #[test]
    fn test_biodiversity_rating() {
        let grid = make_grid(vec![
            ".....",
            ".....",
            ".....",
            "#....",
            ".#...",
        ]);
        assert_eq!(grid.calculate_biodiversity_rating(), 2129920);
    }

    #[test]
    fn test_tick_1() {
        let grid = make_grid(vec![
            "....#",
            "#..#.",
            "#..##",
            "..#..",
            "#....",
        ]);
        let expected = make_grid(vec![
            "#..#.",
            "####.",
            "###.#",
            "##.##",
            ".##..",
        ]);
        assert_eq!(grid.tick(), expected);
    }

    #[test]
    fn test_tick_2() {
        let grid = make_grid(vec![
            "#..#.",
            "####.",
            "###.#",
            "##.##",
            ".##..",
        ]);
        let expected = make_grid(vec![
            "#####",
            "....#",
            "....#",
            "...#.",
            "#.###",
        ]);
        assert_eq!(grid.tick(), expected);
    }

    #[test]
    fn test_tick_3() {
        let grid = make_grid(vec![
            "#####",
            "....#",
            "....#",
            "...#.",
            "#.###",
        ]);
        let expected = make_grid(vec![
            "#....",
            "####.",
            "...##",
            "#.##.",
            ".##.#",
        ]);
        assert_eq!(grid.tick(), expected);
    }

    fn make_grid(test_input: Vec<&str>) -> Grid {
        Grid::parse(test_input.join("\n").as_bytes()).unwrap()
    }
}
