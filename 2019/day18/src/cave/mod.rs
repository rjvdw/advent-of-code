use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{BufRead, BufReader, Read};

use grid::Grid;
use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

use tile::Tile;

use crate::state::State;

mod state;
mod tile;

#[derive(Debug, Clone)]
pub struct Cave {
    pub(crate) layout: Grid<Tile>,
    nr_keys: usize,
}

impl Cave {
    pub fn with_four_entrances(&self) -> Cave {
        let mut cave = self.clone();
        'outer: for y in 0..self.layout.rows() {
            for x in 0..self.layout.cols() {
                if matches!(self.layout[y][x], Tile::Entrance) {
                    cave.layout[y - 1][x - 1] = Tile::Entrance;
                    cave.layout[y + 1][x - 1] = Tile::Entrance;
                    cave.layout[y - 1][x + 1] = Tile::Entrance;
                    cave.layout[y + 1][x + 1] = Tile::Entrance;

                    cave.layout[y - 1][x] = Tile::Wall;
                    cave.layout[y + 1][x] = Tile::Wall;
                    cave.layout[y][x] = Tile::Wall;
                    cave.layout[y][x + 1] = Tile::Wall;
                    cave.layout[y][x - 1] = Tile::Wall;

                    break 'outer;
                }
            }
        }
        cave
    }

    pub fn find_quickest_route(&self) -> Option<usize> {
        let mut seen: HashSet<State> = HashSet::new();
        let mut exploring: VecDeque<(State, usize)> = VecDeque::new();

        let initial_state = State::initial_state(&self);
        seen.insert(initial_state.clone());
        exploring.push_back((initial_state, 0));

        while let Some((state, distance)) = exploring.pop_front() {
            for ((x, y), idx, mut neighbour) in state.get_neighbours() {
                match self.layout[y][x] {
                    Tile::Wall => {
                        continue;
                    }
                    Tile::Open => {}
                    Tile::Entrance => {}
                    Tile::Door(key) => {
                        if !neighbour.has_key(key) {
                            continue;
                        }
                    }
                    Tile::Key(key) => {
                        if !neighbour.has_key(key) {
                            neighbour.add_key(key, idx);
                            if neighbour.nr_keys() == self.nr_keys {
                                return Some(distance + 1);
                            }
                        }
                    }
                }
                if !seen.contains(&neighbour) {
                    seen.insert(neighbour.clone());
                    exploring.push_back((neighbour, distance + 1));
                }
            }
        }

        None
    }

    pub fn parse<R: Read>(r: R) -> Result<Cave, ParseError> {
        let mut tiles = vec![];
        let mut width = 0;
        let mut nr_keys = 0;

        for line in BufReader::new(r).lines() {
            let line = line?;
            width = line.len();
            for ch in line.chars() {
                match ch {
                    '@' => {
                        tiles.push(Tile::Entrance);
                    }
                    '#' => {
                        tiles.push(Tile::Wall);
                    }
                    '.' => {
                        tiles.push(Tile::Open);
                    }
                    _ => {
                        if ch.is_ascii_uppercase() {
                            tiles.push(Tile::Door(ch.to_ascii_lowercase()));
                        } else if ch.is_ascii_lowercase() {
                            nr_keys += 1;
                            tiles.push(Tile::Key(ch));
                        } else {
                            return Err(parse_error!(
                                "Invalid character encountered ({}) in line: {}",
                                ch,
                                line
                            ));
                        }
                    }
                };
            }
        }

        Ok(Cave {
            layout: Grid::from_vec(tiles, width),
            nr_keys,
        })
    }
}

impl Display for Cave {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..self.layout.rows() {
            if y != 0 {
                writeln!(f)?;
            }
            for x in 0..self.layout.cols() {
                write!(f, "{}", self.layout[y][x])?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_1() {
        let test_input = vec![
            "########################",
            "#...............b.C.D.f#",
            "#.######################",
            "#.....@.a.B.c.d.A.e.F.g#",
            "########################",
        ];
        let cave = Cave::parse(test_input.join("\n").as_bytes()).unwrap();
        assert_eq!(cave.find_quickest_route(), Some(132));
    }

    #[test]
    fn test_route_2() {
        let test_input = vec![
            "#################",
            "#i.G..c...e..H.p#",
            "########.########",
            "#j.A..b...f..D.o#",
            "########@########",
            "#k.E..a...g..B.n#",
            "########.########",
            "#l.F..d...h..C.m#",
            "#################",
        ];
        let cave = Cave::parse(test_input.join("\n").as_bytes()).unwrap();
        assert_eq!(cave.find_quickest_route(), Some(136));
    }

    #[test]
    fn test_route_3() {
        let test_input = vec![
            "########################",
            "#@..............ac.GI.b#",
            "###d#e#f################",
            "###A#B#C################",
            "###g#h#i################",
            "########################",
        ];
        let cave = Cave::parse(test_input.join("\n").as_bytes()).unwrap();
        assert_eq!(cave.find_quickest_route(), Some(81));
    }

    #[test]
    fn test_route_4() {
        let test_input = vec![
            "#############",
            "#g#f.D#..h#l#",
            "#F###e#E###.#",
            "#dCba@#@BcIJ#",
            "#############",
            "#nK.L@#@G...#",
            "#M###N#H###.#",
            "#o#m..#i#jk.#",
            "#############",
        ];
        let cave = Cave::parse(test_input.join("\n").as_bytes()).unwrap();
        assert_eq!(cave.find_quickest_route(), Some(72));
    }
}
