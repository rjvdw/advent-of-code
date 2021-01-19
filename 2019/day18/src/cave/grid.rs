use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader, Read};
use std::ops::{Index, IndexMut};

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

use crate::cave::gridterator::Gridterator;
use crate::cave::reachable::Reachable;
use crate::cave::tile::Tile;

#[derive(Debug, Clone)]
pub(in crate::cave) struct Grid {
    tiles: Vec<Tile>,
    pub(in crate::cave) width: usize,
    pub(in crate::cave) height: usize,
    pub(in crate::cave) entrances: Vec<(usize, usize)>,
    pub(in crate::cave) keys: HashSet<(usize, usize)>,
    pub(in crate::cave) doors: HashSet<(usize, usize)>,
}

impl Grid {
    /// Parses an input file.
    pub(in crate::cave) fn parse<R: Read>(r: R) -> Result<Grid, ParseError> {
        let mut grid = Grid {
            tiles: vec![],
            width: 0,
            height: 0,
            entrances: vec![],
            keys: HashSet::new(),
            doors: HashSet::new(),
        };

        for (y, line) in BufReader::new(r).lines().enumerate() {
            let line = line?;
            grid.width = line.len();
            grid.height += 1;

            for (x, ch) in line.chars().enumerate() {
                grid.tiles.push(match ch {
                    '@' => {
                        grid.entrances.push((x, y));
                        Tile::Entrance
                    }
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    a if a.is_ascii_lowercase() => {
                        grid.keys.insert((x, y));
                        Tile::Key(a)
                    }
                    a if a.is_ascii_uppercase() => {
                        grid.doors.insert((x, y));
                        Tile::Door(a.to_ascii_lowercase())
                    }
                    a => {
                        return Err(parse_error!(
                            "Invalid character '{}' encountered: {}",
                            a,
                            line
                        ));
                    }
                });
            }
        }

        Ok(grid)
    }

    /// Returns all interesting points that can be reached from your current position.
    pub(in crate::cave) fn get_reachable(
        &self,
        from: (usize, usize),
        keys: &[char],
    ) -> Vec<Reachable> {
        let mut reachable = vec![];

        let mut exploring = VecDeque::new();
        exploring.push_back((from, 0));

        let mut seen = HashSet::new();
        seen.insert(from);

        while let Some((position, distance)) = exploring.pop_front() {
            for neighbour in self.get_neighbours(position, keys) {
                if !seen.contains(&neighbour) {
                    seen.insert(neighbour);
                    if self.keys.contains(&neighbour) {
                        let key = self[neighbour].get_key().unwrap();
                        if !keys.contains(&key) {
                            reachable.push(Reachable {
                                position: neighbour,
                                distance: distance + 1,
                                key,
                            });
                            continue;
                        }
                    }
                    exploring.push_back((neighbour, distance + 1));
                }
            }
        }

        reachable
    }

    /// Returns all neighbouring tiles that are open.
    fn get_neighbours(&self, (x, y): (usize, usize), keys: &[char]) -> Vec<(usize, usize)> {
        let mut neighbours = vec![];
        if x > 0 && self[(x - 1, y)].is_open(keys) {
            neighbours.push((x - 1, y));
        }
        if y > 0 && self[(x, y - 1)].is_open(keys) {
            neighbours.push((x, y - 1));
        }
        if x + 1 < self.width && self[(x + 1, y)].is_open(keys) {
            neighbours.push((x + 1, y));
        }
        if y + 1 < self.height && self[(x, y + 1)].is_open(keys) {
            neighbours.push((x, y + 1));
        }
        neighbours
    }
}

impl IntoIterator for &Grid {
    type Item = (usize, usize);
    type IntoIter = Gridterator;

    fn into_iter(self) -> Self::IntoIter {
        Gridterator::new(&self)
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = Tile;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.tiles[x + y * self.width]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.tiles[x + y * self.width]
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (idx, tile) in self.tiles.iter().enumerate() {
            if idx != 0 && idx % self.width == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", tile)?;
        }
        Ok(())
    }
}
