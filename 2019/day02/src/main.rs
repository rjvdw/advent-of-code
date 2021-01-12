use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use shared::intcode;
use shared::intcode::program_parse_error::ProgramParseError;
use shared::intcode::Program;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let program = parse_input(file).or_exit_with(1);

    println!("The value at position 0 is {}.", run(&program, 12, 2));
    match find_correct_inputs(&program, 19690720) {
        Some((noun, verb)) => {
            println!(
                "The desired output is produced with noun={} and verb={}. This means your answer is {}.",
                noun, verb, 100 * noun + verb
            );
        }
        None => {
            eprintln!("Could not find a valid noun and verb to produce the desired output.");
        }
    }
}

fn run(program: &Program, noun: usize, verb: usize) -> usize {
    let mut program = program.clone();
    program[1] = noun;
    program[2] = verb;
    program.run();
    program[0]
}

fn find_correct_inputs(program: &Program, output: usize) -> Option<(usize, usize)> {
    for noun in 0..100 {
        for verb in 0..100 {
            if run(program, noun, verb) == output {
                return Some((noun, verb));
            }
        }
    }
    None
}

fn parse_input<R: Read>(r: R) -> Result<intcode::Program, ProgramParseError> {
    match BufReader::new(r).lines().next() {
        Some(Ok(line)) => line.parse(),
        _ => Err(ProgramParseError::EmptyProgram),
    }
}
