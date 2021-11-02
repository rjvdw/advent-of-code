use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Display, Formatter};

use grid::Grid;

use shared::intcode;

use crate::tile::Tile;

pub mod tile;

pub struct Game {
    screen: Grid<Tile>,
    score: i64,
    program: intcode::Program,
    ball: (i64, i64),
    paddle: (i64, i64),
}

impl Game {
    fn remaining_blocks(&self) -> usize {
        self.screen
            .iter()
            .filter(|&&tile| matches!(tile, Tile::Block))
            .count()
    }

    pub fn from_program(program: intcode::Program, nr_quarters_inserted: Option<i64>) -> Game {
        let mut game = Game {
            screen: Grid::new(1, 1),
            score: 0,
            program,
            ball: (0, 0),
            paddle: (0, 0),
        };
        if let Some(v) = nr_quarters_inserted {
            game.program[0] = v;
        }
        game.run();
        game
    }

    pub fn input(&mut self, value: i64) {
        self.program.send_message(value);
        self.run();
    }

    pub fn game_over(&self) -> bool {
        self.program.has_halted()
    }

    pub fn get_score(&self) -> i64 {
        self.score
    }

    pub fn cmp_paddle_and_ball(&self) -> Ordering {
        self.ball.0.cmp(&self.paddle.0)
    }

    fn run(&mut self) {
        // run the program
        self.program.run();

        // collect the output
        let mut tiles_map: HashMap<(usize, usize), Tile> = HashMap::new();
        let mut width = 0;
        let mut height = 0;
        while let (Some(x), Some(y), Some(id)) = (
            self.program.receive_message(),
            self.program.receive_message(),
            self.program.receive_message(),
        ) {
            if x == -1 && y == 0 {
                self.score = id;
            } else {
                let tile = match id {
                    0 => Tile::Empty,
                    1 => Tile::Wall,
                    2 => Tile::Block,
                    3 => {
                        self.paddle = (x, y);
                        Tile::Paddle
                    }
                    4 => {
                        self.ball = (x, y);
                        Tile::Ball
                    }
                    _ => unreachable!(),
                };
                let x = x as usize;
                let y = y as usize;
                tiles_map.insert((x, y), tile);
                width = width.max(x + 1);
                height = height.max(y + 1);
            }
        }

        if self.screen.rows() == 1 && self.screen.cols() == 1 {
            // arrange the tiles in a grid
            let mut tiles = vec![];
            for y in 0..height {
                for x in 0..width {
                    tiles.push(*tiles_map.get(&(x, y)).unwrap());
                }
            }
            self.screen = Grid::from_vec(tiles, width);
        } else {
            for y in 0..height {
                for x in 0..width {
                    if let Some(tile) = tiles_map.get(&(x, y)) {
                        self.screen[y][x] = *tile;
                    }
                }
            }
        }
    }

    fn fmt_hr(&self, f: &mut Formatter<'_>, screen_width: usize) -> fmt::Result {
        write!(f, "+-{:-<width$}-+\r\n", "", width = screen_width)
    }

    fn fmt_segment_display(&self, f: &mut Formatter<'_>, screen_width: usize) -> fmt::Result {
        write!(
            f,
            "| {:^width$} |\r\n",
            format!("[score: {:08}]", self.score),
            width = screen_width
        )?;
        write!(f, "| {:^width$} |\r\n", "", width = screen_width)?;
        write!(
            f,
            "| {:^width$} |\r\n",
            format!("{} blocks remaining", self.remaining_blocks()),
            width = screen_width
        )
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let screen_width = 2 * self.screen.cols();
        self.fmt_hr(f, screen_width)?;
        for y in 0..self.screen.rows() {
            write!(f, "| ")?;
            for x in 0..self.screen.cols() {
                match self.screen[y][x] {
                    Tile::Empty => write!(f, "  ")?,
                    Tile::Wall => write!(f, "##")?,
                    Tile::Block => write!(f, "[]")?,
                    Tile::Paddle => write!(f, "==")?,
                    Tile::Ball => write!(f, "()")?,
                }
            }
            write!(f, " |\r\n")?;
        }
        self.fmt_hr(f, screen_width)?;
        self.fmt_segment_display(f, screen_width)?;
        self.fmt_hr(f, screen_width)?;

        Ok(())
    }
}
