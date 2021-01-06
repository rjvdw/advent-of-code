use std::fmt;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::MultilineFromStr;
use termion::color;

use crate::tile::Tile;

mod tile;

#[derive(Debug, Clone)]
pub struct Forest {
    tiles: Vec<Vec<Tile>>,
    print_colors: bool,
}

impl Forest {
    /// Set whether or not colors should be used when printing.
    pub fn set_print_colors(&mut self, print_colors: bool) {
        self.print_colors = print_colors;
    }

    /// Get the next iteration of this forest, and return a code indicating its current state.
    pub fn next_iteration(&mut self) -> String {
        let mut tree_counts = self.get_new_counts();
        let mut lumberyard_counts = self.get_new_counts();

        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Open => {}
                    Tile::Trees => self.update_counts(&mut tree_counts, (x, y)),
                    Tile::Lumberyard => self.update_counts(&mut lumberyard_counts, (x, y)),
                }
            }
        }

        let mut state = String::new();

        for (y, row) in self.tiles.iter_mut().enumerate() {
            for (x, tile) in row.iter_mut().enumerate() {
                match tile {
                    Tile::Open => {
                        if tree_counts[y + 1][x + 1] >= 3 {
                            *tile = Tile::Trees;
                        }
                    }
                    Tile::Trees => {
                        if lumberyard_counts[y + 1][x + 1] >= 3 {
                            *tile = Tile::Lumberyard;
                        }
                    }
                    Tile::Lumberyard => {
                        if tree_counts[y + 1][x + 1] == 0 || lumberyard_counts[y + 1][x + 1] == 0 {
                            *tile = Tile::Open;
                        }
                    }
                }
                state.push(tile.get_state());
            }
        }

        state
    }

    /// Returns the total resource value.
    pub fn get_resource_value(&self) -> usize {
        let mut nr_trees = 0;
        let mut nr_lumberyards = 0;
        for row in &self.tiles {
            for tile in row {
                match tile {
                    Tile::Open => {}
                    Tile::Trees => nr_trees += 1,
                    Tile::Lumberyard => nr_lumberyards += 1,
                }
            }
        }
        nr_trees * nr_lumberyards
    }

    fn get_new_counts(&self) -> Vec<Vec<usize>> {
        // we add two to both the width and the height, so we don't have to do any bound checks
        let mut counts = Vec::with_capacity(self.tiles.len() + 2);
        counts.resize_with(self.tiles.len() + 2, || vec![0; self.tiles[0].len() + 2]);
        counts
    }

    fn update_counts(&self, counts: &mut Vec<Vec<usize>>, (x, y): (usize, usize)) {
        for i in 0..3 {
            for j in 0..3 {
                if i != 1 || j != 1 {
                    counts[y + i][x + j] += 1;
                }
            }
        }
    }
}

impl fmt::Display for Forest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let open;
        let trees;
        let lumberyard;

        if self.print_colors {
            open = format!("{}{}  ", color::Fg(color::Black), color::Bg(color::Black));
            trees = format!("{}{}  ", color::Fg(color::Green), color::Bg(color::Green));
            lumberyard = format!("{}{}  ", color::Fg(color::Red), color::Bg(color::Red));
        } else {
            open = ".".to_string();
            trees = "|".to_string();
            lumberyard = "#".to_string();
        }

        for (y, row) in self.tiles.iter().enumerate() {
            if y != 0 {
                writeln!(f)?;
            }
            for tile in row {
                match tile {
                    Tile::Open => write!(f, "{}", open)?,
                    Tile::Trees => write!(f, "{}", trees)?,
                    Tile::Lumberyard => write!(f, "{}", lumberyard)?,
                }
            }
            if self.print_colors {
                write!(f, "{}{}", color::Fg(color::Reset), color::Bg(color::Reset))?;
            }
        }

        Ok(())
    }
}

impl MultilineFromStr for Forest {
    type Err = ParseError;

    fn new() -> Self {
        Forest {
            tiles: Vec::new(),
            print_colors: false,
        }
    }

    fn indicates_new_record(&self, _line: &str) -> bool {
        false
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        let mut row = Vec::new();
        for ch in line.chars() {
            match ch {
                '.' => row.push(Tile::Open),
                '|' => row.push(Tile::Trees),
                '#' => row.push(Tile::Lumberyard),
                _ => return Err(ParseError::of("Invalid character encountered.")),
            }
        }
        self.tiles.push(row);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsMultilineRecords;

    use super::*;

    #[test]
    fn test() {
        let mut forest = get_test_input(vec![
            ".#.#...|#.",
            ".....#|##|",
            ".|..|...#.",
            "..|#.....#",
            "#.#|||#|#|",
            "...#.||...",
            ".|....|...",
            "||...#|.#|",
            "|.||||..|.",
            "...#.|..|.",
        ]);

        forest.next_iteration();

        assert_eq!(
            format!("{}", forest),
            vec![
                ".......##.",
                "......|###",
                ".|..|...#.",
                "..|#||...#",
                "..##||.|#|",
                "...#||||..",
                "||...|||..",
                "|||||.||.|",
                "||||||||||",
                "....||..|.",
            ]
            .join("\n")
        );

        forest.next_iteration();

        assert_eq!(
            format!("{}", forest),
            vec![
                ".......#..",
                "......|#..",
                ".|.|||....",
                "..##|||..#",
                "..###|||#|",
                "...#|||||.",
                "|||||||||.",
                "||||||||||",
                "||||||||||",
                ".|||||||||",
            ]
            .join("\n")
        );

        forest.next_iteration();

        assert_eq!(
            format!("{}", forest),
            vec![
                ".......#..",
                "....|||#..",
                ".|.||||...",
                "..###|||.#",
                "...##|||#|",
                ".||##|||||",
                "||||||||||",
                "||||||||||",
                "||||||||||",
                "||||||||||",
            ]
            .join("\n")
        );

        forest.next_iteration();

        assert_eq!(
            format!("{}", forest),
            vec![
                ".....|.#..",
                "...||||#..",
                ".|.#||||..",
                "..###||||#",
                "...###||#|",
                "|||##|||||",
                "||||||||||",
                "||||||||||",
                "||||||||||",
                "||||||||||",
            ]
            .join("\n")
        );

        for _ in 4..10 {
            forest.next_iteration();
        }

        assert_eq!(forest.get_resource_value(), 1147);
    }

    fn get_test_input(lines: Vec<&str>) -> Forest {
        lines
            .as_multiline_records::<Forest>()
            .unwrap()
            .first()
            .unwrap()
            .clone()
    }
}
