use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::machine::Machine;

use shared::device::hook::Hook;
use shared::device::parse_instructions;

fn main() {
    let args = get_args(&["<input file>", "<start value>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let (instruction_pointer, instructions) = parse_instructions(file).or_exit_with(1);
    let start_value = args[2].parse().or_exit_with(1);

    let mut pre_execute_hook = Hook::new(instruction_pointer);
    // pre_execute_hook.enable_debugging();
    // pre_execute_hook.add_log_points(&[28]);

    let mut machine = Machine::new_simple_machine(&instructions);
    machine.register.write('a', start_value);
    machine.run(&mut pre_execute_hook);
    println!(
        "The program halted after executing {} instructions. Its registers are:\n{}",
        pre_execute_hook.get_instructions_count(),
        machine.register
    );
}
