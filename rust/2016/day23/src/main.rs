use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::input::WithReadLines;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::machine::Machine;

use shared::instruction::Instruction;
use shared::program::Hook;

fn main() {
    let args = get_args(&["<input file>", "<plain eggs>", "<colored eggs>"], 1);
    let instructions = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Instruction>>();
    let plain_eggs = args[2].parse().or_exit_with(1);
    let colored_eggs = args[3].parse().or_exit_with(1);

    let mut machine = Machine::new_simple_machine(&instructions);
    machine.register.write('a', plain_eggs);
    machine.run(&mut Hook);
    println!(
        "Starting with {}, the value at register a is: {}",
        plain_eggs,
        machine.register.read('a')
    );

    let mut machine = Machine::new_simple_machine(&instructions);
    machine.register.write('a', colored_eggs);
    machine.run(&mut Hook);
    println!(
        "Starting with {}, the value at register a is: {}",
        colored_eggs,
        machine.register.read('a')
    );
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_execute() {
        let instructions = vec![
            "cpy 2 a", "tgl a", "tgl a", "tgl a", "cpy 1 a", "dec a", "dec a",
        ]
        .as_records::<Instruction>()
        .unwrap();

        let mut machine = Machine::new_simple_machine(&instructions);
        machine.register.write('a', 7);
        machine.run(&mut Hook);

        assert_eq!(machine.register.read('a'), 3);
    }
}
