use std::io;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

#[allow(clippy::unusual_byte_groupings)]
const DARK_REGION: usize = 0b000_000_000;
#[allow(clippy::unusual_byte_groupings)]
const LIGHT_REGION: usize = 0b111_111_111;

#[derive(Debug, Clone)]
pub struct Image {
    /// Image enhancement algorithm.
    iea: [bool; 512],

    /// The pixels which are currently lit.
    lit: Vec<Vec<bool>>,

    height: usize,
    width: usize,

    /// Iff true, all pixels that are out of bounds are lit.
    are_out_of_bounds_pixels_lit: bool,
}

impl Image {
    pub fn next(self) -> Image {
        let new_height = self.height + 2;
        let new_width = self.width + 2;

        let lit = (0..new_height)
            .map(|row| {
                (0..new_width)
                    .map(|col| self.get_iea_index(row, col))
                    .map(|idx| self.iea[idx])
                    .collect()
            })
            .collect();

        let are_out_of_bounds_pixels_lit = if self.are_out_of_bounds_pixels_lit {
            self.iea[LIGHT_REGION]
        } else {
            self.iea[DARK_REGION]
        };

        Image {
            iea: self.iea,
            lit,
            height: new_height,
            width: new_width,
            are_out_of_bounds_pixels_lit,
        }
    }

    /// Counts the number of pixels that are lit. Returns `Result::Ok` if the number of lit pixels
    /// is finite, `Result::Err` if infinite.
    pub fn count_lit_pixels(&self) -> Result<usize, usize> {
        let count = self.lit.iter().flatten().filter(|x| **x).count();

        if self.are_out_of_bounds_pixels_lit {
            Err(count)
        } else {
            Ok(count)
        }
    }

    fn get_iea_index(&self, row: usize, col: usize) -> usize {
        let mut iea_index = 0;
        #[allow(clippy::unusual_byte_groupings)]
        let mut mask = 0b100_000_000;

        for y in row..=(row + 2) {
            for x in col..=(col + 2) {
                let on_edge = y < 2 || x < 2;
                let is_lit = if on_edge {
                    self.are_out_of_bounds_pixels_lit
                } else {
                    self.is_lit(y - 2, x - 2)
                };

                if is_lit {
                    iea_index |= mask;
                }

                mask >>= 1;
            }
        }

        iea_index
    }

    fn is_lit(&self, row: usize, col: usize) -> bool {
        if row < self.height && col < self.width {
            self.lit[row][col]
        } else {
            self.are_out_of_bounds_pixels_lit
        }
    }

    pub fn parse<I>(mut lines: I) -> Result<Image, ParseError>
    where
        I: Iterator<Item = io::Result<String>>,
    {
        // first line contains the image enhancement algorithm
        let iea = match lines.next() {
            Some(Ok(v)) => Image::parse_iea(&v)?,
            _ => return Err(parse_error!("Invalid input")),
        };

        // second line must be blank
        match lines.next() {
            Some(Ok(v)) if v.is_empty() => {}
            _ => return Err(parse_error!("Invalid input")),
        };

        // rest of the lines describe the pixels
        let (lit, height, width) = Image::parse_pixels(lines)?;

        Ok(Image {
            iea,
            lit,
            height,
            width,
            are_out_of_bounds_pixels_lit: false,
        })
    }

    fn parse_iea(s: &str) -> Result<[bool; 512], ParseError> {
        let mut iea = [false; 512];
        for (idx, ch) in s.chars().enumerate() {
            match ch {
                '#' => {
                    iea[idx] = true;
                }
                '.' => {}
                _ => {
                    return Err(parse_error!("Invalid character {} @ {} : {}", ch, idx, s));
                }
            }
        }
        Ok(iea)
    }

    fn parse_pixels<I>(lines: I) -> Result<(Vec<Vec<bool>>, usize, usize), ParseError>
    where
        I: Iterator<Item = io::Result<String>>,
    {
        let mut lit = Vec::new();
        let mut height = 0;
        let mut width = 0;
        for line in lines {
            let line = line?;
            height += 1;
            width = line.len();
            lit.push(line.chars().map(|ch| ch == '#').collect());
        }
        Ok((lit, height, width))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_lit_pixels() {
        let image = test_image();
        assert_eq!(image.count_lit_pixels(), Ok(10));
        let image = image.next();
        assert_eq!(image.count_lit_pixels(), Ok(24));
        let image = image.next();
        assert_eq!(image.count_lit_pixels(), Ok(35));
    }

    #[test]
    fn test_get_iea_index() {
        let image = test_image();
        assert_eq!(image.get_iea_index(3, 3), 34);
    }

    fn test_image() -> Image {
        let iea = vec![
            "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##",
            "#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###",
            ".######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.",
            ".#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....",
            ".#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..",
            "...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....",
            "..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#",
        ]
        .join("");

        let lines = vec![
            Ok(iea),
            Ok("".to_string()),
            Ok("#..#.".to_string()),
            Ok("#....".to_string()),
            Ok("##..#".to_string()),
            Ok("..#..".to_string()),
            Ok("..###".to_string()),
        ];
        Image::parse(lines.into_iter()).unwrap()
    }
}
