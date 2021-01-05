use std::collections::HashMap;
use std::fmt;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::MultilineFromStr;

use crate::rail_map::cart_state::CartState;
use crate::rail_map::track::Track;

mod cart_state;
mod track;

#[derive(Debug, Clone)]
pub struct RailMap {
    rails: Vec<Vec<Track>>,
    carts: HashMap<(usize, usize), CartState>,
    crashed: Vec<(usize, usize)>,
}

impl RailMap {
    /// Returns true if so far no cart has crashed.
    pub fn is_crash_free(&self) -> bool {
        self.crashed.is_empty()
    }

    /// Returns the coordinates of the first crash site.
    pub fn get_first_crash(&self) -> Option<(usize, usize)> {
        self.crashed.first().copied()
    }

    /// Returns the number of carts that are still operational.
    pub fn get_nr_carts_remaining(&self) -> usize {
        self.carts.len()
    }

    /// If there is only one cart remaining, returns its position.
    pub fn get_last_remaining_cart(&self) -> Option<(usize, usize)> {
        if self.carts.len() == 1 {
            self.carts.keys().next().copied()
        } else {
            None
        }
    }

    /// Let every cart move ahead one position.
    pub fn tick(&mut self) {
        let mut positions: Vec<(usize, usize)> = self.carts.keys().copied().collect();
        positions.sort_unstable();
        for position in positions {
            if let Some(cart) = self.carts.remove(&position) {
                let next_position = cart.next(position);
                if self.carts.remove(&next_position).is_some() {
                    self.crashed.push(next_position);
                } else {
                    let (x, y) = next_position;
                    let next_state = match self.rails[y][x] {
                        Track::Vertical => cart,
                        Track::Horizontal => cart,
                        Track::Intersection => cart.pick_direction(),
                        Track::TopLeft if cart.faces_left() => cart.turn_left(),
                        Track::TopLeft if cart.faces_up() => cart.turn_right(),
                        Track::TopRight if cart.faces_up() => cart.turn_left(),
                        Track::TopRight if cart.faces_right() => cart.turn_right(),
                        Track::BottomLeft if cart.faces_down() => cart.turn_left(),
                        Track::BottomLeft if cart.faces_left() => cart.turn_right(),
                        Track::BottomRight if cart.faces_right() => cart.turn_left(),
                        Track::BottomRight if cart.faces_down() => cart.turn_right(),
                        _ => panic!(
                            "A cart ({:?}) has reached an invalid state: {:?} -> {:?} ({:?}).",
                            cart, position, next_position, self.rails[y][x]
                        ),
                    };
                    self.carts.insert(next_position, next_state);
                }
            }
        }
    }
}

impl fmt::Display for RailMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (y, row) in self.rails.iter().enumerate() {
            if y != 0 {
                writeln!(f)?;
            }
            for (x, track) in row.iter().enumerate() {
                if let Some(cart_state) = self.carts.get(&(x, y)) {
                    write!(f, "{}", cart_state)?;
                } else if self.crashed.contains(&(x, y)) {
                    write!(f, "X")?;
                } else {
                    write!(f, "{}", track)?;
                }
            }
        }
        Ok(())
    }
}

impl MultilineFromStr for RailMap {
    type Err = ParseError;

    fn new() -> Self {
        RailMap {
            rails: Vec::new(),
            carts: HashMap::new(),
            crashed: Vec::new(),
        }
    }

    fn indicates_new_record(&self, _line: &str) -> bool {
        false
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        let mut row = Vec::new();
        let mut prev_was_horizontal = false;
        let y = self.rails.len();
        for (x, ch) in line.chars().enumerate() {
            match ch {
                ' ' => {
                    row.push(Track::None);
                    prev_was_horizontal = false;
                }
                '-' => {
                    row.push(Track::Horizontal);
                    prev_was_horizontal = true;
                }
                '|' => {
                    row.push(Track::Vertical);
                    prev_was_horizontal = false;
                }
                '+' => {
                    row.push(Track::Intersection);
                    prev_was_horizontal = true;
                }
                '/' => {
                    if prev_was_horizontal {
                        row.push(Track::BottomRight);
                        prev_was_horizontal = false;
                    } else {
                        row.push(Track::TopLeft);
                        prev_was_horizontal = true;
                    }
                }
                '\\' => {
                    if prev_was_horizontal {
                        row.push(Track::TopRight);
                        prev_was_horizontal = false;
                    } else {
                        row.push(Track::BottomLeft);
                        prev_was_horizontal = true;
                    }
                }
                '^' => {
                    row.push(Track::Vertical);
                    self.carts.insert((x, y), CartState::Up(0));
                    prev_was_horizontal = false;
                }
                'v' => {
                    row.push(Track::Vertical);
                    self.carts.insert((x, y), CartState::Down(0));
                    prev_was_horizontal = false;
                }
                '<' => {
                    row.push(Track::Horizontal);
                    self.carts.insert((x, y), CartState::Left(0));
                    prev_was_horizontal = true;
                }
                '>' => {
                    row.push(Track::Horizontal);
                    self.carts.insert((x, y), CartState::Right(0));
                    prev_was_horizontal = true;
                }
                _ => {
                    return Err(ParseError(format!(
                        "Invalid input encountered @{},{}: {}",
                        x, y, ch
                    )));
                }
            }
        }
        self.rails.push(row);
        Ok(())
    }
}
