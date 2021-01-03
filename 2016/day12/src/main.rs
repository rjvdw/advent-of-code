use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::machine::Machine;

use shared::instruction::Instruction;
use shared::program::Hook;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let instructions = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Instruction>>();

    let mut machine = Machine::new_simple_machine(&instructions);
    machine.run(&mut Hook);
    println!("The value at register a is: {}", machine.register.read('a'));

    let mut machine = Machine::new_simple_machine(&instructions);
    machine.register.write('c', 1);
    machine.run(&mut Hook);
    println!(
        "If we first set register c to 1, then the value at register a is: {}",
        machine.register.read('a')
    );
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_execute() {
        let instructions = vec!["cpy 41 a", "inc a", "inc a", "dec a", "jnz a 2", "dec a"]
            .as_records::<Instruction>()
            .unwrap();

        let mut machine = Machine::new_simple_machine(&instructions);
        machine.run(&mut Hook);

        assert_eq!(machine.register.read('a'), 42);
    }
}
