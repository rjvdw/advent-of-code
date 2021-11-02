use std::fs;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use shared::intcode::parse::parse_input;

fn main() {
    let args = get_args(&["<input file>", "<solution walk>", "<solution run>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let program = parse_input(file).or_exit_with(1);

    let solution_walk = fs::read_to_string(&args[2]).or_exit_with(1);
    let mut program_walk = program.clone();
    program_walk.load_ascii(&solution_walk);
    program_walk.run();
    println!("{}", program_walk.dump_ascii());
    match program_walk.output_dump().last() {
        Some(damage) if *damage > 255 => println!("The hull damage is: {}", damage),
        _ => {}
    }

    let solution_run = fs::read_to_string(&args[3]).or_exit_with(1);
    let mut program_run = program;
    program_run.load_ascii(&solution_run);
    program_run.run();
    println!("{}", program_run.dump_ascii());
    match program_run.output_dump().last() {
        Some(damage) if *damage > 255 => println!("The hull damage is: {}", damage),
        _ => {}
    }
}
