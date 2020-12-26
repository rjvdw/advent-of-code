extern crate rdcl_aoc_helpers;

use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;

use instruction::Instruction;

use crate::program_fixer::ProgramFixer;

mod instruction;
mod program_fixer;

/// The state of the program, consisting of an accumulator and an index pointer.
pub type ProgramState = (i32, usize);

/// The end state of the program, consisting of a boolean indicating whether the program terminated
/// correctly, and the final value of the accumulator.
pub type ProgramEndState = (bool, i32);

/// https://adventofcode.com/2020/day/8
fn main() {
    let args = get_args(&["<input file>"], 1);

    let instructions = File::open(&args[1])
        .read_lines(1)
        .collect::<Vec<Instruction>>();

    let (terminated, acc) = run_program(&instructions);
    if terminated {
        println!("Program terminated correctly. Final value is {}", acc);
    } else {
        println!(
            "Program did not terminate correctly. Final value is {}",
            acc
        );
        let fixer = ProgramFixer::new(&instructions, run_program);
        for (i, acc) in fixer {
            println!(
                "Terminated correctly by altering instruction at line {}, final value is {}",
                i + 1,
                acc
            );
        }
    }
}

fn run_program(instructions: &[Instruction]) -> ProgramEndState {
    let mut seen: Vec<bool> = Vec::with_capacity(instructions.len());
    let mut state = (0, 0);
    seen.resize(instructions.len(), false);

    while state.1 < instructions.len() {
        if seen[state.1] {
            return (false, state.0);
        }
        seen[state.1] = true;
        state = instructions[state.1].run(state);
    }

    (true, state.0)
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_run() {
        let values = vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ]
        .as_records::<Instruction>()
        .unwrap();

        assert_eq!(run_program(&values), (false, 5));
    }

    #[test]
    fn test_fix() {
        let values = vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ]
        .as_records::<Instruction>()
        .unwrap();

        let fixer = ProgramFixer::new(&values, run_program);
        let fixes: Vec<(usize, i32)> = fixer.collect();

        assert_eq!(fixes.len(), 1);
        assert_eq!(fixes[0], (7, 8));
    }
}
