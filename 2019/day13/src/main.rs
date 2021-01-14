use std::cmp::Ordering;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use arcade::Game;
use shared::intcode;
use shared::intcode::parse::parse_input;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let program = parse_input(file).or_exit_with(1);
    println!("{}", Game::from_program(program.clone(), None));
    println!(
        "If you beat the game, your score will be {}.",
        beat_game(program)
    )
}

fn beat_game(program: intcode::Program) -> i64 {
    let mut game = Game::from_program(program, Some(2));
    while !game.game_over() {
        match game.cmp_paddle_and_ball() {
            Ordering::Less => game.input(-1),
            Ordering::Equal => game.input(0),
            Ordering::Greater => game.input(1),
        }
    }
    game.get_score()
}
