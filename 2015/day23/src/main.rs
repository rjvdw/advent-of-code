use std::collections::HashMap;
use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

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

fn run_program(instructions: &[Instruction], reg_a: u64, reg_b: u64) -> (u64, u64) {
    let mut registers: HashMap<char, u64> = HashMap::new();
    registers.insert('a', reg_a);
    registers.insert('b', reg_b);
    let mut idx: isize = 0;
    let ilen = instructions.len() as isize;

    while idx >= 0 && idx < ilen {
        idx = instructions[idx as usize].execute(idx, &mut registers);
    }

    (
        *registers.get(&'a').unwrap_or(&0),
        *registers.get(&'b').unwrap_or(&0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_program_0_0() {
        let instructions = vec![
            Instruction::Increment('a'),
            Instruction::JumpIfOne('a', 2),
            Instruction::Triple('a'),
            Instruction::Increment('a'),
        ];

        assert_eq!(run_program(&instructions, 0, 0), (2, 0));
    }

    #[test]
    fn test_run_program_1_0() {
        let instructions = vec![
            Instruction::Increment('a'),
            Instruction::JumpIfOne('a', 2),
            Instruction::Triple('a'),
            Instruction::Increment('a'),
        ];

        assert_eq!(run_program(&instructions, 1, 0), (7, 0));
    }
}
