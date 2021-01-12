use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadMultiLines;

use turing_machine::turing_machine::TuringMachine;
use turing_machine::turing_machine_parser::TuringMachineParser;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let turing_machine = File::open(&args[1])
        .read_multi_lines::<TuringMachineParser>(1)
        .next()
        .or_exit_with(1)
        .get();

    println!("The diagnostic checksum is {}.", run(&turing_machine));
}

fn run(turing_machine: &TuringMachine) -> usize {
    let mut turing_machine = turing_machine.clone();
    let nr_steps = turing_machine.get_diagnostic_checksum_after();

    for _ in 1..=nr_steps {
        turing_machine.step();
    }

    turing_machine.checksum()
}
