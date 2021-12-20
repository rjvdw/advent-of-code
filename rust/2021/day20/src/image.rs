use std::collections::HashSet;
use std::sync::{Arc, Mutex, RwLock};
use std::{fmt, io, thread};

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

use crate::bounds::Bounds;
use crate::point::Point;
use crate::shared_state::SharedState;

#[allow(clippy::unusual_byte_groupings)]
const DARK_REGION: u16 = 0b000_000_000;
#[allow(clippy::unusual_byte_groupings)]
const LIGHT_REGION: u16 = 0b111_111_111;

#[derive(Debug, Clone)]
pub struct Image {
    /// Image enhancement algorithm.
    iea: HashSet<u16>,

    /// The pixels which are currently lit.
    lit: HashSet<Point>,

    /// Only pixels within this region are described, all pixels outside of this region have the
    /// default state.
    bounds: Bounds,

    /// Iff true, all pixels that are out of bounds are lit.
    default_state: bool,
}

impl Image {
    /// Counts the number of pixels that are lit. Returns `Result::Ok` if the number of lit pixels
    /// is finite, `Result::Err` if infinite. In the latter case, the number of lit pixels within
    /// bounds is also returned.
    pub fn count_lit_pixels(&self) -> Result<usize, usize> {
        if self.default_state {
            Err(self.lit.len())
        } else {
            Ok(self.lit.len())
        }
    }

    /// Advances one generation.
    pub fn next(&self) -> Image {
        let mut lit = HashSet::new();
        let mut bounds = Bounds::default();
        let default_state = if self.default_state {
            self.iea.contains(&LIGHT_REGION)
        } else {
            self.iea.contains(&DARK_REGION)
        };

        for (row, col) in self.bounds.stretched(1).iter_row_col() {
            let iea_index = self.get_iea_index(row, col);
            if self.iea.contains(&iea_index) {
                lit.insert(Point::new(row, col));
                bounds.update_with(row, col);
            }
        }

        Image {
            iea: self.iea.clone(),
            lit,
            bounds,
            default_state,
        }
    }

    /// Advances one generation (multi-threaded).
    pub fn next_mt(&self, nr_threads: i64, min_region_size: i64) -> Image {
        assert!(nr_threads > 0, "Invalid number of threads supplied.");
        assert!(
            min_region_size >= 0,
            "Invalid minimal region size supplied.",
        );

        let shared_state = Arc::new(Mutex::new(SharedState {
            lit: HashSet::new(),
            bounds: Bounds::default(),
        }));
        let default_state = if self.default_state {
            self.iea.contains(&LIGHT_REGION)
        } else {
            self.iea.contains(&DARK_REGION)
        };

        let mut handles = vec![];
        let shared_self = Arc::new(RwLock::new(self.clone()));
        for region in self.bounds.stretched(1).divide(nr_threads, min_region_size) {
            let shared_state = Arc::clone(&shared_state);
            let shared_self = Arc::clone(&shared_self);

            let mut lit = HashSet::<Point>::new();
            let mut bounds = Bounds::default();

            let handle = thread::spawn(move || {
                let image = shared_self.read().unwrap();
                for (row, col) in region.iter_row_col() {
                    let iea_index = image.get_iea_index(row, col);
                    if image.iea.contains(&iea_index) {
                        lit.insert(Point::new(row, col));
                        bounds.update_with(row, col);
                    }
                }

                let mut state = shared_state.lock().unwrap();
                state.lit.extend(&lit);
                state.bounds.join_with(&bounds);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let state = &*shared_state.lock().unwrap();
        Image {
            iea: self.iea.clone(),
            lit: state.lit.clone(),
            bounds: state.bounds,
            default_state,
        }
    }

    fn get_iea_index(&self, row: i64, col: i64) -> u16 {
        let mut iea_index = 0;
        #[allow(clippy::unusual_byte_groupings)]
        let mut mask = 0b100_000_000;

        for y in (row - 1)..=(row + 1) {
            for x in (col - 1)..=(col + 1) {
                if self.is_lit(y, x) {
                    iea_index |= mask;
                }
                mask >>= 1;
            }
        }

        iea_index
    }

    fn is_lit(&self, row: i64, col: i64) -> bool {
        let point = Point::new(row, col);
        (self.default_state && !self.bounds.contains(&point)) || self.lit.contains(&point)
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
        let (lit, bounds) = Image::parse_pixels(lines)?;

        Ok(Image {
            iea,
            lit,
            bounds,
            default_state: false,
        })
    }

    fn parse_iea(s: &str) -> Result<HashSet<u16>, ParseError> {
        let mut iea = HashSet::new();
        for (idx, ch) in s.chars().enumerate() {
            match ch {
                '#' => {
                    iea.insert(idx as u16);
                }
                '.' => {}
                _ => {
                    return Err(parse_error!("Invalid character {} @ {} : {}", ch, idx, s));
                }
            }
        }
        Ok(iea)
    }

    fn parse_pixels<I>(lines: I) -> Result<(HashSet<Point>, Bounds), ParseError>
    where
        I: Iterator<Item = io::Result<String>>,
    {
        let mut lit = HashSet::new();
        let mut bounds = Bounds::default();
        for (row, line) in lines.enumerate() {
            let line = line?;
            for (col, ch) in line.chars().enumerate() {
                match ch {
                    '#' => {
                        let row = row as i64;
                        let col = col as i64;
                        lit.insert(Point::new(row, col));
                        bounds.update_with(row, col);
                    }
                    '.' => {}
                    _ => {
                        return Err(parse_error!(
                            "Invalid character {} @ {} in line {} : {}",
                            ch,
                            col,
                            row,
                            line,
                        ));
                    }
                }
            }
        }
        Ok((lit, bounds))
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..512 {
            if self.iea.contains(&i) {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        writeln!(f)?;
        let mut start_col = None;
        for (row, col) in self.bounds.iter_row_col() {
            if start_col == None {
                start_col = Some(col);
            }
            if Some(col) == start_col {
                writeln!(f)?;
            }
            if self.is_lit(row, col) {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_iea_index() {
        let image = test_image();
        assert_eq!(image.get_iea_index(2, 2), 34);
    }

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
    fn test_count_lit_pixels_mt() {
        let image = test_image();
        assert_eq!(image.count_lit_pixels(), Ok(10));
        let image = image.next_mt(2, 10);
        assert_eq!(image.count_lit_pixels(), Ok(24));
        let image = image.next_mt(2, 10);
        assert_eq!(image.count_lit_pixels(), Ok(35));
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