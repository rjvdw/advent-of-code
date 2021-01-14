use std::cmp::Ordering;
use std::fs::File;
use std::thread;
use std::time::Duration;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use termion::{clear, cursor};

use arcade::Game;
use shared::intcode::parse::parse_input;

fn main() {
    let args = get_args(&["<input file>", "<speed>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let program = parse_input(file).or_exit_with(1);
    let speed = args[2].parse::<u64>().or_exit_with(1);

    let time = Duration::from_millis(speed);

    let mut game = Game::from_program(program, Some(2));

    while !game.game_over() {
        paint(&game);
        match game.cmp_paddle_and_ball() {
            Ordering::Less => game.input(-1),
            Ordering::Equal => game.input(0),
            Ordering::Greater => game.input(1),
        }
        thread::sleep(time);
    }
    paint(&game);
}

fn paint(game: &Game) {
    println!("{}{}{}", clear::All, cursor::Goto(1, 1), game);
}
