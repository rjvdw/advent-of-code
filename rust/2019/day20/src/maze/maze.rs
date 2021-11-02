use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader, Read};

use grid::Grid;
use rdcl_aoc_helpers::error::ParseError;

use crate::recursive_maze::RecursiveMaze;
use crate::tile::Tile;

#[derive(Debug, Clone)]
pub struct Maze {
    layout: Grid<Tile>,
    portals: HashMap<(usize, usize), (usize, usize)>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Maze {
    pub fn as_recursive_maze(&self) -> RecursiveMaze {
        RecursiveMaze {
            layout: self.layout.clone(),
            portals: self.portals.clone(),
            start: (self.start.0, self.start.1, 1),
            end: (self.end.0, self.end.1, 1),
        }
    }

    pub fn find_shortest_route(&self) -> Option<usize> {
        let mut exploring = VecDeque::new();
        exploring.push_back((self.start, 0));
        let mut seen = HashSet::new();
        seen.insert(self.start);

        while let Some((p, distance)) = exploring.pop_front() {
            // println!("{} {:?}", distance, p);
            for neighbour in self.get_neighbours(p) {
                if neighbour == self.end {
                    return Some(distance + 1);
                }
                if !seen.contains(&neighbour) {
                    seen.insert(neighbour);
                    exploring.push_back((neighbour, distance + 1));
                }
            }
        }

        None
    }

    fn get_neighbours(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        // assumption: the edges of the map are not used, so we do not need to do bound checks
        let mut neighbours = vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)];

        if let Some(other) = self.portals.get(&(x, y)) {
            neighbours.push(*other);
        }

        neighbours
            .iter()
            .filter(|&&(x, y)| self.layout[y][x].is_open())
            .copied()
            .collect()
    }

    pub fn parse_input<R: Read>(r: R) -> Result<Maze, ParseError> {
        let mut width = 0;
        let mut tiles = vec![];
        for line in BufReader::new(r).lines() {
            let line = line?;
            width = line.len();
            for ch in line.chars() {
                tiles.push(match ch {
                    '#' => Tile::Wall,
                    '.' => Tile::Open,
                    ' ' => Tile::Empty,
                    _ => Tile::Portal(ch.to_string()),
                });
            }
        }

        let mut layout = Grid::from_vec(tiles, width);
        let mut half_portals: HashMap<String, (usize, usize)> = HashMap::new();
        let mut portals = HashMap::new();

        // assumption: the basic layout of the maze is the same for all inputs, so we only have to
        // check specific lines/columns for portals

        let rows_to_check = vec![(0, 1, 2), (29, 30, 28), (82, 83, 84), (111, 112, 110)];
        for (y1, y2, y3) in rows_to_check {
            for x in 2..layout.cols() - 2 {
                let r1 = &layout[y1][x];
                let r2 = &layout[y2][x];
                if let (Tile::Portal(ch1), Tile::Portal(ch2)) = (r1, r2) {
                    let mut name = String::new();
                    name.push_str(ch1);
                    name.push_str(ch2);
                    layout[y1][x] = Tile::Empty;
                    layout[y2][x] = Tile::Empty;
                    layout[y3][x] = Tile::Portal(name.to_string());

                    if let Some(p) = half_portals.get(&name) {
                        portals.insert((x, y3), *p);
                        portals.insert(*p, (x, y3));
                    } else {
                        half_portals.insert(name.to_string(), (x, y3));
                    }
                }
            }
        }

        let columns_to_check = vec![(0, 1, 2), (29, 30, 28), (76, 77, 78), (105, 106, 104)];
        for y in 2..layout.rows() - 2 {
            for (x1, x2, x3) in columns_to_check.iter().copied() {
                let r1 = &layout[y][x1];
                let r2 = &layout[y][x2];
                if let (Tile::Portal(ch1), Tile::Portal(ch2)) = (r1, r2) {
                    let mut name = String::new();
                    name.push_str(ch1);
                    name.push_str(ch2);
                    layout[y][x1] = Tile::Empty;
                    layout[y][x2] = Tile::Empty;
                    layout[y][x3] = Tile::Portal(name.to_string());

                    if let Some(p) = half_portals.get(&name) {
                        portals.insert((x3, y), *p);
                        portals.insert(*p, (x3, y));
                    } else {
                        half_portals.insert(name.to_string(), (x3, y));
                    }
                }
            }
        }

        let start = *half_portals.get("AA").unwrap();
        let end = *half_portals.get("ZZ").unwrap();

        // check we did not make any mistakes
        let maze = Maze {
            layout,
            portals,
            start,
            end,
        };
        for (key, value) in &maze.portals {
            assert_eq!(&maze.portals.get(value), &Some(key));
            let portal1 = maze.layout.get(key.1, key.0);
            let portal2 = maze.layout.get(value.1, value.0);
            if let (Some(Tile::Portal(n1)), Some(Tile::Portal(n2))) = (portal1, portal2) {
                assert_eq!(n1, n2);
            } else {
                println!("{}", maze);
                panic!(
                    "Problem with tiles: {:?} ({:?}), {:?} ({:?})",
                    portal1, key, portal2, value
                );
            }
        }
        Ok(maze)
    }
}

impl Display for Maze {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..self.layout.rows() {
            if y != 0 {
                writeln!(f)?;
            }
            for x in 0..self.layout.cols() {
                match self.layout[y][x] {
                    Tile::Wall => write!(f, "#")?,
                    Tile::Open => write!(f, ".")?,
                    Tile::Portal(_) => write!(f, "@")?,
                    Tile::Empty => write!(f, " ")?,
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use grid::grid;

    use Tile::{Empty as E, Open as O, Wall as W};

    use super::*;

    #[test]
    fn test_route_1() {
        let a = Tile::Portal("AA".to_string());
        let b1 = Tile::Portal("BC".to_string());
        let b2 = Tile::Portal("BC".to_string());
        let d1 = Tile::Portal("DE".to_string());
        let d2 = Tile::Portal("DE".to_string());
        let f1 = Tile::Portal("FG".to_string());
        let f2 = Tile::Portal("FG".to_string());
        let z = Tile::Portal("ZZ".to_string());
        let mut maze = Maze {
            layout: grid![
                [E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E]
                [E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E]
                [E, E, W, W, W, W, W, W, W, a, W, W, W, W, W, W, W, W, W, E, E]
                [E, E, W, W, W, W, W, W, W, O, O, O, O, O, O, O, O, O, W, E, E]
                [E, E, W, W, W, W, W, W, W, O, W, W, W, W, W, W, W, O, W, E, E]
                [E, E, W, W, W, W, W, W, W, O, W, W, W, W, W, W, W, O, W, E, E]
                [E, E, W, W, W, W, W, W, W, b1, W, W, W, W, W, W, W, O, W, E, E]
                [E, E, W, W, W, W, W, E, E, E, E, E, E, E, W, W, W, O, W, E, E]
                [E, E, b2, O, O, W, W, E, E, E, E, E, E, E, W, W, W, O, W, E, E]
                [E, E, W, W, O, W, W, E, E, E, E, E, E, E, W, W, W, O, W, E, E]
                [E, E, W, W, O, O, d1, E, E, E, E, E, E, E, W, W, W, O, W, E, E]
                [E, E, W, W, W, W, W, E, E, E, E, E, E, E, W, W, W, O, W, E, E]
                [E, E, W, W, W, W, W, W, W, W, W, f1, W, W, W, W, W, O, W, E, E]
                [E, E, d2, O, W, W, W, W, W, W, W, O, O, O, W, W, W, O, W, E, E]
                [E, E, W, O, W, W, W, W, W, W, W, W, W, O, W, W, W, O, W, E, E]
                [E, E, f2, O, W, W, W, W, W, W, W, W, W, O, O, O, O, O, W, E, E]
                [E, E, W, W, W, W, W, W, W, W, W, W, W, z, W, W, W, W, W, E, E]
                [E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E]
                [E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E, E]
            ],
            portals: HashMap::new(),
            start: (9, 2),
            end: (13, 16),
        };
        maze.portals.insert((9, 6), (2, 8));
        maze.portals.insert((2, 8), (9, 6));
        maze.portals.insert((6, 10), (2, 13));
        maze.portals.insert((2, 13), (6, 10));
        maze.portals.insert((11, 12), (2, 15));
        maze.portals.insert((2, 15), (11, 12));
        assert_eq!(maze.find_shortest_route(), Some(23));
    }
}
