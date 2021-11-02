use std::fs::File;
use std::io;
use std::io::BufRead;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use termion::{clear, cursor};

use shared::intcode::parse::parse_input;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let input_file = File::open(&args[1]).or_exit_with(1);
    let mut program = parse_input(input_file).or_exit_with(1);

    program.run();
    print!("{}{}", clear::All, cursor::Goto(1, 1));
    print!("{}", program.receive_ascii());

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.unwrap();
        program.load_ascii(&line);
        program.send_message(10);
        program.run();
        print!("{}{}", clear::All, cursor::Goto(1, 1));
        print!("{}", program.receive_ascii());
    }
}
