use std::collections::HashSet;
use std::fmt;

use rdcl_aoc_helpers::error::ParseError;
use rdcl_aoc_helpers::input::MultilineFromStr;
use rdcl_aoc_helpers::parse_error;

pub type Point = (usize, usize);

#[derive(Debug, Clone)]
pub struct Blueprints {
    width: usize,
    height: usize,
    open_spaces: HashSet<Point>,
    points_of_interest: Vec<Point>,
}

impl Blueprints {
    pub fn visited_all_points_of_interest(&self, points: &[Point]) -> bool {
        points.len() == self.points_of_interest.len()
    }

    pub fn starting_point(&self) -> Option<Point> {
        self.points_of_interest.first().cloned()
    }

    pub fn find_shortest_path(&self, from: Point, to: Point) -> Option<usize> {
        let mut seen = HashSet::new();
        seen.insert(from);

        let mut checking = HashSet::new();
        checking.insert(from);

        let mut distance = 0;

        while !checking.is_empty() {
            distance += 1;
            let mut next_checking = HashSet::new();
            for point in checking {
                for neighbour in self.neighbours(point) {
                    if neighbour == to {
                        return Some(distance);
                    }
                    if self.open_spaces.contains(&neighbour) && !seen.contains(&neighbour) {
                        seen.insert(neighbour);
                        next_checking.insert(neighbour);
                    }
                }
            }
            checking = next_checking;
        }

        None
    }

    pub fn find_closest_points_of_interest(
        &self,
        from: Point,
        exclude: &HashSet<Point>,
    ) -> Vec<(Point, usize)> {
        let mut found = Vec::new();

        let mut seen = HashSet::new();
        seen.insert(from);

        let mut checking = HashSet::new();
        checking.insert(from);

        let mut distance = 0;

        while !checking.is_empty() {
            distance += 1;
            let mut next_checking = HashSet::new();
            for point in checking {
                for neighbour in self.neighbours(point) {
                    if self.open_spaces.contains(&neighbour) && !seen.contains(&neighbour) {
                        seen.insert(neighbour);
                        if !exclude.contains(&neighbour)
                            && self.points_of_interest.contains(&neighbour)
                        {
                            found.push((neighbour, distance));
                        } else {
                            next_checking.insert(neighbour);
                        }
                    }
                }
            }
            checking = next_checking;
        }

        found
    }

    fn neighbours(&self, point: Point) -> Vec<Point> {
        // Assumption: The entire blueprint is always surrounded by a wall, so we don't need to
        // check if our indexes become 0.

        vec![
            (point.0, point.1 - 1),
            (point.0, point.1 + 1),
            (point.0 - 1, point.1),
            (point.0 + 1, point.1),
        ]
    }
}

impl fmt::Display for Blueprints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            if y != 0 {
                writeln!(f)?;
            }
            for x in 0..self.width {
                if let Some(idx) = self
                    .points_of_interest
                    .iter()
                    .position(|point| *point == (y, x))
                {
                    write!(f, "{}", idx)?;
                } else if self.open_spaces.contains(&(y, x)) {
                    write!(f, ".")?;
                } else {
                    write!(f, "#")?;
                }
            }
        }
        Ok(())
    }
}

impl MultilineFromStr for Blueprints {
    type Err = ParseError;

    fn new() -> Self {
        Blueprints {
            width: 0,
            height: 0,
            open_spaces: HashSet::new(),
            points_of_interest: Vec::new(),
        }
    }

    fn indicates_new_record(&self, _line: &str) -> bool {
        false
    }

    fn parse(&mut self, line: &str) -> Result<(), Self::Err> {
        let row = self.height;
        self.height += 1;

        for (column, ch) in line.chars().enumerate() {
            if row == 0 {
                self.width += 1;
            } else if column >= self.width {
                return Err(parse_error!(
                    "Could not parse input, as lines have inconsistent width.",
                ));
            }
            match ch {
                '#' => {}
                '.' => {
                    self.open_spaces.insert((row, column));
                }
                _ => {
                    self.open_spaces.insert((row, column));
                    let idx = ((ch as u8) - b'0') as usize;
                    if idx >= self.points_of_interest.len() {
                        self.points_of_interest.resize(idx + 1, (0, 0));
                    }
                    self.points_of_interest[idx] = (row, column);
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use rdcl_aoc_helpers::input::WithAsMultilineRecords;

    use super::*;

    #[test]
    fn test() {
        let blueprints = get_test_blueprints();

        assert_eq!(
            blueprints.points_of_interest,
            vec![(1, 1), (1, 3), (1, 9), (3, 9), (3, 1)]
        );
        assert_eq!(blueprints.open_spaces.len(), 20);
        assert!(blueprints
            .points_of_interest
            .iter()
            .all(|point| blueprints.open_spaces.contains(point)));
    }

    pub fn get_test_blueprints() -> Blueprints {
        vec![
            "###########",
            "#0.1.....2#",
            "#.#######.#",
            "#4.......3#",
            "###########",
        ]
        .as_multiline_records::<Blueprints>()
        .unwrap()
        .first()
        .unwrap()
        .clone()
    }
}
