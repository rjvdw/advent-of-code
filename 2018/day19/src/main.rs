use std::fs::File;
use std::io::{BufRead, BufReader};

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::error::{ParseError, WithOrExit};
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::machine::Machine;

use shared::device::instruction::{reg, Instruction};

use crate::hook::Hook;

mod hook;

fn main() {
    let args = get_args(&["<input file>"], 1);
    let (instruction_pointer, instructions) = parse_input(&args[1]).or_exit_with(1);

    let mut machine = Machine::new_simple_machine(&instructions);
    machine.run(&mut Hook::new(instruction_pointer));
    println!(
        "The value in register 0 is {}.",
        machine.register.read(reg(0))
    );

    let mut machine = Machine::new_simple_machine(&instructions);
    machine.register.write(reg(0), 1);
    machine.run(&mut Hook::new(instruction_pointer));
    println!(
        "The value in register 0 is {}.",
        machine.register.read(reg(0))
    );
}

fn parse_input(path: &str) -> Result<(i64, Vec<Instruction>), ParseError> {
    let file = File::open(path)?;
    let mut lines = BufReader::new(file).lines();
    if let Some(Ok(line)) = lines.next() {
        if let Some(instruction_pointer) = line.strip_prefix("#ip ") {
            let mut instructions = Vec::new();
            for line in lines {
                let line = line?;
                instructions.push(line.parse()?);
            }
            Ok((instruction_pointer.parse()?, instructions))
        } else {
            Err(ParseError(format!("Invalid instruction pointer: {}", line)))
        }
    } else {
        Err(ParseError::of("Empty input file"))
    }
}

#[cfg(test)]
mod tests {
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
        let mut machine = Machine::new_simple_machine(&instructions);
        machine.run(&mut Hook::new(instruction_pointer));
        assert_eq!(format!("{}", machine.register), "[a=7, b=5, c=6, f=9]");
    }
}
