use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader, Read};

use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug, Clone)]
pub struct RecursiveGrid {
    bugs: HashSet<(i64, i64, i64)>,
    x_range: (i64, i64),
    y_range: (i64, i64),
}

impl RecursiveGrid {
    pub fn parse<R: Read>(r: R) -> Result<RecursiveGrid, ParseError> {
        let mut grid = RecursiveGrid {
            bugs: HashSet::new(),
            x_range: (0, 0),
            y_range: (0, 0),
        };

        let mut bugs: HashSet<(i64, i64)> = HashSet::new();
        let mut max_x = 0;
        let mut max_y = 0;
        for (y, line) in BufReader::new(r).lines().enumerate() {
            let line = line?;
            let y = y as i64;
            max_y += 1;
            max_x = line.len() as i64;
            for (x, ch) in line.chars().enumerate() {
                if ch == '#' {
                    let x = x as i64;
                    bugs.insert((x, y));
                }
            }
        }

        // grid may not have even dimensions, as the "portal" is at the center
        assert_eq!(max_x % 2, 1);
        assert_eq!(max_y % 2, 1);

        // shift the entire grid so that the "portal" will be at 0,0
        let offset_x = max_x / 2;
        let offset_y = max_y / 2;

        grid.x_range = (-offset_x, offset_x);
        grid.y_range = (-offset_y, offset_y);

        for (x, y) in bugs {
            grid.bugs.insert((x - offset_x, y - offset_y, 0));
        }

        Ok(grid)
    }

    pub fn count_bugs(&self) -> usize {
        self.bugs.len()
    }

    pub fn tick(&self) -> RecursiveGrid {
        let mut counts: HashMap<(i64, i64, i64), usize> = HashMap::new();
        for &p in &self.bugs {
            for neighbour in self.get_neighbours(p) {
                *counts.entry(neighbour).or_insert(0) += 1;
            }
        }

        let mut bugs = HashSet::new();
        for (p, count) in counts {
            if count == 1 || (count == 2 && !self.bugs.contains(&p)) {
                bugs.insert(p);
            }
        }

        RecursiveGrid {
            bugs,
            ..self.clone()
        }
    }

    fn get_neighbours(&self, (x, y, z): (i64, i64, i64)) -> Vec<(i64, i64, i64)> {
        let mut neighbours = vec![];

        if x == self.x_range.0 {
            neighbours.push((-1, 0, z - 1));
        } else if y == 0 && x - 1 == 0 {
            for j in self.y_range.0..=self.y_range.1 {
                neighbours.push((self.x_range.1, j, z + 1));
            }
        } else {
            neighbours.push((x - 1, y, z));
        }

        if x == self.x_range.1 {
            neighbours.push((1, 0, z - 1));
        } else if y == 0 && x + 1 == 0 {
            for j in self.y_range.0..=self.y_range.1 {
                neighbours.push((self.x_range.0, j, z + 1));
            }
        } else {
            neighbours.push((x + 1, y, z));
        }

        if y == self.y_range.0 {
            neighbours.push((0, -1, z - 1));
        } else if x == 0 && y - 1 == 0 {
            for i in self.x_range.0..=self.x_range.1 {
                neighbours.push((i, self.y_range.1, z + 1));
            }
        } else {
            neighbours.push((x, y - 1, z));
        }

        if y == self.y_range.1 {
            neighbours.push((0, 1, z - 1));
        } else if x == 0 && y + 1 == 0 {
            for i in self.x_range.0..=self.x_range.1 {
                neighbours.push((i, self.y_range.0, z + 1));
            }
        } else {
            neighbours.push((x, y + 1, z));
        }

        neighbours
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_1() {
        let mut grid = make_grid(vec!["....#", "#..#.", "#..##", "..#..", "#...."]);
        for _ in 0..10 {
            grid = grid.tick();
        }
        assert_eq!(grid.count_bugs(), 99);
    }

    fn make_grid(test_input: Vec<&str>) -> RecursiveGrid {
        RecursiveGrid::parse(test_input.join("\n").as_bytes()).unwrap()
    }
}
