use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Read;

use rdcl_aoc_helpers::error::ParseError;

use crate::cave::grid::Grid;
use crate::cave::tile::Tile;
use crate::cave::Cave;

#[derive(Debug, Clone)]
pub struct FourWayCave {
    layout: Grid,
    entrances: [(usize, usize); 4],
}

impl Cave for FourWayCave {
    fn find_shortest_path(&self) -> Option<usize> {
        todo!()
    }

    fn parse<R: Read>(r: R) -> Result<Self, ParseError> {
        let mut layout = Grid::parse(r)?;
        let mut entrance = (0, 0);
        for (x, y) in &layout {
            if let Tile::Entrance = layout[(x, y)] {
                entrance = (x, y);
                break;
            }
        }

        let (x, y) = entrance;
        let entrances = [
            (x - 1, y - 1),
            (x + 1, y - 1),
            (x - 1, y + 1),
            (x + 1, y + 1),
        ];

        layout[(x, y)] = Tile::Wall;
        layout[(x, y - 1)] = Tile::Wall;
        layout[(x, y + 1)] = Tile::Wall;
        layout[(x - 1, y)] = Tile::Wall;
        layout[(x + 1, y)] = Tile::Wall;
        layout[(x - 1, y - 1)] = Tile::Entrance;
        layout[(x + 1, y - 1)] = Tile::Entrance;
        layout[(x - 1, y + 1)] = Tile::Entrance;
        layout[(x + 1, y + 1)] = Tile::Entrance;

        Ok(FourWayCave { layout, entrances })
    }
}

impl Display for FourWayCave {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.layout)
    }
}
