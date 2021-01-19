use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::Read;

use rdcl_aoc_helpers::error::ParseError;

use crate::cave::grid::Grid;
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
        let layout = Grid::parse(r)?;
        assert_eq!(layout.entrances.len(), 4);
        let entrances = [
            layout.entrances[0],
            layout.entrances[1],
            layout.entrances[2],
            layout.entrances[3],
        ];
        Ok(FourWayCave { layout, entrances })
    }
}

impl Display for FourWayCave {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.layout)
    }
}
