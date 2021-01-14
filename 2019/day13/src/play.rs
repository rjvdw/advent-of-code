use std::fs::File;
use std::io::{stdin, stdout, Stdout, Write};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, cursor};

use arcade::Game;
use shared::intcode::parse::parse_input;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let program = parse_input(file).or_exit_with(1);

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut game = Game::from_program(program, Some(2));

    paint(&mut stdout, &game);
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => {
                break;
            }
            Key::Left => {
                game.input(-1);
            }
            Key::Up => {
                game.input(0);
            }
            Key::Right => {
                game.input(1);
            }
            _ => {}
        }

        paint(&mut stdout, &game);

        if game.game_over() {
            break;
        }
    }
}

fn paint(stdout: &mut RawTerminal<Stdout>, game: &Game) {
    write!(stdout, "{}{}{}", clear::All, cursor::Goto(1, 1), game).unwrap();
    write!(
        stdout,
        "\r\nControls: arrow keys to move the paddle, ctrl+C to stop.\r\n"
    )
    .unwrap();
    stdout.flush().unwrap();
}
