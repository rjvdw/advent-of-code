use std::collections::{HashMap, HashSet};
use std::fmt;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::MultilineFromStr;
use rdcl_aoc_helpers::math::taxi_cab_2d;
use rdcl_aoc_helpers::search::Navigable;

use crate::tile::Tile;
use crate::tool::Tool;

mod tile;
mod tool;

/// The cave in which the target is lost.
#[derive(Debug)]
pub struct Cave {
    /// The depth of the cave.
    depth: usize,

    /// The location of the target.
    target: (usize, usize),

    /// The cache of geologic indices.
    geologic_indices: HashMap<(usize, usize), usize>,

    /// Elements at the left edge of this cave are multiplied with this factor.
    x_factor: usize,

    /// Elements at the top edge of this cave are multiplied with this factor.
    y_factor: usize,

    /// The erosion level is considered module this value.
    modulus: usize,
}

impl Cave {
    /// Compute the risk level of this cave.
    pub fn compute_risk_level(&mut self) -> usize {
        let mut risk_level = 0;
        for y in 0..=self.target.1 {
            for x in 0..=self.target.0 {
                risk_level += match self.compute_type((x, y)) {
                    Tile::Rocky => 0,
                    Tile::Narrow => 2,
                    Tile::Wet => 1,
                };
            }
        }
        risk_level
    }

    /// Find the fastest path to the target.
    pub fn find_fastest_path(&mut self) -> Option<u64> {
        // cannot use the A* implementation from the helpers, as that one does not use a mutable ref

        type Point = <Cave as Navigable>::Point;

        let start: Point = ((0, 0), Tool::Torch);
        let end: Point = (self.target, Tool::Torch);

        let mut open_set: HashSet<Point> = HashSet::new();
        open_set.insert(start);

        let mut came_from: HashMap<Point, Point> = HashMap::new();

        let mut g_score: HashMap<Point, u64> = HashMap::new();
        g_score.insert(start, 0);

        let mut f_score: HashMap<Point, u64> = HashMap::new();
        f_score.insert(start, self.distance_score(&start, &end));

        while !open_set.is_empty() {
            let current = open_set
                .iter()
                .min_by_key(|node| f_score.get(node).unwrap_or(&u64::MAX))
                .copied()
                .unwrap();

            if current == end {
                return g_score.get(&current).copied();
            }

            self.compute_geologic_index((current.0 .0 + 1, current.0 .1));
            self.compute_geologic_index((current.0 .0, current.0 .1 + 1));

            open_set.remove(&current);
            let current_distance = *g_score.get(&current).unwrap_or(&u64::MAX);

            for &(d, neighbour) in &self.get_neighbours(&current) {
                let distance = current_distance + d;
                let neighbour_distance = *g_score.get(&neighbour).unwrap_or(&u64::MAX);

                if distance < neighbour_distance {
                    came_from.insert(neighbour, current);
                    g_score.insert(neighbour, distance);
                    f_score.insert(neighbour, distance + self.distance_score(&neighbour, &end));
                    open_set.insert(neighbour);
                }
            }
        }

        None
    }

    /// Computes the geologic index at (x, y).
    fn compute_geologic_index(&mut self, (x, y): (usize, usize)) -> usize {
        if let Some(index) = self.get_geologic_index((x, y)) {
            index
        } else {
            let a = (x - 1, y);
            let b = (x, y - 1);

            self.compute_geologic_index(a);
            self.compute_geologic_index(b);

            let index = self.get_erosion_level(a).unwrap() * self.get_erosion_level(b).unwrap();

            self.geologic_indices.insert((x, y), index);
            index
        }
    }

    /// Computes the terrain type at (x, y).
    fn compute_type(&mut self, (x, y): (usize, usize)) -> Tile {
        self.compute_geologic_index((x, y));
        self.get_type((x, y)).unwrap()
    }

    /// Gets the geologic index at (x, y) from the cache.
    fn get_geologic_index(&self, (x, y): (usize, usize)) -> Option<usize> {
        if (x, y) == (0, 0) || (x, y) == self.target {
            Some(0)
        } else if y == 0 {
            Some(x * self.x_factor)
        } else if x == 0 {
            Some(y * self.y_factor)
        } else {
            self.geologic_indices.get(&(x, y)).copied()
        }
    }

    /// Gets the erosion level at (x, y) using the geologic index from the cache.
    fn get_erosion_level(&self, (x, y): (usize, usize)) -> Option<usize> {
        self.get_geologic_index((x, y))
            .map(|gi| (gi + self.depth) % self.modulus)
    }

    /// Gets the terrain type at (x, y) using the geologic index from the cache.
    fn get_type(&self, (x, y): (usize, usize)) -> Option<Tile> {
        match self.get_erosion_level((x, y)).map(|el| el % 3) {
            Some(0) => Some(Tile::Rocky),
            Some(1) => Some(Tile::Wet),
            Some(2) => Some(Tile::Narrow),
            _ => None,
        }
    }
}

impl Default for Cave {
    fn default() -> Self {
        Cave {
            depth: 0,
            target: (0, 0),
            geologic_indices: Default::default(),
            x_factor: 16807,
            y_factor: 48271,
            modulus: 20183,
        }
    }
}

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut upper_x = self.target.0;
        let mut upper_y = self.target.1;

        for &(x, y) in self.geologic_indices.keys() {
            if x > upper_x {
                upper_x = x;
            }
            if y > upper_y {
                upper_y = y;
            }
        }

        for y in 0..=upper_y {
            if y != 0 {
                writeln!(f)?;
            }
            for x in 0..=upper_x {
                if (x, y) == (0, 0) {
                    write!(f, "M")?;
                } else if (x, y) == self.target {
                    write!(f, "T")?;
                } else {
                    match self.get_type((x, y)) {
                        Some(Tile::Rocky) => write!(f, ".")?,
                        Some(Tile::Narrow) => write!(f, "|")?,
                        Some(Tile::Wet) => write!(f, "=")?,
                        None => write!(f, "?")?,
                    }
                }
            }
        }
        Ok(())
    }
}

impl MultilineFromStr for Cave {
    type Err = ParseError;

    fn new() -> Self {
        Self::default()
    }

    fn indicates_new_record(&self, _line: &str) -> bool {
        false
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        if let Some(r) = line.strip_prefix("depth: ") {
            self.depth = r.parse()?;
            Ok(())
        } else if let Some(r) = line.strip_prefix("target: ") {
            if let Some(idx) = r.find(',') {
                let x = r[..idx].parse()?;
                let y = r[idx + 1..].parse()?;
                self.target = (x, y);
                Ok(())
            } else {
                Err(ParseError(format!("invalid line: {}", line)))
            }
        } else {
            Err(ParseError(format!("invalid line: {}", line)))
        }
    }
}

impl Navigable for Cave {
    type Point = ((usize, usize), Tool);

    fn distance_score(&self, (a, _): &Self::Point, (b, _): &Self::Point) -> u64 {
        taxi_cab_2d(*a, *b) as u64
    }

    fn get_neighbours(&self, (point, equipped): &Self::Point) -> Vec<(u64, Self::Point)> {
        let neighbours = match *point {
            (0, 0) => vec![(0, 1), (1, 0)],
            (x, 0) => vec![(x - 1, 0), (x, 1), (x + 1, 0)],
            (0, y) => vec![(0, y - 1), (1, y), (0, y + 1)],
            (x, y) => vec![(x - 1, y), (x, y + 1), (x + 1, y), (x, y - 1)],
        };

        neighbours
            .iter()
            .flat_map(|&neighbour| {
                let tile = self.get_type(neighbour).unwrap();
                tile.suitable_tools()
                    .iter()
                    .map(move |&tool| {
                        let distance = if *equipped == tool { 1 } else { 8 };
                        (distance, (neighbour, tool))
                    })
                    .collect::<Vec<(u64, Self::Point)>>()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_geologic_index() {
        let mut cave = Cave {
            depth: 510,
            target: (10, 10),
            ..Default::default()
        };
        cave.compute_geologic_index((11, 11));

        assert_eq!(cave.get_geologic_index((0, 0)), Some(0));
        assert_eq!(cave.get_erosion_level((0, 0)), Some(510));
        assert_eq!(cave.get_type((0, 0)), Some(Tile::Rocky));

        assert_eq!(cave.get_geologic_index((1, 0)), Some(16807));
        assert_eq!(cave.get_erosion_level((1, 0)), Some(17317));
        assert_eq!(cave.get_type((1, 0)), Some(Tile::Wet));

        assert_eq!(cave.get_geologic_index((0, 1)), Some(48271));
        assert_eq!(cave.get_erosion_level((0, 1)), Some(8415));
        assert_eq!(cave.get_type((0, 1)), Some(Tile::Rocky));

        assert_eq!(cave.get_geologic_index((1, 1)), Some(145722555));
        assert_eq!(cave.get_erosion_level((1, 1)), Some(1805));
        assert_eq!(cave.get_type((1, 1)), Some(Tile::Narrow));

        assert_eq!(cave.get_geologic_index((10, 10)), Some(0));
        assert_eq!(cave.get_erosion_level((10, 10)), Some(510));
        assert_eq!(cave.get_type((10, 10)), Some(Tile::Rocky));
    }

    #[test]
    fn test_display() {
        let mut cave = Cave {
            depth: 510,
            target: (10, 10),
            ..Default::default()
        };
        cave.compute_geologic_index((15, 15));

        assert_eq!(
            format!("{}", cave),
            vec![
                "M=.|=.|.|=.|=|=.",
                ".|=|=|||..|.=...",
                ".==|....||=..|==",
                "=.|....|.==.|==.",
                "=|..==...=.|==..",
                "=||.=.=||=|=..|=",
                "|.=.===|||..=..|",
                "|..==||=.|==|===",
                ".=..===..=|.|||.",
                ".======|||=|=.|=",
                ".===|=|===T===||",
                "=|||...|==..|=.|",
                "=.=|=.=..=.||==|",
                "||=|=...|==.=|==",
                "|=.=||===.|||===",
                "||.|==.|.|.||=||",
            ]
            .join("\n")
        );
    }

    #[test]
    fn test_risk_level() {
        let mut cave = Cave {
            depth: 510,
            target: (10, 10),
            ..Default::default()
        };
        assert_eq!(cave.compute_risk_level(), 114);
    }

    #[test]
    fn test_find_shortest_path() {
        let mut cave = Cave {
            depth: 510,
            target: (10, 10),
            ..Default::default()
        };
        assert_eq!(cave.find_fastest_path(), Some(45));
    }
}
