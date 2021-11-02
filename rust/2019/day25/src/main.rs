use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use shared::intcode::parse::parse_input;

fn main() {
    let args = get_args(&["<input file>", "<solution>"], 1);
    let input_file = File::open(&args[1]).or_exit_with(1);
    let mut program = parse_input(input_file).or_exit_with(1);

    program.run();
    print!("{}", program.receive_ascii());

    let solution_file = File::open(&args[2]).or_exit_with(1);
    for line in BufReader::new(solution_file).lines() {
        let line = line.or_exit_with(1);
        println!("{}", line);
        program.load_ascii(&line);
        program.send_message(10);
        program.run();
        print!("{}", program.receive_ascii());
    }
}
