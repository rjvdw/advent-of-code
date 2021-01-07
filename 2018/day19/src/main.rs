use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::WithOrExit;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::machine::Machine;

use shared::device::hook::Hook;
use shared::device::instruction::reg;
use shared::device::parse_instructions;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let file = File::open(&args[1]).or_exit_with(1);
    let (instruction_pointer, instructions) = parse_instructions(file).or_exit_with(1);

    let mut pre_execute_hook = Hook::new(instruction_pointer);
    pre_execute_hook.enable_day19_optimization();

    let mut machine = Machine::new_simple_machine(&instructions);
    machine.run(&mut pre_execute_hook);
    println!(
        "The value in register 0 is {}.",
        machine.register.read(reg(0))
    );

    let mut machine = Machine::new_simple_machine(&instructions);
    machine.register.write(reg(0), 1);
    machine.run(&mut pre_execute_hook);
    println!(
        "The value in register 0 is {}.",
        machine.register.read(reg(0))
    );
}

#[cfg(test)]
mod tests {
    use shared::device::instruction::Instruction;

    use super::*;

    #[test]
    fn test_program() {
        let instruction_pointer = 0;
        let instructions = vec![
            Instruction::Seti(5, 0, 1),
            Instruction::Seti(6, 0, 2),
            Instruction::Addi(0, 1, 0),
            Instruction::Addr(1, 2, 3),
            Instruction::Setr(1, 0, 0),
            Instruction::Seti(8, 0, 4),
            Instruction::Seti(9, 0, 5),
        ];
        let mut pre_execute_hook = Hook::new(instruction_pointer);
        let mut machine = Machine::new_simple_machine(&instructions);
        machine.run(&mut pre_execute_hook);
        assert_eq!(format!("{}", machine.register), "[a=7, b=5, c=6, f=9]");
    }
}
