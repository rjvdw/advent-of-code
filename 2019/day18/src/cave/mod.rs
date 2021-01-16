use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::io::{BufRead, BufReader, Read};

use grid::Grid;
use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::parse_error;

use state::drones::WithFourDrones;
use state::you::WithJustYou;
use tile::Tile;

use crate::state::State;

mod actor;
mod state;
mod tile;

#[derive(Debug, Clone)]
pub struct Cave {
    pub(crate) layout: Grid<Tile>,
    nr_keys: usize,
}

impl Cave {
    pub fn find_quickest_route_by_yourself(&self) -> Option<usize> {
        self.find_quickest_route(WithJustYou::get_initial_state(&self))
    }

    pub fn find_quickest_route_with_four_drones(&self) -> Option<usize> {
        let cave = self.with_four_entrances();
        cave.find_quickest_route(WithFourDrones::get_initial_state(&cave))
    }

    pub(crate) fn with_four_entrances(&self) -> Cave {
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

    pub(crate) fn find_quickest_route<T>(&self, initial_state: T) -> Option<usize>
    where
        T: State + Clone + Debug + Display + Eq + PartialEq + Hash,
    {
        let mut seen: HashSet<T> = HashSet::new();
        let mut exploring: VecDeque<(T, usize)> = VecDeque::new();

        seen.insert(initial_state.clone());
        exploring.push_back((initial_state, 0));

        let mut pd = 0;
        while let Some((state, distance)) = exploring.pop_front() {
            if pd != distance {
                pd = distance;
                println!("distance={}", pd);
            }
            // println!("distance={}, state={}", distance, state);
            for (mut next_state, actor) in state.get_next_states() {
                if actor.is_done() {
                    // no keys left for this actor to collect, disregard
                    continue;
                }

                match self.layout[&actor] {
                    Tile::Entrance => {}
                    Tile::Wall => {
                        continue;
                    }
                    Tile::Open => {}
                    Tile::Door(key) => {
                        if !actor.has_key(key) {
                            continue;
                        }
                    }
                    Tile::Key(key) => {
                        if !next_state.has_key(key) {
                            next_state.receive_key(key);
                            if next_state.nr_keys() == self.nr_keys {
                                return Some(distance + 1);
                            }
                        }
                    }
                }

                if !seen.contains(&next_state) {
                    seen.insert(next_state.clone());
                    exploring.push_back((next_state, distance + 1));
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
    use crate::state::you::WithJustYou;

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
        let initial_state = WithJustYou::get_initial_state(&cave);
        assert_eq!(cave.find_quickest_route(initial_state), Some(132));
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
        let initial_state = WithJustYou::get_initial_state(&cave);
        assert_eq!(cave.find_quickest_route(initial_state), Some(136));
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
        let initial_state = WithJustYou::get_initial_state(&cave);
        assert_eq!(cave.find_quickest_route(initial_state), Some(81));
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
        let initial_state = WithFourDrones::get_initial_state(&cave);
        assert_eq!(cave.find_quickest_route(initial_state), Some(72));
    }
}
