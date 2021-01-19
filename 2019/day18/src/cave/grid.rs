use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader, Read};
use std::ops::{Index, IndexMut};

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

use crate::cave::gridterator::Gridterator;
use crate::cave::tile::Tile;

#[derive(Debug, Clone)]
pub(in crate::cave) struct Grid {
    tiles: Vec<Tile>,
    pub(in crate::cave) width: usize,
    pub(in crate::cave) height: usize,
}

impl Grid {
    /// Parses an input file.
    pub(in crate::cave) fn parse<R: Read>(r: R) -> Result<Grid, ParseError> {
        let mut tiles = vec![];
        let mut width = 0;
        let mut height = 0;

        for line in BufReader::new(r).lines() {
            let line = line?;
            width = line.len();
            height += 1;

            for ch in line.chars() {
                tiles.push(match ch {
                    '@' => Tile::Entrance,
                    '#' => Tile::Wall,
                    '.' => Tile::Open,
                    a if a.is_ascii_lowercase() => Tile::Key(a),
                    a if a.is_ascii_uppercase() => Tile::Door(a.to_ascii_lowercase()),
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

        Ok(Grid {
            tiles,
            width,
            height,
        })
    }

    /// Returns all neighbouring tiles that are not walls.
    pub(in crate::cave) fn get_neighbours(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbours = vec![];
        if x > 0 && self[(x - 1, y)] != Tile::Wall {
            neighbours.push((x - 1, y));
        }
        if y > 0 && self[(x, y - 1)] != Tile::Wall {
            neighbours.push((x, y - 1));
        }
        if x + 1 < self.width && self[(x + 1, y)] != Tile::Wall {
            neighbours.push((x + 1, y));
        }
        if y + 1 < self.height && self[(x, y + 1)] != Tile::Wall {
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
