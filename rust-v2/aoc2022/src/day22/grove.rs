use std::fmt;

use grid::Grid;

use rdcl_aoc_core::error::ParseError;

use crate::direction::Direction;
use crate::face::Face;
use crate::next::Next;
use crate::tile::Tile;

#[derive(Debug)]
pub struct Grove {
    tiles: Grid<Tile>,
    position: (usize, usize),
    direction: Direction,
    face: Face,
}

impl Grove {
    pub fn parse<T>(input: &mut T, is_cube: bool) -> Result<Grove, ParseError>
    where
        T: Iterator<Item = String>,
    {
        let mut rows: Vec<Vec<Tile>> = vec![];
        let mut width = 0;
        let mut position = (0, 0);

        for (y, line) in input.take_while(|line| !line.is_empty()).enumerate() {
            width = width.max(line.len() + 1);
            let mut row: Vec<Tile> = Vec::with_capacity(width);
            // add an additional out-of-bounds tile to the left
            row.push(Tile::OutOfBounds);

            for (x, tile) in line.chars().map(Tile::from).enumerate() {
                if position == (0, 0) && tile.is_within_bounds() {
                    position = (x + 1, y + 1);
                }
                row.push(tile);
            }
            rows.push(row);
        }

        // add additional out-of-bounds tiles to the right
        width += 1;

        let mut tiles = Grid::new(0, width);
        // add additional out-of-bounds tiles to the top
        tiles.push_row(vec![Tile::OutOfBounds; width]);
        for mut row in rows {
            if row.len() > width {
                panic!("Error while parsing, computed an incorrect value for width");
            }
            row.resize(width, Tile::OutOfBounds);
            tiles.push_row(row);
        }
        // add additional out-of-bounds tiles to the bottom
        tiles.push_row(vec![Tile::OutOfBounds; width]);

        Ok(Grove {
            tiles,
            position,
            direction: Direction::default(),
            face: if is_cube { Face::F1 } else { Face::F0 },
        })
    }

    pub fn follow_instructions(&mut self, instructions: String) {
        let mut rotating = false;
        let instructions: Vec<char> = instructions.chars().collect();
        let mut i = 0;

        while i < instructions.len() {
            if rotating {
                match instructions[i] {
                    'L' => self.turn_left(),
                    'R' => self.turn_right(),
                    v => panic!("Unexpected character while rotating: {}", v),
                }
                i += 1;
                rotating = false;
            } else {
                let next_i = instructions[i..]
                    .iter()
                    .enumerate()
                    .find(|(_, v)| !v.is_numeric())
                    .map(|(i, _)| i);

                let next_i = match next_i {
                    Some(v) => i + v,
                    None => instructions.len(),
                };

                let steps = instructions[i..next_i]
                    .iter()
                    .fold(String::new(), |mut acc, v| {
                        acc.push(*v);
                        acc
                    })
                    .parse::<usize>()
                    .unwrap();

                self.walk(steps);

                i = next_i;
                rotating = true;
            }
        }
    }

    pub fn password(&self) -> usize {
        let (x, y) = self.position;
        let facing = match self.direction {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        };

        1000 * y + 4 * x + facing
    }

    fn turn_left(&mut self) {
        self.direction = self.direction.turn_left();
    }

    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }

    fn walk(&mut self, steps: usize) {
        for _ in 0..steps {
            let next = if self.face.is_cube() {
                self.check_cube_bounds()
            } else {
                self.check_bounds()
            };

            match self.tiles.get(next.y(), next.x()).unwrap() {
                Tile::Open => {
                    self.position = next.position;
                    self.direction = next.direction;
                    self.face = next.face;
                }
                Tile::Closed => {
                    return;
                }
                Tile::OutOfBounds => {
                    panic!(); // not possible
                }
            }
        }
    }

    fn check_bounds(&self) -> Next {
        let rows = self.tiles.rows();
        let cols = self.tiles.cols();
        let wrap = move |(mut x, mut y): (usize, usize)| -> (usize, usize) {
            if x == 0 {
                x = cols - 2;
            }
            if x == cols - 1 {
                x = 1;
            }
            if y == 0 {
                y = rows - 2;
            }
            if y == rows - 1 {
                y = 1;
            }

            (x, y)
        };

        let mut next = wrap(self.direction.walk(self.position));
        let mut tile = *self.tiles.get(next.1, next.0).unwrap();
        while !tile.is_within_bounds() {
            next = wrap(self.direction.walk(next));
            if next == self.position {
                panic!(
                    "Wrapped all the way round while facing {:?} (position={:?})",
                    self.direction, self.position
                );
            }
            tile = *self.tiles.get(next.1, next.0).unwrap();
        }

        Next {
            position: next,
            direction: self.direction,
            face: self.face,
        }
    }

    fn check_cube_bounds(&self) -> Next {
        let cube_size = self.cube_size();
        let mut next = Next {
            position: self.direction.walk(self.position),
            direction: self.direction,
            face: self.face,
        };
        let tile = *self.tiles.get(next.y(), next.x()).unwrap();

        if tile.is_within_bounds() {
            next.face = Face::determine_face(cube_size, next.position).unwrap();
        } else {
            let (x, y) = self.position;
            let (face_x, face_y) = self.face.xy(cube_size);

            let side = cube_size - 1;

            let x_before = x - face_x;
            let y_before = y - face_y;
            let y_after = side - y_before;

            next = match (self.face, self.direction) {
                (Face::F1, Direction::Up) => Next {
                    position: (Face::F5.x(cube_size), Face::F5.y(cube_size) + x_before),
                    direction: Direction::Right,
                    face: Face::F5,
                },
                (Face::F1, Direction::Left) => Next {
                    position: (Face::F4.x(cube_size), Face::F4.y(cube_size) + y_after),
                    direction: Direction::Right,
                    face: Face::F4,
                },
                (Face::F2, Direction::Left) => Next {
                    position: (Face::F4.x(cube_size) + y_before, Face::F4.y(cube_size)),
                    direction: Direction::Down,
                    face: Face::F4,
                },
                (Face::F2, Direction::Right) => Next {
                    position: (
                        Face::F3.x(cube_size) + y_before,
                        Face::F3.y(cube_size) + side,
                    ),
                    direction: Direction::Up,
                    face: Face::F3,
                },
                (Face::F3, Direction::Up) => Next {
                    position: (
                        Face::F5.x(cube_size) + x_before,
                        Face::F5.y(cube_size) + side,
                    ),
                    direction: Direction::Up,
                    face: Face::F5,
                },
                (Face::F3, Direction::Down) => Next {
                    position: (
                        Face::F2.x(cube_size) + side,
                        Face::F2.y(cube_size) + x_before,
                    ),
                    direction: Direction::Left,
                    face: Face::F2,
                },
                (Face::F3, Direction::Right) => Next {
                    position: (
                        Face::F6.x(cube_size) + side,
                        Face::F6.y(cube_size) + y_after,
                    ),
                    direction: Direction::Left,
                    face: Face::F6,
                },
                (Face::F4, Direction::Up) => Next {
                    position: (Face::F2.x(cube_size), Face::F2.y(cube_size) + x_before),
                    direction: Direction::Right,
                    face: Face::F2,
                },
                (Face::F4, Direction::Left) => Next {
                    position: (Face::F1.x(cube_size), Face::F1.y(cube_size) + y_after),
                    direction: Direction::Right,
                    face: Face::F1,
                },
                (Face::F5, Direction::Down) => Next {
                    position: (Face::F3.x(cube_size) + x_before, Face::F3.y(cube_size)),
                    direction: Direction::Down,
                    face: Face::F3,
                },
                (Face::F5, Direction::Left) => Next {
                    position: (Face::F1.x(cube_size) + y_before, Face::F1.y(cube_size)),
                    direction: Direction::Down,
                    face: Face::F1,
                },
                (Face::F5, Direction::Right) => Next {
                    position: (
                        Face::F6.x(cube_size) + y_before,
                        Face::F6.y(cube_size) + side,
                    ),
                    direction: Direction::Up,
                    face: Face::F6,
                },
                (Face::F6, Direction::Down) => Next {
                    position: (
                        Face::F5.x(cube_size) + side,
                        Face::F5.y(cube_size) + x_before,
                    ),
                    direction: Direction::Left,
                    face: Face::F5,
                },
                (Face::F6, Direction::Right) => Next {
                    position: (
                        Face::F3.x(cube_size) + side,
                        Face::F3.y(cube_size) + y_after,
                    ),
                    direction: Direction::Left,
                    face: Face::F3,
                },
                _ => panic!(),
            };
        }

        next
    }

    fn cube_size(&self) -> usize {
        self.tiles.cols() / 3
    }
}

impl fmt::Display for Grove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 1..self.tiles.rows() {
            if y != 0 {
                writeln!(f)?;
            }

            for x in 1..self.tiles.cols() {
                if (x, y) == self.position {
                    write!(f, "{}", self.direction)?;
                } else {
                    write!(f, "{}", self.tiles.get(y, x).unwrap())?;
                }
            }
        }

        Ok(())
    }
}
