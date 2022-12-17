use std::collections::HashSet;
use std::fmt;

use crate::point::Point;
use crate::rock::Rock;

const WIDTH: usize = 7;

#[derive(Debug)]
pub struct Chamber {
    spawn_offset: (usize, usize),
    falling: FallingRock,
    fallen: usize,
    covered: HashSet<Point>,
    height: usize,
}

impl Chamber {
    pub fn new(left: usize, bottom: usize) -> Chamber {
        Chamber {
            spawn_offset: (left, bottom),
            falling: FallingRock {
                rock: Rock::nth(0),
                position: Point(left, bottom),
            },
            fallen: 0,
            covered: HashSet::new(),
            height: 0,
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    fn top(&self) -> usize {
        self.height
            .max(self.falling.position.1 + self.falling.rock.height())
    }

    pub fn fallen_rocks_count(&self) -> usize {
        self.fallen
    }

    pub fn step(&mut self, wind: char) {
        match wind {
            '<' => self.blow_left(),
            '>' => self.blow_right(),
            _ => panic!("Invalid character: {}", wind),
        }

        self.fall();
    }

    pub fn summarize(&self) -> String {
        let mut y = self.top();
        let mut summary = self.falling.summarize(y);

        while !self.row_blocks(y) {
            for x in 0..WIDTH {
                let point = Point(x, y);
                if self.covered.contains(&point) {
                    summary.push('#');
                } else {
                    summary.push('.');
                }
            }

            y = match y.checked_sub(1) {
                Some(v) => v,
                None => {
                    return summary;
                }
            };
        }

        summary
    }

    /// Checks whether the given row would block the falling rock from passing.
    fn row_blocks(&self, row: usize) -> bool {
        let mut gaps = 0;
        for x in 0..WIDTH {
            let blocked = (row..=self.top())
                .rev()
                .any(|y| self.covered.contains(&Point(x, y)));
            if !blocked {
                gaps += 1;
            }
        }

        gaps < self.falling.rock.width()
    }

    fn blow_left(&mut self) {
        if self.falling.position.0 > 0 {
            let next_position = Point(self.falling.position.0 - 1, self.falling.position.1);
            if !self.test_position_for_overlap(next_position) {
                self.falling.position = next_position;
            }
        }
    }

    fn blow_right(&mut self) {
        if self.falling.position.0 + self.falling.rock.width() < WIDTH {
            let next_position = Point(self.falling.position.0 + 1, self.falling.position.1);
            if !self.test_position_for_overlap(next_position) {
                self.falling.position = next_position;
            }
        }
    }

    fn fall(&mut self) {
        if self.falling.position.1 == 0 {
            self.rock_comes_to_rest();
        } else {
            let next_position = Point(self.falling.position.0, self.falling.position.1 - 1);
            if self.test_position_for_overlap(next_position) {
                self.rock_comes_to_rest();
            } else {
                self.falling.position = next_position;
            }
        }
    }

    fn test_position_for_overlap(&self, position: Point) -> bool {
        position.1 <= self.height
            && self
                .falling
                .rock
                .points(position)
                .iter()
                .any(|p| self.covered.contains(p))
    }

    fn rock_comes_to_rest(&mut self) {
        for point in self.falling.rock.points(self.falling.position) {
            self.covered.insert(point);
        }
        self.height = self
            .height
            .max(self.falling.position.1 + self.falling.rock.height());
        self.fallen += 1;
        self.falling = FallingRock {
            rock: Rock::nth(self.fallen),
            position: Point(self.spawn_offset.0, self.height + self.spawn_offset.1),
        };
    }
}

impl fmt::Display for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rock = self.falling.rock.points(self.falling.position);

        for y in (0..=self.top()).rev() {
            write!(f, "|")?;
            for x in 0..WIDTH {
                let point = Point(x, y);
                if rock.contains(&point) {
                    write!(f, "@")?;
                } else if self.covered.contains(&point) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "|")?;
        }

        write!(f, "+")?;
        for _ in 0..WIDTH {
            write!(f, "-")?;
        }
        write!(f, "+")?;

        Ok(())
    }
}

#[derive(Debug)]
struct FallingRock {
    rock: Rock,
    position: Point,
}

impl FallingRock {
    fn summarize(&self, y_offset: usize) -> String {
        format!(
            "{}@{},{}:",
            match self.rock {
                Rock::HorizontalLine => 0,
                Rock::Plus => 1,
                Rock::Corner => 2,
                Rock::VerticalLine => 3,
                Rock::Square => 4,
            },
            self.position.0,
            y_offset - self.position.1
        )
    }
}
