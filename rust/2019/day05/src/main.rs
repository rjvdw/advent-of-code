use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use shared::intcode::parse::parse_input;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let program = parse_input(file).or_exit_with(1);

    let mut ac_program = program.clone();
    ac_program.send_message(1);
    println!("Running diagnostics for the air conditioning...");
    ac_program.run();
    while let Some(output) = ac_program.receive_message() {
        println!("  -> {}", output);
    }

    let mut tr_program = program;
    tr_program.send_message(5);
    tr_program.run();
    println!("Running diagnostics for the thermal radiator controller...");
    while let Some(output) = tr_program.receive_message() {
        println!("  -> {}", output);
    }
}
