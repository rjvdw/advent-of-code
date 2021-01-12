use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;

use shared::intcode;
use shared::intcode::parse::parse_input;

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

fn run(program: &intcode::Program, noun: i64, verb: i64) -> i64 {
    let mut program = program.clone();
    program[1] = noun;
    program[2] = verb;
    program.run();
    program[0]
}

fn find_correct_inputs(program: &intcode::Program, output: i64) -> Option<(i64, i64)> {
    for noun in 0..100 {
        for verb in 0..100 {
            if run(program, noun, verb) == output {
                return Some((noun, verb));
            }
        }
    }
    None
}
