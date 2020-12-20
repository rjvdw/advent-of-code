use std::collections::HashMap;
use std::fmt;

use helpers::{FromMultilineStr, ParseError};

use crate::edges::{EdgeName, Edges};
use crate::orientation_helpers::{get_orientation, orient_x_y};

pub const SIZE: usize = 10;
const MAX_MASK: u128 = 0b1000000000;

/// A single tile of the image.
#[derive(Copy, Clone, Debug)]
pub struct Tile {
    /// The tile ID.
    pub id: u64,

    /// The pixel data. Only the last `SIZE * SIZE` bits are used.
    pub pixels: u128,

    /// The edges of this tile.
    pub edges: Edges,

    /// Only used during parsing.
    pub next_row: usize,
}

impl Tile {
    /// Returns whether the pixel at `(row; column)` is active.
    pub fn at(&self, row: usize, column: usize) -> bool {
        (self.pixels >> (SIZE * row + column)) % 2 == 1
    }

    /// Returns a list of all tile IDs of tiles that are adjacent to this tile.
    pub fn find_adjecent_tiles(&self, tiles: &HashMap<u64, Tile>) -> Vec<u64> {
        let mut result = Vec::new();

        result.extend(self.find_adjacent_tiles_to_edge(tiles, EdgeName::Top));
        result.extend(self.find_adjacent_tiles_to_edge(tiles, EdgeName::Right));
        result.extend(self.find_adjacent_tiles_to_edge(tiles, EdgeName::Bottom));
        result.extend(self.find_adjacent_tiles_to_edge(tiles, EdgeName::Left));

        result.iter().map(|&(id, _)| id).collect()
    }

    /// Returns a list of all tile IDs of tiles that are adjacent to a specific edge of this tile.
    /// With every tile ID an orientation is also returned, which specifies how the adjacent tile
    /// should be reoriented to line up with this tile.
    pub fn find_adjacent_tiles_to_edge(
        &self,
        tiles: &HashMap<u64, Tile>,
        edge_name: EdgeName,
    ) -> Option<(u64, u8)> {
        let edge = self.edges.get(edge_name);

        for tile in tiles.values().filter(|&t| t.id != self.id) {
            let mut res: Option<(u64, EdgeName)> = None;

            if edge == tile.edges.top {
                res = Some((tile.id, EdgeName::Top));
            } else if edge == tile.edges.top_rev {
                res = Some((tile.id, EdgeName::TopRev));
            } else if edge == tile.edges.right {
                res = Some((tile.id, EdgeName::Right));
            } else if edge == tile.edges.right_rev {
                res = Some((tile.id, EdgeName::RightRev));
            } else if edge == tile.edges.bottom {
                res = Some((tile.id, EdgeName::Bottom));
            } else if edge == tile.edges.bottom_rev {
                res = Some((tile.id, EdgeName::BottomRev));
            } else if edge == tile.edges.left {
                res = Some((tile.id, EdgeName::Left));
            } else if edge == tile.edges.left_rev {
                res = Some((tile.id, EdgeName::LeftRev));
            }

            if let Some((id, matched_edge_name)) = res {
                let orientation = get_orientation(edge_name, matched_edge_name);
                return Some((id, orientation));
            }
        }

        None
    }

    /// Orients the tile according to `orientation`.
    pub fn orient(&self, orientation: u8) -> Tile {
        let mut transformed = Tile {
            id: self.id,
            pixels: 0,
            edges: self.edges.orient(orientation),
            next_row: 0,
        };

        let mut idx: u128 = 0;
        for row in 0..SIZE {
            for column in 0..SIZE {
                let (row, column) = orient_x_y(row, column, SIZE, orientation);
                if self.at(row, column) {
                    transformed.pixels |= 1 << idx;
                }
                idx += 1;
            }
        }

        transformed
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.pixels == other.pixels && self.edges == other.edges
    }
}

impl Eq for Tile {}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Tile {}:", self.id)?;

        for row in 0..SIZE {
            for column in 0..SIZE {
                write!(f, "{}", if self.at(row, column) { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl FromMultilineStr for Tile {
    const DISCARD_FIRST_RECORD: bool = true;

    type Err = ParseError;

    fn new() -> Self {
        Tile {
            id: 0,
            pixels: 0,
            edges: Edges {
                top: 0,
                right: 0,
                bottom: 0,
                left: 0,

                top_rev: 0,
                right_rev: 0,
                bottom_rev: 0,
                left_rev: 0,
            },
            next_row: 0,
        }
    }

    fn indicates_new_record(line: &str) -> bool {
        line.starts_with("Tile")
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        if !line.is_empty() {
            if line.starts_with("Tile") {
                self.id = line[5..line.len() - 1].parse()?;
            } else {
                let mut pixels_rev = 0;
                let mut pixels = 0;
                for ch in line.chars() {
                    pixels_rev >>= 1;
                    pixels <<= 1;
                    if ch == '#' {
                        pixels_rev |= MAX_MASK;
                        pixels |= 1;
                    }
                }
                self.pixels |= pixels_rev << (SIZE * self.next_row);

                // determine the edges
                let mask = 1 << ((SIZE - 1 - self.next_row) as u128);
                let rev_mask = 1 << (self.next_row as u128);

                if self.next_row == 0 {
                    self.edges.top = pixels;
                    self.edges.top_rev = pixels_rev;
                }

                if line.ends_with('#') {
                    self.edges.right |= mask;
                    self.edges.right_rev |= rev_mask;
                }

                if self.next_row == SIZE - 1 {
                    self.edges.bottom = pixels;
                    self.edges.bottom_rev = pixels_rev;
                }

                if line.starts_with('#') {
                    self.edges.left |= mask;
                    self.edges.left_rev |= rev_mask;
                }

                self.next_row += 1;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use helpers::parse_multiline_input_as_single;

    use super::*;

    mod test_orient {
        use super::*;

        #[test]
        fn test_identity() {
            let tile = parse_multiline_input_as_single::<Tile>(vec![
                "#.#...#...",
                "###..#...#",
                ".....##...",
                "..#.##..#.",
                ".#....#..#",
                "...#######",
                ".....#...#",
                "#.#......#",
                "##........",
                "########.#",
            ])
            .unwrap();

            assert_eq!(tile.orient(0b000), tile);
        }

        #[test]
        fn test_flip() {
            let tile = parse_multiline_input_as_single::<Tile>(vec![
                "#.#...#...",
                "###..#...#",
                ".....##...",
                "..#.##..#.",
                ".#....#..#",
                "...#######",
                ".....#...#",
                "#.#......#",
                "##........",
                "########.#",
            ])
            .unwrap();

            let flipped = parse_multiline_input_as_single::<Tile>(vec![
                "##.....###",
                ".#..#...##",
                "##.#...#.#",
                ".....#...#",
                "...#.#...#",
                ".###.##..#",
                "#.#.##...#",
                ".....#...#",
                "...#.#....",
                ".#..####.#",
            ])
            .unwrap();

            assert_eq!(tile.orient(0b100), flipped);
        }

        #[test]
        fn test_flip_twice() {
            let tile = parse_multiline_input_as_single::<Tile>(vec![
                "#.#...#...",
                "###..#...#",
                ".....##...",
                "..#.##..#.",
                ".#....#..#",
                "...#######",
                ".....#...#",
                "#.#......#",
                "##........",
                "########.#",
            ])
            .unwrap();

            assert_eq!(tile.orient(0b100).orient(0b100), tile);
        }

        #[test]
        fn test_rotate_once() {
            let tile = parse_multiline_input_as_single::<Tile>(vec![
                "#.#...#...",
                "###..#...#",
                ".....##...",
                "..#.##..#.",
                ".#....#..#",
                "...#######",
                ".....#...#",
                "#.#......#",
                "##........",
                "########.#",
            ])
            .unwrap();

            let rotated = parse_multiline_input_as_single::<Tile>(vec![
                ".#..####.#",
                "...#.#....",
                ".....#...#",
                "#.#.##...#",
                ".###.##..#",
                "...#.#...#",
                ".....#...#",
                "##.#...#.#",
                ".#..#...##",
                "##.....###",
            ])
            .unwrap();

            assert_eq!(tile.orient(0b001), rotated);
        }

        #[test]
        fn test_rotate_twice() {
            let tile = parse_multiline_input_as_single::<Tile>(vec![
                "#.#...#...",
                "###..#...#",
                ".....##...",
                "..#.##..#.",
                ".#....#..#",
                "...#######",
                ".....#...#",
                "#.#......#",
                "##........",
                "########.#",
            ])
            .unwrap();

            let rotated = parse_multiline_input_as_single::<Tile>(vec![
                "#.########",
                "........##",
                "#......#.#",
                "#...#.....",
                "#######...",
                "#..#....#.",
                ".#..##.#..",
                "...##.....",
                "#...#..###",
                "...#...#.#",
            ])
            .unwrap();

            assert_eq!(tile.orient(0b010), rotated);
        }

        #[test]
        fn test_rotate_thrice() {
            let tile = parse_multiline_input_as_single::<Tile>(vec![
                "#.#...#...",
                "###..#...#",
                ".....##...",
                "..#.##..#.",
                ".#....#..#",
                "...#######",
                ".....#...#",
                "#.#......#",
                "##........",
                "########.#",
            ])
            .unwrap();

            let rotated = parse_multiline_input_as_single::<Tile>(vec![
                "###.....##",
                "##...#..#.",
                "#.#...#.##",
                "#...#.....",
                "#...#.#...",
                "#..##.###.",
                "#...##.#.#",
                "#...#.....",
                "....#.#...",
                "#.####..#.",
            ])
            .unwrap();

            assert_eq!(tile.orient(0b011), rotated);
        }

        #[test]
        fn test_rotate_quad() {
            let tile = parse_multiline_input_as_single::<Tile>(vec![
                "#.#...#...",
                "###..#...#",
                ".....##...",
                "..#.##..#.",
                ".#....#..#",
                "...#######",
                ".....#...#",
                "#.#......#",
                "##........",
                "########.#",
            ])
            .unwrap();

            assert_eq!(tile.orient(0b010).orient(0b010), tile);
        }
    }

    #[test]
    fn test_edges() {
        let tile = parse_multiline_input_as_single::<Tile>(vec![
            "#.#...#...",
            "###..#...#",
            ".....##...",
            "..#.##..#.",
            ".#....#..#",
            "...#######",
            ".....#...#",
            "#.#......#",
            "##........",
            "########.#",
        ])
        .unwrap();

        assert_eq!(
            tile.edges,
            Edges {
                top: 0b1010001000,
                right: 0b0100111101,
                bottom: 0b1111111101,
                left: 0b1100000111,

                top_rev: 0b0001000101,
                right_rev: 0b1011110010,
                bottom_rev: 0b1011111111,
                left_rev: 0b1110000011,
            }
        );
    }
}
