use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;
use rdcl_aoc_helpers::machine::hook::NoopHook;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::machine::Machine;

use crate::instruction::Instruction;

mod instruction;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let instructions = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Instruction>>();

    let (reg_a, reg_b) = run_program(&instructions, 0, 0);
    println!(
        "After running the program (a=0, b=0), the registers contain: a={}, b={}",
        reg_a, reg_b
    );

    let (reg_a, reg_b) = run_program(&instructions, 1, 0);
    println!(
        "After running the program (a=1, b=0), the registers contain: a={}, b={}",
        reg_a, reg_b
    );
}

fn run_program(instructions: &[Instruction], reg_a: i64, reg_b: i64) -> (i64, i64) {
    let mut machine = Machine::new_simple_machine(instructions);
    machine.register.write('a', reg_a);
    machine.register.write('b', reg_b);

    machine.run(&mut NoopHook::default());

    (machine.register.read('a'), machine.register.read('b'))
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::machine::instruction::Value;

    use super::*;

    #[test]
    fn test_run_program_0_0() {
        let instructions = vec![
            Instruction::Increment('a'),
            Instruction::JumpIfOne(Value::Register('a'), 2),
            Instruction::Triple('a'),
            Instruction::Increment('a'),
        ];

        assert_eq!(run_program(&instructions, 0, 0), (2, 0));
    }

    #[test]
    fn test_run_program_1_0() {
        let instructions = vec![
            Instruction::Increment('a'),
            Instruction::JumpIfOne(Value::Register('a'), 2),
            Instruction::Triple('a'),
            Instruction::Increment('a'),
        ];

        assert_eq!(run_program(&instructions, 1, 0), (7, 0));
    }
}
