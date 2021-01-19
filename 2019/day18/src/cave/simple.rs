use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Read;

use rdcl_aoc_helpers::error::ParseError;

use crate::cave::grid::Grid;
use crate::cave::tile::Tile;
use crate::cave::Cave;

#[derive(Debug, Clone)]
pub struct SimpleCave {
    layout: Grid,
    entrance: (usize, usize),
}

impl Cave for SimpleCave {
    fn find_shortest_path(&self) -> Option<usize> {
        todo!()
    }

    fn parse<R: Read>(r: R) -> Result<SimpleCave, ParseError> {
        let layout = Grid::parse(r)?;
        let mut entrance = (0, 0);
        for (x, y) in &layout {
            if let Tile::Entrance = layout[(x, y)] {
                entrance = (x, y);
                break;
            }
        }
        Ok(SimpleCave { layout, entrance })
    }
}

impl Display for SimpleCave {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.layout)
    }
}
