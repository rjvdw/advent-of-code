use std::io::Read;

use rdcl_aoc_helpers::error::ParseError;

pub mod four_way;
mod grid;
mod gridterator;
pub mod simple;
mod tile;

pub trait Cave {
    /// Find the shortest path which collects all keys.
    fn find_shortest_path(&self) -> Option<usize>;

    /// Parse the puzzle input.
    fn parse<R: Read>(r: R) -> Result<Self, ParseError>
    where
        Self: Sized;
}
