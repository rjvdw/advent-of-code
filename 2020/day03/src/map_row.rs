use std::str::FromStr;

use rdcl_aoc_helpers::error::ParseError;

#[derive(Debug)]
pub struct MapRow(Vec<bool>);

impl MapRow {
    pub fn test_index(&self, i: usize) -> bool {
        self.0[i % self.0.len()]
    }
}

impl FromStr for MapRow {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MapRow(s.chars().map(|x| x == '#').collect()))
    }
}
