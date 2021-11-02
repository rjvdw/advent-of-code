use std::collections::HashSet;
use std::fmt;
use std::num::ParseIntError;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::MultilineFromStr;
use termion::color;

type Coordinate = (i64, i64);

#[derive(Debug, Clone)]
pub struct Ground {
    spring: Coordinate,
    clay: HashSet<Coordinate>,
    wet: HashSet<Coordinate>,
    settled: HashSet<Coordinate>,
    flowing: HashSet<Coordinate>,
    min_x: i64,
    max_x: i64,
    min_depth: i64,
    max_depth: i64,
    print_color: bool,
}

impl Ground {
    /// Let the water flow.
    pub fn flow(&mut self) {
        let mut next_flowing = HashSet::new();
        for &(x, y) in &self.flowing {
            if self.can_flow_down((x, y)) {
                // flow down
                if y < self.max_depth {
                    self.wet.insert((x, y + 1));
                    next_flowing.insert((x, y + 1));
                }
            } else {
                let (left, next_left) = self.flow_to_side((x, y), Side::Left);
                let (right, next_right) = self.flow_to_side((x, y), Side::Right);

                if next_left.is_none() && next_right.is_none() {
                    // we are in a basin, so mark all tiles as wet and settled, and flow up one level
                    for tile in left {
                        self.wet.insert(tile);
                        self.settled.insert(tile);
                    }
                    for tile in right {
                        self.wet.insert(tile);
                        self.settled.insert(tile);
                    }
                    next_flowing.insert((x, y - 1));
                } else {
                    // we can flow to one or both sides
                    for tile in left {
                        self.wet.insert(tile);
                    }
                    for tile in right {
                        self.wet.insert(tile);
                    }
                    if let Some(t) = next_left {
                        next_flowing.insert(t);
                    }
                    if let Some(t) = next_right {
                        next_flowing.insert(t);
                    }
                }
            }
        }
        self.flowing = next_flowing;
    }

    /// Check if it is possible for the water to flow down.
    fn can_flow_down(&self, (x, y): Coordinate) -> bool {
        !self.clay.contains(&(x, y + 1)) && !self.settled.contains(&(x, y + 1))
    }

    /// Flow to the side until you can either flow down again, or until you hit a wall.
    fn flow_to_side(
        &self,
        (x, y): Coordinate,
        side: Side,
    ) -> (Vec<Coordinate>, Option<Coordinate>) {
        let mut tiles = Vec::new();

        let mut i = x;
        while side.within_bounds(self, i) {
            // clay encountered, stop flowing
            if self.clay.contains(&(i, y)) {
                return (tiles, None);
            }

            // sand, so the tile becomes wet
            tiles.push((i, y));

            // if we can flow down, we can also stop flowing and provide the next tile to flow to
            if self.can_flow_down((i, y)) {
                return (tiles, Some((i, y)));
            }

            // otherwise, we keep on going.
            i = side.next(i);
        }

        (tiles, None)
    }

    /// Count the number of tiles that got wet.
    pub fn get_nr_of_wet_tiles(&self) -> usize {
        self.wet
            .iter()
            .filter(|(_, y)| *y >= self.min_depth && *y <= self.max_depth)
            .count()
    }

    pub fn get_nr_of_settled_tiles(&self) -> usize {
        self.settled.len()
    }

    /// Check if the water can still flow any further.
    pub fn done(&self) -> bool {
        self.flowing.is_empty()
    }

    /// Sets whether colors should be used when printing.
    pub fn set_print_color(&mut self, value: bool) {
        self.print_color = value;
    }
}

enum Side {
    Left,
    Right,
}

impl Side {
    fn within_bounds(&self, ground: &Ground, i: i64) -> bool {
        match self {
            Side::Left => i >= ground.min_x,
            Side::Right => i <= ground.max_x,
        }
    }

    fn next(&self, i: i64) -> i64 {
        match self {
            Side::Left => i - 1,
            Side::Right => i + 1,
        }
    }
}

impl fmt::Display for Ground {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "x: {}..{}; y: 0..{}",
            self.min_x, self.max_x, self.max_depth
        )?;

        let water;
        let clay;
        let sand;

        if self.print_color {
            water = format!("{}{}", color::Bg(color::Blue), color::Fg(color::White));
            clay = format!("{}{}", color::Bg(color::Yellow), color::Fg(color::Yellow));
            sand = format!(
                "{}{}",
                color::Bg(color::LightYellow),
                color::Fg(color::LightYellow)
            );
        } else {
            water = String::new();
            clay = String::new();
            sand = String::new();
        }

        for y in 0..=self.max_depth + 2 {
            if y != 0 {
                writeln!(f)?;
            }
            write!(f, "[{:5}]  ", y)?;
            for x in self.min_x - 2..=self.max_x + 2 {
                if (x, y) == self.spring {
                    write!(f, "{}+", water)?;
                } else if self.settled.contains(&(x, y)) {
                    write!(f, "{}~", water)?;
                } else if self.wet.contains(&(x, y)) {
                    write!(f, "{}|", water)?;
                } else if self.clay.contains(&(x, y)) {
                    write!(f, "{}#", clay)?;
                } else if y > 0 {
                    write!(f, "{}.", sand)?;
                } else {
                    write!(f, " ")?;
                }
            }
            if self.print_color {
                write!(f, "{}{}", color::Bg(color::Reset), color::Fg(color::Reset))?;
            }
        }

        Ok(())
    }
}

impl MultilineFromStr for Ground {
    type Err = ParseError;

    fn new() -> Self {
        let mut ground = Ground {
            spring: (500, 0),
            clay: Default::default(),
            wet: Default::default(),
            settled: Default::default(),
            flowing: Default::default(),
            min_x: i64::MAX,
            max_x: i64::MIN,
            min_depth: i64::MAX,
            max_depth: 0,
            print_color: false,
        };
        ground.wet.insert(ground.spring);
        ground.flowing.insert(ground.spring);
        ground
    }

    fn indicates_new_record(&self, _line: &str) -> bool {
        false
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        let x_range = parse_part(line, 'x')?;
        let y_range = parse_part(line, 'y')?;

        for &x in &x_range {
            if x - 1 < self.min_x {
                self.min_x = x - 1;
            }
            if x + 1 > self.max_x {
                self.max_x = x + 1;
            }
            for &y in &y_range {
                if y < self.min_depth {
                    self.min_depth = y;
                }
                if y > self.max_depth {
                    self.max_depth = y;
                }
                self.clay.insert((x, y));
            }
        }

        Ok(())
    }
}

fn parse_part(line: &str, ch: char) -> Result<Vec<i64>, ParseIntError> {
    let mut range_start = String::new();
    let range_end = line
        .chars()
        .skip_while(|c| *c != ch)
        .skip(2)
        .take_while(|c| *c != ',')
        .fold(String::new(), |mut acc, c| {
            if c == '.' {
                if range_start.is_empty() {
                    range_start = acc;
                    String::new()
                } else {
                    acc
                }
            } else {
                acc.push(c);
                acc
            }
        });

    if range_start.is_empty() {
        Ok(vec![range_end.parse()?])
    } else {
        let range_start = range_start.parse::<i64>()?;
        let range_end = range_end.parse::<i64>()?;
        Ok((range_start..=range_end).collect())
    }
}
