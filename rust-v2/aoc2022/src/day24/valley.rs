use std::collections::HashMap;
use std::fmt;

use rdcl_aoc_core::err_parse_error;
use rdcl_aoc_core::error::ParseError;
use rdcl_aoc_core::input::FromInput;
use rdcl_aoc_pathfinding::taxi_cab_2d;

use crate::direction::Direction;

pub type BlizzardsMap = HashMap<(usize, usize), Vec<Direction>>;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Valley {
    blizzards: BlizzardsMap,
    width: usize,
    height: usize,
    position: (usize, usize),
    start: (usize, usize),
    goal: (usize, usize),
}

impl Valley {
    /// Has the goal been reached?
    pub fn done(&self) -> bool {
        self.position == self.goal
    }

    /// Flip the start and goal.
    pub fn flip(&self) -> Valley {
        Valley {
            blizzards: self.blizzards.clone(),
            width: self.width,
            height: self.height,
            position: self.position,
            start: self.goal,
            goal: self.start,
        }
    }

    /// Determine all possible states the valley could be in during the next minute.
    pub fn get_possible_transitions(&self) -> Vec<Valley> {
        let mut next: BlizzardsMap = HashMap::new();

        for (&position, directions) in &self.blizzards {
            for &direction in directions {
                let mut next_position = direction.travel(position);

                if next_position.0 == 0 {
                    next_position.0 = self.width - 2;
                }
                if next_position.0 + 1 == self.width {
                    next_position.0 = 1;
                }
                if next_position.1 == 0 {
                    next_position.1 = self.height - 2;
                }
                if next_position.1 + 1 == self.height {
                    next_position.1 = 1;
                }

                next.entry(next_position).or_default().push(direction);
            }
        }

        Direction::neighbours(self.position)
            .into_iter()
            .filter(|&p| self.within_bounds(p))
            .filter(|&p| Valley::is_valid_position(p, &next))
            .map(|position| Valley {
                blizzards: next.clone(),
                width: self.width,
                height: self.height,
                position,
                start: self.start,
                goal: self.goal,
            })
            .collect()
    }

    fn within_bounds(&self, position: (usize, usize)) -> bool {
        if position == self.start || position == self.goal {
            true
        } else {
            let (x, y) = position;
            x != 0 && x + 1 < self.width && y != 0 && y + 1 < self.height
        }
    }

    fn is_valid_position(position: (usize, usize), blizzards: &BlizzardsMap) -> bool {
        !blizzards.contains_key(&position)
    }

    /// The minimum distance that needs to be travelled to reach the goal.
    pub fn distance_to_goal(&self) -> usize {
        taxi_cab_2d(self.position, self.goal)
    }
}

impl FromInput for Valley {
    fn parse<T>(input: T) -> Result<Valley, ParseError>
    where
        T: Iterator<Item = String>,
    {
        let mut blizzards: BlizzardsMap = HashMap::new();
        let mut width = 0;
        let mut height = 0;
        let mut start = (0, 0);
        let mut goal = (0, 0);

        for (y, line) in input.enumerate() {
            if width == 0 {
                width = line.len();
            } else if width != line.len() {
                panic!("Inconsistent width");
            }

            for (x, ch) in line.chars().enumerate() {
                match ch {
                    // must be at the bottom row, use this to set the height
                    '#' if x != 0 && x + 1 != width && y != 0 && height == 0 => {
                        height = y + 1;
                    }

                    // walls on the edge
                    '#' if x == 0 || x + 1 == width || y == 0 || y + 1 == height => {
                        // noop
                    }

                    // walls at any other position
                    '#' => {
                        return err_parse_error!("unexpected wall @({}, {})", x, y);
                    }

                    // the entrance
                    '.' if y == 0 && start == (0, 0) => {
                        start = (x, y);
                    }

                    // the exit
                    '.' if y + 1 == height && goal == (0, 0) => {
                        goal = (x, y);
                    }

                    // any other space where a wall is expected
                    _ if x == 0 || x + 1 == width || y == 0 || y + 1 == height => {
                        return err_parse_error!(
                            "unexpected '{}' @({}, {}) where a wall is expected",
                            ch,
                            x,
                            y
                        );
                    }

                    // open spaces
                    '.' => {
                        // noop
                    }

                    // blizzards
                    '^' | '>' | 'v' | '<' => {
                        blizzards
                            .entry((x, y))
                            .or_default()
                            .push(Direction::from(ch));
                    }

                    // anything else
                    _ => {
                        return err_parse_error!("unexpected '{}' @({}, {})", ch, x, y);
                    }
                }
            }
        }

        Ok(Valley {
            blizzards,
            width,
            height,
            position: start,
            start,
            goal,
        })
    }
}

impl fmt::Display for Valley {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if (x, y) == self.position {
                    write!(f, "E")?;
                } else if (x, y) == self.start || (x, y) == self.goal {
                    write!(f, ".")?;
                } else if x == 0 || x + 1 == self.width || y == 0 || y + 1 == self.height {
                    write!(f, "#")?;
                } else if let Some(directions) = self.blizzards.get(&(x, y)) {
                    if directions.len() == 1 {
                        write!(f, "{}", directions[0])?;
                    } else {
                        write!(f, "{}", directions.len())?;
                    }
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use rdcl_aoc_core::input::InputReader;

    use super::*;

    pub fn test_data() -> Valley {
        InputReader::from("./src/day24/test.txt").parse::<Valley>()
    }

    #[test]
    fn test_parse() {
        let valley = test_data();
        assert_eq!(valley.width, 8);
        assert_eq!(valley.height, 6);
        assert_eq!(valley.position, (1, 0));
        assert_eq!(valley.start, (1, 0));
        assert_eq!(valley.goal, (6, 5));
    }
}
