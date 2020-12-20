use std::{fmt, mem};

use crate::orientation_helpers::{FLIP_BITMASK, ROTATION_BITMASK};
use crate::tile;

/// If you take into account flipping, a tile has eight edges (four "normal" edges and four
/// "flipped" edges.
#[derive(Copy, Clone, Debug)]
pub enum EdgeName {
    /// The top edge of a tile.
    Top,

    /// The right edge of a tile.
    Right,

    /// The bottom edge of a tile.
    Bottom,

    /// The left edge of a tile.
    Left,

    /// The top edge of a tile, but mirrored along the x-axis.
    TopRev,

    /// The right edge of a tile, but mirrored along the y-axis.
    RightRev,

    /// The bottom edge of a tile, but mirrored along the x-axis.
    BottomRev,

    /// The left edge of a tile, but mirrored along the y-axis.
    LeftRev,
}

/// The collection of all edges of a specific tile.
#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Edges {
    pub top: u128,
    pub right: u128,
    pub bottom: u128,
    pub left: u128,

    pub top_rev: u128,
    pub right_rev: u128,
    pub bottom_rev: u128,
    pub left_rev: u128,
}

impl Edges {
    /// Get the edge identified by `name`.
    pub fn get(&self, name: EdgeName) -> u128 {
        match name {
            EdgeName::Top => self.top,
            EdgeName::Right => self.right,
            EdgeName::Bottom => self.bottom,
            EdgeName::Left => self.left,

            EdgeName::TopRev => self.top_rev,
            EdgeName::RightRev => self.right_rev,
            EdgeName::BottomRev => self.bottom_rev,
            EdgeName::LeftRev => self.left_rev,
        }
    }

    pub fn orient(&self, orientation: u8) -> Edges {
        let mut edges = *self;

        if orientation & FLIP_BITMASK != 0 {
            mem::swap(&mut edges.top, &mut edges.left);
            mem::swap(&mut edges.top_rev, &mut edges.left_rev);
            mem::swap(&mut edges.bottom, &mut edges.right);
            mem::swap(&mut edges.bottom_rev, &mut edges.right_rev);
        }

        match orientation & ROTATION_BITMASK {
            1 => Edges {
                top: edges.right,
                right: edges.bottom_rev,
                bottom: edges.left,
                left: edges.top_rev,
                top_rev: edges.right_rev,
                right_rev: edges.bottom,
                bottom_rev: edges.left_rev,
                left_rev: edges.top,
            },
            2 => Edges {
                top: edges.bottom_rev,
                right: edges.left_rev,
                bottom: edges.top_rev,
                left: edges.right_rev,
                top_rev: edges.bottom,
                right_rev: edges.left,
                bottom_rev: edges.top,
                left_rev: edges.right,
            },
            3 => Edges {
                top: edges.left_rev,
                right: edges.top,
                bottom: edges.right_rev,
                left: edges.bottom,
                top_rev: edges.left,
                right_rev: edges.top_rev,
                bottom_rev: edges.right,
                left_rev: edges.bottom_rev,
            },
            _ => edges,
        }
    }
}

impl fmt::Debug for Edges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Edges:")?;
        writeln!(f, "  top:    {}", format(self.top))?;
        writeln!(f, "  right:  {}", format(self.right))?;
        writeln!(f, "  bottom: {}", format(self.bottom))?;
        write!(f, "  left:   {}", format(self.left))?;

        Ok(())
    }
}

fn format(mut edge: u128) -> String {
    let mut result = String::new();

    for _ in 0..tile::SIZE {
        let bit = edge % 2 == 1;
        edge >>= 1;
        result.push(if bit { '#' } else { '.' });
    }

    result.chars().rev().collect()
}
