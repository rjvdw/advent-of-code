use std::fs::File;

use rdcl_aoc_helpers::args::get_args;
use rdcl_aoc_helpers::input::WithReadLines;
use rdcl_aoc_helpers::machine::register::MachineRegister;
use rdcl_aoc_helpers::machine::Machine;

use crate::instruction::Instruction;
use crate::loop_detector::LoopDetector;
use crate::mutator::Mutator;

mod instruction;
mod loop_detector;
mod mutator;

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
        for (i, acc) in fix_program(&instructions) {
            println!(
                "Terminated correctly by altering instruction at line {}, final value is {}",
                i + 1,
                acc
            );
        }
    }
}

fn run_program(instructions: &[Instruction]) -> (bool, i64) {
    let mut machine = Machine::new_simple_machine(instructions);
    let mut loop_detector = LoopDetector::new(instructions.len());

    machine.run(&mut loop_detector);

    (
        loop_detector.terminated_normally(),
        machine.register.read('a'),
    )
}

fn fix_program(instructions: &[Instruction]) -> Vec<(usize, i64)> {
    let fixer = Mutator::new(instructions);
    let mut valid_programs = Vec::new();
    for (idx, mut machine) in fixer {
        let mut loop_detector = LoopDetector::new(instructions.len());
        machine.run(&mut loop_detector);

        if loop_detector.terminated_normally() {
            valid_programs.push((idx, machine.register.read('a')));
        }
    }
    valid_programs
}

#[cfg(test)]
mod tests {
    use rdcl_aoc_helpers::input::WithAsRecords;

    use super::*;

    #[test]
    fn test_run() {
        let instructions = vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ]
        .as_records::<Instruction>()
        .unwrap();

        assert_eq!(run_program(&instructions), (false, 5));
    }

    #[test]
    fn test_fix() {
        let instructions = vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ]
        .as_records::<Instruction>()
        .unwrap();

        let fixes: Vec<(usize, i64)> = fix_program(&instructions);

        assert_eq!(fixes.len(), 1);
        assert_eq!(fixes[0], (7, 8));
    }
}
